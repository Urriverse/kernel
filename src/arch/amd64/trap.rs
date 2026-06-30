//! # Trap Frame (x86_64)
//!
//! This module defines the `TrapFrame` structure, which captures the CPU state
//! at the moment of an interrupt, exception, or system call. It is used by the
//! interrupt handlers, the scheduler, and the syscall dispatcher to save and
//! restore the context of a task.
//!
//! ## Overview
//!
//! When an interrupt or exception occurs, the CPU automatically pushes certain
//! registers onto the stack (the "hardware‑saved" part). The kernel's interrupt
//! handlers then push the remaining general‑purpose registers (the "software‑
//! saved" part) to form a complete `TrapFrame`. The same structure is used for
//! system calls, where the `syscall_entry` wrapper builds a trap frame manually.
//!
//! ## Layout
//!
//! The `TrapFrame` is laid out as follows:
//!
//! ```text
//! +------------------+  <-- lower addresses (top of stack)
//! | rax              |  software‑saved (15 registers)
//! | rbx              |
//! | rcx              |
//! | rdx              |
//! | rsi              |
//! | rdi              |
//! | rbp              |
//! | r8               |
//! | r9               |
//! | r10              |
//! | r11              |
//! | r12              |
//! | r13              |
//! | r14              |
//! | r15              |
//! +------------------+
//! | rip              |  hardware‑saved (5 registers)
//! | cs               |
//! | rflags           |
//! | rsp              |
//! | ss               |
//! +------------------+  <-- higher addresses (original stack)
//! ```
//!
//! The hardware‑saved part is pushed by the CPU automatically:
//! - On interrupts/exceptions: the CPU pushes `SS`, `RSP`, `RFLAGS`, `CS`, `RIP`
//!   (and sometimes an error code, which is handled separately).
//! - On `syscall`: the CPU does not push these; the wrapper builds them manually.
//!
//! The software‑saved part is pushed by the handler (or wrapper) to preserve all
//! general‑purpose registers that may be clobbered.
//!
//! ## Usage
//!
//! - **Interrupt handlers**: The `timer_wrapper` and `yield_wrapper` functions
//!   save the context into a `TrapFrame` and pass it to the scheduler.
//! - **Syscall dispatcher**: The `syscall_entry` wrapper builds a trap frame
//!   and passes it to `sched::syscall_dispatcher`.
//! - **Scheduler**: During context switching, the scheduler saves the current
//!   task's trap frame and loads the next task's trap frame.
//! - **Exception handlers**: The IDT exception handlers receive a similar
//!   frame (via the `x86-interrupt` ABI) and may convert it to a `TrapFrame`
//!   or use it directly.
//!
//! ## Safety
//!
//! The `TrapFrame` is `repr(C)` and is accessed via raw pointers in assembly
//! and in the scheduler. The layout must match the assembly code exactly.
//! Changing the order or size of fields will break the interrupt handlers
//! and context switching.

// ============================================================================
// TRAP FRAME STRUCTURE
// ============================================================================

/// A complete snapshot of the CPU state at the time of an interrupt, exception,
/// or system call.
///
/// This struct is `repr(C)` to guarantee a stable layout, matching the
/// assembly code that builds and restores the frame.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapFrame {
    // ========================================================================
    // Software‑saved registers (pushed by the kernel)
    // ========================================================================

    /// RAX – general‑purpose register, often used as the syscall number and
    /// return value.
    pub rax: u64,
    /// RBX – general‑purpose register, saved but not used for syscalls.
    pub rbx: u64,
    /// RCX – general‑purpose register; on `syscall` entry, it holds the user
    /// return address (`RIP`).
    pub rcx: u64,
    /// RDX – general‑purpose register, often used as the third syscall argument.
    pub rdx: u64,
    /// RSI – general‑purpose register, used as the second syscall argument.
    pub rsi: u64,
    /// RDI – general‑purpose register, used as the first syscall argument.
    pub rdi: u64,
    /// RBP – base pointer, saved for stack frame debugging.
    pub rbp: u64,
    /// R8 – general‑purpose register, used as the fifth syscall argument.
    pub r8: u64,
    /// R9 – general‑purpose register, used as the sixth syscall argument.
    pub r9: u64,
    /// R10 – general‑purpose register, used as the fourth syscall argument
    /// (the `syscall` instruction clobbers RCX and R11, so R10 is used instead
    /// of RCX for the fourth argument).
    pub r10: u64,
    /// R11 – general‑purpose register; on `syscall` entry, it holds the user
    /// `RFLAGS`.
    pub r11: u64,
    /// R12 – general‑purpose register, callee‑saved.
    pub r12: u64,
    /// R13 – general‑purpose register, callee‑saved.
    pub r13: u64,
    /// R14 – general‑purpose register, callee‑saved.
    pub r14: u64,
    /// R15 – general‑purpose register, callee‑saved.
    pub r15: u64,

    // pub gs: u64,

    // ========================================================================
    // Hardware‑saved registers (pushed by the CPU or manually constructed)
    // ========================================================================

    /// Instruction pointer – the address to return to after the interrupt.
    pub rip: u64,
    /// Code segment selector (with RPL) – indicates the privilege level of
    /// the interrupted context (e.g., `0x08 | 0` for kernel, `0x18 | 3` for user).
    pub cs: u64,
    /// RFLAGS register – contains CPU flags (interrupt flag, direction flag, etc.).
    pub rflags: u64,
    /// Stack pointer – the user or kernel stack pointer at the time of the
    /// interrupt.
    pub rsp: u64,
    /// Stack segment selector – used with `RSP` to form the full stack address.
    pub ss: u64,
}
