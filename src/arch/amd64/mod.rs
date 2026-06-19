pub mod acpi;
pub mod idt;
pub mod gdt;
pub mod percpu;
pub mod paging;
pub mod timer;
pub mod trap;
pub mod syscall;

use core::arch::x86_64;
use core::arch::asm;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
pub use syscall::init as init_syscall;

const IA32_TSC_AUX: u32 = 0xC0000103;

const CPUID_MAX_LEAF: u32 = 0x00;
const CPUID_PROC_INFO: u32 = 0x01;
const CPUID_X2APIC: u32 = 0x0B;
const CPUID_EXT_FEATURES: u32 = 0x07;

#[derive(Debug, Clone, Copy)]
pub struct CpuidResult {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

#[inline]
pub fn cpuid(leaf: u32, subleaf: u32) -> CpuidResult {
    let res = x86_64::__cpuid_count(leaf, subleaf);
    CpuidResult {
        eax: res.eax,
        ebx: res.ebx,
        ecx: res.ecx,
        edx: res.edx,
    }
}

#[inline]
pub unsafe fn rdmsr(msr: u32) -> u64 {
    let (lo, hi): (u32, u32);
    unsafe {
        asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") lo,
            out("edx") hi,
            options(nostack, preserves_flags),
        );
    }
    ((hi as u64) << 32) | (lo as u64)
}

#[inline]
pub unsafe fn wrmsr(msr: u32, value: u64) {
    let lo = value as u32;
    let hi = (value >> 32) as u32;
    unsafe {
        asm!(
            "wrmsr",
            in("ecx") msr,
            in("eax") lo,
            in("edx") hi,
            options(nostack, preserves_flags),
        );
    }
}

#[inline]
fn max_cpuid_leaf() -> u32 {
    cpuid(CPUID_MAX_LEAF, 0).eax
}

fn read_apic_id() -> u32 {
    let max_leaf = max_cpuid_leaf();

    let x2apic_supported = if max_leaf >= CPUID_PROC_INFO {
        let r = cpuid(CPUID_PROC_INFO, 0);
        (r.ecx & (1 << 21)) != 0
    } else {
        false
    };

    if x2apic_supported && max_leaf >= CPUID_X2APIC {
        let r = cpuid(CPUID_X2APIC, 0);
        return r.edx;
    }

    if max_leaf >= CPUID_PROC_INFO {
        let r = cpuid(CPUID_PROC_INFO, 0);
        return (r.ebx >> 24) & 0xFF;
    }

    0
}

fn has_rdpid() -> bool {
    let max_leaf = max_cpuid_leaf();
    if max_leaf < CPUID_EXT_FEATURES {
        return false;
    }
    let r = cpuid(CPUID_EXT_FEATURES, 0);
    (r.ecx & (1 << 22)) != 0
}

#[inline(always)]
fn rdpid_raw() -> usize {
    let id: u64;
    unsafe {
        asm!(
            "rdpid {}",
            out(reg) id,
            options(nostack, preserves_flags),
        );
    }
    id as usize
}

static RDPID_AVAILABLE: AtomicBool = AtomicBool::new(false);

pub fn early_init_bs() {
    early_init();
}

pub fn early_init() {
    let apic_id = read_apic_id();

    unsafe {
        wrmsr(IA32_TSC_AUX, apic_id as u64);
    }

    let rdpid_ok = has_rdpid();
    RDPID_AVAILABLE.store(rdpid_ok, Ordering::Release);

    let cpu_id = current_cpu();

    let pcpu = percpu::current();
    pcpu.cpu_id = cpu_id;

    crate::info!(
        "APIC ID = {}, RDPID = {}",
        apic_id,
        if rdpid_ok { "yes" } else { "no" }
    );

    if cpu_id > MAX_CPUS - 1 {
        error!("Too high CPU detected. Gonna sleep (Zzz...)");
        unsafe {
            core::arch::asm! {
                "2:",
                "cli",
                "hlt",
                "jmp 2b"
            }
        }
        unreachable!()
    }
}

#[inline(always)]
pub fn current_cpu() -> usize {
    if RDPID_AVAILABLE.load(Ordering::Acquire) {
        rdpid_raw()
    } else {
        unsafe { rdmsr(IA32_TSC_AUX) as usize }
    }
}

pub const MAX_CPUS: usize = 64;

pub fn init_bsp() {
    percpu::init();
    gdt::init_bsp();
    idt::init_bsp();
    percpu::init_syscall_gs(0, 0);
}

pub fn init_ap() {
    gdt::init_ap(current_cpu());
    idt::init_bsp();
    percpu::init_syscall_gs(0, 0);
}

pub fn late_init_bsp() {
    acpi::init_bsp();
}

pub fn late_init() {
    acpi::init();
    unsafe {
        core::arch::asm! {
            "sti"
        }
    }
}

pub fn num_cpus() -> usize {
    #[allow(static_mut_refs)]
    unsafe {
        acpi::TOTAL_CPUS
    }
}

pub static TIME_FROM_BOOT: AtomicU64 = AtomicU64::new(0);

pub fn get_time_from_boot() -> u64 {
    TIME_FROM_BOOT.load(Ordering::Relaxed)
}

pub fn get_time_from_boot_s() -> f32 {
    get_time_from_boot() as f32 / 1000.0
}

pub fn exit() -> ! {
    hang!();
}
