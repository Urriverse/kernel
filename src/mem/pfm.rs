use core::{
    ptr,
    sync::atomic::{AtomicU8, AtomicU32, Ordering},
};

use crate::mem::pmr::{self, Kind};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PageFlags: u32 {
        const RESERVED   = 1 << 0;
        const FREE       = 1 << 1;
        const ALLOCATED  = 1 << 2;
        const BUDDY_HEAD = 1 << 3;
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Page {
    pub flags: AtomicU32,
    pub order: AtomicU8,
    pub _pad: [u8; 3],
    pub count: AtomicU32,
    pub private: AtomicU32,
}

impl Default for Page {
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

const SECTION_SHIFT: u32 = 24;
const SECTION_SIZE: usize = 1 << SECTION_SHIFT;

const PAGES_PER_SECTION: usize = SECTION_SIZE / 4096;

static mut SECTIONS: *mut *mut Page = ptr::null_mut();

static mut MAX_SECTIONS: usize = 0;

pub fn init() {
    // debug!("PFM: Initializing SPARSEMEM page frame metadata");

    let size_of_page = core::mem::size_of::<Page>();
    let bytes_per_section = PAGES_PER_SECTION * size_of_page;
    let pages_needed_per_section = (bytes_per_section + 4095) / 4096;

    let mut max_pfn = 0;
    for region in pmr::iter() {
        let end_pfn = (region.base + region.len + 4095) / 4096;
        if end_pfn > max_pfn {
            max_pfn = end_pfn;
        }
    }

    if max_pfn == 0 {
        warn!("PFM: No memory regions found");
        return;
    }

    let max_sec = (max_pfn + PAGES_PER_SECTION - 1) / PAGES_PER_SECTION;
    // debug!("PFM: Max PFN = {}, Max sections = {}", max_pfn, max_sec);

    let sec_array_bytes = max_sec * core::mem::size_of::<*mut Page>();
    let sec_array_pages = (sec_array_bytes + 4095) / 4096;
    // debug!("PFM: Allocating {} pages for sections array", sec_array_pages);

    let sec_array_paddr = crate::mem::ema::alloc(sec_array_pages);
    if sec_array_paddr.to_raw() == 0 {
        panic!("PFM: Failed to allocate sections array");
    }

    let sec_array_ptr: *mut *mut Page = sec_array_paddr.to_virt().to_ptr_mut();
    // debug!("PFM: Sections array at {:#X}", sec_array_ptr as usize);

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

    for region in pmr::iter() {
        let start_pfn = region.base / 4096;
        let end_pfn = (region.base + region.len + 4095) / 4096;

        let start_sec = start_pfn / PAGES_PER_SECTION;
        let end_sec = (end_pfn + PAGES_PER_SECTION - 1) / PAGES_PER_SECTION;

        for sec in start_sec..end_sec {
            unsafe {
                let current_ptr = *SECTIONS.add(sec);
                if current_ptr.is_null() {
                    let paddr = crate::mem::ema::alloc(pages_needed_per_section);
                    if paddr.to_raw() == 0 {
                        panic!("PFM: Failed to allocate section {}", sec);
                    }

                    let ptr: *mut Page = paddr.to_virt().to_ptr_mut();
                    *SECTIONS.add(sec) = ptr;

                    for i in 0..PAGES_PER_SECTION {
                        ptr::write(ptr.add(i), Page::default());
                    }

                    allocated_sections += 1;
                }
            }
        }
    }

    for region in pmr::iter() {
        let start_pfn = region.base / 4096;
        let end_pfn = (region.base + region.len + 4095) / 4096;

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

#[inline(always)]
pub fn get_page(pfn: usize) -> Option<&'static Page> {
    let ptr = get_page_ptr(pfn)?;
    Some(unsafe { &*ptr })
}

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

#[inline(always)]
pub fn paddr_to_pfn(paddr: crate::mem::kdm::Paddr) -> usize {
    paddr.to_raw() / 4096
}

#[inline(always)]
pub fn get_page_by_paddr(paddr: crate::mem::kdm::Paddr) -> Option<&'static Page> {
    get_page(paddr_to_pfn(paddr))
}

#[inline(always)]
pub fn get_page_ptr_by_paddr(paddr: crate::mem::kdm::Paddr) -> Option<*mut Page> {
    get_page_ptr(paddr_to_pfn(paddr))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageFrame(usize);

impl PageFrame {
    #[inline(always)]
    pub const fn new(pfn: usize) -> Self {
        Self(pfn)
    }

    #[inline(always)]
    pub const fn from_paddr(paddr: crate::mem::kdm::Paddr) -> Self {
        Self(paddr.to_raw() / 4096)
    }

    #[inline(always)]
    pub fn from_vaddr(vaddr: crate::mem::kdm::Vaddr) -> Self {
        Self::from_paddr(vaddr.to_phys())
    }

    #[inline(always)]
    pub const fn pfn(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub const fn paddr(self) -> crate::mem::kdm::Paddr {
        crate::mem::kdm::Paddr::from_raw(self.0 * 4096)
    }

    #[inline(always)]
    pub fn vaddr(self) -> crate::mem::kdm::Vaddr {
        self.paddr().to_virt()
    }

    #[inline(always)]
    pub fn page(self) -> Option<&'static Page> {
        get_page(self.0)
    }

    #[inline(always)]
    pub fn page_ptr(self) -> Option<*mut Page> {
        get_page_ptr(self.0)
    }

    #[inline(always)]
    pub fn is_valid(self) -> bool {
        get_page_ptr(self.0).is_some()
    }

    #[inline(always)]
    pub fn flags(self) -> PageFlags {
        if let Some(page) = self.page() {
            PageFlags::from_bits_truncate(page.flags.load(Ordering::Acquire))
        } else {
            PageFlags::empty()
        }
    }

    #[inline(always)]
    pub fn set_flags(self, flags: PageFlags) -> Self {
        if let Some(page) = self.page() {
            page.flags.store(flags.bits(), Ordering::Release);
        }
        self
    }

    #[inline(always)]
    pub fn flags_or(self, flags: PageFlags) -> Self {
        if let Some(page) = self.page() {
            page.flags.fetch_or(flags.bits(), Ordering::AcqRel);
        }
        self
    }

    #[inline(always)]
    pub fn flags_and(self, flags: PageFlags) -> Self {
        if let Some(page) = self.page() {
            page.flags.fetch_and(flags.bits(), Ordering::AcqRel);
        }
        self
    }

    #[inline(always)]
    pub fn clear_flags(self, flags: PageFlags) -> Self {
        if let Some(page) = self.page() {
            page.flags.fetch_and(!flags.bits(), Ordering::AcqRel);
        }
        self
    }

    #[inline(always)]
    pub fn is_free(self) -> bool {
        self.flags().contains(PageFlags::FREE)
    }

    #[inline(always)]
    pub fn is_allocated(self) -> bool {
        self.flags().contains(PageFlags::ALLOCATED)
    }

    #[inline(always)]
    pub fn is_reserved(self) -> bool {
        self.flags().contains(PageFlags::RESERVED)
    }

    #[inline(always)]
    pub fn count(self) -> u32 {
        if let Some(page) = self.page() {
            page.count.load(Ordering::Acquire)
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn inc_count(self) -> u32 {
        if let Some(page) = self.page() {
            page.count.fetch_add(1, Ordering::AcqRel) + 1
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn dec_count(self) -> u32 {
        if let Some(page) = self.page() {
            page.count.fetch_sub(1, Ordering::AcqRel) - 1
        } else {
            0
        }
    }

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

    #[inline(always)]
    pub fn order(self) -> u8 {
        if let Some(page) = self.page() {
            page.order.load(Ordering::Relaxed)
        } else {
            0
        }
    }

    #[inline(always)]
    pub unsafe fn set_order(self, order: u8) -> Self {
        if let Some(page_ptr) = self.page_ptr() {
            (unsafe { &*page_ptr }).order.store(order, Ordering::Relaxed);
        }
        self
    }

    #[inline(always)]
    pub fn private(self) -> u32 {
        if let Some(page) = self.page() {
            page.private.load(Ordering::Relaxed)
        } else {
            0
        }
    }

    #[inline(always)]
    pub unsafe fn set_private(self, private: u32) -> Self {
        if let Some(page_ptr) = self.page_ptr() {
            (unsafe { &*page_ptr }).private.store(private, Ordering::Relaxed);
        }
        self
    }
}
