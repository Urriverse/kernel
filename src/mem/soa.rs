//! Sized Object Allocator (SOA) – slab allocator with lock‑free fast path.
//!
//! Manages fixed‑size object allocations using a slab‑based design.
//! Each size class has its own cache with:
//! - A **lock‑free LIFO freelist** (fast path) using atomic CAS.
//! - A **nutex‑protected slab list** for allocating new slab pages when the
//!   freelist is exhausted (slow path).
//!
//! Slab pages are obtained from the page allocator (UPA)
//! and carved into fixed‑size slots. Free objects store an intrusive
//! next‑pointer in their first 8 bytes, forming a per‑cache linked list of
//! available slots.

use core::sync::atomic::{AtomicPtr, AtomicBool, AtomicUsize, Ordering};
use crate::sync::Nutex;
use crate::mem::pfm::{PAGE_SIZE, pfn_to_page, SLAB_NO_CACHE, PF_SLAB};

// ─── Constants ────────────────────────────────────────────────────────────────

/// Number of distinct size classes.
const NUM_CACHES: usize = 9;

/// Size classes in bytes. Must be powers of 2 and ≥ 8 (pointer size on x86‑64).
const SIZE_CLASSES: [u32; NUM_CACHES] = [8, 16, 32, 64, 128, 256, 512, 1024, 2048];

/// Sentinel PFN for "end of list" in the slab linked list.
const INVALID_PFN: u32 = 0xFFFF_FFFF;

// ─── Intrusive free‑list node ─────────────────────────────────────────────────

/// Stored in the first 8 bytes of every free object.
/// When an object is allocated, this space is available for the caller.
#[repr(C)]
struct FreeObject {
    next: *mut FreeObject,
}

// ─── Per‑size‑class cache ─────────────────────────────────────────────────────

struct SoaCache {
    /// Size of each object in this cache (bytes).
    object_size: u32,
    /// Number of objects that fit in one 4 KiB page.
    objects_per_page: u32,
    /// Lock‑free LIFO stack of free objects – the fast‑path freelist.
    /// Each free object stores a pointer to the next free object in its
    /// first bytes (intrusive linked list).
    freelist: AtomicPtr<FreeObject>,
    /// Mutex‑protected list of slab pages owned by this cache.
    /// Slab PFNs are linked via `PageFrame::next_free`.
    slabs: Nutex<SlabList>,
    /// Number of slab pages allocated for this cache.
    slab_pages: AtomicUsize,
    /// Monotonic counter of allocations served (for statistics).
    alloc_total: AtomicUsize,
}

/// Singly‑linked list of slab page PFNs.
struct SlabList {
    head: Option<usize>,
}

impl SoaCache {
    const fn new(object_size: u32) -> Self {
        Self {
            object_size,
            objects_per_page: PAGE_SIZE as u32 / object_size,
            freelist: AtomicPtr::new(core::ptr::null_mut()),
            slabs: Nutex::new(SlabList { head: None }),
            slab_pages: AtomicUsize::new(0),
            alloc_total: AtomicUsize::new(0),
        }
    }

    // ── Fast path (lock‑free) ──────────────────────────────────────────

    /// Pop a free object from the lock‑free freelist.
    /// Uses `compare_exchange_weak` in a retry loop (standard lock‑free LIFO pop).
    fn alloc_fast(&self) -> Option<*mut u8> {
        loop {
            let head = self.freelist.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }
            // SAFETY: `head` points to a valid FreeObject while it sits on the
            // freelist. We read its `next` pointer before attempting the CAS.
            let next = unsafe { (*head).next };
            if self.freelist.compare_exchange_weak(
                head,
                next,
                Ordering::Acquire,
                Ordering::Relaxed,
            ).is_ok() {
                self.alloc_total.fetch_add(1, Ordering::Relaxed);
                return Some(head as *mut u8);
            }
            // CAS failed – another thread raced us. Retry.
        }
    }

    /// Push an object back onto the lock‑free freelist.
    /// Uses `compare_exchange_weak` in a retry loop (standard lock‑free LIFO push).
    fn dealloc_fast(&self, ptr: *mut u8) {
        let obj = ptr as *mut FreeObject;
        loop {
            let head = self.freelist.load(Ordering::Acquire);
            // SAFETY: `ptr` is a valid, allocated object that the caller is
            // returning. We are allowed to repurpose its first bytes for the
            // intrusive next‑pointer while it is on the freelist.
            unsafe { (*obj).next = head; }
            if self.freelist.compare_exchange_weak(
                head,
                obj,
                Ordering::Release,
                Ordering::Relaxed,
            ).is_ok() {
                return;
            }
            // CAS failed – retry.
        }
    }

    // ── Slow path (under lock) ─────────────────────────────────────────

    /// Allocate a new slab page from the buddy allocator, carve it into
    /// fixed‑size objects, push them all onto the freelist (single batch CAS),
    /// then pop one for the caller.
    fn alloc_slow(&self, cache_idx: u32) -> Option<*mut u8> {
        // 1. Allocate one page from the buddy allocator.
        let paddr = crate::mem::upa::alloc(1);
        if paddr.to_raw() == 0 {
            error!("SOA: buddy alloc failed for slab page (cache {})", cache_idx);
            return None;
        }

        let pfn = paddr.to_raw() / PAGE_SIZE;
        let page = pfn_to_page(pfn);

        // 2. Tag the PageFrame with slab metadata.
        page.set_slab_cache(cache_idx);
        page.set_slab_total(self.objects_per_page as u16);
        page.set_slab_inuse(0);
        page.set_flags(page.flags() | PF_SLAB);

        let base = paddr.to_virt().to_raw();
        let obj_sz = self.object_size as usize;
        let n = self.objects_per_page as usize;

        debug!("SOA[{}]: new slab PFN {} – {} objects of {} bytes",
               cache_idx, pfn, n, obj_sz);

        // 3. Build the intrusive free list inside the page.
        //    obj[0] → obj[1] → … → obj[n‑1] → null
        for i in 0..(n - 1) {
            let obj = (base + i * obj_sz) as *mut FreeObject;
            unsafe {
                (*obj).next = (base + (i + 1) * obj_sz) as *mut FreeObject;
            }
        }
        let last = (base + (n - 1) * obj_sz) as *mut FreeObject;

        // 4. Prepend the entire chain to the global freelist (single CAS).
        //    This is an O(1) batch push: we link the tail to the old head,
        //    then CAS the head to our chain's first element.
        loop {
            let old_head = self.freelist.load(Ordering::Acquire);
            unsafe { (*last).next = old_head; }
            if self.freelist.compare_exchange_weak(
                old_head,
                base as *mut FreeObject,
                Ordering::Release,
                Ordering::Relaxed,
            ).is_ok() {
                break;
            }
        }

        // 5. Record the slab in the bookkeeping list.
        {
            let mut list = self.slabs.lock();
            page.set_next_free(
                list.head.map(|p| p as u32).unwrap_or(INVALID_PFN)
            );
            list.head = Some(pfn);
        }
        self.slab_pages.fetch_add(1, Ordering::Relaxed);

        // 6. Pop one object for the caller.
        self.alloc_fast()
    }

    /// Return (slab_page_count, total_allocations) for this cache.
    fn usage(&self) -> (usize, usize) {
        (
            self.slab_pages.load(Ordering::Relaxed),
            self.alloc_total.load(Ordering::Relaxed),
        )
    }
}

// ─── Global SOA state ─────────────────────────────────────────────────────────

struct Soa {
    caches: [SoaCache; NUM_CACHES],
}

/// Global SOA instance. `None` before [`init`], `Some` after.
static mut SOA: Option<Soa> = None;
static SOA_INIT: AtomicBool = AtomicBool::new(false);

impl Soa {
    fn new() -> Self {
        Self {
            caches: [
                SoaCache::new(8),
                SoaCache::new(16),
                SoaCache::new(32),
                SoaCache::new(64),
                SoaCache::new(128),
                SoaCache::new(256),
                SoaCache::new(512),
                SoaCache::new(1024),
                SoaCache::new(2048),
            ],
        }
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Map a requested allocation size to the smallest cache index whose
/// `object_size ≥ size`. Returns `None` if no cache is large enough.
fn size_to_index(size: usize) -> Option<usize> {
    for (i, &cls) in SIZE_CLASSES.iter().enumerate() {
        if cls as usize >= size {
            return Some(i);
        }
    }
    None
}

/// Convert a direct‑mapped virtual address to its page frame number.
#[inline]
fn vaddr_to_pfn(vaddr: usize) -> usize {
    (vaddr - *crate::mem::kdm::HHDM) / PAGE_SIZE
}

// ─── Public API ───────────────────────────────────────────────────────────────

/// Initialise the SOA. Must be called after BSA is ready.
pub fn init() {
    if SOA_INIT.swap(true, Ordering::SeqCst) {
        panic!("SOA already initialised");
    }
    #[allow(static_mut_refs)]
    unsafe {
        SOA = Some(Soa::new());
    }
    info!("SOA: ready ({} size classes: 8–2048 bytes)", NUM_CACHES);
}

/// Allocate an object of at least `size` bytes.
///
/// Returns `None` if `size` exceeds the largest size class or if the buddy
/// allocator cannot provide a new slab page.
pub fn alloc(size: usize) -> Option<*mut u8> {
    let idx = size_to_index(size)?;

    #[allow(static_mut_refs)]
    let cache = unsafe { &SOA.as_ref().unwrap().caches[idx] };

    // Fast path: lock‑free pop from the freelist.
    if let Some(ptr) = cache.alloc_fast() {
        return Some(ptr);
    }

    // Slow path: freelist is empty – carve a new slab page.
    cache.alloc_slow(idx as u32)
}

/// Deallocate an object previously returned by [`alloc`].
///
/// The cache is determined automatically from the page frame metadata.
///
/// # Safety
///
/// `ptr` must have been returned by a previous call to [`alloc`] and must not
/// have been freed already (double‑free is UB).
pub unsafe fn dealloc(ptr: *mut u8) {
    // SAFETY: caller guarantees `ptr` is valid, allocated, and not double‑freed.
    unsafe {
        debug_assert!(!ptr.is_null(), "SOA: dealloc(null)");

        let pfn = vaddr_to_pfn(ptr as usize);
        let page = pfn_to_page(pfn);
        let idx = page.slab_cache();
        debug_assert_ne!(idx, SLAB_NO_CACHE, "SOA: dealloc of non‑slab pointer");

        #[allow(static_mut_refs)]
        let cache = &SOA.as_ref().unwrap().caches[idx as usize];

        // Fast path: lock‑free push onto the freelist.
        cache.dealloc_fast(ptr);
    }
}

/// Per‑cache usage statistics: `(slab_pages, total_allocations)`.
pub fn usage() -> [(usize, usize); NUM_CACHES] {
    let mut out = [(0usize, 0usize); NUM_CACHES];
    #[allow(static_mut_refs)]
    unsafe {
        if let Some(soa) = SOA.as_ref() {
            for i in 0..NUM_CACHES {
                out[i] = soa.caches[i].usage();
            }
        }
    }
    out
}