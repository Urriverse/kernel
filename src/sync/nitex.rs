//! A no‑interrupt mutex (implicit lock).
//!
//! [`Nitex`] provides a “mutex” that does not actually spin; instead it
//! simply disables interrupts on `lock()` and restores them on `drop()`.
//! There is no competition – the lock is always acquired immediately.
//! This is useful for temporarily disabling interrupts around a critical
//! section without any actual locking.

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
    arch::asm
};

/// A mutex that only disables interrupts, no spinning.
///
/// The `lock()` method always succeeds because there is no actual lock flag.
/// It saves the current interrupt state, disables interrupts, and returns
/// a guard that restores the original interrupt state when dropped.
pub struct Nitex<T>
{
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Nitex<T> {}
unsafe impl<T: Send> Sync for Nitex<T> {}

impl<T> Nitex<T>
{
    /// Create a new `Nitex` containing `t`.
    pub const fn new(t: T) -> Self
    {
        Self { data: UnsafeCell::new(t) }
    }

    /// Acquire the “lock” (disable interrupts).
    ///
    /// This function always returns immediately; there is no contention.
    pub fn lock(&self) -> NitexGuard<'_, T>
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

        NitexGuard { mutex: self, saved_if: (rflags & (1 << 9)) != 0 }
    }
}

/// A guard that restores interrupts when dropped.
pub struct NitexGuard<'a, T>
{
    mutex: &'a Nitex<T>,
    saved_if: bool,
}

impl<T> core::ops::Deref for NitexGuard<'_, T>
{
    type Target = T;
    fn deref(&self) -> &T
    {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> core::ops::DerefMut for NitexGuard<'_, T>
{
    fn deref_mut(&mut self) -> &mut T
    {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for NitexGuard<'_, T>
{
    fn drop(&mut self)
    {
        unsafe
        {
            if self.saved_if {
                asm!("sti", options(nomem, nostack, preserves_flags));
            }
        }
    }
}
