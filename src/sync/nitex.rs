use core::{
    cell::UnsafeCell,
    arch::asm
};

pub struct Nitex<T>
{
    data: UnsafeCell<T>,
}

impl<T: Clone> Clone for Nitex<T> {
    fn clone(&self) -> Self {
        Self::new((unsafe { &*self.data.get() }).clone())
    }
}

unsafe impl<T: Send> Send for Nitex<T> {}
unsafe impl<T: Send> Sync for Nitex<T> {}

impl<T> Nitex<T>
{
    pub const fn new(t: T) -> Self
    {
        Self { data: UnsafeCell::new(t) }
    }

    pub unsafe fn inner(&self) -> &'_ mut T {
        unsafe {
            self.data.as_mut_unchecked()
        }
    }

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
