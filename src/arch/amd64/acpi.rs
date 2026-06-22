//! # ACPI Subsystem (x86_64)
//!
//! This module provides the interface to the ACPI (Advanced Configuration and Power Interface)
//! tables and hardware, including the MADT (Multiple APIC Description Table), HPET, and APIC
//! (Advanced Programmable Interrupt Controller) initialization. It is responsible for
//! discovering and initialising the Local APIC and I/O APIC, parsing the ACPI tables for
//! processor topology, and providing functions for inter‑processor interrupts (IPIs).
//!
//! ## Overview
//!
//! ACPI is the standard for hardware discovery and power management on x86_64 systems.
//! The kernel uses ACPI to:
//! - Enumerate all CPU cores (via the MADT).
//! - Locate the Local APIC (LAPIC) and I/O APIC base addresses.
//! - Initialise the APIC timer and calibrate it using the HPET.
//! - Send IPIs to other cores for bootstrapping and inter‑core communication.
//!
//! ## Structure
//!
//! The module is divided into three sub‑modules:
//! - **`lapic`**: Local APIC programming (MMIO registers, EOI, timer, IPI sending).
//! - **`handler`**: An implementation of `acpi::Handler` that provides the ACPI library
//!   with a way to map physical memory, read/write I/O ports, and handle AML operations.
//! - **`acpi.rs` (this file)**: High‑level ACPI initialisation, table parsing, and IPI
//!   functions that use the `lapic` module and the global `TABLES` lazy‑static.
//!
//! ## Global State
//!
//! - **`TABLES`**: A `lazy_static` holding the parsed ACPI tables, obtained from the RSDP.
//! - **`TOTAL_CPUS`**: A `static mut` counter set during ACPI init to the number of CPUs.
//! - **`LAPIC_PHYS_ADDR`**: A `static mut` holding the physical address of the Local APIC.
//!
//! ## Initialisation Flow
//!
//! 1. **BSP**:
//!    - `acpi::init_bsp()` is called from `arch::late_init_bsp()`.
//!    - It calls `lapic::init()`, which maps the Local APIC into the kernel's virtual
//!      address space and parses the MADT to set `TOTAL_CPUS`.
//!    - It then calls `timer::init_bsp()` to set up the HPET mapping.
//!
//! 2. **All CPUs (BSP + APs)**:
//!    - `acpi::init()` is called from `arch::late_init()`.
//!    - It calls `lapic::enable()` to enable the Local APIC (set SVR, mask LVT entries).
//!    - It calls `timer::init()` to calibrate the APIC timer using the HPET.
//!    - Finally, interrupts are enabled with `sti`.
//!
//! ## IPI Functions
//!
//! - **`send_ipi(target_apic_id, vector, mode)`**: Sends an IPI with a specific delivery mode.
//! - **`send_fixed_ipi(target_apic_id, vector)`**: Convenience wrapper for a fixed IPI.
//! - **`eoi()`**: Writes to the EOI register of the Local APIC to signal the end of
//!   interrupt processing.
//!
//! ## Safety
//!
//! - The module uses `unsafe` to access MMIO registers of the Local APIC and HPET.
//! - The `TABLES` lazy‑static uses a `static mut` for the RSDP address, which is set
//!   during early boot.
//! - The `limine!` macro creates a static request for the RSDP, which is guaranteed
//!   by the bootloader.

use crate::{arch::timer, mem::kdm::Vaddr};

// ============================================================================
// SUBMODULES
// ============================================================================

pub mod lapic;
pub mod handler;

// ============================================================================
// RE-EXPORTS FROM ACPI CRATE
// ============================================================================

#[allow(unused_imports)]
pub use ::acpi::platform::Processor;

// ============================================================================
// LIMINE REQUEST FOR RSDP
// ============================================================================

/// Limine request for the RSDP (Root System Description Pointer).
///
/// The RSDP is the entry point to the ACPI tables. It is provided by the
/// bootloader and is used by the ACPI library to parse all other tables.
limine! { RSDP <= RsdpRequest }

// ============================================================================
// LAZY‑STATIC ACPI TABLES
// ============================================================================

/// Parsed ACPI tables, lazily initialised from the RSDP.
///
/// The `AcpiTables` struct provides access to all ACPI tables (MADT, HPET,
/// DSDT, FADT, etc.) and includes a handler (`Hdl`) for platform‑specific
/// operations (memory mapping, I/O access).
///
/// # Panics
/// Panics if the RSDP response is unavailable or if table parsing fails.
lazy_static! {
    pub static ref TABLES: acpi::AcpiTables<handler::Hdl> = unsafe {
        acpi::AcpiTables::from_rsdp(
            handler::Hdl,
            Vaddr::from_raw(
                RSDP
                    .response()
                    .expect("Can't obtain RSDP")
                    .address as usize
                ).to_phys().to_raw()
        ).expect("Failed to parse ACPI tables")
    };
}

// ============================================================================
// GLOBAL STATE
// ============================================================================

/// Total number of CPU cores detected by ACPI.
///
/// This is set by `lapic::init()` and used by the scheduler and other subsystems.
/// It is `static mut` because it is written during early boot (single‑threaded)
/// and read thereafter.
pub static mut TOTAL_CPUS: usize = 0;

/// Physical address of the Local APIC.
///
/// This is set by `lapic::init()` and used internally for mapping.
pub static mut LAPIC_PHYS_ADDR: usize = 0;

/// Spurious interrupt vector used by the Local APIC.
pub const SPURIOUS_VECTOR: u8 = 0xFF;

// ============================================================================
// ACPI INITIALISATION
// ============================================================================

/// Initialises the Local APIC and HPET on the BSP.
///
/// This is called early in BSP initialisation, before memory management is
/// fully set up. It:
/// 1. Calls `lapic::init()` to parse the MADT and map the Local APIC.
/// 2. Calls `timer::init_bsp()` to map the HPET.
pub fn init_bsp() {
    lapic::init();
    timer::init_bsp();
}

/// Initialises the APIC and timer on all CPUs (BSP and APs).
///
/// This is called after memory management and the device model are ready.
/// It:
/// 1. Calls `lapic::enable()` to enable the Local APIC (set SVR, mask LVTs).
/// 2. Calls `timer::init()` to calibrate the APIC timer.
///
/// After this function returns, interrupts are enabled globally.
pub fn init() {
    lapic::enable();
    timer::init();
}

// ============================================================================
// END OF INTERRUPT (EOI)
// ============================================================================

/// Sends an End‑Of‑Interrupt (EOI) to the Local APIC.
///
/// This function writes a zero to the EOI register, signalling that the
/// current interrupt has been handled. It must be called at the end of
/// every interrupt handler.
///
/// # Safety
/// This function performs a volatile write to an MMIO register.
#[inline(always)]
pub fn eoi() {
    *lapic::LocalApic::new().eoi() = 0;
}

// ============================================================================
// INTER‑PROCESSOR INTERRUPTS (IPIs)
// ============================================================================

/// Delivery modes for IPIs.
///
/// These are the bits that are OR‑ed into the ICR (Interrupt Command Register)
/// to specify the delivery semantics of the IPI.
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum DeliveryMode {
    /// Deliver the interrupt to the target processor(s).
    Fixed        = 0b000 << 8,
    /// Deliver to the processor with the lowest priority.
    LowestPri    = 0b001 << 8,
    /// System Management Interrupt.
    Smi          = 0b010 << 8,
    /// Non‑Maskable Interrupt.
    Nmi          = 0b100 << 8,
    /// INIT IPI (reset the target processor).
    Init         = 0b101 << 8,
    /// Startup IPI (used for AP boot).
    StartUp      = 0b110 << 8,
}

/// Constants for the Interrupt Command Register (ICR).
pub const ICR_LEVEL_ASSERT:  u32 = 1 << 14;   // Assert the interrupt (vs. deassert).
pub const ICR_DEST_MODE_PHYS: u32 = 0 << 11;  // Physical destination mode (APIC ID).
pub const ICR_DEST_MODE_LOG:  u32 = 1 << 11;  // Logical destination mode.

/// Sends an IPI to a target APIC ID.
///
/// This function:
/// 1. Waits for the ICR to become free (bit 12 of ICR low is cleared).
/// 2. Writes the target APIC ID to the ICR high register.
/// 3. Writes the vector, delivery mode, and flags to the ICR low register,
///    which sends the IPI.
///
/// # Arguments
/// * `target_apic_id` – The APIC ID of the target CPU.
/// * `vector` – The interrupt vector to deliver.
/// * `mode` – The delivery mode.
#[inline]
pub fn send_ipi(target_apic_id: u32, vector: u8, mode: DeliveryMode) {
    let lapic = lapic::LocalApic::new();

    // Wait for the ICR to be free (bit 12 is the "delivery status" bit).
    while (*lapic.iclo() & (1 << 12)) != 0 {
        core::hint::spin_loop();
    }

    // Set the target APIC ID in the high register.
    *lapic.ichi() = target_apic_id << 24;

    // Set the vector, mode, and flags in the low register.
    let icr_low = (vector as u32)
        |   (mode as u32)
        |   ICR_DEST_MODE_PHYS
        |   ICR_LEVEL_ASSERT;

    *lapic.iclo() = icr_low;
}

/// Sends a fixed IPI (delivery mode = Fixed) to a target APIC ID.
///
/// This is a convenience wrapper around `send_ipi` with `DeliveryMode::Fixed`.
///
/// # Arguments
/// * `target_apic_id` – The APIC ID of the target CPU.
/// * `vector` – The interrupt vector to deliver.
#[inline]
pub fn send_fixed_ipi(target_apic_id: u32, vector: u8) {
    send_ipi(target_apic_id, vector, DeliveryMode::Fixed);
}
