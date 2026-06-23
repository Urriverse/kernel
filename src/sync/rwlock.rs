//! # Read-Write Lock (RwLock)
//!
//! A spinlock‑based reader‑writer lock that allows multiple concurrent readers
//! or a single exclusive writer. This is a fundamental synchronization primitive
//! for data structures that are frequently read but only occasionally written.
//!
//! ## Overview
//!
//! The `RwLock<T>` provides two locking modes:
//! - **Read lock** (`read()`): Allows multiple readers to access the data
//!   concurrently. Readers spin while a writer holds the lock.
//! - **Write lock** (`write()`): Provides exclusive access to the data. Writers
//!   spin until no readers or writers hold the lock.
//!
//! The lock state is represented by a single `AtomicUsize`:
//! - `0` : Unlocked.
//! - `WRITER_BIT` (highest bit): Write‑locked.
//! - Any other value: Read‑locked, with the lower bits counting the number of
//!   active readers.
//!
//! ## Characteristics
//!
//! - **Spinlock‑based**: Both readers and writers spin (busy‑wait) until the
//!   lock is available. This is suitable for short critical sections.
//! - **Fairness**: No fairness guarantees; readers and writers contend equally.
//!   This could lead to writer starvation if there is a steady stream of readers.
//! - **Interrupts**: This lock does **not** disable interrupts. It is not safe
//!   to use in interrupt handlers if the same lock can be acquired in task context.
//!   For interrupt‑safe rwlocks, use `Nutex` or `Litex` wrapping a custom implementation.
//! - **No recursion**: Recursive locking is not supported. A writer attempting
//!   to acquire a read lock or vice versa will deadlock.
//!
//! ## Usage
//!
//! ```ignore
//! use crate::sync::RwLock;
//!
//! static DATA: RwLock<Vec<u32>> = RwLock::new(Vec::new());
//!
//! fn read_data() {
//!     let guard = DATA.read();
//!     println!("Length: {}", guard.len());
//! }
//!
//! fn write_data() {
//!     let mut guard = DATA.write();
//!     guard.push(42);
//! }
//! ```
//!
//! ## Safety
//!
//! - The lock is `Send` and `Sync` when `T` is `Send`.
//! - The `read()` and `write()` methods use atomic operations to manage the state.
//! - The lock is safe for use on multi‑CPU systems because all state updates
//!   are atomic with appropriate memory ordering.
//! - The `try_read()` and `try_write()` methods provide non‑blocking attempts.
//!
//! ## Implementation Details
//!
//! The lock state is stored in an `AtomicU64`:
//! - **Writer bit**: The most significant bit (`1 << (u64::BITS - 1)`).
//!   When set, the lock is write‑locked.
//! - **Reader count**: The lower bits (excluding the writer bit) store the
//!   number of active readers.
//!
//! ### Read Lock (shared)
//! 1. Spin while the writer bit is set.
//! 2. Atomically increment the reader count (with overflow check).
//! 3. If CAS fails, retry from step 1.
//!
//! ### Write Lock (exclusive)
//! 1. Spin while the state is not `0`.
//! 2. Atomically set the state to `WRITER_BIT`.
//! 3. If CAS fails, retry from step 1.
//!
//! ### Unlocking
//! - **Read unlock**: Decrement the reader count (`fetch_sub(1)`).
//! - **Write unlock**: Set the state to `0`.
//!
//! ## Comparison with Other Primitives
//!
//! | Primitive | Readers | Writers | Interrupt‑Safe | Use Case |
//! |-----------|---------|---------|----------------|----------|
//! | `Mutex`   | No      | Yes     | No             | Single exclusive access, not interrupt context. |
//! | `Nutex`   | No      | Yes     | Yes            | Exclusive access with interrupt safety. |
//! | `RwLock`  | Yes     | Yes     | No             | Read‑heavy workloads, not interrupt context. |
//! | `Litex`   | No      | Yes     | Yes            | Exclusive access with interrupt safety and unsafe inner. |

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicU64, Ordering},
    hint,
};

// ============================================================================
// CONSTANTS
// ============================================================================

/// The bit used to indicate that the lock is write‑locked.
/// This is the most significant bit of a `u64`.
const WRITER_BIT: u64 = 1 << (u64::BITS - 1);

// ============================================================================
// RWLOCK STRUCTURE
// ============================================================================

/// A reader‑writer lock that spins until the lock is available.
///
/// This lock allows multiple readers or a single writer at any time.
///
/// # Type Parameters
/// * `T` – The type of the data protected by the lock.
///
/// # Examples
/// ```ignore
/// static SHARED: RwLock<Vec<u32>> = RwLock::new(vec![]);
///
/// fn read() {
///     let guard = SHARED.read();
///     println!("Length: {}", guard.len());
/// }
///
/// fn write() {
///     let mut guard = SHARED.write();
///     guard.push(42);
/// }
/// ```
pub struct RwLock<T> {
    state: AtomicU64,
    data: UnsafeCell<T>,
}

// Safety: RwLock is Send and Sync if T is Send.
unsafe impl<T: Send> Send for RwLock<T> {}
unsafe impl<T: Send> Sync for RwLock<T> {}

#[allow(dead_code)]
impl<T> RwLock<T> {
    /// Creates a new `RwLock` in the unlocked state.
    ///
    /// # Arguments
    /// * `t` – The initial value to protect.
    pub const fn new(t: T) -> Self {
        Self {
            state: AtomicU64::new(0),
            data: UnsafeCell::new(t),
        }
    }

    /// Returns a mutable reference to the inner data **without locking**.
    ///
    /// # Safety
    /// This is unsafe because it bypasses the lock. The caller must ensure that
    /// no other code is accessing the data concurrently.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn inner(&self) -> &mut T {
        unsafe {
            self.data.as_mut_unchecked()
        }
    }

    /// Acquires a shared (read) lock, spinning until it is available.
    ///
    /// This function will spin in a loop until the lock can be acquired for reading.
    /// It returns a `RwLockReadGuard` that dereferences to `T` and releases the
    /// lock when dropped.
    ///
    /// # Returns
    /// A `RwLockReadGuard` that provides shared access to the data.
    ///
    /// # Deadlocks
    /// This function will deadlock if the lock is already held by the current
    /// thread for writing.
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        loop {
            let old = self.state.load(Ordering::Acquire);
            // If the writer bit is set, spin.
            if old & WRITER_BIT != 0 {
                hint::spin_loop();
                continue;
            }
            // Try to increment the reader count.
            let new = old + 1;
            // The increment must not overflow into the writer bit.
            debug_assert!(new & WRITER_BIT == 0);
            if self
                .state
                .compare_exchange_weak(old, new, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
            {
                return RwLockReadGuard { lock: self };
            }
            // CAS failed, retry.
        }
    }

    /// Attempts to acquire a shared (read) lock without spinning.
    ///
    /// If the lock is available for reading, it is acquired and a guard is returned.
    /// Otherwise, `None` is returned immediately.
    ///
    /// # Returns
    /// `Some(RwLockReadGuard)` if the lock was acquired, `None` otherwise.
    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        let old = self.state.load(Ordering::Acquire);
        if old & WRITER_BIT != 0 {
            return None;
        }
        let new = old + 1;
        if new & WRITER_BIT == 0
            && self
                .state
                .compare_exchange(old, new, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
        {
            Some(RwLockReadGuard { lock: self })
        } else {
            None
        }
    }

    /// Acquires an exclusive (write) lock, spinning until it is available.
    ///
    /// This function will spin in a loop until the lock can be acquired for writing.
    /// It returns a `RwLockWriteGuard` that dereferences to `&mut T` and releases
    /// the lock when dropped.
    ///
    /// # Returns
    /// A `RwLockWriteGuard` that provides exclusive access to the data.
    ///
    /// # Deadlocks
    /// This function will deadlock if the lock is already held by the current
    /// thread for reading or writing.
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        loop {
            let old = self.state.load(Ordering::Acquire);
            // If the lock is not free, spin.
            if old != 0 {
                hint::spin_loop();
                continue;
            }
            // Try to set the writer bit.
            if self
                .state
                .compare_exchange_weak(0, WRITER_BIT, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
            {
                return RwLockWriteGuard { lock: self };
            }
        }
    }

    /// Attempts to acquire an exclusive (write) lock without spinning.
    ///
    /// If the lock is free, it is acquired and a guard is returned.
    /// Otherwise, `None` is returned immediately.
    ///
    /// # Returns
    /// `Some(RwLockWriteGuard)` if the lock was acquired, `None` otherwise.
    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        let old = self.state.load(Ordering::Acquire);
        if old != 0 {
            return None;
        }
        if self
            .state
            .compare_exchange(0, WRITER_BIT, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
        {
            Some(RwLockWriteGuard { lock: self })
        } else {
            None
        }
    }
}

// ============================================================================
// READ GUARD
// ============================================================================

/// A guard that holds a shared (read) lock on an `RwLock`.
///
/// The guard dereferences to `T` (via `Deref`) and releases the lock when dropped.
pub struct RwLockReadGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<T> core::ops::Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    /// Returns a reference to the protected data.
    ///
    /// # Safety
    /// The read lock is held, so the data is not being mutated.
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> Drop for RwLockReadGuard<'_, T> {
    /// Releases the read lock by decrementing the reader count.
    fn drop(&mut self) {
        // Decrement the reader count with Release ordering to ensure that all
        // reads are visible before any future writes.
        self.lock.state.fetch_sub(1, Ordering::Release);
    }
}

// ============================================================================
// WRITE GUARD
// ============================================================================

/// A guard that holds an exclusive (write) lock on an `RwLock`.
///
/// The guard dereferences to `&mut T` (via `DerefMut`) and releases the lock
/// when dropped.
pub struct RwLockWriteGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<T> core::ops::Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    /// Returns a reference to the protected data.
    ///
    /// # Safety
    /// The write lock is held exclusively, so no other access is possible.
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> core::ops::DerefMut for RwLockWriteGuard<'_, T> {
    /// Returns a mutable reference to the protected data.
    ///
    /// # Safety
    /// The write lock is held exclusively, so mutation is safe.
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for RwLockWriteGuard<'_, T> {
    /// Releases the write lock by clearing the state.
    fn drop(&mut self) {
        // Release the write lock with Release ordering.
        self.lock.state.store(0, Ordering::Release);
    }
}

// ============================================================================
// TESTS (only run in std environment)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use core::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn smoke() {
        let lock = RwLock::new(42);
        {
            let r = lock.read();
            assert_eq!(*r, 42);
        }
        {
            let mut w = lock.write();
            *w = 43;
        }
        assert_eq!(*lock.read(), 43);
    }

    #[test]
    fn try_lock() {
        let lock = RwLock::new(0);
        assert!(lock.try_read().is_some());
        assert!(lock.try_write().is_some());
        // Write lock held
        let _w = lock.write();
        assert!(lock.try_read().is_none());
        assert!(lock.try_write().is_none());
        drop(_w);
        // Read lock held
        let _r = lock.read();
        assert!(lock.try_read().is_some());
        assert!(lock.try_write().is_none());
    }

    #[test]
    fn concurrent_readers() {
        let lock = RwLock::new(0);
        let counter = AtomicU32::new(0);
        let mut handles = vec![];
        for _ in 0..10 {
            let lock = &lock;
            let counter = &counter;
            handles.push(std::thread::spawn(move || {
                for _ in 0..100 {
                    let _g = lock.read();
                    counter.fetch_add(1, Ordering::Relaxed);
                    // Simulate some work
                    core::hint::spin_loop();
                    counter.fetch_sub(1, Ordering::Relaxed);
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn writer_exclusive() {
        let lock = RwLock::new(0);
        let counter = AtomicU32::new(0);
        let mut handles = vec![];
        // One writer
        handles.push(std::thread::spawn(move || {
            for _ in 0..50 {
                let _g = lock.write();
                counter.fetch_add(1, Ordering::Relaxed);
                core::hint::spin_loop();
                counter.fetch_sub(1, Ordering::Relaxed);
            }
        }));
        // Several readers
        for _ in 0..5 {
            let lock = &lock;
            let counter = &counter;
            handles.push(std::thread::spawn(move || {
                for _ in 0..50 {
                    let _g = lock.read();
                    // Readers should see counter at 0 if writer exclusive
                    assert_eq!(counter.load(Ordering::Relaxed), 0);
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
    }
}
