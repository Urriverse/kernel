use core::arch::naked_asm;
use crate::arch::{rdmsr, wrmsr};

pub const IA32_EFER: u32 = 0xC0000080;
pub const IA32_STAR: u32 = 0xC0000081;
pub const IA32_LSTAR: u32 = 0xC0000082;
pub const IA32_FMASK: u32 = 0xC0000084;

#[unsafe(naked)]
pub unsafe extern "C" fn syscall_entry() -> ! {
    naked_asm!(
        "swapgs",

        "mov rbx, rsp",

        "mov rsp, gs:[8]",

        "sub rsp, 160",

        "mov [rsp + 0], rax",
        "mov [rsp + 8], rbx",
        "mov [rsp + 16], rcx",
        "mov [rsp + 24], rdx",
        "mov [rsp + 32], rsi",
        "mov [rsp + 40], rdi",
        "mov [rsp + 48], rbp",
        "mov [rsp + 56], r8",
        "mov [rsp + 64], r9",
        "mov [rsp + 72], r10",
        "mov [rsp + 80], r11",
        "mov [rsp + 88], r12",
        "mov [rsp + 96], r13",
        "mov [rsp + 104], r14",
        "mov [rsp + 112], r15",

        "mov [rsp + 120], rcx",
        "mov [rsp + 128], 0x18",
        "mov [rsp + 136], r11",
        "mov [rsp + 144], rbx",
        "mov [rsp + 152], 0x20",

        "mov rdi, rsp",
        "call {syscall_dispatcher}",

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

        "mov rcx, [rsp + 120]",
        "mov r11, [rsp + 136]",

        "mov rsp, [rsp + 144]",

        "swapgs",

        "sysret",

        syscall_dispatcher = sym crate::sched::syscall_dispatcher,
    );
}

pub fn init() {
    let efer = unsafe { rdmsr(IA32_EFER) };
    unsafe { wrmsr(IA32_EFER, efer | 1); }

    let star = (0x08u64 << 48) | (0x08u64 << 32);
    unsafe { wrmsr(IA32_STAR, star); }

    unsafe { wrmsr(IA32_LSTAR, syscall_entry as *const () as u64); }

    unsafe { wrmsr(IA32_FMASK, 0x300); }
}
