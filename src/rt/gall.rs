use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicPtr, Ordering};
use core::ptr::addr_of_mut;

pub trait Gall: Sync {
    fn alloc(&self, l: Layout) -> *mut u8;
    fn free(&self, l: Layout, ptr: *mut u8);
}

struct Fake;
unsafe impl Sync for Fake {}
impl Gall for Fake {
    fn alloc(&self, l: Layout) -> *mut u8 {
        error!(
            "Heap unavailable. Requested {} bytes, {}-aligned.",
            l.size(), l.align()
        );
        core::ptr::null_mut::<u8>()
    }
    fn free(&self, l: Layout, ptr: *mut u8) {
        error!(
            "Heap unavailable. Memory leak of size {} at {:#X}, {}-aligned.",
            l.size(), ptr as usize, l.align()
        );
    }
}

static mut FAKE_ITSELF: Fake = Fake;
#[allow(static_mut_refs)]
static mut FAKE_REF: &'static mut dyn Gall = unsafe { &mut FAKE_ITSELF };

struct SoaBackend;
unsafe impl Sync for SoaBackend {}
impl Gall for SoaBackend {
    fn alloc(&self, l: Layout) -> *mut u8 {
        crate::mem::soa::alloc(l)
    }
    fn free(&self, l: Layout, ptr: *mut u8) {
        crate::mem::soa::free(ptr, l)
    }
}

static mut SOA_ITSELF: SoaBackend = SoaBackend;
#[allow(static_mut_refs)]
static mut SOA_REF: &'static mut dyn Gall = unsafe { &mut SOA_ITSELF };

static GALL: AtomicPtr<&'static mut dyn Gall> = AtomicPtr::new(addr_of_mut!(FAKE_REF));

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

pub fn set_soa() {
    let new_ptr: *mut &'static mut dyn Gall = addr_of_mut!(SOA_REF);
    match GALL.compare_exchange(
        addr_of_mut!(FAKE_REF),
        new_ptr,
        Ordering::SeqCst,
        Ordering::Relaxed,
    ) {
        Ok(_) => {}
        Err(_) => panic!("Global allocator already set"),
    }
    info!("gall: switched to SOA backend");
}
