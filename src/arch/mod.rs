//! # Architecture‑Specific Code (x86_64)
//!
//! This module acts as the architecture abstraction layer for the kernel.
//! It re‑exports all architecture‑specific functionality from the `amd64`
//! submodule and provides a small set of generic helper functions that are
//! independent of the underlying CPU architecture.
//!
//! ## Overview
//!
//! The kernel is designed to be portable across multiple CPU architectures.
//! However, the current implementation targets only x86_64. The `arch` module
//! serves as a facade that abstracts the low‑level hardware details.
//! All architecture‑dependent code is contained within the `amd64` submodule,
//! and this module re‑exports its public interface.
//!
//! When porting the kernel to another architecture (e.g., ARM64, RISC‑V), this
//! module would conditionally include the appropriate submodule and provide
//! the same public API, allowing the rest of the kernel to remain architecture‑
//! agnostic.
//!
//! ## Re‑exports
//!
//! The module re‑exports all public items from `amd64`, including:
//! - CPU initialisation (`init_bsp`, `init_ap`, `early_init`)
//! - Interrupt handling (IDT, exceptions)
//! - Memory management (paging, per‑CPU data, GDT)
//! - System calls
//! - Timers (HPET, APIC)
//! - ACPI support
//! - Trap frame
//!
//! ## Helper Functions
//!
//! The module provides a simple `blocking_sleep` function that busy‑waits for
//! a given number of seconds. This is used for early‑boot delays and should be
//! replaced with a proper scheduler‑based sleep in the future.

// ============================================================================
// ARCHITECTURE SELECTION
// ============================================================================

#[cfg(target_arch = "x86_64")]
mod amd64;

// ============================================================================
// RE‑EXPORTS
// ============================================================================

#[cfg(target_arch = "x86_64")]
pub use amd64::*;

// ============================================================================
// GENERIC HELPERS
// ============================================================================

/// Busy‑waits for a given number of seconds.
///
/// This function is a simple blocking sleep that spins in a tight loop,
/// repeatedly checking the system time. It is intended for use during early
/// boot, before the scheduler is fully initialised and proper sleep primitives
/// are available.
///
/// # Arguments
/// * `s` – The number of seconds to sleep.
///
/// # Note
/// This function is not suitable for long‑duration sleeps or production use,
/// as it consumes CPU resources and does not yield to other tasks. It should
/// be replaced with a scheduler‑based sleep mechanism in the future.
///
/// # Examples
/// ```ignore
/// // Wait for 2 seconds during device initialisation.
/// crate::arch::blocking_sleep(2.0);
/// ```
pub fn blocking_sleep(s: f32) {
    let start = get_time_from_boot_s();
    while get_time_from_boot_s() < start + s {
        core::hint::spin_loop();
    }
}
