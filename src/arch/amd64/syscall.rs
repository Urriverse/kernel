// src/arch/amd64/syscall.rs

use core::arch::naked_asm;
use crate::arch::{rdmsr, wrmsr};

pub const IA32_EFER: u32 = 0xC0000080;
pub const IA32_STAR: u32 = 0xC0000081;
pub const IA32_LSTAR: u32 = 0xC0000082;
pub const IA32_FMASK: u32 = 0xC0000084;

/// Точка входа для инструкции `syscall`.
/// Сохраняет все регистры в TrapFrame на ядерном стеке,
/// вызывает диспетчер, затем восстанавливает регистры и выполняет `sysret`.
#[unsafe(naked)]
pub unsafe extern "C" fn syscall_entry() -> ! {
    naked_asm!(
        // переключиться на ядерный GS (per‑CPU)
        "swapgs",

        // сохранить пользовательский RSP
        "mov rbx, rsp",

        // загрузить kernel_stack_top из per‑CPU (смещение 8)
        "mov rsp, gs:[8]",

        // выделить место под TrapFrame (20 * 8 = 160 байт)
        "sub rsp, 160",

        // --- сохранить все регистры общего назначения ---
        "mov [rsp + 0], rax",
        "mov [rsp + 8], rbx",
        "mov [rsp + 16], rcx",   // RCX = пользовательский RIP
        "mov [rsp + 24], rdx",
        "mov [rsp + 32], rsi",
        "mov [rsp + 40], rdi",
        "mov [rsp + 48], rbp",
        "mov [rsp + 56], r8",
        "mov [rsp + 64], r9",
        "mov [rsp + 72], r10",
        "mov [rsp + 80], r11",   // R11 = пользовательские RFLAGS
        "mov [rsp + 88], r12",
        "mov [rsp + 96], r13",
        "mov [rsp + 104], r14",
        "mov [rsp + 112], r15",

        // --- поля, аналогичные аппаратному стеку прерывания ---
        "mov [rsp + 120], rcx",  // RIP
        "mov [rsp + 128], 0x18", // CS  (USER_CS)
        "mov [rsp + 136], r11",  // RFLAGS
        "mov [rsp + 144], rbx",  // пользовательский RSP
        "mov [rsp + 152], 0x20", // SS  (USER_SS)

        // вызвать диспетчер (frame = RSP)
        "mov rdi, rsp",
        "call {syscall_dispatcher}",

        // --- восстановить регистры (кроме RCX, R11, RSP) ---
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

        // подготовить RCX и R11 для sysret
        "mov rcx, [rsp + 120]",  // RIP
        "mov r11, [rsp + 136]",  // RFLAGS

        // восстановить пользовательский RSP
        "mov rsp, [rsp + 144]",

        // вернуться к пользовательскому GS
        "swapgs",

        // выполнить возврат в пользовательский режим
        "sysret",

        syscall_dispatcher = sym crate::sched::syscall_dispatcher,
    );
}

/// Инициализация MSR для системных вызовов.
/// Должна быть вызвана один раз на BSP после настройки GDT/IDT.
pub fn init() {
    // Включить бит SCE в IA32_EFER
    let efer = unsafe { rdmsr(IA32_EFER) };
    unsafe { wrmsr(IA32_EFER, efer | 1); }

    // STAR: SYSRET_CS = USER_CS - 16 (0x18 - 0x10 = 0x08)
    // SYSCALL_CS = KERNEL_CS = 0x08
    let star = (0x08u64 << 48) | (0x08u64 << 32);
    unsafe { wrmsr(IA32_STAR, star); }

    // LSTAR = адрес обработчика
    unsafe { wrmsr(IA32_LSTAR, syscall_entry as *const () as u64); }

    // FMASK: сбросить IF (бит 9) и DF (бит 10)
    unsafe { wrmsr(IA32_FMASK, 0x300); }
}
