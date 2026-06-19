use core::sync::atomic::{AtomicBool, Ordering};
use core::hint;

pub struct Fueue {
    open: AtomicBool,
}

impl Fueue {
    pub const fn new() -> Self {
        Self {
            open: AtomicBool::new(false),
        }
    }

    pub fn open(&self) {
        self.open.store(true, Ordering::Release);
    }

    pub fn close(&self) {
        self.open.store(false, Ordering::Release);
    }

    pub fn is_open(&self) -> bool {
        self.open.load(Ordering::Acquire)
    }

    pub fn wait(&self) {
        while !self.is_open() {
            hint::spin_loop();
        }
    }
}
