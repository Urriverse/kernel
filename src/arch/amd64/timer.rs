//! # HPET and APIC Timer Management (x86_64)
//!
//! This module manages the High Precision Event Timer (HPET) and the Local APIC
//! timer on x86_64 systems. The APIC timer is used as the primary system tick
//! source, while the HPET is used for calibration during early boot.
//!
//! ## Overview
//!
//! The kernel uses two timers:
//! - **HPET**: A high‑resolution timer that is used to calibrate the APIC timer.
//!   The HPET is memory‑mapped and provides a stable counter with a known
//!   frequency (usually 10 MHz or higher).
//! - **APIC Timer**: A per‑CPU timer that generates periodic interrupts. It is
//!   calibrated against the HPET to determine the number of ticks per 10 ms.
//!
//! ## Calibration Process
//!
//! The calibration is performed during `timer::init()`:
//!
//! 1. The HPET is disabled and reset to zero.
//! 2. The APIC timer is set to one‑shot mode with a maximum count (`!0`).
//! 3. The HPET is enabled.
//! 4. The kernel spins, waiting for a fixed number of HPET ticks (1 second).
//! 5. The APIC timer's current count is read and subtracted from the initial
//!    maximum value to determine the number of APIC ticks in 1 second.
//! 6. The result is stored in `TICKS_PER_10MS` (divided by 100 to get 10 ms ticks).
//! 7. The APIC timer is set to periodic mode with the calibrated count.
//!
//! ## Timer Interrupt
//!
//! The APIC timer fires at vector `TIMER_VECTOR` (32). The interrupt handler
//! (`timer_wrapper`) is a naked function that saves the CPU context and calls
//! `sched::timer_tick()`, which updates the system time and performs scheduling.
//!
//! ## HPET Mapping
//!
//! The HPET is mapped into the kernel's virtual address space at a fixed address
//! (`HPET_VMA`) during `init_bsp()`. The mapping uses cache‑disabled and write‑
//! through attributes to ensure correct timing.
//!
//! ## Safety
//!
//! - The HPET and APIC registers are accessed via MMIO and MSRs, which are
//!   privileged operations.
//! - The `timer_wrapper` is a naked function that uses inline assembly to save
//!   and restore the CPU context.
//! - The calibration function spins with interrupts disabled; this is safe
//!   because it is called before interrupts are enabled.

use crate::{arch::paging::EntryFlags, mem::kdm::{Paddr, Vaddr}, sync::{Mutex, Nutex}};
use core::arch::naked_asm;
use core::sync::atomic::{AtomicU64, Ordering};

// ============================================================================
// GLOBAL STATE
// ============================================================================

/// The number of APIC timer ticks per 10 ms, calibrated at boot.
static TICKS_PER_10MS: AtomicU64 = AtomicU64::new(0);
static CALIBRATED_TICKS: AtomicU64 = AtomicU64::new(0);

// ============================================================================
// TIMER WRAPPER (NAKED INTERRUPT HANDLER)
// ============================================================================

/// Naked interrupt wrapper for the APIC timer.
///
/// This function is called on vector 32. It:
/// 1. Saves the CPU context (including `swapgs` if coming from user mode).
/// 2. Calls `sched::timer_tick` with the trap frame.
/// 3. Restores the context and returns via `iretq`.
///
/// # Safety
/// This is a naked function that manipulates the stack and registers directly.
#[unsafe(naked)]
pub unsafe extern "C" fn timer_wrapper() -> ! {
    naked_asm!(
        // If we came from user mode (RPL 3), swap GS.
        "mov rax, [rsp + 8]",
        "and rax, 3",
        "cmp rax, 3",
        "jne 1f",
        "swapgs",
        "1:",

        // Save all general‑purpose registers on the stack.
        "push r15", "push r14", "push r13", "push r12",
        "push r11", "push r10", "push r9", "push r8",
        "push rbp", "push rdi", "push rsi", "push rdx",
        "push rcx", "push rbx", "push rax",

        // Call the scheduler tick handler with the trap frame.
        "mov rdi, rsp",
        "call {scheduler_tick}",

        // Restore all general‑purpose registers.
        "pop rax", "pop rbx", "pop rcx", "pop rdx",
        "pop rsi", "pop rdi", "pop rbp", "pop r8",
        "pop r9", "pop r10", "pop r11", "pop r12",
        "pop r13", "pop r14", "pop r15",

        // If we came from user mode, swap GS back.
        "mov rax, [rsp + 8]",
        "and rax, 3",
        "cmp rax, 3",
        "jne 2f",
        "swapgs",
        "2:",

        // Return from interrupt.
        "iretq",

        scheduler_tick = sym crate::sched::timer_tick,
    );
}

// ============================================================================
// HPET CONSTANTS AND STRUCTURE
// ============================================================================

/// Virtual address where the HPET is mapped.
///
/// The HPET is mapped to a fixed address just below the LAPIC mapping.
const HPET_VMA: usize = 0xFFFFFFFFFFFFE000;

/// HPET register offsets.
struct HpetOffsets;
impl HpetOffsets {
    const HPET_CAP: usize = 0x000;     // Capabilities register (RO)
    const HPET_CFG: usize = 0x010;     // Configuration register (RW)
    const HPET_COUNTER: usize = 0x0F0; // Main counter (RW)
}

/// A handle to the HPET.
///
/// This struct provides methods to access the HPET registers via MMIO.
#[derive(Debug, Clone, Copy)]
pub struct Hpet;

impl Hpet {
    /// Enable bit for the HPET configuration register.
    pub const ENABLE: u32 = 1;

    /// Disables the HPET (clears the enable bit).
    #[inline(always)]
    pub fn disable(&self) {
        *self.cfg() &= !Hpet::ENABLE;
    }

    /// Enables the HPET (sets the enable bit).
    #[inline(always)]
    pub fn enable(&self) {
        *self.cfg() |= Hpet::ENABLE;
    }

    /// Resets the HPET counter to zero.
    #[inline(always)]
    pub fn reset(&self) {
        *self.counter() = 0;
    }

    // ---- Register accessors ----

    /// Returns a reference to the capabilities register (read‑only).
    #[inline(always)]
    pub fn cap(&self) -> u64 {
        *Vaddr::from_raw(HPET_VMA + HpetOffsets::HPET_CAP).to_ref::<u64>()
    }

    /// Returns a mutable reference to the configuration register.
    #[inline(always)]
    pub fn cfg(&self) -> &mut u32 {
        Vaddr::from_raw(HPET_VMA + HpetOffsets::HPET_CFG).to_ref_mut::<u32>()
    }

    /// Returns a mutable reference to the main counter register.
    #[inline(always)]
    pub fn counter(&self) -> &mut u64 {
        Vaddr::from_raw(HPET_VMA + HpetOffsets::HPET_COUNTER).to_ref_mut::<u64>()
    }
}

/// The global HPET instance, protected by a `Nutex`.
#[allow(dead_code)] pub static INSTANCE: Nutex<Hpet> = Nutex::new(Hpet);

// ============================================================================
// HPET INITIALISATION
// ============================================================================

/// Initialises the HPET on the BSP.
///
/// This function:
/// 1. Parses the ACPI HPET table to get the physical base address.
/// 2. Maps the HPET MMIO region into the kernel's virtual address space at
///    `HPET_VMA` using a 4 KiB page with cache‑disabled and write‑through flags.
///
/// # Panics
/// - If the HPET table is not found.
/// - If the mapping fails.
pub fn init_bsp() {
    let hpet_info = acpi::HpetInfo::new(&super::acpi::TABLES).expect("Failed to parse HPET table");
    let hpet_base_paddr = hpet_info.base_address;

    match crate::mem::PTM.lock().map_4k_block(
        HPET_VMA,
        Paddr::from_raw(hpet_base_paddr),
        EntryFlags::PRESENT
            | EntryFlags::WRITABLE
            | EntryFlags::CACHE_DISABLE
            | EntryFlags::WRITE_THROUGH
    ) {
        Ok(_) => {},
        Err(e) => panic!("Can't map HPET: {}", e)
    };
}

// ============================================================================
// APIC TIMER CALIBRATION AND INITIALISATION
// ============================================================================

pub fn calibrate() {
    let inst = Hpet;
    let lapic = super::acpi::lapic::LocalApic;

    let cap = inst.cap();
    let period_fs = cap >> 32;
    if period_fs == 0 { panic!("HPET period is 0"); }

    let target_fs = 1_000_000_000_000u64;
    let hpet_ticks_to_wait = target_fs / period_fs;

    inst.disable();
    inst.reset();
    
    *lapic.div() = 3;               // x16
    *lapic.lvt_timer() = 0x00010000; // oneshot, masked
    *lapic.icr() = !0;              // max value

    inst.enable();
    let start_hpet = *inst.counter();
    
    // Wait 1 second
    loop {
        core::hint::spin_loop();
        if (*inst.counter() - start_hpet) >= hpet_ticks_to_wait { break; }
    }
    inst.disable();

    let cur_lapic = *lapic.ccr();
    let elapsed = !0 - cur_lapic;

    // Store globally for all CPUs to use
    CALIBRATED_TICKS.store(elapsed as u64, Ordering::Release);
    TICKS_PER_10MS.store(elapsed as u64 / 100, Ordering::Release); // Ticks per 10ms
    info!("APIC timer calibrated: {} ticks per 10ms", elapsed as u64 / 100);
}

static TIMER_SEQ: Mutex<()> = Mutex::new(());

/// Programs the local APIC timer on ALL CPUs (BSP + APs) using the BSP's calibration.
pub fn init() {
    let _ = TIMER_SEQ.lock();
    let lapic = super::acpi::lapic::LocalApic;
    let elapsed = CALIBRATED_TICKS.load(Ordering::Acquire);

    // Set the APIC timer to periodic mode with the calibrated count.
    *lapic.div() = 3; // x16
    *lapic.lvt_timer() = (1 << 17) | (crate::arch::idt::TIMER_VECTOR as u32); // periodic
    *lapic.icr() = elapsed as u32;
}

// ============================================================================
// PUBLIC API
// ============================================================================

/// Returns the number of APIC timer ticks per 10 ms.
///
/// This value is used by the scheduler to convert real time (10 ms ticks)
/// into virtual runtime increments.
#[inline]
pub fn get_ticks_per_10ms() -> u64 {
    TICKS_PER_10MS.load(Ordering::Relaxed)
}
