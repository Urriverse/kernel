use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
    arch::asm
};

#[derive(Debug)]
pub struct Nutex<T>
{
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Nutex<T> {}
unsafe impl<T: Send> Sync for Nutex<T> {}

impl<T> Nutex<T>
{
    pub const fn new(t: T) -> Self
    {
        Self
        {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(t),
        }
    }

    pub fn lock(&self) -> NutexGuard<'_, T>
    {
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
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

pub struct NutexGuard<'a, T>
{
    mutex: &'a Nutex<T>,
    saved_if: bool,
}

impl<'a, T> core::ops::Deref for NutexGuard<'a, T>
{
    type Target = T;
    fn deref(&self) -> &'a T
    {
        unsafe { self.mutex.data.get().as_mut_unchecked() }
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
        self.mutex.lock.store(false, Ordering::Release);
        unsafe
        {
            if self.saved_if {
                asm!("sti", options(nomem, nostack, preserves_flags));
            }
        }
    }
}
