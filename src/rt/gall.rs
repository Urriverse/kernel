//! Global allocator management (swappable backend).
//!
//! This module provides a mechanism to set a custom [`GlobalAlloc`] backend
//! after the kernel has initialised a proper heap. Initially it uses a dummy
//! allocator (`Fake`) that logs errors. The real backend can be installed
//! exactly once via [`set`].

use core::alloc::{GlobalAlloc, Layout};
use core::ops::Deref as _;
use core::sync::atomic::{AtomicPtr, Ordering};
use core::ptr::{self, addr_of, addr_of_mut};

use crate::sync::Nutex;

/// Trait for a global allocator backend.
///
/// This is a simplified version of [`GlobalAlloc`] that splits the methods.
pub trait Gall: Sync
{
    /// Allocate memory according to `layout`.
    fn alloc(&self, l: Layout  ) -> *mut u8 ;
    /// Deallocate memory previously allocated with `alloc`.
    fn  free(&self, l: Layout, ptr: *mut u8);
}

/// Dummy allocator that logs errors instead of actually allocating.
struct Fake;

unsafe impl Sync for Fake {}

impl Gall for Fake
{
    fn alloc(&self, l: Layout  ) -> *mut u8
    {
        error!
        {
            "Heap unavailable. Requested {} bytes, {}-aligned.",
            
            l.size(),
            l.align()
        }
        
        0 as *mut u8
    }

    fn  free(&self, l: Layout, ptr: *mut u8)
    {
        error!
        {
            "Heap unavailable. Memory leak of size {} at {:#X}, {}-aligned.",

            l.size(),
            ptr as usize,
            l.align()
        }
    }
}

static mut FAKE_ITSELF: Fake = Fake;

#[allow(static_mut_refs)]
static mut FAKE_REF: &'static mut dyn Gall = unsafe { &mut FAKE_ITSELF };

type GallRef = *const &'static dyn Gall;

/// Atomic pointer to the current global allocator backend.
static GALL: AtomicPtr<&'static mut dyn Gall> = AtomicPtr::new(addr_of_mut!(FAKE_REF));

/// The actual `#[global_allocator]` that forwards to the current backend.
struct GallMuxer;

unsafe impl GlobalAlloc for GallMuxer {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let this = unsafe { GALL.load(Ordering::Acquire).as_ref_unchecked() };
        this.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let this = unsafe { GALL.load(Ordering::Acquire).as_ref_unchecked() };
        this.free(layout, ptr)
    }
}

#[global_allocator]
static MUXER: GallMuxer = GallMuxer;

/// Replace the dummy global allocator with a real one.
///
/// This function may be called only once. Any subsequent call will panic.
pub fn set(mut new: &'static mut dyn Gall)
{
    let new_ptr = addr_of_mut!(new);
    match GALL.compare_exchange
    (
        addr_of_mut!(FAKE_REF),
        new_ptr,
        Ordering::SeqCst,
        Ordering::Relaxed,
    )
    {
        Ok(_) => {}
        Err(_) => panic!("Global allocator already set"),
    }
}
