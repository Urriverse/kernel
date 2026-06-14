//! A spinlock mutex that disables interrupts while the lock is held.
//!
//! [`Nutex`] (No‑Interrupts Mutex) is a mutual exclusion primitive that uses
//! a spinlock and additionally saves and disables interrupts (IF flag) when
//! acquiring the lock. Interrupts are restored on drop.
//!
//! This is useful for synchronising data that is also accessed from interrupt
//! handlers.

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
    arch::asm
};

/// A mutual exclusion primitive that disables interrupts while locked.
///
/// The lock is acquired by spinning until the inner `AtomicBool` becomes
/// `false`, then setting it to `true` with `Acquire` ordering. While the lock
/// is held, interrupts are disabled (`cli`). On unlock, interrupts are
/// restored to their previous state.
pub struct Nutex<T>
{
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Nutex<T> {}
unsafe impl<T: Send> Sync for Nutex<T> {}

impl<T> Nutex<T>
{
    /// Create a new `Nutex` containing `t`.
    pub const fn new(t: T) -> Self
    {
        Self
        {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(t),
        }
    }

    /// Acquire the lock, disabling interrupts.
    ///
    /// If the lock is already held, this function spins until it becomes
    /// available.
    pub fn lock(&self) -> NutexGuard<'_, T>
    {
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            unsafe { asm!("pause", options(nomem, nostack, preserves_flags)) };
        }

        let rflags: u64;
        unsafe
        {
            asm!(
                "pushfq",
                "pop {0}",
                out(reg) rflags,
                options(nomem, preserves_flags)
            );
            asm!("cli", options(nomem, nostack, preserves_flags));
        }

        NutexGuard
        {
            mutex: self,
            saved_if: (rflags & (1 << 9)) != 0,
        }
    }

    /// Attempt to acquire the lock without spinning.
    ///
    /// Returns `Some(guard)` if the lock was acquired, otherwise `None`.
    /// Interrupts are disabled if the lock is acquired.
    pub fn try_lock(&self) -> Option<NutexGuard<'_, T>> {
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            let rflags: u64;
            unsafe
            {
                asm!(
                    "pushfq",
                    "pop {0}",
                    out(reg) rflags,
                    options(nomem, preserves_flags)
                );
                asm!("cli", options(nomem, nostack, preserves_flags));
            }
            Some(NutexGuard { mutex: self, saved_if: (rflags & (1 << 9)) != 0 })
        }
        else
        {
            None
        }
    }
}

/// A guard that holds the lock and provides access to the protected data.
///
/// The lock is released and interrupts are restored when this guard is dropped.
pub struct NutexGuard<'a, T>
{
    mutex: &'a Nutex<T>,
    saved_if: bool,
}

impl<T> core::ops::Deref for NutexGuard<'_, T>
{
    type Target = T;
    fn deref(&self) -> &T
    {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> core::ops::DerefMut for NutexGuard<'_, T>
{
    fn deref_mut(&mut self) -> &mut T
    {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for NutexGuard<'_, T>
{
    fn drop(&mut self)
    {
        unsafe
        {
            if self.saved_if {
                asm!("sti", options(nomem, nostack, preserves_flags));
            }
        }
        self.mutex.lock.store(false, Ordering::Release);
    }
}
