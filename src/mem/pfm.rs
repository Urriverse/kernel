//! # Page Frame Manager (PFM) – SPARSEMEM Physical Memory Metadata
//!
//! This module implements the **SPARSEMEM** model for managing physical page frame metadata.
//! It provides a way to track the state of each physical 4 KiB page frame in the system,
//! including its allocation status, order (for buddy allocation), and additional
//! per‑page information.
//!
//! ## Overview
//!
//! The kernel manages physical memory in 4 KiB pages. For each page frame, we need
//! to store metadata such as:
//! - Whether the page is free, allocated, or reserved.
//! - The order of the page (for buddy allocator).
//! - A reference count (for shared pages).
//! - Private data (used by the allocator for free list linking).
//!
//! Instead of allocating a large contiguous array for all page frames (which could be
//! enormous on systems with 64+ GiB of RAM), we use the **SPARSEMEM** model:
//! - Memory is divided into **sections** (each 16 MiB, i.e., 4096 pages).
//! - We allocate a page frame metadata array for each section only when that section
//!   contains usable memory.
//! - A top‑level array of pointers (`SECTIONS`) maps section indices to the per‑section
//!   metadata arrays.
//!
//! This approach reduces memory overhead for sparse memory layouts (e.g., NUMA systems
//! with large holes) and allows the metadata to be allocated on demand.
//!
//! ## Structure
//!
//! - **`Page`**: The metadata structure for a single 4 KiB page frame. It contains
//!   atomic fields for flags, order, count, and private data. The fields are atomic
//!   to allow lock‑free updates from multiple CPUs.
//! - **`PageFlags`**: Bit flags indicating the page's state (`RESERVED`, `FREE`,
//!   `ALLOCATED`, `BUDDY_HEAD`).
//! - **`PageFrame`**: A convenient wrapper around a physical frame number (PFN) that
//!   provides methods to access the associated `Page` and manipulate its fields.
//!
//! ## Initialization (`pfm::init`)
//!
//! 1. **Determine the maximum PFN**: Iterates over all physical memory regions
//!    (from PMR) to find the highest PFN.
//! 2. **Allocate the section pointer array**: Allocates a contiguous array of
//!    pointers (using EMA) to hold the base address of each section's metadata.
//! 3. **Allocate per‑section metadata**: For each section that overlaps with a
//!    usable memory region, allocate enough pages (from EMA) to store `Page`
//!    structures for all 4096 frames in that section, and initialize them.
//! 4. **Mark page states**: Iterates over all memory regions and sets each page's
//!    flags: `FREE` for usable regions, `RESERVED` for all other types.
//!
//! After initialization, the PFM provides functions to retrieve the `Page` for any
//! physical address or PFN.
//!
//! ## Usage
//!
//! The PFM is used by the buddy allocator (BSA) and other memory management components
//! to query and update page states. For example:
//! - `get_page(pfn)` returns a reference to the page metadata.
//! - `PageFrame::try_alloc()` atomically sets the page's state from `FREE` to `ALLOCATED`.
//! - `PageFrame::order()` returns the buddy order of the page.
//!
//! ## Safety
//!
//! - The module uses `static mut` for `SECTIONS` and `MAX_SECTIONS`, which are
//!   only accessed after initialization and before any other CPU cores are active.
//! - The `Page` fields are atomic, allowing safe concurrent access from multiple
//!   CPUs without locks.
//! - The `get_page` and `get_page_ptr` functions perform bounds and null checks
//!   before returning a pointer, ensuring memory safety.
//! - The `PageFrame` methods use `unsafe` internally to dereference pointers, but
//!   they are safe wrappers that validate the existence of the page.

use core::{
    ptr,
    sync::atomic::{AtomicU8, AtomicU32, Ordering},
};

use crate::mem::pmr::{self, Kind};

// ============================================================================
// PAGE FLAGS
// ============================================================================

bitflags! {
    /// Flags describing the state of a physical page frame.
    ///
    /// These are stored in the `flags` field of `Page` and are updated atomically.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PageFlags: u32 {
        /// The page is reserved (e.g., for firmware, ACPI, or kernel image).
        const RESERVED   = 1 << 0;
        /// The page is free and available for allocation.
        const FREE       = 1 << 1;
        /// The page is currently allocated.
        const ALLOCATED  = 1 << 2;
        /// This page is the head of a buddy block (used by the buddy allocator).
        const BUDDY_HEAD = 1 << 3;
    }
}

// ============================================================================
// PAGE METADATA STRUCTURE
// ============================================================================

/// Metadata for a single physical page frame (4 KiB).
///
/// This structure is stored in the per‑section metadata arrays.
/// All fields are atomic to support lock‑free updates from multiple CPUs.
#[derive(Debug)]
#[repr(C)]
pub struct Page {
    /// Page state flags (`PageFlags`), updated atomically.
    pub flags: AtomicU32,
    /// Buddy order (0 for 4 KiB, 1 for 8 KiB, etc.), updated atomically.
    pub order: AtomicU8,
    /// Padding to align the following fields.
    pub _pad: [u8; 3],
    /// Reference count (used for shared pages, e.g., CoW).
    pub count: AtomicU32,
    /// Private data (used by the allocator, e.g., next pointer in free list).
    pub private: AtomicU32,
}

impl Default for Page {
    /// Creates a new `Page` with all flags cleared and fields set to zero.
    fn default() -> Self {
        Self {
            flags: AtomicU32::new(PageFlags::empty().bits()),
            order: AtomicU8::new(0),
            _pad: [0; 3],
            count: AtomicU32::new(0),
            private: AtomicU32::new(0),
        }
    }
}

// ============================================================================
// SPARSEMEM CONSTANTS
// ============================================================================

/// Shift for section size: 24 bits → 16 MiB.
const SECTION_SHIFT: u32 = 24;
/// Size of a section in bytes (16 MiB).
const SECTION_SIZE: usize = 1 << SECTION_SHIFT;
/// Number of 4 KiB pages in one section.
const PAGES_PER_SECTION: usize = SECTION_SIZE / 4096;

// ============================================================================
// GLOBAL STATIC DATA
// ============================================================================

/// Pointer to the array of section pointers.
///
/// Each entry points to the per‑section `Page` array, or is `null` if the section
/// has no usable memory.
///
/// # Safety
/// This is `static mut` and is initialized once during `pfm::init()`.
static mut SECTIONS: *mut *mut Page = ptr::null_mut();

/// The number of sections (i.e., length of the `SECTIONS` array).
///
/// # Safety
/// This is `static mut` and is set during `pfm::init()`.
static mut MAX_SECTIONS: usize = 0;

// ============================================================================
// INITIALIZATION
// ============================================================================

/// Initializes the Page Frame Manager (SPARSEMEM).
///
/// This function must be called once, early in the boot process (on the BSP),
/// before any other memory management components use the PFM.
///
/// # Operations
///
/// 1. **Determine maximum PFN**: Scans all physical memory regions (from PMR)
///    to find the highest page frame number.
/// 2. **Allocate section pointer array**: Uses the Early Memory Allocator (EMA)
///    to allocate a contiguous array of `*mut Page` pointers, one per section.
/// 3. **Allocate per‑section metadata**: For each section that overlaps with a
///    usable memory region, allocates pages (via EMA) to hold `Page` structures
///    for all 4096 frames in that section, and zero‑initializes them.
/// 4. **Initialize page flags**: Iterates over all memory regions and sets each
///    page's flags: `FREE` for usable regions, `RESERVED` for all other types.
///
/// # Panics
/// - If the section pointer array or any per‑section metadata allocation fails
///   (i.e., EMA returns 0).
/// - If there is no usable memory.
///
/// # Notes
/// - This function uses the EMA, which is still active at this point. After
///   `upa::migrate()` is called, the EMA is no longer available.
/// - The PFM must be initialized before the Buddy System Allocator (BSA) or
///   any other allocator that relies on page metadata.
pub fn init() {
    // Calculate the maximum physical frame number (PFN) from all memory regions.
    let mut max_pfn = 0;
    for region in pmr::iter() {
        let end_pfn = (region.base + region.len).div_ceil(4096);
        if end_pfn > max_pfn {
            max_pfn = end_pfn;
        }
    }

    if max_pfn == 0 {
        warn!("PFM: No memory regions found");
        return;
    }

    // Determine the number of sections required.
    let max_sec = max_pfn.div_ceil(PAGES_PER_SECTION);

    // Allocate the section pointer array using EMA.
    let sec_array_bytes = max_sec * size_of::<*mut Page>();
    let sec_array_pages = sec_array_bytes.div_ceil(4096);
    let sec_array_paddr = crate::mem::ema::alloc(sec_array_pages);
    if sec_array_paddr.to_raw() == 0 {
        panic!("PFM: Failed to allocate sections array");
    }

    let sec_array_ptr: *mut *mut Page = sec_array_paddr.to_virt().to_ptr_mut();
    // Initialize all section pointers to null.
    for i in 0..max_sec {
        unsafe {
            ptr::write(sec_array_ptr.add(i), ptr::null_mut());
        }
    }

    unsafe {
        SECTIONS = sec_array_ptr;
        MAX_SECTIONS = max_sec;
    }

    let mut allocated_sections = 0;

    // For each usable region, allocate per‑section metadata.
    for region in pmr::iter() {
        let start_pfn = region.base / 4096;
        let end_pfn = (region.base + region.len).div_ceil(4096);

        let start_sec = start_pfn / PAGES_PER_SECTION;
        let end_sec = end_pfn.div_ceil(PAGES_PER_SECTION);

        for sec in start_sec..end_sec {
            unsafe {
                let current_ptr = *SECTIONS.add(sec);
                if current_ptr.is_null() {
                    // Allocate one or more pages for the section's Page array.
                    let pages_needed_per_section = (size_of::<Page>() * PAGES_PER_SECTION).div_ceil(4096);
                    let paddr = crate::mem::ema::alloc(pages_needed_per_section);
                    if paddr.to_raw() == 0 {
                        panic!("PFM: Failed to allocate section {}", sec);
                    }

                    let ptr: *mut Page = paddr.to_virt().to_ptr_mut();
                    *SECTIONS.add(sec) = ptr;

                    // Initialize all Page structures to default.
                    for i in 0..PAGES_PER_SECTION {
                        ptr::write(ptr.add(i), Page::default());
                    }

                    allocated_sections += 1;
                }
            }
        }
    }

    // Set page flags based on region type.
    for region in pmr::iter() {
        let start_pfn = region.base / 4096;
        let end_pfn = (region.base + region.len).div_ceil(4096);

        for pfn in start_pfn..end_pfn {
            if let Some(page) = get_page(pfn) {
                match region.kind {
                    Kind::USABLE => {
                        page.flags.store(PageFlags::FREE.bits(), Ordering::Release);
                    }
                    _ => {
                        page.flags.store(PageFlags::RESERVED.bits(), Ordering::Release);
                    }
                }
            }
        }
    }

    info!(
        "PFM: Initialized. Max PFN: {}, Sections allocated: {}, Max Sections: {}",
        max_pfn, allocated_sections, max_sec
    );
}

// ============================================================================
// PAGE LOOKUP FUNCTIONS
// ============================================================================

/// Returns a reference to the `Page` metadata for the given PFN, if the page exists.
///
/// # Arguments
/// * `pfn` – The physical frame number.
///
/// # Returns
/// `Some(&'static Page)` if the PFN is valid and the section has been allocated,
/// otherwise `None`.
#[inline(always)]
pub fn get_page(pfn: usize) -> Option<&'static Page> {
    let ptr = get_page_ptr(pfn)?;
    Some(unsafe { &*ptr })
}

/// Returns a raw pointer to the `Page` metadata for the given PFN, if it exists.
///
/// This is a lower‑level version of `get_page` that returns a `*mut Page`
/// instead of a reference. Useful when the caller needs to mutate the page
/// without re‑borrowing.
///
/// # Arguments
/// * `pfn` – The physical frame number.
///
/// # Returns
/// `Some(*mut Page)` if the PFN is valid and the section has been allocated,
/// otherwise `None`.
#[inline(always)]
pub fn get_page_ptr(pfn: usize) -> Option<*mut Page> {
    let paddr = pfn * 4096;
    if !crate::mem::kdm::is_mapped(paddr) {
        return None;
    }
    unsafe {
        let sec = pfn / PAGES_PER_SECTION;
        if sec >= MAX_SECTIONS {
            return None;
        }
        let ptr = *SECTIONS.add(sec);
        if ptr.is_null() {
            return None;
        }
        let idx = pfn % PAGES_PER_SECTION;
        Some(ptr.add(idx))
    }
}

/// Converts a physical address (Paddr) to a PFN.
#[inline(always)]
pub fn paddr_to_pfn(paddr: crate::mem::kdm::Paddr) -> usize {
    paddr.to_raw() / 4096
}

/// Returns the `Page` metadata for the page frame containing the given physical address.
#[inline(always)]
pub fn get_page_by_paddr(paddr: crate::mem::kdm::Paddr) -> Option<&'static Page> {
    get_page(paddr_to_pfn(paddr))
}

/// Returns a raw pointer to the `Page` metadata for the page frame containing
/// the given physical address.
#[inline(always)]
pub fn get_page_ptr_by_paddr(paddr: crate::mem::kdm::Paddr) -> Option<*mut Page> {
    get_page_ptr(paddr_to_pfn(paddr))
}

// ============================================================================
// PAGE FRAME WRAPPER
// ============================================================================

/// A safe wrapper around a physical frame number (PFN) that provides methods
/// to access and manipulate the associated page metadata.
///
/// This struct is intended to be used by the buddy allocator and other
/// memory management components to perform atomic operations on page state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageFrame(usize);

impl PageFrame {
    /// Creates a new `PageFrame` from a PFN.
    #[inline(always)]
    pub const fn new(pfn: usize) -> Self {
        Self(pfn)
    }

    /// Creates a `PageFrame` from a physical address (Paddr).
    #[inline(always)]
    pub const fn from_paddr(paddr: crate::mem::kdm::Paddr) -> Self {
        Self(paddr.to_raw() / 4096)
    }

    /// Creates a `PageFrame` from a virtual address (Vaddr).
    #[inline(always)]
    pub fn from_vaddr(vaddr: crate::mem::kdm::Vaddr) -> Self {
        Self::from_paddr(vaddr.to_phys())
    }

    /// Returns the PFN.
    #[inline(always)]
    pub const fn pfn(self) -> usize {
        self.0
    }

    /// Returns the physical address of this page frame.
    #[inline(always)]
    pub const fn paddr(self) -> crate::mem::kdm::Paddr {
        crate::mem::kdm::Paddr::from_raw(self.0 * 4096)
    }

    /// Returns the virtual address (HHDM mapping) of this page frame.
    #[inline(always)]
    pub fn vaddr(self) -> crate::mem::kdm::Vaddr {
        self.paddr().to_virt()
    }

    /// Returns a reference to the `Page` metadata, if it exists.
    #[inline(always)]
    pub fn page(self) -> Option<&'static Page> {
        get_page(self.0)
    }

    /// Returns a raw pointer to the `Page` metadata, if it exists.
    #[inline(always)]
    pub fn page_ptr(self) -> Option<*mut Page> {
        get_page_ptr(self.0)
    }

    /// Returns `true` if the PFN is valid and has metadata.
    #[inline(always)]
    pub fn is_valid(self) -> bool {
        get_page_ptr(self.0).is_some()
    }

    /// Returns the current page flags.
    #[inline(always)]
    pub fn flags(self) -> PageFlags {
        if let Some(page) = self.page() {
            PageFlags::from_bits_truncate(page.flags.load(Ordering::Acquire))
        } else {
            PageFlags::empty()
        }
    }

    /// Sets the page flags to the given value.
    #[inline(always)]
    pub fn set_flags(self, flags: PageFlags) -> Self {
        if let Some(page) = self.page() {
            page.flags.store(flags.bits(), Ordering::Release);
        }
        self
    }

    /// Atomically ORs the given flags into the page's flags.
    #[inline(always)]
    pub fn flags_or(self, flags: PageFlags) -> Self {
        if let Some(page) = self.page() {
            page.flags.fetch_or(flags.bits(), Ordering::AcqRel);
        }
        self
    }

    /// Atomically ANDs the page's flags with the given mask (clears other bits).
    #[inline(always)]
    pub fn flags_and(self, flags: PageFlags) -> Self {
        if let Some(page) = self.page() {
            page.flags.fetch_and(flags.bits(), Ordering::AcqRel);
        }
        self
    }

    /// Atomically clears the given flags.
    #[inline(always)]
    pub fn clear_flags(self, flags: PageFlags) -> Self {
        if let Some(page) = self.page() {
            page.flags.fetch_and(!flags.bits(), Ordering::AcqRel);
        }
        self
    }

    /// Returns `true` if the page is free.
    #[inline(always)]
    pub fn is_free(self) -> bool {
        self.flags().contains(PageFlags::FREE)
    }

    /// Returns `true` if the page is allocated.
    #[inline(always)]
    pub fn is_allocated(self) -> bool {
        self.flags().contains(PageFlags::ALLOCATED)
    }

    /// Returns `true` if the page is reserved.
    #[inline(always)]
    pub fn is_reserved(self) -> bool {
        self.flags().contains(PageFlags::RESERVED)
    }

    /// Returns the current reference count.
    #[inline(always)]
    pub fn count(self) -> u32 {
        if let Some(page) = self.page() {
            page.count.load(Ordering::Acquire)
        } else {
            0
        }
    }

    /// Atomically increments the reference count and returns the new value.
    #[inline(always)]
    pub fn inc_count(self) -> u32 {
        if let Some(page) = self.page() {
            page.count.fetch_add(1, Ordering::AcqRel) + 1
        } else {
            0
        }
    }

    /// Atomically decrements the reference count and returns the new value.
    #[inline(always)]
    pub fn dec_count(self) -> u32 {
        if let Some(page) = self.page() {
            page.count.fetch_sub(1, Ordering::AcqRel) - 1
        } else {
            0
        }
    }

    /// Attempts to atomically change the page state from `FREE` to `ALLOCATED`.
    ///
    /// # Returns
    /// `true` if the transition succeeded, `false` if the page was not free.
    #[inline(always)]
    pub fn try_alloc(self) -> bool {
        if let Some(page) = self.page() {
            let expected = PageFlags::FREE.bits();
            let desired = PageFlags::ALLOCATED.bits();
            page.flags
                .compare_exchange(expected, desired, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
        } else {
            false
        }
    }

    /// Attempts to atomically change the page state from `ALLOCATED` to `FREE`.
    ///
    /// # Returns
    /// `true` if the transition succeeded, `false` if the page was not allocated.
    #[inline(always)]
    pub fn try_free(self) -> bool {
        if let Some(page) = self.page() {
            let expected = PageFlags::ALLOCATED.bits();
            let desired = PageFlags::FREE.bits();
            page.flags
                .compare_exchange(expected, desired, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
        } else {
            false
        }
    }

    /// Returns the buddy order of this page (used by the buddy allocator).
    #[inline(always)]
    pub fn order(self) -> u8 {
        if let Some(page) = self.page() {
            page.order.load(Ordering::Relaxed)
        } else {
            0
        }
    }

    /// Sets the buddy order of this page.
    ///
    /// # Safety
    /// This is an atomic store but does not check any invariants. The caller
    /// must ensure the order is consistent with the page's state and the buddy
    /// allocator's logic.
    #[inline(always)]
    pub unsafe fn set_order(self, order: u8) -> Self {
        if let Some(page_ptr) = self.page_ptr() {
            (unsafe { &*page_ptr }).order.store(order, Ordering::Relaxed);
        }
        self
    }

    /// Returns the private field (used by the allocator for free list linking).
    #[inline(always)]
    pub fn private(self) -> u32 {
        if let Some(page) = self.page() {
            page.private.load(Ordering::Relaxed)
        } else {
            0
        }
    }

    /// Sets the private field.
    ///
    /// # Safety
    /// The caller must ensure that the value is valid and consistent with the
    /// allocator's state.
    #[inline(always)]
    pub unsafe fn set_private(self, private: u32) -> Self {
        if let Some(page_ptr) = self.page_ptr() {
            (unsafe { &*page_ptr }).private.store(private, Ordering::Relaxed);
        }
        self
    }
}
