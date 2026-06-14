//! Page Frame Manager (PFM) – a sparsemem / vmemmap‑style metadata array.

use core::sync::atomic::{AtomicU8, AtomicU16, AtomicU32, Ordering};
use heapless::Vec;
use crate::mem::kdm::Paddr;
use crate::mem::ptm::{EntryFlags, Polen};
use crate::mem::upa;
use crate::mem::reg::{self, Kind};

pub const PAGE_SIZE: usize = 4096;
pub const VMEMMAP_BASE: usize = 0xffff_D000_0000_0000;
pub const PF_RESERVED: u16 = 1 << 0;

#[repr(C, align(8))]
pub struct PageFrame {
    refcount: AtomicU32,
    flags: AtomicU16,
    order: AtomicU8,
    _pad: u8,
    next_free: AtomicU32,
}

const INVALID_PFN: u32 = 0xffff_ffff;

impl PageFrame {
    pub const fn zero() -> Self {
        Self {
            refcount: AtomicU32::new(0),
            flags: AtomicU16::new(0),
            order: AtomicU8::new(0),
            _pad: 0,
            next_free: AtomicU32::new(INVALID_PFN),
        }
    }

    pub fn inc_ref(&self) { self.refcount.fetch_add(1, Ordering::Relaxed); }
    pub fn dec_ref(&self) -> u32 { self.refcount.fetch_sub(1, Ordering::Release) }
    pub fn refcount(&self) -> u32 { self.refcount.load(Ordering::Acquire) }
    pub fn set_refcount(&self, val: u32) { self.refcount.store(val, Ordering::Release); }

    pub fn set_flags(&self, flags: u16) { self.flags.store(flags, Ordering::Release); }
    pub fn flags(&self) -> u16 { self.flags.load(Ordering::Acquire) }

    pub fn set_order(&self, order: u8) { self.order.store(order, Ordering::Release); }
    pub fn order(&self) -> u8 { self.order.load(Ordering::Acquire) }

    pub fn set_next_free(&self, pfn: u32) { self.next_free.store(pfn, Ordering::Release); }
    pub fn next_free(&self) -> Option<usize> {
        let v = self.next_free.load(Ordering::Acquire);
        if v == INVALID_PFN { None } else { Some(v as _) }
    }
}

pub fn pfn_to_page(pfn: usize) -> &'static PageFrame {
    let offset = pfn * core::mem::size_of::<PageFrame>();
    unsafe { &*( (VMEMMAP_BASE + offset) as *const PageFrame ) }
}

pub fn page_to_pfn(page: &PageFrame) -> usize {
    ( (page as *const _ as usize) - VMEMMAP_BASE ) / core::mem::size_of::<PageFrame>()
}

pub fn reserve(pfn: usize) {
    trace!("PFM: reserving PFN {}", pfn);
    let page = pfn_to_page(pfn);
    let flags = page.flags();
    page.set_flags(flags | PF_RESERVED);
    page.set_refcount(1);
}

pub fn unreserve(pfn: usize) {
    trace!("PFM: unreserving PFN {}", pfn);
    let page = pfn_to_page(pfn);
    let flags = page.flags();
    page.set_flags(flags & !PF_RESERVED);
    page.set_refcount(0);
}

pub fn is_reserved(pfn: usize) -> bool {
    pfn_to_page(pfn).flags() & PF_RESERVED != 0
}

static mut ALLOCATED_METADATA_PFNS: Vec<usize, 65536> = Vec::new(); // enough for max PFN/256

/// Initialise the PFM: allocate and map metadata pages for all usable memory.
#[allow(static_mut_refs)]
pub fn init(polen: &mut Polen) {
    info!("PFM: initialising vmemmap at {:#X}", VMEMMAP_BASE);

    // 1. Determine maximum PFN from usable regions
    let mut max_phys = 0;
    for region in reg::iter() {
        if region.kind == Kind::USABLE {
            let end = region.base + region.len;
            if end > max_phys { max_phys = end; }
            trace!("PFM: usable region base={:#X}, len={:#X}, end={:#X}", region.base, region.len, end);
        }
    }
    let max_pfn = (max_phys + PAGE_SIZE - 1) >> 12;
    info!("PFM: max PFN = {}", max_pfn);

    // 2. Process each usable region, allocate and map metadata pages
    for region in reg::iter() {
        if region.kind != Kind::USABLE {
            trace!("PFM: skipping non‑usable region kind {:?}", region.kind);
            continue;
        }
        let start_pfn = (region.base >> 12) as u32;
        let end_pfn = ((region.base + region.len + PAGE_SIZE - 1) >> 12) as u32;
        let start_offset = (start_pfn as usize) * core::mem::size_of::<PageFrame>();
        let end_offset   = (end_pfn   as usize) * core::mem::size_of::<PageFrame>();
        let start_vaddr = VMEMMAP_BASE + start_offset;
        let len = end_offset - start_offset;

        let map_start = start_vaddr & !(PAGE_SIZE - 1);
        let map_end   = (start_vaddr + len + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        let total_bytes = map_end - map_start;

        info!("PFM: processing region PFN {}..{} -> VA {:#X}..{:#X} ({} bytes)",
              start_pfn, end_pfn, map_start, map_end, total_bytes);

        let mut curr_vaddr = map_start;
        let mut remaining = total_bytes;
        while remaining > 0 {
            debug!("PFM: allocating metadata page for VA {:#X}, remaining bytes {}", curr_vaddr, remaining);
            let phys = upa::alloc(1);
            if phys.to_raw() == 0 {
                panic!("PFM: out of memory while allocating metadata page at VA {:#X}", curr_vaddr);
            }
            let pfn = phys.to_raw() / PAGE_SIZE;
            info!("PFM: allocated phys page {:#X} (PFN {}) for VA {:#X}", phys.to_raw(), pfn, curr_vaddr);

            // Map the page into the vmemmap area
            debug!("PFM: mapping VA {:#X} -> PA {:#X}", curr_vaddr, phys.to_raw());
            if let Err(e) = polen.exco.try_map4k(curr_vaddr, phys, EntryFlags::PRESENT | EntryFlags::WRITABLE) {
                error!("PFM: mapping failed at VA {:#X}: {}", curr_vaddr, e);
                panic!("PFM: try_map4k failed");
            }
            trace!("PFM: mapped successfully");

            // Zero the page
            unsafe {
                let slice = core::slice::from_raw_parts_mut(curr_vaddr as *mut u8, PAGE_SIZE);
                slice.fill(0);
                trace!("PFM: zeroed page at VA {:#X}", curr_vaddr);
            }

            // Store PFN for later reservation (cannot reserve now because the vmemmap for this PFN may not be mapped yet)
            unsafe {
                if ALLOCATED_METADATA_PFNS.push(pfn).is_err() {
                    panic!("PFM: too many metadata pages");
                }
            }

            curr_vaddr += PAGE_SIZE;
            remaining -= PAGE_SIZE;
        }
    }

    // 3. Now that all vmemmap pages are mapped, reserve the physical pages used for metadata
    info!("PFM: reserving {} metadata physical pages", unsafe { ALLOCATED_METADATA_PFNS.len() });
    for i in 0..unsafe { ALLOCATED_METADATA_PFNS.len() } {
        let pfn = unsafe { ALLOCATED_METADATA_PFNS[i] };
        reserve(pfn);
    }

    info!("PFM: initialisation complete");
}
