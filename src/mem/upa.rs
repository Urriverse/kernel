//! Unified Page Allocator – front‑end for physical pages.
//!
//! Initially uses the Early Memory Allocator (EMA). After BSA is initialised,
//! `migrate()` switches to the BSA backend. The `free` function retrieves the
//! allocation order from the page frame metadata.

use core::ptr::addr_of;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::mem::kdm::Paddr;
use crate::mem::ema;

// -----------------------------------------------------------------------------
// Backend definition – plain function pointers (thin, Sync, safe to copy)
// -----------------------------------------------------------------------------
struct Backend {
    alloc: fn(usize) -> Paddr,
    free: fn(Paddr),
}

// -----------------------------------------------------------------------------
// The two backends: early (pre‑migration) and late (post‑migration).
// Both are 'static and will never be deallocated.
// -----------------------------------------------------------------------------
/// Dummy free stub used before migration.
fn free_stub(_p: Paddr) {
    warn!("upa::free called before migration – memory leak");
}

static EARLY_BACKEND: Backend = Backend {
    alloc: ema::alloc,
    free: free_stub,
};

// The late backend is defined here, but its functions are only called after
// `migrate()` has run – by which time the BSA is fully initialised.
static LATE_BACKEND: Backend = Backend {
    alloc: crate::mem::bua::alloc,
    free: crate::mem::bua::free,
};

// -----------------------------------------------------------------------------
// Global atomic pointer to the currently active backend.
// -----------------------------------------------------------------------------
/// Initially points to `EARLY_BACKEND`. After `migrate()` it points to
/// `LATE_BACKEND`. The pointer is never null and never points to invalid data.
static CURRENT_BACKEND: AtomicPtr<Backend> =
    AtomicPtr::new(addr_of!(EARLY_BACKEND) as *mut Backend);

// -----------------------------------------------------------------------------
// Public API
// -----------------------------------------------------------------------------

/// Allocate `count` physical pages (each 4 KiB).
pub fn alloc(count: usize) -> Paddr {
    // SAFETY:
    // - `CURRENT_BACKEND` is always initialised to a non‑null pointer.
    // - The pointer always points to a valid `Backend` (either `EARLY_BACKEND`
    //   or `LATE_BACKEND`), which are `'static` and never destroyed.
    // - The `Backend::alloc` function is a plain function pointer; calling it
    //   through a safe reference is correct and does not violate aliasing.
    let backend = unsafe { &*CURRENT_BACKEND.load(Ordering::Acquire) };
    (backend.alloc)(count)
}

/// Free a block of pages previously allocated with `alloc`.
pub fn free(p: Paddr) {
    // SAFETY: Same reasoning as in `alloc`.
    let backend = unsafe { &*CURRENT_BACKEND.load(Ordering::Acquire) };
    (backend.free)(p)
}

/// Switch from EMA to the BSA backend. Called exactly once after BSA is ready.
pub fn migrate() {
    let expected = addr_of!(EARLY_BACKEND) as *mut Backend;
    let new = addr_of!(LATE_BACKEND) as *mut Backend;

    // Atomically replace the early backend with the late backend.
    // The `SeqCst` ordering guarantees that any thread that loads the pointer
    // after this exchange (with `Acquire`) will see the new value.
    if CURRENT_BACKEND
        .compare_exchange(expected, new, Ordering::SeqCst, Ordering::Relaxed)
        .is_err()
    {
        panic!("UPA already migrated");
    }

    info!("UPA migrated to BSA backend");
}
