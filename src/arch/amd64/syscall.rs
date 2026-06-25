//! # System Call Entry (x86_64)
//!
//! This module implements the low‑level system call entry point for the x86_64
//! architecture, using the `syscall` instruction. It sets up the necessary MSRs
//! to enable fast system calls from user mode and provides the assembly wrapper
//! that switches to the kernel stack, saves the context, and dispatches the
//! system call to the scheduler.
//!
//! ## Overview
//!
//! On x86_64, the `syscall` instruction is the preferred mechanism for making
//! system calls from user space (Ring 3) to the kernel (Ring 0). It provides a
//! fast, low‑overhead entry point that is supported on all modern CPUs.
//!
//! The `syscall` instruction:
//! - Loads `RIP` from `IA32_LSTAR` MSR.
//! - Loads `CS` and `SS` from `IA32_STAR` MSR.
//! - Switches to Ring 0.
//! - Saves the return address in `RCX` and the old RFLAGS in `R11`.
//!
//! ## MSR Configuration
//!
//! The following MSRs are configured during `syscall::init()`:
//!
//! - **`IA32_EFER`** (0xC0000080): The `SCE` (System Call Extensions) bit is set
//!   to enable the `syscall` instruction in 64‑bit mode.
//! - **`IA32_STAR`** (0xC0000081): The upper 32 bits hold the kernel CS/SS
//!   selectors; the lower 32 bits hold the user CS/SS selectors.
//!   - Kernel CS = `0x08`, Kernel SS = `0x10`
//!   - User CS = `0x18`, User SS = `0x20`
//! - **`IA32_LSTAR`** (0xC0000082): Holds the address of the syscall entry point
//!   (`syscall_entry`).
//! - **`IA32_FMASK`** (0xC0000084): Sets the RFLAGS mask applied during `syscall`;
//!   we mask out bits 8 and 9 (TF and IF) to disable single‑step and interrupts.
//!
//! ## Syscall Entry Wrapper (`syscall_entry`)
//!
//! The entry point is a **naked** function written in inline assembly. It:
//! 1. **Swaps GS** (via `swapgs`) to switch from the user `gs` base to the
//!    kernel `gs` base (which points to per‑CPU data).
//! 2. **Saves the user stack pointer** (`RSP`) and switches to the kernel stack
//!    (stored in `gs:[8]`).
//! 3. **Saves all general‑purpose registers** on the kernel stack, building a
//!    `TrapFrame` that can be passed to the dispatcher.
//! 4. **Calls the syscall dispatcher** (`sched::syscall_dispatcher`) with a
//!    pointer to the trap frame.
//! 5. **Restores all registers** (except `RCX` and `R11`, which are used by
//!    `sysret` to restore the user RIP and RFLAGS).
//! 6. **Swaps GS back** and executes `sysret` to return to user mode.
//!
//! ## Syscall Dispatcher
//!
//! The dispatcher is implemented in the scheduler module (`sched::syscall_dispatcher`).
//! It retrieves the current process and delegates to its `syscall_handler` function,
//! which interprets the syscall number and arguments.
//!
//! ## Safety
//!
//! - The `syscall_entry` function is a naked function that manipulates the stack
//!   and registers directly. It must be written with extreme care to avoid
//!   corrupting the CPU state.
//! - The `swapgs` instruction is used to switch between user and kernel GS.
//!   It must be paired correctly with `swapgs` on the return path.
//! - The kernel stack pointer is read from `gs:[8]`, which is set up by the
//!   scheduler when switching tasks.
//! - The MSR writes (`wrmsr`) are privileged operations that require the kernel
//!   to be running in Ring 0.

use core::arch::naked_asm;
use crate::arch::{rdmsr, wrmsr};

// ============================================================================
// MSR CONSTANTS
// ============================================================================

/// Extended Feature Enable Register (EFER) – enables `syscall` in 64‑bit mode.
pub const IA32_EFER: u32 = 0xC0000080;

/// System Call Target Address Register (STAR) – holds the CS/SS selectors.
/// - Upper 32 bits: Kernel CS (bits 32‑47) and Kernel SS (bits 48‑63).
/// - Lower 32 bits: User CS (bits 0‑15) and User SS (bits 16‑31).
pub const IA32_STAR: u32 = 0xC0000081;

/// System Call Target Address Register (LSTAR) – holds the RIP of the syscall handler.
pub const IA32_LSTAR: u32 = 0xC0000082;

/// System Call Flag Mask (FMASK) – masks RFLAGS bits during `syscall`.
/// We mask out TF (bit 8) and IF (bit 9) to disable single‑step and interrupts.
pub const IA32_FMASK: u32 = 0xC0000084;

// ============================================================================
// SYSCALL ENTRY (NAKED FUNCTION)
// ============================================================================

/// The entry point for all system calls.
///
/// This function is called by the CPU via the `syscall` instruction. It is a
/// naked function written in inline assembly that:
/// 1. Switches GS from user to kernel.
/// 2. Saves the user stack pointer and switches to the kernel stack.
/// 3. Saves all registers on the kernel stack (building a trap frame).
/// 4. Calls the system call dispatcher (`sched::syscall_dispatcher`).
/// 5. Restores registers and returns to user mode via `sysret`.
///
/// # Registers at Entry
///
/// On entry via `syscall`:
/// - `RCX` = user `RIP` (return address)
/// - `R11` = user `RFLAGS`
/// - `RAX` = system call number
/// - `RDI`, `RSI`, `RDX`, `R10`, `R8`, `R9` = arguments
///
/// # Safety
/// This is a naked function that manipulates the stack and registers directly.
/// It must not be called directly; it is only invoked by the CPU via the
/// `syscall` instruction.
#[unsafe(naked)]
pub unsafe extern "C" fn syscall_entry() -> ! {
    naked_asm!(
        // --------------------------------------------------------------------
        // 1. Switch from user GS to kernel GS (per‑CPU data).
        // --------------------------------------------------------------------
        "swapgs",

        // --------------------------------------------------------------------
        // 2. Save the user stack pointer and switch to the kernel stack.
        //    The kernel stack top is stored in gs:[8] (offset 8 in PerCpu).
        // --------------------------------------------------------------------
        "mov rbx, rsp",             // Save user RSP in RBX (will be saved later).
        "mov rsp, gs:[8]",          // Load kernel stack top from per‑CPU data.

        // --------------------------------------------------------------------
        // 3. Allocate space for the trap frame on the kernel stack.
        //    The trap frame layout matches the TrapFrame struct:
        //    RAX, RBX, RCX, RDX, RSI, RDI, RBP, R8, R9, R10, R11, R12, R13, R14, R15,
        //    then the hardware‑saved part: RIP, CS, RFLAGS, RSP, SS.
        // --------------------------------------------------------------------
        "sub rsp, 160",             // 15 general‑purpose registers (8 bytes each) = 120 bytes,
                                    // plus 5 hardware fields (40 bytes) = 160 bytes total.

        // Save all general‑purpose registers to the trap frame.
        "mov [rsp + 0], rax",
        "mov [rsp + 8], rbx",       // User RSP (saved earlier).
        "mov [rsp + 16], rcx",      // User RIP (from syscall).
        "mov [rsp + 24], rdx",
        "mov [rsp + 32], rsi",
        "mov [rsp + 40], rdi",
        "mov [rsp + 48], rbp",
        "mov [rsp + 56], r8",
        "mov [rsp + 64], r9",
        "mov [rsp + 72], r10",
        "mov [rsp + 80], r11",      // User RFLAGS (from syscall).
        "mov [rsp + 88], r12",
        "mov [rsp + 96], r13",
        "mov [rsp + 104], r14",
        "mov [rsp + 112], r15",

        // Save the hardware‑saved fields of the trap frame.
        "mov [rsp + 120], rcx",     // RIP (from syscall).
        "mov word ptr [rsp + 128], 0x18",    // CS (user code selector + RPL 3).
        "mov [rsp + 136], r11",     // RFLAGS (from syscall).
        "mov [rsp + 144], rbx",     // RSP (user stack).
        "mov word ptr [rsp + 152], 0x20",    // SS (user data selector + RPL 3).

        // --------------------------------------------------------------------
        // 4. Call the syscall dispatcher.
        //    RDI = pointer to the trap frame (RSP).
        // --------------------------------------------------------------------
        "mov rdi, rsp",
        "call {syscall_dispatcher}",

        // --------------------------------------------------------------------
        // 5. Restore registers (except RCX and R11, which are restored by sysret).
        // --------------------------------------------------------------------
        "mov rax, [rsp + 0]",
        "mov rbx, [rsp + 8]",
        "mov rdx, [rsp + 24]",
        "mov rsi, [rsp + 32]",
        "mov rdi, [rsp + 40]",
        "mov rbp, [rsp + 48]",
        "mov r8, [rsp + 56]",
        "mov r9, [rsp + 64]",
        "mov r10, [rsp + 72]",
        "mov r12, [rsp + 88]",
        "mov r13, [rsp + 96]",
        "mov r14, [rsp + 104]",
        "mov r15, [rsp + 112]",

        // Restore RCX (user RIP) and R11 (user RFLAGS) for sysret.
        "mov rcx, [rsp + 120]",
        "mov r11, [rsp + 136]",

        // Restore the user stack pointer (RSP) from the trap frame.
        "mov rsp, [rsp + 144]",

        // --------------------------------------------------------------------
        // 6. Switch back to user GS and return via sysret.
        // --------------------------------------------------------------------
        "swapgs",
        "sysret",

        // The dispatcher symbol (defined in the scheduler module).
        syscall_dispatcher = sym crate::sched::syscall_dispatcher,
    );
}

// ============================================================================
// SYSCALL INITIALISATION
// ============================================================================

/// Initialises the system call infrastructure.
///
/// This function sets up the MSRs required for `syscall`:
/// 1. **IA32_EFER**: Sets the `SCE` bit (System Call Extensions) to enable
///    `syscall` in 64‑bit mode.
/// 2. **IA32_STAR**: Sets the CS and SS selectors for both kernel and user mode.
///    - Kernel CS = `0x08` (GDT index 1)
///    - Kernel SS = `0x10` (GDT index 2)
///    - User CS = `0x18` (GDT index 3)
///    - User SS = `0x20` (GDT index 4)
/// 3. **IA32_LSTAR**: Sets the address of `syscall_entry`.
/// 4. **IA32_FMASK**: Masks RFLAGS bits 8 and 9 (TF and IF) to disable
///    single‑step and interrupts during syscall handling.
///
/// # Safety
/// This function uses `wrmsr` to write to privileged MSRs. It is called during
/// early boot with interrupts disabled.
pub fn init() {
    // Enable the System Call Extensions (SCE) bit in EFER.
    let efer = unsafe { rdmsr(IA32_EFER) };
    unsafe { wrmsr(IA32_EFER, efer | 1); }

    // Set STAR: Kernel CS (0x08) at bits 48‑63, Kernel SS at bits 32‑47,
    // User CS (0x18) at bits 16‑31, User SS (0x20) at bits 0‑15.
    let star = (0x08u64 << 48) | (0x08u64 << 32);
    unsafe { wrmsr(IA32_STAR, star); }

    // Set LSTAR to the address of the syscall entry point.
    unsafe { wrmsr(IA32_LSTAR, syscall_entry as *const () as u64); }

    // Set FMASK to mask TF (bit 8) and IF (bit 9).
    // 0x300 = bits 8 and 9 set.
    unsafe { wrmsr(IA32_FMASK, 0x300); }
}
