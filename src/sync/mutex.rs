//! # Simple Spinlock (Mutex)
//!
//! This module provides a basic spinlock‑based mutual exclusion primitive,
//! `Mutex<T>`. It is a straightforward implementation that uses a single
//! `AtomicBool` as the lock flag and spins (busy‑waits) when the lock is
//! already held.
//!
//! ## Overview
//!
//! The `Mutex<T>` is the simplest locking primitive in the kernel. It is
//! suitable for protecting data that is accessed from multiple CPU cores but
//! where the critical sections are short and the lock is not held for long
//! periods.
//!
//! Unlike `Nutex` and `Litex`, this mutex **does not disable interrupts**.
//! This makes it unsuitable for use in interrupt handlers or in contexts where
//! the lock might be taken by an interrupt handler that interrupts a lock‑holder.
//! For such cases, use `Nutex` or `Litex`.
//!
//! ## Characteristics
//!
//! - **Spinlock**: The lock spins in a tight loop (`spin_loop`) until the lock
//!   is acquired.
//! - **No interrupt disabling**: Interrupts remain enabled while the lock is
//!   held. This means the lock is not safe for use in interrupt handlers or
//!   with code that can be preempted by interrupts.
//! - **Fairness**: No fairness guarantees; it is a simple test‑and‑set lock.
//! - **Atomic operations**: Uses `AtomicBool` with `Acquire`/`Release` ordering
//!   to ensure memory visibility.
//!
//! ## Usage
//!
//! ```ignore
//! use crate::sync::Mutex;
//!
//! static MY_DATA: Mutex<u32> = Mutex::new(0);
//!
//! fn increment() {
//!     let mut guard = MY_DATA.lock();
//!     *guard += 1;
//! }
//! ```
//!
//! The `lock()` method returns a `MutexGuard` that dereferences to the inner
//! data. When the guard goes out of scope, the lock is automatically released.
//!
//! ## Safety
//!
//! - This mutex is `Send` and `Sync` when `T` is `Send`.
//! - The lock is safe to use from multiple CPUs, but the user must ensure that
//!   the protected data is not accessed concurrently outside the lock.
//! - Because interrupts are not disabled, the lock must not be used in code
//!   that can be preempted by interrupt handlers that also try to acquire the
//!   same lock (deadlock risk).
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
//! The lock uses `compare_exchange_weak` in a loop to attempt to set the flag
//! from `false` to `true`. If it fails, it calls `spin_loop` to yield the CPU
//! briefly before retrying. The `try_lock` method attempts a single CAS and
//! returns `None` if it fails.

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering}
};

// ============================================================================
// MUTEX STRUCTURE
// ============================================================================

/// A spinlock‑based mutual exclusion primitive.
///
/// This struct protects a value of type `T` with a spinlock. The lock is
/// acquired by calling `lock()`, which returns a guard that releases the lock
/// when dropped.
///
/// # Type Parameters
/// * `T` – The type of the data protected by the mutex.
///
/// # Examples
/// ```ignore
/// static COUNTER: Mutex<usize> = Mutex::new(0);
///
/// fn increment() {
///     let mut guard = COUNTER.lock();
///     *guard += 1;
/// }
/// ```
#[allow(dead_code)]
pub struct Mutex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

// Safety: Mutex is Send and Sync if T is Send.
unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

#[allow(dead_code)]
impl<T> Mutex<T> {
    /// Creates a new `Mutex` in the unlocked state.
    ///
    /// # Arguments
    /// * `t` – The initial value to protect.
    pub const fn new(t: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(t),
        }
    }

    /// Acquires the lock, spinning until it is available.
    ///
    /// This function will spin in a loop until the lock is acquired. Once
    /// acquired, it returns a `MutexGuard` that provides access to the data.
    ///
    /// # Returns
    /// A `MutexGuard` that dereferences to the protected data.
    ///
    /// # Panics
    /// This function does not panic.
    ///
    /// # Deadlocks
    /// This function will deadlock if the lock is already held by the current
    /// CPU (i.e., recursive locking is not supported).
    pub fn lock(&self) -> MutexGuard<'_, T> {
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }

        MutexGuard { mutex: self }
    }

    /// Attempts to acquire the lock without spinning.
    ///
    /// If the lock is currently free, it is acquired and a guard is returned.
    /// Otherwise, `None` is returned.
    ///
    /// # Returns
    /// `Some(MutexGuard)` if the lock was acquired, `None` otherwise.
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(MutexGuard { mutex: self })
        } else {
            None
        }
    }
}

// ============================================================================
// GUARD STRUCTURE
// ============================================================================

/// A guard that holds the mutex lock and provides access to the protected data.
///
/// The guard dereferences to `T` (via `Deref` and `DerefMut`) and releases the
/// lock when dropped.
pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> core::ops::Deref for MutexGuard<'_, T> {
    type Target = T;

    /// Returns a reference to the protected data.
    ///
    /// # Safety
    /// The lock is held, so the data is not being mutated by other threads.
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> core::ops::DerefMut for MutexGuard<'_, T> {
    /// Returns a mutable reference to the protected data.
    ///
    /// # Safety
    /// The lock is held exclusively, so no other thread can access the data.
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    /// Releases the lock.
    ///
    /// When the guard is dropped, the lock flag is set to `false` with `Release`
    /// ordering, ensuring that all writes to the protected data are visible
    /// to the next lock acquirer.
    fn drop(&mut self) {
        self.mutex.lock.store(false, Ordering::Release);
    }
}
