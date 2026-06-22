//! # Litex – Interrupt-Disabling Spinlock with Unsafe Inner Access
//!
//! A mutual exclusion primitive that combines the features of `Nutex` (interrupt
//! disabling) with an additional `unsafe inner()` method for raw access to the
//! protected data without locking. It is a spinlock that disables interrupts
//! when acquired and restores them when released.
//!
//! ## Overview
//!
//! The `Litex` is very similar to `Nutex` but provides an `unsafe inner()` method
//! that returns a mutable reference to the data without acquiring the lock. This
//! is useful in situations where the caller can guarantee that no other code is
//! accessing the data concurrently (e.g., during early boot, or when the lock is
//! already held and the caller wants to bypass the guard).
//!
//! ## Characteristics
//!
//! - **Interrupt disabling**: Disables interrupts (`cli`) on lock acquisition and
//!   restores the previous state on release.
//! - **Spinlock**: Spins in a tight loop (`spin_loop`) if the lock is already held.
//! - **Unsafe inner access**: Provides an `unsafe inner()` method to get a mutable
//!   reference to the data without locking. This is useful for initialization or
//!   when the lock is already known to be held.
//! - **No recursion**: Does not support recursive locking; attempting to lock the
//!   same `Litex` from the same CPU will cause a deadlock.
//!
//! ## Usage
//!
//! ```ignore
//! use crate::sync::Litex;
//!
//! static SHARED: Litex<u32> = Litex::new(0);
//!
//! // Safe locking:
//! let mut guard = SHARED.lock();
//! *guard += 1;
//!
//! // Unsafe inner access (only safe if no concurrent access):
//! unsafe {
//!     *SHARED.inner() = 42;
//! }
//! ```
//!
//! ## Safety
//!
//! - The `inner()` method is unsafe because it bypasses the lock. The caller must
//!   ensure that no other code is accessing the data concurrently (e.g., during
//!   early boot, or when the lock is already held).
//! - The lock is `Send` and `Sync` when `T` is `Send`.
//! - Interrupts are disabled on the current CPU, but other CPUs are not affected.
//! - The saved interrupt state ensures that interrupts are restored correctly
//!   on drop, even if they were previously disabled.
//!
//! ## Comparison with Other Synchronization Primitives
//!
//! | Primitive | Interrupts Disabled | Spins | Unsafe Inner | Multi‑CPU Safe |
//! |-----------|---------------------|-------|--------------|----------------|
//! | `Mutex`   | No                  | Yes   | No           | Yes            |
//! | `Nutex`   | Yes                 | Yes   | No           | Yes            |
//! | `Litex`   | Yes                 | Yes   | Yes          | Yes            |
//! | `Nitex`   | Yes                 | No    | Yes          | No (per‑CPU)   |
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
//!
//! The `inner()` method simply returns a mutable reference to the `UnsafeCell`
//! data without any atomic operations.

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
    arch::asm
};

// ============================================================================
// LITEX STRUCTURE
// ============================================================================

/// An interrupt‑disabling spinlock with unsafe inner access.
///
/// # Type Parameters
/// * `T` – The type of the data protected by the lock.
///
/// # Examples
/// ```ignore
/// static COUNTER: Litex<usize> = Litex::new(0);
///
/// fn increment() {
///     let mut guard = COUNTER.lock();
///     *guard += 1;
/// }
///
/// // Unsafe, but safe if called during early boot:
/// unsafe {
///     *COUNTER.inner() = 100;
/// }
/// ```
#[derive(Debug)]
pub struct Litex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

// Safety: Litex is Send and Sync if T is Send.
unsafe impl<T: Send> Send for Litex<T> {}
unsafe impl<T: Send> Sync for Litex<T> {}

impl<T> Litex<T> {
    /// Creates a new `Litex` in the unlocked state.
    ///
    /// # Arguments
    /// * `t` – The initial value to protect.
    pub const fn new(t: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(t),
        }
    }

    /// Returns a mutable reference to the inner data **without locking**.
    ///
    /// # Safety
    /// This is unsafe because it bypasses the lock. The caller must ensure that
    /// no other code is accessing the data concurrently. This is typically safe
    /// during early boot (single‑threaded) or when the lock is already held.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn inner(&self) -> &mut T {
        unsafe {
            self.data.as_mut_unchecked()
        }
    }

    /// Acquires the lock, disabling interrupts and spinning until the lock is available.
    ///
    /// This function:
    /// 1. Saves the current interrupt state.
    /// 2. Disables interrupts with `cli`.
    /// 3. Spins until the lock flag is acquired.
    /// 4. Returns a guard that will release the lock and restore the interrupt
    ///    state when dropped.
    ///
    /// # Returns
    /// A `LitexGuard` that dereferences to the protected data.
    ///
    /// # Deadlocks
    /// This function will deadlock if the lock is already held by the current CPU.
    pub fn lock(&self) -> LitexGuard<'_, T> {
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

        LitexGuard {
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
    /// `Some(LitexGuard)` if the lock was acquired, `None` otherwise.
    pub fn try_lock(&self) -> Option<LitexGuard<'_, T>> {
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
            
            Some(LitexGuard {
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

/// A guard that holds the `Litex` lock and provides access to the protected data.
///
/// The guard dereferences to `T` (via `Deref` and `DerefMut`) and releases the
/// lock and restores the interrupt state when dropped.
pub struct LitexGuard<'a, T> {
    mutex: &'a Litex<T>,
    saved_if: bool,  // Whether interrupts were enabled before locking.
}

impl<T> core::ops::Deref for LitexGuard<'_, T> {
    type Target = T;

    /// Returns a reference to the protected data.
    ///
    /// # Safety
    /// The lock is held and interrupts are disabled, so the data is safe to read.
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> core::ops::DerefMut for LitexGuard<'_, T> {
    /// Returns a mutable reference to the protected data.
    ///
    /// # Safety
    /// The lock is held exclusively and interrupts are disabled.
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for LitexGuard<'_, T> {
    /// Releases the lock and restores the interrupt state.
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
