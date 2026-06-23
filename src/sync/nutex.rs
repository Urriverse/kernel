//! # Interrupt-Disabling Mutex (Nutex)
//!
//! A mutual exclusion primitive that **disables interrupts** while the lock is held.
//! This ensures that the critical section is not interrupted, making it safe to use
//! in interrupt handlers and other contexts where preemption must be prevented.
//!
//! ## Overview
//!
//! The `Nutex` (Non‑preemptive Mutex) is a spinlock‑based mutex that disables
//! interrupts on the current CPU when the lock is acquired, and restores them
//! when the lock is released. This guarantees that:
//! - The critical section is atomic with respect to interrupts on the current CPU.
//! - No interrupt handler can run while the lock is held, preventing deadlocks
//!   where an interrupt handler tries to acquire the same lock.
//! - The lock is safe to use from both task context and interrupt context.
//!
//! ## Characteristics
//!
//! - **Interrupt disabling**: Interrupts are disabled (`cli`) when the lock is
//!   acquired and restored to their previous state when the lock is released.
//! - **Spinlock**: If the lock is already held, the CPU spins in a tight loop
//!   (`spin_loop`) until the lock is released.
//! - **IRQ state restoration**: The `NutexGuard` saves the previous interrupt
//!   state (whether interrupts were enabled) and restores it on drop. This
//!   ensures that interrupts are only disabled if they were enabled before,
//!   and are not incorrectly left disabled.
//! - **No recursion**: The `Nutex` does not support recursive locking. Attempting
//!   to lock the same `Nutex` from the same CPU will cause a deadlock.
//!
//! ## Usage
//!
//! ```ignore
//! use crate::sync::Nutex;
//!
//! static SHARED_DATA: Nutex<u32> = Nutex::new(0);
//!
//! fn critical_section() {
//!     let mut guard = SHARED_DATA.lock();
//!     *guard += 1;  // Interrupts are disabled here
//!     // Guard is dropped, interrupts are restored
//! }
//! ```
//!
//! ## Safety
//!
//! - This mutex is `Send` and `Sync` when `T` is `Send`.
//! - The lock is safe to use from multiple CPUs, but the user must ensure that
//!   the protected data is not accessed concurrently outside the lock.
//! - Interrupts are disabled on the current CPU, but other CPUs are not affected.
//! - The `lock` and `try_lock` methods use `cli` to disable interrupts, and the
//!   guard restores the previous interrupt state. This is safe because:
//!   - Interrupts are not re‑enabled until the guard is dropped.
//!   - The saved state ensures that interrupts are not incorrectly enabled if
//!     they were disabled before locking.
//!
//! ## Comparison with Other Synchronization Primitives
//!
//! | Primitive | Interrupts Disabled | Use Case |
//! |-----------|---------------------|----------|
//! | `Mutex`   | No                  | Data shared between tasks (threads) on multiple CPUs, but not in interrupt context. |
//! | `Nutex`   | Yes                 | Data that can be accessed from both task and interrupt context. |
//! | `Litex`   | Yes                 | Similar to `Nutex` but with an `unsafe inner()` method for raw access. |
//! | `Nitex`   | Yes                 | A lock‑free (or rather, interrupt‑only) mutex that only disables interrupts, no spinning. Actually `Nitex` does not spin; it just disables interrupts. |
//!
//! ## Implementation Details
//!
//! The lock uses `AtomicBool` as the spinlock flag. On acquisition:
//! 1. The current interrupt state is saved using `pushfq`/`pop`.
//! 2. Interrupts are disabled with `cli`.
//! 3. The lock flag is set to `true` using a spinloop with `compare_exchange_weak`.
//! 4. On release, the lock flag is cleared (`store(false)`), and the saved
//!    interrupt state is restored (if interrupts were previously enabled, `sti` is
//!    executed).

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
    arch::asm
};

// ============================================================================
// NUTEX STRUCTURE
// ============================================================================

/// An interrupt‑disabling spinlock mutex.
///
/// This mutex disables interrupts when locked and restores them when unlocked.
/// It is safe to use in interrupt handlers and task contexts.
///
/// # Type Parameters
/// * `T` – The type of the data protected by the mutex.
///
/// # Examples
/// ```ignore
/// static COUNTER: Nutex<usize> = Nutex::new(0);
///
/// fn increment() {
///     let mut guard = COUNTER.lock();
///     *guard += 1;
/// }
/// ```
#[derive(Debug)]
pub struct Nutex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

// Safety: Nutex is Send and Sync if T is Send.
unsafe impl<T: Send> Send for Nutex<T> {}
unsafe impl<T: Send> Sync for Nutex<T> {}

impl<T> Nutex<T> {
    /// Creates a new `Nutex` in the unlocked state.
    ///
    /// # Arguments
    /// * `t` – The initial value to protect.
    pub const fn new(t: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(t),
        }
    }

    /// Acquires the lock, disabling interrupts and spinning until the lock is available.
    ///
    /// This function:
    /// 1. Saves the current interrupt state (whether interrupts were enabled).
    /// 2. Disables interrupts with `cli`.
    /// 3. Spins until the lock flag is acquired.
    /// 4. Returns a guard that will release the lock and restore the interrupt
    ///    state when dropped.
    ///
    /// # Returns
    /// A `NutexGuard` that dereferences to the protected data.
    ///
    /// # Deadlocks
    /// This function will deadlock if the lock is already held by the current CPU.
    pub fn lock(&self) -> NutexGuard<'_, T> {
        // Save the current interrupt state before we disable interrupts.
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

        // Spin until we acquire the lock.
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }

        NutexGuard {
            mutex: self,
            saved_if: (rflags & (1 << 9)) != 0,  // Bit 9 is the IF flag in RFLAGS.
        }
    }

    /// Attempts to acquire the lock without spinning.
    ///
    /// If the lock is free, it is acquired, interrupts are disabled, and a guard
    /// is returned. If the lock is already held, `None` is returned immediately.
    ///
    /// # Returns
    /// `Some(NutexGuard)` if the lock was acquired, `None` otherwise.
    #[allow(dead_code)]
    pub fn try_lock(&self) -> Option<NutexGuard<'_, T>> {
        // Save the current interrupt state and disable interrupts.
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

        // Try to acquire the lock with a single CAS attempt.
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(NutexGuard {
                mutex: self,
                saved_if: (rflags & (1 << 9)) != 0,
            })
        } else {
            unsafe {
                if (rflags & (1 << 9)) != 0 {
                    asm!("sti", options(nomem, nostack, preserves_flags));
                }
            }
            None
        }
    }
}

// ============================================================================
// GUARD STRUCTURE
// ============================================================================

/// A guard that holds the `Nutex` lock and provides access to the protected data.
///
/// The guard dereferences to `T` (via `Deref` and `DerefMut`) and releases the
/// lock and restores the interrupt state when dropped.
pub struct NutexGuard<'a, T> {
    mutex: &'a Nutex<T>,
    saved_if: bool,  // Whether interrupts were enabled before locking.
}

impl<T> core::ops::Deref for NutexGuard<'_, T> {
    type Target = T;

    /// Returns a reference to the protected data.
    ///
    /// # Safety
    /// The lock is held and interrupts are disabled, so the data is not being
    /// mutated by other threads or interrupt handlers.
    fn deref(&self) -> &T {
        unsafe { self.mutex.data.get().as_ref_unchecked() }
    }
}

impl<T> core::ops::DerefMut for NutexGuard<'_, T> {
    /// Returns a mutable reference to the protected data.
    ///
    /// # Safety
    /// The lock is held exclusively and interrupts are disabled.
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for NutexGuard<'_, T> {
    /// Releases the lock and restores the interrupt state.
    ///
    /// When the guard is dropped:
    /// 1. The lock flag is cleared with `Release` ordering.
    /// 2. If interrupts were enabled before locking, `sti` is executed.
    fn drop(&mut self) {
        // Release the lock.
        self.mutex.lock.store(false, Ordering::Release);

        // Restore the interrupt state if it was previously enabled.
        unsafe {
            if self.saved_if {
                asm!("sti", options(nomem, nostack, preserves_flags));
            }
        }
    }
}
