use core::ptr::addr_of;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::mem::kdm::Paddr;
use crate::mem::ema;

struct Backend {
    alloc: fn(usize) -> Paddr,
    free: fn(Paddr),
}

fn free_stub(_p: Paddr) {
    warn!("upa::free called before migration – memory leak");
}

static EARLY_BACKEND: Backend = Backend {
    alloc: ema::alloc,
    free: free_stub,
};

static LATE_BACKEND: Backend = Backend {
    alloc: crate::mem::bsa::alloc,
    free: crate::mem::bsa::free,
};

static CURRENT_BACKEND: AtomicPtr<Backend> =
    AtomicPtr::new(addr_of!(EARLY_BACKEND) as *mut Backend);

pub fn alloc(count: usize) -> Paddr {
    if count == 0 {
        return Paddr::from_raw(0)
    }
    let backend = unsafe { &*CURRENT_BACKEND.load(Ordering::Acquire) };
    (backend.alloc)(count)
}

pub fn free(p: Paddr) {
    if p.to_raw() == 0 {
        return
    }
    let backend = unsafe { &*CURRENT_BACKEND.load(Ordering::Acquire) };
    (backend.free)(p)
}

pub fn migrate() {
    let expected = addr_of!(EARLY_BACKEND) as *mut Backend;
    let new = addr_of!(LATE_BACKEND) as *mut Backend;

    if CURRENT_BACKEND
        .compare_exchange(expected, new, Ordering::SeqCst, Ordering::Relaxed)
        .is_err()
    {
        panic!("UPA already migrated");
    }

    info!("UPA migrated to BSA backend");
}
