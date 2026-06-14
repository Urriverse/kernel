//! A plain spinlock mutex.
//!
//! [`Mutex`] provides mutual exclusion using a spinlock. It does **not**
//! manipulate interrupts. This is suitable for synchronising data that is
//! never accessed from interrupt handlers.

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
    arch::asm
};

/// A spinlock‑based mutex.
///
/// The lock is acquired by spinning until the inner `AtomicBool` becomes
/// `false`, then setting it to `true` with `Acquire` ordering.
pub struct Mutex<T>
{
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T>
{
    /// Create a new `Mutex` containing `t`.
    pub const fn new(t: T) -> Self
    {
        Self
        {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(t),
        }
    }

    /// Acquire the lock, spinning until it becomes available.
    pub fn lock(&self) -> MutexGuard<'_, T>
    {
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            unsafe { asm!("pause", options(nomem, nostack, preserves_flags)) };
        }

        MutexGuard { mutex: self }
    }

    /// Attempt to acquire the lock without spinning.
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(MutexGuard { mutex: self })
        }
        else
        {
            None
        }
    }
}

/// A guard that holds the lock and provides access to the protected data.
///
/// The lock is released when this guard is dropped.
pub struct MutexGuard<'a, T>
{
    mutex: &'a Mutex<T>,
}

impl<T> core::ops::Deref for MutexGuard<'_, T>
{
    type Target = T;
    fn deref(&self) -> &T
    {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> core::ops::DerefMut for MutexGuard<'_, T>
{
    fn deref_mut(&mut self) -> &mut T
    {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T>
{
    fn drop(&mut self)
    {
        self.mutex.lock.store(false, Ordering::Release);
    }
}
