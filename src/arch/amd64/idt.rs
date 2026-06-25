//! # Interrupt Descriptor Table (IDT) and Exception Handling
//!
//! This module manages the x86_64 Interrupt Descriptor Table (IDT), which defines
//! the handlers for all exceptions and interrupts. It sets up the gate descriptors
//! for CPU exceptions, hardware interrupts (including the timer and IPI), and
//! provides the low‑level interrupt handler functions.
//!
//! ## Overview
//!
//! The IDT is a table of 256 entries, each containing the address of an interrupt
//! handler (ISR), along with privilege and type information. When an interrupt or
//! exception occurs, the CPU looks up the corresponding entry in the IDT and
//! transfers control to the handler.
//!
//! This module defines handlers for:
//! - **CPU Exceptions** (0‑19): Divide error, page fault, double fault, etc.
//! - **Hardware Interrupts**: Timer (vector 32), IPI (vector 128), etc.
//! - **Software Interrupts**: Yield (vector 33), used by the scheduler.
//!
//! ## Handler Types
//!
//! The IDT uses two types of handlers:
//! - **Exception handlers**: Handle CPU‑generated exceptions (page faults, GPF, etc.).
//!   These are defined with the `x86-interrupt` ABI and receive an `InterruptStackFrame`.
//! - **Interrupt handlers**: Handle external interrupts (timer, IPI). These are also
//!   defined with the `x86-interrupt` ABI.
//! - **Naked wrappers**: The timer handler is a naked function that saves the
//!   context and calls the scheduler. This is required because the scheduler
//!   needs to access and modify the trap frame.
//!
//! ## Exception Handling
//!
//! Most exceptions are treated as critical and cause a kernel panic. The handlers
//! log the exception details (error code, RIP, RSP, etc.) and then panic. The
//! following exceptions are handled specially:
//! - **Page Fault**: Delegated to the scheduler's `handle_page_fault` for
//!   demand paging and copy‑on‑write.
//! - **Double Fault**: Uses an Interrupt Stack Table (IST) entry to avoid
//!   stack corruption.
//! - **Breakpoint**: Logged as a warning (useful for debugging).
//! - **Debug**: Logged as a warning.
//!
//! ## Interrupt Handlers
//!
//! - **Timer (vector 32)**: Calls `timer_wrapper`, which saves the context and
//!   invokes `sched::timer_tick` to update vruntime and reschedule.
//! - **Yield (vector 33)**: Calls `yield_wrapper`, which invokes `sched::reschedule`
//!   to perform a voluntary context switch.
//! - **IPI (vector 128)**: Logs the IPI reception and sends an EOI to the APIC.
//!
//! ## Interrupt Stack Table (IST)
//!
//! The double fault handler uses IST entry 1, which provides a dedicated stack
//! for handling double faults. This prevents a stack overflow from corrupting
//! the handler's own stack. Other IST entries are reserved for future use.
//!
//! ## Initialisation
//!
//! - **BSP**: `idt::init_bsp()` is called during `arch::init_bsp()`. It sets up
//!   all handlers and loads the IDT via `lidt`.
//! - **APs**: `idt::init_ap()` is called during `arch::init_ap()`. It loads the
//!   same IDT (the table is shared across all CPUs).
//!
//! ## Safety
//!
//! - The IDT is a `static mut` and is modified during early boot (single‑threaded).
//! - The naked wrapper functions use inline assembly to manipulate the stack and
//!   registers; they are carefully written to preserve the ABI.
//! - The `transmute` calls for the timer and yield handlers are required because
//!   the scheduler functions have a different signature from the IDT handler type.
//!
//! ## Layout
//!
//! | Vector | Description                | Handler                     |
//! |--------|----------------------------|-----------------------------|
//! | 0      | Divide Error               | divide_error_handler        |
//! | 1      | Debug                      | debug_handler               |
//! | 2      | NMI                        | nmi_handler                 |
//! | 3      | Breakpoint                 | breakpoint_handler          |
//! | 4      | Overflow                   | overflow_handler            |
//! | 5      | Bound Range Exceeded       | bound_range_exceeded_handler|
//! | 6      | Invalid Opcode             | invalid_opcode_handler      |
//! | 7      | Device Not Available       | device_not_available_handler|
//! | 8      | Double Fault (IST1)        | double_fault_handler        |
//! | 10     | Invalid TSS                | invalid_tss_handler         |
//! | 11     | Segment Not Present        | segment_not_present_handler |
//! | 12     | Stack Segment Fault        | stack_segment_fault_handler |
//! | 13     | General Protection Fault   | general_protection_fault_handler |
//! | 14     | Page Fault                 | page_fault_handler          |
//! | 16     | x87 FPU Exception          | x87_fpu_exception_handler   |
//! | 17     | Alignment Check            | alignment_check_handler     |
//! | 18     | Machine Check              | machine_check_handler       |
//! | 19     | SIMD Floating Point        | simd_floating_point_handler |
//! | 32     | Timer (APIC)               | timer_wrapper (naked)       |
//! | 33     | Yield (software)           | yield_wrapper (naked)       |
//! | 128    | IPI                        | ipi_handler                 |

use crate::arch::current_cpu;
use x86_64::structures::idt::{
    EntryOptions, InterruptDescriptorTable, InterruptStackFrame,
    PageFaultErrorCode
};
use x86_64::registers::control::Cr2;

// ============================================================================
// CONSTANTS
// ============================================================================

/// IPI (Inter‑Processor Interrupt) vector.
///
/// This vector is used for sending IPIs between CPUs, typically for TLB shootdown
/// or rescheduling requests.
pub const IPI_VECTOR: u8 = 128;

/// Timer interrupt vector (APIC timer).
///
/// The APIC timer is programmed to fire at this vector on each tick (10 ms).
pub const TIMER_VECTOR: u8 = 32;

// ============================================================================
// GLOBAL IDT
// ============================================================================

/// The global Interrupt Descriptor Table.
///
/// This table is shared by all CPU cores. It is `static mut` because it is
/// modified during early boot (single‑threaded) and read‑only thereafter.
pub static mut GLOBAL_IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

// ============================================================================
// HELPER: PRINT TRAP FRAME
// ============================================================================

/// Prints the contents of an `InterruptStackFrame` for debugging.
///
/// This function is called from exception handlers to log the CPU state at
/// the time of the exception.
fn print_frame(frame: &InterruptStackFrame) {
    error!(
        "\n  RIP: {:#018X}\n  CS : {:#08X}\n  RFLAGS: {:#018X}\n  RSP: {:#018X}\n  SS : {:#08X}",
        frame.instruction_pointer.as_u64(),
        frame.code_segment.0,
        frame.cpu_flags,
        frame.stack_pointer.as_u64(),
        frame.stack_segment.0,
    );
}

// ============================================================================
// EXCEPTION HANDLERS
// ============================================================================

/// Divide error (vector 0).
///
/// Occurs when a division by zero or an overflow in division is attempted.
extern "x86-interrupt" fn divide_error_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 0: DIVIDE_ERROR on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Divide Error");
}

/// Debug (vector 1).
///
/// Triggered by the `int3` instruction or single‑step debugging.
extern "x86-interrupt" fn debug_handler(frame: InterruptStackFrame) {
    warn!("EXCEPTION 1: DEBUG on CPU#{}", current_cpu());
    print_frame(&frame);
}

/// Non‑Maskable Interrupt (vector 2).
///
/// Typically triggered by hardware errors.
extern "x86-interrupt" fn nmi_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 2: NMI on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: NMI");
}

/// Breakpoint (vector 3).
///
/// Triggered by the `int3` instruction; used for debugging.
extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
    warn!("EXCEPTION 3: BREAKPOINT on CPU#{}", current_cpu());
    print_frame(&frame);
}

/// Overflow (vector 4).
///
/// Triggered by the `into` instruction when the overflow flag is set.
extern "x86-interrupt" fn overflow_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 4: OVERFLOW on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Overflow");
}

/// Bound range exceeded (vector 5).
///
/// Triggered by the `bound` instruction when the index is out of range.
extern "x86-interrupt" fn bound_range_exceeded_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 5: BOUND_RANGE_EXCEEDED on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Bound Range Exceeded");
}

/// Invalid opcode (vector 6).
///
/// Occurs when the CPU tries to execute an invalid instruction.
extern "x86-interrupt" fn invalid_opcode_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 6: INVALID_OPCODE on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Invalid Opcode");
}

/// Device not available (vector 7).
///
/// Occurs when an x87 FPU or SIMD instruction is executed without the device
/// being present (or with CR0.EM set).
extern "x86-interrupt" fn device_not_available_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 7: DEVICE_NOT_AVAILABLE on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Device Not Available");
}

/// Double fault (vector 8) – uses IST1.
///
/// Occurs when an exception occurs while trying to deliver another exception.
/// This uses a dedicated Interrupt Stack Table entry to avoid stack corruption.
extern "x86-interrupt" fn double_fault_handler(frame: InterruptStackFrame, error_code: u64) -> ! {
    error!("!!! CRITICAL EXCEPTION 8: DOUBLE_FAULT on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: Double Fault");
}

/// Invalid TSS (vector 10).
extern "x86-interrupt" fn invalid_tss_handler(frame: InterruptStackFrame, error_code: u64) {
    error!("!!! CRITICAL EXCEPTION 10: INVALID_TSS on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: Invalid TSS");
}

/// Segment not present (vector 11).
extern "x86-interrupt" fn segment_not_present_handler(frame: InterruptStackFrame, error_code: u64) {
    error!("!!! CRITICAL EXCEPTION 11: SEGMENT_NOT_PRESENT on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: Segment Not Present");
}

/// Stack segment fault (vector 12).
extern "x86-interrupt" fn stack_segment_fault_handler(frame: InterruptStackFrame, error_code: u64) {
    error!("!!! CRITICAL EXCEPTION 12: STACK_SEGMENT_FAULT on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: Stack Segment Fault");
}

/// General protection fault (vector 13).
extern "x86-interrupt" fn general_protection_fault_handler(frame: InterruptStackFrame, error_code: u64) {
    error!("!!! CRITICAL EXCEPTION 13: GENERAL_PROTECTION_FAULT on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: General Protection Fault");
}

/// Page fault (vector 14).
///
/// This handler delegates to `sched::handle_page_fault` to handle demand paging,
/// copy‑on‑write, and segmentation faults.
extern "x86-interrupt" fn page_fault_handler(frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    let cr2 = Cr2::read().unwrap().as_u64() as usize;
    let rip = frame.instruction_pointer.as_u64();

    let is_user = (frame.code_segment.0 & 0x3) != 0;

    crate::sched::handle_page_fault(cr2, error_code.bits(), rip, is_user);
}

/// x87 FPU exception (vector 16).
extern "x86-interrupt" fn x87_fpu_exception_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 16: X87_FPU_EXCEPTION on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: x87 FPU Exception");
}

/// Alignment check (vector 17).
extern "x86-interrupt" fn alignment_check_handler(frame: InterruptStackFrame, error_code: u64) {
    error!("!!! CRITICAL EXCEPTION 17: ALIGNMENT_CHECK on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: Alignment Check");
}

/// Machine check (vector 18) – does not return.
extern "x86-interrupt" fn machine_check_handler(frame: InterruptStackFrame) -> ! {
    error!("!!! CRITICAL EXCEPTION 18: MACHINE_CHECK on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Machine Check");
}

/// SIMD floating point exception (vector 19).
extern "x86-interrupt" fn simd_floating_point_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 19: SIMD_FLOATING_POINT on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: SIMD Floating Point");
}

// ============================================================================
// INTERRUPT HANDLERS
// ============================================================================

/// IPI (Inter‑Processor Interrupt) handler (vector 128).
///
/// Logs the IPI reception and sends an EOI to the APIC.
extern "x86-interrupt" fn ipi_handler(_frame: InterruptStackFrame) {
    warn!("IPI received on CPU#{}", current_cpu());
    crate::arch::acpi::eoi();
}

// ============================================================================
// HELPER: SET ENTRY OPTIONS
// ============================================================================

/// Sets the common options for an IDT entry: present, Ring 0, and optional IST.
///
/// # Arguments
/// * `entry` – The IDT entry to configure.
/// * `ist_index` – Optional IST index (1..7) for the entry.
fn set_entry_options(entry: &mut EntryOptions, ist_index: Option<u16>) {
    entry.set_present(true);
    entry.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
    if let Some(index) = ist_index {
        unsafe { entry.set_stack_index(index); }
    }
}

// ============================================================================
// INITIALISATION
// ============================================================================

/// Initialises the IDT for the BSP (Bootstrap Processor).
///
/// This function:
/// 1. Sets up all exception handlers (vectors 0‑19).
/// 2. Sets up the timer handler (vector 32) with a naked wrapper.
/// 3. Sets up the yield handler (vector 33) with a naked wrapper.
/// 4. Sets up the IPI handler (vector 128).
/// 5. Loads the IDT with `lidt`.
///
/// # Safety
/// This function modifies the global IDT and performs `lidt`. It is called
/// during early boot with interrupts disabled.
pub fn init_bsp() {
    #[allow(static_mut_refs)]
    let idt = unsafe { &mut GLOBAL_IDT };

    // Set up exception handlers (vectors 0‑19).
    set_entry_options(idt.divide_error.set_handler_fn(divide_error_handler), None);
    set_entry_options(idt.debug.set_handler_fn(debug_handler), None);
    set_entry_options(idt.non_maskable_interrupt.set_handler_fn(nmi_handler), None);
    set_entry_options(idt.breakpoint.set_handler_fn(breakpoint_handler), None);
    set_entry_options(idt.overflow.set_handler_fn(overflow_handler), None);
    set_entry_options(idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded_handler), None);
    set_entry_options(idt.invalid_opcode.set_handler_fn(invalid_opcode_handler), None);
    set_entry_options(idt.device_not_available.set_handler_fn(device_not_available_handler), None);

    // Double fault uses IST1.
    set_entry_options(idt.double_fault.set_handler_fn(double_fault_handler), Some(1));

    set_entry_options(idt.invalid_tss.set_handler_fn(invalid_tss_handler), None);
    set_entry_options(idt.segment_not_present.set_handler_fn(segment_not_present_handler), None);
    set_entry_options(idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler), None);
    set_entry_options(idt.general_protection_fault.set_handler_fn(general_protection_fault_handler), None);
    set_entry_options(idt.page_fault.set_handler_fn(page_fault_handler), None);

    set_entry_options(idt.x87_floating_point.set_handler_fn(x87_fpu_exception_handler), None);
    set_entry_options(idt.alignment_check.set_handler_fn(alignment_check_handler), None);
    set_entry_options(idt.machine_check.set_handler_fn(machine_check_handler), None);
    set_entry_options(idt.simd_floating_point.set_handler_fn(simd_floating_point_handler), None);

    // Timer handler (vector 32) – uses a naked wrapper.
    set_entry_options(
        idt[TIMER_VECTOR].set_handler_fn(unsafe {
            core::mem::transmute(crate::arch::timer::timer_wrapper as *const ())
        }),
        None,
    );

    // Yield handler (vector 33) – uses a naked wrapper.
    set_entry_options(
        idt[TIMER_VECTOR + 1].set_handler_fn(unsafe {
            core::mem::transmute(crate::sched::yield_wrapper as *const ())
        }),
        None,
    );

    // IPI handler (vector 128).
    set_entry_options(idt[IPI_VECTOR].set_handler_fn(ipi_handler), None);

    // Load the IDT.
    idt.load();

    info!("Initialized");
}

/// Initialises the IDT for an AP (Application Processor).
///
/// This function simply loads the global IDT (which was already set up by the BSP).
///
/// # Safety
/// This function performs `lidt` and is called during AP boot with interrupts disabled.
pub fn init_ap() {
    #[allow(static_mut_refs)]
    unsafe { GLOBAL_IDT.load() }
    info!("Initialized");
}
