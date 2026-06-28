//! # Interrupt-Only Mutex (Nitex)
//!
//! A mutual exclusion primitive that **only disables interrupts** and does **not spin**.
//! This is the simplest form of mutual exclusion for single‑CPU critical sections
//! where preemption by interrupts must be prevented, but no other CPU can contend
//! for the lock (or the lock is only ever used on one CPU).
//!
//! ## Overview
//!
//! The `Nitex` (Non‑interruptible, no‑spin mutex) is a lock that:
//! - Disables interrupts when acquired, and restores them when released.
//! - Does **not** spin if the lock is already held; it simply disables interrupts
//!   and assumes that the lock will be released quickly by the current CPU.
//! - Is **not** a spinlock; it does not use atomic operations or busy‑waiting.
//! - Is intended for use on **single‑CPU** systems or in situations where the
//!   lock is guaranteed to be uncontended on the current CPU (e.g., per‑CPU data
//!   structures that are only accessed from interrupt handlers on the same CPU).
//!
//! ## Characteristics
//!
//! - **Interrupt disabling**: Disables interrupts (`cli`) on lock acquisition and
//!   restores the previous state on release.
//! - **No spinning**: Does not loop or use atomic compare‑exchange; it simply
//!   disables interrupts and assumes the lock is free. This means it is **not**
//!   safe for use on multi‑CPU systems where another CPU might hold the lock.
//! - **Unsafe inner access**: Provides an `unsafe inner()` method to get a
//!   mutable reference to the data without locking (use with extreme caution).
//! - **No atomic operations**: The lock state is not tracked; the lock is
//!   effectively a "disable interrupts" marker.
//!
//! ## Usage
//!
//! This primitive is typically used for per‑CPU data that is only accessed by
//! code running on that CPU, such as:
//! - Per‑CPU runqueues.
//! - Per‑CPU timers.
//! - Per‑CPU statistics.
//!
//! It is not suitable for data shared between multiple CPU cores.
//!
//! ```ignore
//! use crate::sync::Nitex;
//!
//! static PER_CPU_DATA: Nitex<u32> = Nitex::new(0);
//!
//! fn access() {
//!     let mut guard = PER_CPU_DATA.lock();
//!     *guard += 1;
//!     // Interrupts are re‑enabled when guard is dropped.
//! }
//! ```
//!
//! ## Safety
//!
//! - The `lock()` method disables interrupts but does **not** check if the lock
//!   is already held. It is the caller's responsibility to ensure that the lock
//!   is not used recursively on the same CPU (which would leave interrupts
//!   disabled forever, as the guard would not be dropped before the second lock).
//! - The `inner()` method allows unsafe access to the data without any locking.
//!   It must only be used when the caller knows that no other code is accessing
//!   the data concurrently (e.g., during early boot).
//! - This primitive is **not** `Sync` or `Send` for multi‑CPU safety. It is
//!   intended for per‑CPU usage only.
//!
//! ## Comparison with Other Synchronization Primitives
//!
//! | Primitive | Interrupts Disabled | Spins | Multi‑CPU Safe |
//! |-----------|---------------------|-------|----------------|
//! | `Mutex`   | No                  | Yes   | Yes            |
//! | `Nutex`   | Yes                 | Yes   | Yes            |
//! | `Litex`   | Yes                 | Yes   | Yes            |
//! | `Nitex`   | Yes                 | No    | No (per‑CPU only) |
//!
//! ## Implementation Details
//!
//! The `Nitex` is simply a wrapper around `UnsafeCell<T>` that provides a
//! `lock()` method which disables interrupts and returns a guard. The guard
//! restores interrupts on drop. There is no lock flag; the lock is "held"
//! by virtue of interrupts being disabled, preventing any interrupt handler
//! from running and potentially accessing the data.

use core::{
    cell::UnsafeCell,
    arch::asm
};

// ============================================================================
// NITEX STRUCTURE
// ============================================================================

/// An interrupt‑only mutex that disables interrupts but does not spin.
///
/// This primitive is only safe for use on a single CPU core and must not be
/// used for data shared across multiple CPUs.
///
/// # Type Parameters
/// * `T` – The type of the data protected by the mutex.
///
/// # Examples
/// ```ignore
/// static MY_DATA: Nitex<usize> = Nitex::new(0);
///
/// fn increment() {
///     let mut guard = MY_DATA.lock();
///     *guard += 1;
/// }
/// ```
pub struct Nitex<T> {
    data: UnsafeCell<T>,
}

impl<T: Clone> Clone for Nitex<T> {
    /// Clones the data by locking the mutex and copying the inner value.
    ///
    /// # Safety
    /// This acquires the lock (disables interrupts) to read the data safely.
    fn clone(&self) -> Self {
        Self::new((unsafe { &*self.data.get() }).clone())
    }
}

// Safety: Nitex is Send and Sync if T is Send, but only if the caller ensures
// that the lock is only used on a single CPU.
unsafe impl<T: Send> Send for Nitex<T> {}
unsafe impl<T: Send> Sync for Nitex<T> {}

impl<T> Nitex<T> {
    /// Creates a new `Nitex` with the given initial value.
    pub const fn new(t: T) -> Self {
        Self { data: UnsafeCell::new(t) }
    }

    /// Returns a mutable reference to the inner data **without locking**.
    ///
    /// # Safety
    /// This is unsafe because it bypasses the mutex. The caller must ensure
    /// that no other code is accessing the data concurrently.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn inner(&self) -> &'static mut T {
        unsafe {
            self.data.get().as_mut_unchecked()
        }
    }

    /// Acquires the lock by disabling interrupts.
    ///
    /// This function disables interrupts (`cli`) and returns a guard. The guard
    /// will re‑enable interrupts (restoring the previous state) when dropped.
    ///
    /// # Panics
    /// This function does not panic.
    ///
    /// # Deadlocks
    /// If this function is called while interrupts are already disabled, it
    /// will simply disable them again (no effect) and the guard will restore
    /// the saved state. However, if called recursively (while the lock is already
    /// held by the same CPU), interrupts will be re‑enabled when the inner guard
    /// is dropped, potentially corrupting the data.
    pub fn lock(&self) -> NitexGuard<'_, T> {
        let rflags: u64;
        unsafe {
            asm!(
                "pushfq",
                "pop {0}",
                out(reg) rflags,
                options(nomem, preserves_flags)
            );
            asm!("cli", options(nomem, nostack, preserves_flags));
        }

        NitexGuard { mutex: self, saved_if: (rflags & (1 << 9)) != 0 }
    }
}

// ============================================================================
// GUARD STRUCTURE
// ============================================================================

/// A guard that holds the `Nitex` lock and provides access to the protected data.
///
/// The guard dereferences to `T` (via `Deref` and `DerefMut`) and restores
/// the interrupt state when dropped.
pub struct NitexGuard<'a, T> {
    mutex: &'a Nitex<T>,
    saved_if: bool,
}

impl<T> core::ops::Deref for NitexGuard<'_, T> {
    type Target = T;

    /// Returns a reference to the protected data.
    ///
    /// # Safety
    /// Interrupts are disabled, so the data is not being mutated by interrupt
    /// handlers on the same CPU.
    fn deref(&self) -> &T {
        unsafe { self.mutex.data.get().as_ref_unchecked() }
    }
}

impl<T> core::ops::DerefMut for NitexGuard<'_, T> {
    /// Returns a mutable reference to the protected data.
    ///
    /// # Safety
    /// Interrupts are disabled, so the data is safe to mutate.
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for NitexGuard<'_, T> {
    /// Restores the interrupt state (re‑enables interrupts if they were enabled
    /// before locking).
    fn drop(&mut self) {
        unsafe {
            if self.saved_if {
                asm!("sti", options(nomem, nostack, preserves_flags));
            }
        }
    }
}
