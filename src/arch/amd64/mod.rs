//! # x86_64 Architecture Support
//!
//! This module provides all architecture‑specific code for the x86_64 target.
//! It implements CPU initialization, interrupt handling, memory management,
//! system calls, timers, and ACPI support.
//!
//! ## Overview
//!
//! The architecture module is organized into several sub‑modules, each
//! responsible for a distinct aspect of the x86_64 platform:
//!
//! - **ACPI**: Advanced Configuration and Power Interface – discovers and
//!   manages hardware resources (APIC, HPET, CPUs, etc.).
//! - **GDT**: Global Descriptor Table – defines segmentation and TSS entries.
//! - **IDT**: Interrupt Descriptor Table – handles exceptions and interrupts.
//! - **Paging**: 4‑level page tables (PML4, PDPT, PD, PT) with support for
//!   huge pages (2 MiB, 1 GiB) and merging/splitting.
//! - **Timer**: HPET and APIC timer calibration and management.
//! - **Trap**: Trap frame definition for context switching.
//! - **Syscall**: System call entry point (via `syscall` instruction).
//! - **Per‑CPU**: Per‑CPU data structures (via `gs` segment).
//!
//! ## Initialization Flow
//!
//! The architecture subsystem is initialized in phases:
//!
//! 1. **Early Init** (`early_init`, `early_init_bs`)
//!    - Called very early in the boot process, before paging is fully set up.
//!    - Reads CPUID, APIC ID, detects support for `rdpid`.
//!    - Sets up the `IA32_TSC_AUX` MSR for per‑CPU identification.
//!    - Initializes the per‑CPU data structure.
//!
//! 2. **BSP Init** (`init_bsp`)
//!    - Called on the bootstrap processor (CPU 0) after early init.
//!    - Initializes GDT, IDT, and per‑CPU GS base.
//!    - Sets up the TSS and interrupt stack tables.
//!
//! 3. **Late Init** (`late_init_bsp`, `late_init`)
//!    - Called after memory management is initialized.
//!    - Initializes ACPI (MADT, HPET) and the APIC timer.
//!    - Enables interrupts (`sti`).
//!
//! 4. **AP Init** (`init_ap`)
//!    - Called on each Application Processor (AP) after the BSP has opened
//!      the appropriate Fueue barriers.
//!    - Initializes GDT, IDT, and per‑CPU GS base for the AP.
//!
//! ## CPU Identification
//!
//! Each CPU core is identified by its APIC ID. The kernel uses the `rdpid`
//! instruction (if available) or reads the `IA32_TSC_AUX` MSR to get the
//! current CPU's ID. This is used for per‑CPU data access and logging.
//!
//! ## Safety
//!
//! This module contains a significant amount of unsafe code, including:
//! - Inline assembly for privileged instructions (`rdmsr`, `wrmsr`, `cpuid`, etc.).
//! - Manipulation of the GDT, IDT, and TSS via raw pointers.
//! - Naked functions for interrupt and syscall entry points.
//! - Access to `static mut` data (e.g., `GLOBAL_GDT`, `GLOBAL_IDT`).
//!
//! The unsafe operations are required for kernel‑level hardware control and
//! are carefully encapsulated in safe interfaces.

// ============================================================================
// SUBMODULES
// ============================================================================

pub mod acpi;     // ACPI tables, MADT, HPET, APIC
pub mod idt;      // Interrupt Descriptor Table, exception handlers
pub mod gdt;      // Global Descriptor Table, TSS, segmentation
pub mod percpu;   // Per‑CPU data (via GS base)
pub mod paging;   // 4‑level paging, huge pages, page table manipulation
pub mod timer;    // HPET and APIC timer calibration
pub mod trap;     // Trap frame definition
pub mod syscall;  // System call entry point

// ============================================================================
// IMPORTS
// ============================================================================

use core::arch::x86_64;
use core::arch::asm;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

// ============================================================================
// MSR CONSTANTS
// ============================================================================

/// IA32_TSC_AUX MSR – stores the APIC ID for `rdtscp` and `rdpid` fallback.
const IA32_TSC_AUX: u32 = 0xC0000103;

// ============================================================================
// CPUID CONSTANTS
// ============================================================================

const CPUID_MAX_LEAF: u32 = 0x00;
const CPUID_PROC_INFO: u32 = 0x01;
const CPUID_X2APIC: u32 = 0x0B;
const CPUID_EXT_FEATURES: u32 = 0x07;

// ============================================================================
// CPUID HELPERS
// ============================================================================

/// Result of a `cpuid` instruction.
#[derive(Debug, Clone, Copy)]
pub struct CpuidResult {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

/// Executes the `cpuid` instruction.
///
/// # Arguments
/// * `leaf` – The main CPUID leaf.
/// * `subleaf` – The sub‑leaf (ECX value).
///
/// # Returns
/// A `CpuidResult` containing the values of EAX, EBX, ECX, EDX.
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

// ============================================================================
// MSR HELPERS
// ============================================================================

/// Reads a Model‑Specific Register (MSR).
///
/// # Safety
/// The caller must ensure that the MSR is valid and accessible.
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

/// Writes a Model‑Specific Register (MSR).
///
/// # Safety
/// The caller must ensure that the MSR is valid and writable.
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

// ============================================================================
// CPU DETECTION
// ============================================================================

/// Returns the maximum CPUID leaf supported by the CPU.
#[inline]
fn max_cpuid_leaf() -> u32 {
    cpuid(CPUID_MAX_LEAF, 0).eax
}

/// Reads the APIC ID of the current CPU.
///
/// Uses `x2APIC` if available, otherwise falls back to the legacy APIC ID.
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

/// Checks if the `rdpid` instruction is available.
fn has_rdpid() -> bool {
    let max_leaf = max_cpuid_leaf();
    if max_leaf < CPUID_EXT_FEATURES {
        return false;
    }
    let r = cpuid(CPUID_EXT_FEATURES, 0);
    (r.ecx & (1 << 22)) != 0
}

/// Reads the current CPU's ID using `rdpid` (if available).
///
/// # Note
/// This is a raw instruction; the result is the APIC ID stored in `IA32_TSC_AUX`.
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

// ============================================================================
// GLOBAL STATE
// ============================================================================

/// Whether `rdpid` is available on the current CPU.
static RDPID_AVAILABLE: AtomicBool = AtomicBool::new(false);

/// Maximum number of supported CPU cores.
///
/// This is a compile‑time limit; if the system has more cores, the kernel
/// will panic.
pub const MAX_CPUS: usize = 64;

/// Time since boot in milliseconds (updated by timer tick on BSP).
pub static TIME_FROM_BOOT: AtomicU64 = AtomicU64::new(0);

// ============================================================================
// INITIALIZATION FUNCTIONS
// ============================================================================

/// Early initialization for the BSP (Bootstrap Processor).
///
/// This is a thin wrapper around `early_init` for symmetry with APs.
pub fn early_init_bs() {
    early_init();
}

/// Early initialization for all CPUs (BSP and APs).
///
/// This function:
/// 1. Reads the APIC ID.
/// 2. Writes it to the `IA32_TSC_AUX` MSR.
/// 3. Detects support for `rdpid`.
/// 4. Initializes the per‑CPU data structure with the CPU ID.
/// 5. Logs the CPU's APIC ID and `rdpid` support.
/// 6. Panics if the CPU ID exceeds `MAX_CPUS`.
///
/// # Panics
/// If the CPU ID >= `MAX_CPUS`, the kernel halts.
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

/// Returns the ID of the current CPU.
///
/// If `rdpid` is available, uses the `rdpid` instruction.
/// Otherwise, reads the `IA32_TSC_AUX` MSR.
#[inline(always)]
pub fn current_cpu() -> usize {
    if RDPID_AVAILABLE.load(Ordering::Acquire) {
        rdpid_raw()
    } else {
        unsafe { rdmsr(IA32_TSC_AUX) as usize }
    }
}

/// Full initialization for the BSP.
///
/// This function is called after early init and before memory management.
/// It initializes:
/// - Per‑CPU data (`percpu::init`)
/// - GDT (`gdt::init_bsp`)
/// - IDT (`idt::init_bsp`)
/// - Per‑CPU GS base (`percpu::init_syscall_gs`)
pub fn init_bsp() {
    percpu::init();
    gdt::init_bsp();
    idt::init_bsp();
    percpu::init_syscall_gs(0, 0);
}

/// Full initialization for APs.
///
/// This function is called on each AP after waiting for `ARCH_INIT`.
/// It initializes:
/// - GDT (`gdt::init_ap`)
/// - IDT (`idt::init_ap`) – reuses the BSP's IDT
/// - Per‑CPU GS base (`percpu::init_syscall_gs`)
pub fn init_ap() {
    gdt::init_ap(current_cpu());
    idt::init_ap();
    percpu::init_syscall_gs(0, 0);
}

/// Late initialization for the BSP.
///
/// This is called after memory management and the device model are initialized.
/// It initializes ACPI and the HPET timer.
pub fn late_init_bsp() {
    acpi::init_bsp();
}

/// Late initialization for all CPUs.
///
/// This function:
/// 1. Initializes ACPI (APIC, timer) via `acpi::init()`.
/// 2. Enables interrupts (`sti`).
///
/// Interrupts are enabled here, after all exception handlers and the timer
/// are set up.
pub fn late_init() {
    acpi::init();
    unsafe {
        core::arch::asm! {
            "sti"
        }
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Returns the number of CPUs detected by ACPI.
///
/// # Safety
/// This uses a `static mut` value set by `acpi::init_bsp`.
pub fn num_cpus() -> usize {
    #[allow(static_mut_refs)]
    unsafe {
        acpi::TOTAL_CPUS
    }
}

/// Returns the time since boot in milliseconds.
#[inline]
pub fn get_time_from_boot() -> u64 {
    TIME_FROM_BOOT.load(Ordering::Relaxed)
}

/// Returns the time since boot in seconds (as a floating‑point value).
#[inline]
pub fn get_time_from_boot_s() -> f32 {
    get_time_from_boot() as f32 / 1000.0
}

/// Halts the system.
///
/// This function never returns; it enters an infinite loop with `hlt`.
pub fn exit() -> ! {
    loop {
        unsafe {
            core::arch::asm! {
                "2:",
                "cli",
                "hlt",
                "jmp 2b",
            }
        }
    }
}
