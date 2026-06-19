use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering}
};

pub struct Mutex<T>
{
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T>
{
    pub const fn new(t: T) -> Self
    {
        Self
        {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(t),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T>
    {
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }

        MutexGuard { mutex: self }
    }

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
