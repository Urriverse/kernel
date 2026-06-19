//! Interrupt Descriptor Table (IDT) manager.
//!
//! Реализация IDT на основе крейта `x86_64` с использованием ABI `x86-interrupt`.

use crate::arch::current_cpu;
use x86_64::structures::idt::{
    EntryOptions, InterruptDescriptorTable, InterruptStackFrame,
    PageFaultErrorCode
};
use x86_64::registers::control::Cr2;

/// Вектор для межпроцессорных прерываний (IPI).
pub const IPI_VECTOR: u8 = 128;

pub const TIMER_VECTOR: u8 = 32;

/// Глобальная IDT, общая для всех ядер.
pub static mut GLOBAL_IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

// ============================================================================
// Утилиты для логирования
// ============================================================================

fn print_frame(frame: &InterruptStackFrame) {
    error! (
        "\n  RIP: {:#018X}\n  CS : {:#08X}\n  RFLAGS: {:#018X}\n  RSP: {:#018X}\n  SS : {:#08X}",
        frame.instruction_pointer.as_u64(),
        frame.code_segment.0,
        frame.cpu_flags,
        frame.stack_pointer.as_u64(),
        frame.stack_segment.0,
    );
}

// ============================================================================
// Обработчики исключений (векторы 0–19)
// ============================================================================

extern "x86-interrupt" fn divide_error_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 0: DIVIDE_ERROR on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Divide Error");
}

extern "x86-interrupt" fn debug_handler(frame: InterruptStackFrame) {
    warn!("EXCEPTION 1: DEBUG on CPU#{}", current_cpu());
    print_frame(&frame);
}

extern "x86-interrupt" fn nmi_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 2: NMI on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: NMI");
}

extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
    warn!("EXCEPTION 3: BREAKPOINT on CPU#{}", current_cpu());
    print_frame(&frame);
}

extern "x86-interrupt" fn overflow_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 4: OVERFLOW on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Overflow");
}

extern "x86-interrupt" fn bound_range_exceeded_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 5: BOUND_RANGE_EXCEEDED on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Bound Range Exceeded");
}

extern "x86-interrupt" fn invalid_opcode_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 6: INVALID_OPCODE on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Invalid Opcode");
}

extern "x86-interrupt" fn device_not_available_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 7: DEVICE_NOT_AVAILABLE on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Device Not Available");
}

// Вектор 8: Double Fault — критическое исключение с кодом ошибки, не возвращает управление
extern "x86-interrupt" fn double_fault_handler(frame: InterruptStackFrame, error_code: u64) -> ! {
    error!("!!! CRITICAL EXCEPTION 8: DOUBLE_FAULT on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: Double Fault");
}

extern "x86-interrupt" fn invalid_tss_handler(frame: InterruptStackFrame, error_code: u64) {
    error!("!!! CRITICAL EXCEPTION 10: INVALID_TSS on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: Invalid TSS");
}

extern "x86-interrupt" fn segment_not_present_handler(frame: InterruptStackFrame, error_code: u64) {
    error!("!!! CRITICAL EXCEPTION 11: SEGMENT_NOT_PRESENT on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: Segment Not Present");
}

extern "x86-interrupt" fn stack_segment_fault_handler(frame: InterruptStackFrame, error_code: u64) {
    error!("!!! CRITICAL EXCEPTION 12: STACK_SEGMENT_FAULT on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: Stack Segment Fault");
}

extern "x86-interrupt" fn general_protection_fault_handler(frame: InterruptStackFrame, error_code: u64) {
    error!("!!! CRITICAL EXCEPTION 13: GENERAL_PROTECTION_FAULT on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: General Protection Fault");
}

// Вектор 14: Page Fault — специальный обработчик с чтением CR2
extern "x86-interrupt" fn page_fault_handler(frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    let cr2 = Cr2::read().unwrap().as_u64() as usize;
    let rip = frame.instruction_pointer.as_u64();
    
    // Определяем, было ли прерывание из User Mode (Ring 3)
    // В x86_64 коде сегмента (CS) младшие 2 бита указывают на Ring (0x08 = Ring 0, 0x1B = Ring 3)
    let is_user = (frame.code_segment.0 & 0x3) != 0;

    crate::sched::handle_page_fault(cr2, error_code.bits(), rip, is_user);
}

extern "x86-interrupt" fn x87_fpu_exception_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 16: X87_FPU_EXCEPTION on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: x87 FPU Exception");
}

extern "x86-interrupt" fn alignment_check_handler(frame: InterruptStackFrame, error_code: u64) {
    error!("!!! CRITICAL EXCEPTION 17: ALIGNMENT_CHECK on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: Alignment Check");
}

extern "x86-interrupt" fn machine_check_handler(frame: InterruptStackFrame) -> ! {
    error!("!!! CRITICAL EXCEPTION 18: MACHINE_CHECK on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Machine Check");
}

extern "x86-interrupt" fn simd_floating_point_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 19: SIMD_FLOATING_POINT on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: SIMD Floating Point");
}

// ============================================================================
// IPI обработчик (вектор 128)
// ============================================================================

extern "x86-interrupt" fn ipi_handler(_frame: InterruptStackFrame) {
    warn!("IPI received on CPU#{}", current_cpu());
    crate::arch::acpi::eoi();
}

// extern "x86-interrupt" fn timer_handler(_frame: InterruptStackFrame) {
//     unsafe {
//         core::arch::asm! {
//             "cli"  // force disable interrupts
//         }
//     }
//     if crate::arch::current_cpu() == 0 {
//         crate::arch::increment_time_from_boot();
//     }
//     crate::arch::acpi::eoi();
//     unsafe {
//         core::arch::asm! {
//             "sti"  // force enable interrupts
//         }
//     }
// }

// ============================================================================
// Вспомогательная функция для настройки опций IDT
// ============================================================================

fn set_entry_options(entry: &mut EntryOptions, ist_index: Option<u16>) {
    entry.set_present(true);
    entry.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
    if let Some(index) = ist_index {
        // Safety: мы гарантируем, что индекс корректен (0..7) и стек не используется другими прерываниями
        unsafe { entry.set_stack_index(index); }
    }
}

// ============================================================================
// Инициализация для BSP и AP
// ============================================================================

pub fn init_bsp() {
    info!("Initializing exception handlers for BSP...");

    #[allow(static_mut_refs)]
    let idt = unsafe { &mut GLOBAL_IDT };

    // Векторы 0–7 — обработчики без кода ошибки
    set_entry_options(idt.divide_error.set_handler_fn(divide_error_handler), None);
    set_entry_options(idt.debug.set_handler_fn(debug_handler), None);
    set_entry_options(idt.non_maskable_interrupt.set_handler_fn(nmi_handler), None);
    set_entry_options(idt.breakpoint.set_handler_fn(breakpoint_handler), None);
    set_entry_options(idt.overflow.set_handler_fn(overflow_handler), None);
    set_entry_options(idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded_handler), None);
    set_entry_options(idt.invalid_opcode.set_handler_fn(invalid_opcode_handler), None);
    set_entry_options(idt.device_not_available.set_handler_fn(device_not_available_handler), None);

    // Вектор 8: Double Fault — использует IST (Interrupt Stack Table) для защиты от переполнения стека
    set_entry_options(idt.double_fault.set_handler_fn(double_fault_handler), Some(1));

    // Векторы 10–14 — обработчики с кодом ошибки
    set_entry_options(idt.invalid_tss.set_handler_fn(invalid_tss_handler), None);
    set_entry_options(idt.segment_not_present.set_handler_fn(segment_not_present_handler), None);
    set_entry_options(idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler), None);
    set_entry_options(idt.general_protection_fault.set_handler_fn(general_protection_fault_handler), None);
    set_entry_options(idt.page_fault.set_handler_fn(page_fault_handler), None);

    // Векторы 16–19
    set_entry_options(idt.x87_floating_point.set_handler_fn(x87_fpu_exception_handler), None);
    set_entry_options(idt.alignment_check.set_handler_fn(alignment_check_handler), None);
    set_entry_options(idt.machine_check.set_handler_fn(machine_check_handler), None);
    set_entry_options(idt.simd_floating_point.set_handler_fn(simd_floating_point_handler), None);

    set_entry_options(idt[TIMER_VECTOR].set_handler_fn(unsafe { core::mem::transmute(crate::arch::timer::timer_wrapper as *const ()) }), None);
    set_entry_options(idt[TIMER_VECTOR+1].set_handler_fn(unsafe { core::mem::transmute(crate::sched::yield_wrapper as *const ()) }), None);

    // IPI (вектор 128) — доступ через индекс, т.к. это не предопределённое исключение
    set_entry_options(idt[IPI_VECTOR].set_handler_fn(ipi_handler), None);

    idt.load();

    info!("Loaded successfully on BSP.");
}

pub fn init_ap() {
    info!("Loading for AP...");
    #[allow(static_mut_refs)]
    unsafe { GLOBAL_IDT.load() }
    info!("Loaded successfully on AP.");
}
