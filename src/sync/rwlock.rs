use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
    hint,
};

/// A spinlock-based reader‑writer lock.
///
/// The lock is implemented with a single `AtomicUsize` state:
/// - `0`              : unlocked
/// - `WRITER_BIT`     : write‑locked (exclusive)
/// - any other value  : read‑locked, with the lower bits counting the number of active readers.
///
/// Writers spin until the state is `0`, then set it to `WRITER_BIT`.
/// Readers spin while the writer bit is set, then increment the reader count.
pub struct RwLock<T> {
    state: AtomicUsize,
    data: UnsafeCell<T>,
}

const WRITER_BIT: usize = 1 << (usize::BITS - 1); // highest bit

unsafe impl<T: Send> Send for RwLock<T> {}
unsafe impl<T: Send> Sync for RwLock<T> {}

impl<T> RwLock<T> {
    /// Creates a new `RwLock` in the unlocked state.
    pub const fn new(t: T) -> Self {
        Self {
            state: AtomicUsize::new(0),
            data: UnsafeCell::new(t),
        }
    }

    /// Acquires a shared (read) lock, spinning until it is available.
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        loop {
            let old = self.state.load(Ordering::Acquire);
            if old & WRITER_BIT != 0 {
                hint::spin_loop();
                continue;
            }
            // Try to increment the reader count
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
            // CAS failed, retry
        }
    }

    /// Attempts to acquire a shared (read) lock without spinning.
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
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        loop {
            let old = self.state.load(Ordering::Acquire);
            if old != 0 {
                hint::spin_loop();
                continue;
            }
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

/// Guard for a shared (read) lock.
pub struct RwLockReadGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<T> core::ops::Deref for RwLockReadGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> Drop for RwLockReadGuard<'_, T> {
    fn drop(&mut self) {
        // Decrement the reader count.
        self.lock.state.fetch_sub(1, Ordering::Release);
    }
}

/// Guard for an exclusive (write) lock.
pub struct RwLockWriteGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<T> core::ops::Deref for RwLockWriteGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> core::ops::DerefMut for RwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for RwLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        // Release the write lock.
        self.lock.state.store(0, Ordering::Release);
    }
}

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
