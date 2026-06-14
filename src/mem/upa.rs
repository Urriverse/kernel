//! Unified Page Allocator – front‑end for physical pages.
//!
//! Initially uses the Early Memory Allocator (EMA). After BSA is initialised,
//! `migrate()` switches to the BSA backend. The `free` function retrieves the
//! allocation order from the page frame metadata.

use core::mem::transmute;
use core::ptr::addr_of_mut;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::mem::kdm::Paddr;
use crate::mem::ema;

/// Dummy free stub (used while EMA is active).
fn free_stub(_p: Paddr) {
    warn!("upa::free called before migration – memory leak");
}

static mut EMALLOC: &'static dyn Fn(usize) -> Paddr = &ema::alloc;
static mut EMFREE:  &'static dyn Fn(Paddr) -> ()    = &free_stub;

static ALLOC: AtomicPtr<&'static dyn Fn(usize) -> Paddr> =
    AtomicPtr::new(unsafe { transmute(addr_of_mut!(EMALLOC)) });
static FREE: AtomicPtr<&'static dyn Fn(Paddr) -> ()> =
    AtomicPtr::new(unsafe { transmute(addr_of_mut!(EMFREE)) });

/// Allocate `count` physical pages (each 4 KiB).
pub fn alloc(count: usize) -> Paddr {
    (unsafe { ALLOC.load(Ordering::Relaxed).as_ref() }).unwrap()(count)
}

/// Free a block of pages previously allocated with `alloc`.
pub fn free(p: Paddr) {
    (unsafe { FREE.load(Ordering::Relaxed).as_ref() }).unwrap()(p)
}

/// Switch from EMA to the BSA backend. Called once after BSA is ready.
pub fn migrate() {
    use crate::mem::bua;
    let mut new_alloc = &bua::alloc as &'static dyn Fn(usize) -> Paddr;
    let mut new_free  = &bua::free as &'static dyn Fn(Paddr) -> ();
    let new_alloc_ptr = unsafe { transmute(addr_of_mut!(new_alloc)) };
    let new_free_ptr  = unsafe { transmute(addr_of_mut!(new_free)) };
    if ALLOC.compare_exchange(unsafe { transmute(addr_of_mut!(EMALLOC)) }, new_alloc_ptr,
                              Ordering::SeqCst, Ordering::Relaxed).is_err() {
        panic!("UPA already migrated");
    }
    if FREE.compare_exchange(unsafe { transmute(addr_of_mut!(EMFREE)) }, new_free_ptr,
                             Ordering::SeqCst, Ordering::Relaxed).is_err() {
        panic!("UPA already migrated");
    }
    info!("UPA migrated to BSA backend");
}
