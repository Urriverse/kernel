//! # Local APIC (LAPIC) Programming
//!
//! This module provides low‑level access to the x86_64 Local APIC (Advanced Programmable
//! Interrupt Controller). The Local APIC is a per‑CPU interrupt controller that handles
//! local interrupts (timer, performance counters, thermal events) and receives IPIs
//! (Inter‑Processor Interrupts) from other cores.
//!
//! ## Overview
//!
//! The Local APIC is memory‑mapped into the physical address space. Its registers are
//! accessed via MMIO at a base address provided by the ACPI MADT (Multiple APIC
//! Description Table). The kernel maps this physical address into the virtual address
//! space at a fixed location (`LOCAL_APIC_VMA`).
//!
//! ## Registers
//!
//! The Local APIC has several key registers:
//! - **ID Register**: The APIC ID of the current CPU.
//! - **Version Register**: The version of the APIC.
//! - **Task Priority Register (TPR)**: Controls interrupt priority.
//! - **End Of Interrupt (EOI)**: Writing to this register signals that an interrupt
//!   has been handled.
//! - **Spurious Interrupt Vector (SVR)**: Enables the APIC and sets the spurious
//!   interrupt vector.
//! - **Interrupt Command Register (ICR)**: Used to send IPIs to other CPUs.
//! - **Local Vector Table (LVT)**: Configures local interrupts (timer, LINT0, LINT1,
//!   error).
//! - **Timer Registers**: Control the APIC timer (divisor, initial count, current count).
//!
//! ## Initialisation
//!
//! The LAPIC is initialised in two phases:
//! 1. **`lapic::init()`**: Called on the BSP during `acpi::init_bsp()`. It parses
//!    the MADT to find the LAPIC physical address, maps it into the kernel's address
//!    space, and sets the global `TOTAL_CPUS` and `LAPIC_PHYS_ADDR`.
//! 2. **`lapic::enable()`**: Called on all CPUs during `acpi::init()`. It enables
//!    the LAPIC by setting the SVR register, masks all LVT entries, and disables
//!    the timer pending interrupts.
//!
//! ## IPI Sending
//!
//! The `LocalApic` struct provides methods to access the ICR registers:
//! - `iclo()`: Returns a mutable reference to the ICR low register.
//! - `ichi()`: Returns a mutable reference to the ICR high register.
//!
//! IPIs are sent by writing the target APIC ID to the high register and the vector
//! and delivery mode to the low register. The `acpi::send_ipi` function wraps this
//! process.
//!
//! ## Timer
//!
//! The APIC timer is used as the system tick source. It is programmed by writing to:
//! - **Divisor Configuration Register (DCR)**: Sets the divider for the timer.
//! - **Initial Count Register (ICR)**: Sets the initial count.
//! - **Current Count Register (CCR)**: Reads the current count.
//! - **LVT Timer Register**: Configures the timer mode (oneshot or periodic) and
//!   the interrupt vector.
//!
//! The timer is calibrated against the HPET in `timer::init()`.
//!
//! ## Safety
//!
//! - All register access is performed via volatile MMIO reads and writes using
//!   raw pointers. This is safe because the registers are mapped to known physical
//!   addresses and are read/write without side effects (except for EOI and ICR).
//! - The `LocalApic` struct is `Clone` and `Copy` and provides safe wrappers around
//!   unsafe pointer operations.
//! - The `init()` and `enable()` functions use `static mut` variables and are
//!   called during early boot (single‑threaded) or with interrupts disabled.

use crate::{arch::paging::EntryFlags, mem::kdm::{Paddr, Vaddr}};

// ============================================================================
// CONSTANTS
// ============================================================================

/// Virtual address where the Local APIC is mapped.
///
/// The kernel maps the LAPIC MMIO region to the top of the virtual address space,
/// just below the HPET mapping. This address is fixed and used by all CPUs.
const LOCAL_APIC_VMA: usize = 0xFFFFFFFFFFFFF000;

// ============================================================================
// LAPIC REGISTER OFFSETS
// ============================================================================

/// Offsets (in bytes) from the LAPIC base address for each register.
struct RegOffsets;

impl RegOffsets {
    const LAPIC_ID:        usize = 0x020;
    const LAPIC_VERSION:   usize = 0x030;
    const LAPIC_TPR:       usize = 0x080;
    const LAPIC_EOI:       usize = 0x0B0;
    const LAPIC_SVR:       usize = 0x0F0;
    const LAPIC_ICR_LOW:   usize = 0x300;
    const LAPIC_ICR_HIGH:  usize = 0x310;
    const LAPIC_LVT_TIMER: usize = 0x320;
    const LAPIC_LVT_LINT0: usize = 0x350;
    const LAPIC_LVT_LINT1: usize = 0x360;
    const LAPIC_LVT_ERROR: usize = 0x370;
    const LAPIC_TIMER_DCR: usize = 0x3E0;
    const LAPIC_TIMER_ICR: usize = 0x380;
    const LAPIC_TIMER_CCR: usize = 0x390;
}

// ============================================================================
// LOCAL APIC STRUCTURE
// ============================================================================

/// A handle to the Local APIC.
///
/// This struct provides methods to read and write the LAPIC registers using
/// MMIO. It is a zero‑sized type (ZST) because the registers are accessed via
/// fixed virtual addresses.
///
/// # Examples
/// ```ignore
/// let lapic = LocalApic::new();
/// let id = lapic.id();          // Read the APIC ID.
/// *lapic.eoi() = 0;            // Signal end of interrupt.
/// *lapic.iclo() = vector;      // Send an IPI.
/// ```
#[derive(Debug, Clone, Copy)]
pub struct LocalApic;
  // It's full and done LAPIC interface
impl LocalApic {
    /// Returns a handle to the Local APIC.
    ///
    /// The handle is a ZST and can be created cheaply.
    #[inline(always)]
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self { INSTANCE }

    // ========================================================================
    // REGISTER ACCESSORS
    // ========================================================================

    /// Returns a reference to the APIC ID register (read‑only).
    #[inline(always)]
    pub fn id(&self) -> u32 {
        *Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_ID).to_ref::<u32>()
    }

    /// Returns a reference to the APIC version register (read‑only).
    #[inline(always)]
    pub fn version(&self) -> u32 {
        *Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_VERSION).to_ref::<u32>()
    }

    /// Returns a mutable reference to the Task Priority Register (TPR).
    #[inline(always)]
    pub fn tpr(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_TPR).to_ref_mut()
    }

    /// Returns a mutable reference to the End‑Of‑Interrupt (EOI) register.
    ///
    /// Writing any value to this register signals that the current interrupt
    /// has been handled. Typically, we write `0`.
    #[inline(always)]
    pub fn eoi(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_EOI).to_ref_mut()
    }

    /// Returns a mutable reference to the Spurious Interrupt Vector (SVR) register.
    ///
    /// The SVR enables the LAPIC (bit 8) and sets the spurious interrupt vector.
    #[inline(always)]
    pub fn svr(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_SVR).to_ref_mut()
    }

    /// Returns a mutable reference to the Interrupt Command Register (ICR) low word.
    ///
    /// This register is used to send IPIs. It must be written after the high word.
    #[inline(always)]
    pub fn iclo(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_ICR_LOW).to_ref_mut()
    }

    /// Returns a mutable reference to the Interrupt Command Register (ICR) high word.
    ///
    /// This register holds the target APIC ID for the IPI.
    #[inline(always)]
    pub fn ichi(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_ICR_HIGH).to_ref_mut()
    }

    // ========================================================================
    // LVT (Local Vector Table) ACCESSORS
    // ========================================================================

    /// Returns a mutable reference to the LVT Timer register.
    ///
    /// Configures the APIC timer mode (oneshot/periodic) and the interrupt vector.
    #[inline(always)]
    pub fn lvt_timer(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_LVT_TIMER).to_ref_mut()
    }

    /// Returns a mutable reference to the LVT LINT0 register.
    #[inline(always)]
    pub fn lvt_lint0(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_LVT_LINT0).to_ref_mut()
    }

    /// Returns a mutable reference to the LVT LINT1 register.
    #[inline(always)]
    pub fn lvt_lint1(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_LVT_LINT1).to_ref_mut()
    }

    /// Returns a mutable reference to the LVT Error register.
    #[inline(always)]
    pub fn lvt_error(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_LVT_ERROR).to_ref_mut()
    }

    // ========================================================================
    // TIMER REGISTERS
    // ========================================================================

    /// Returns a mutable reference to the Timer Divisor Configuration Register (DCR).
    #[inline(always)]
    pub fn div(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_TIMER_DCR).to_ref_mut()
    }

    /// Returns a mutable reference to the Timer Initial Count Register (ICR).
    #[inline(always)]
    pub fn icr(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_TIMER_ICR).to_ref_mut()
    }

    /// Returns a mutable reference to the Timer Current Count Register (CCR).
    #[inline(always)]
    pub fn ccr(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_TIMER_CCR).to_ref_mut()
    }
}

/// The singleton instance of the Local APIC.
static INSTANCE: LocalApic = LocalApic;

// ============================================================================
// INITIALISATION FUNCTIONS
// ============================================================================

/// Initialises the Local APIC on the BSP.
///
/// This function:
/// 1. Parses the ACPI tables to get the interrupt model (MADT).
/// 2. Extracts the Local APIC physical address.
/// 3. Maps the LAPIC MMIO region into the kernel's virtual address space at
///    `LOCAL_APIC_VMA`.
/// 4. Sets the global `TOTAL_CPUS` to the number of processors found in the MADT.
/// 5. Stores the LAPIC physical address for later use.
///
/// # Panics
/// - If the interrupt model is not APIC (e.g., x2APIC or other unsupported modes).
/// - If the LAPIC mapping fails.
///
/// # Safety
/// - This function uses the `TABLES` lazy‑static, which must already be initialised.
/// - It performs MMIO mapping using `PTM.lock()` (which is safe because it's
///   called during early boot, single‑threaded).
pub fn init() {
    // Get the interrupt model from the ACPI tables.
    let interrupt_model =
        acpi::platform::InterruptModel::new(&super::TABLES).expect("Failed to parse interrupt model (MADT)");

    // Extract the list of application processors.
    let aps = match interrupt_model.1 {
        Some(pi) => pi.application_processors,
        None => panic!("Can't obtain CPU topology from ACPI"),
    };

    // Set the total number of CPUs (BSP + APs).
    unsafe {
        super::TOTAL_CPUS = aps.len() + 1;
    }

    // Extract the Local APIC physical address.
    let local_apic_address = match interrupt_model.0 {
        acpi::platform::InterruptModel::Apic(x) => {
            x.local_apic_address
        },
        _ => panic!("Unsupported host interrupt model"),
    };

    // Map the Local APIC into the kernel's virtual address space.
    match crate::mem::PTM.lock().map_4k_block(
        LOCAL_APIC_VMA,
        Paddr::from_raw(local_apic_address as usize),
        EntryFlags::PRESENT
            | EntryFlags::WRITABLE
            | EntryFlags::WRITE_THROUGH
            | EntryFlags::CACHE_DISABLE
    ) {
        Ok(_) => {},
        Err(e) => panic!("Can't map LAPIC: {}", e)
    };

    // Store the physical address globally.
    unsafe {
        super::LAPIC_PHYS_ADDR = local_apic_address as usize;
    }
}

/// Enables the Local APIC on the current CPU.
///
/// This function:
/// 1. Masks all LVT entries (timer, LINT0, LINT1, error) by setting the mask bit.
/// 2. Sets the SVR register to enable the APIC (bit 8) and sets the spurious
///    interrupt vector to `SPURIOUS_VECTOR`.
///
/// This must be called on every CPU (BSP and APs) after the LAPIC has been
/// initialised and mapped.
///
/// # Safety
/// - This function performs MMIO writes to the LAPIC registers.
/// - It is safe to call from multiple CPUs because each CPU has its own LAPIC
///   and the registers are per‑CPU.
pub fn enable() {
    let lapic = LocalApic::new();

    // Mask all LVT entries to prevent spurious interrupts.
    *lapic.lvt_timer()  = 1 << 16;
    *lapic.lvt_lint0()  = 1 << 16;
    *lapic.lvt_lint1()  = 1 << 16;
    *lapic.lvt_error()  = 1 << 16;

    // Enable the APIC (bit 8) and set the spurious interrupt vector.
    *lapic.svr() = (1u32 << 8) | (super::SPURIOUS_VECTOR as u32);
}
