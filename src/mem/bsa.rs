//! Buddy System Allocator – dispatches to DMA, DMA32, and NORM zones.

use crate::mem::bsu::{Bsu, MAX_ORDER};
use crate::mem::kdm::Paddr;
use crate::mem::pfm::{pfn_to_page, PAGE_SIZE};

// Zone PFN ranges
const DMA_START: usize = 0;
const DMA_END: usize = (16 * 1024 * 1024) / PAGE_SIZE;
const DMA32_START: usize = 0;
const DMA32_END: usize = (4 * 1024 * 1024 * 1024) / PAGE_SIZE;
const NORM_START: usize = DMA32_END;
const NORM_END: usize = usize::MAX;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GfpFlags(u32);

impl GfpFlags {
    pub const GFP_DMA: Self = Self(1 << 0);
    pub const GFP_DMA32: Self = Self(1 << 1);
    pub const GFP_NORMAL: Self = Self(1 << 2);
    pub const GFP_KERNEL: Self = Self::GFP_NORMAL;
}

pub struct Bsa {
    dma: Bsu<DMA_START, DMA_END>,
    dma32: Bsu<DMA32_START, DMA32_END>,
    norm: Bsu<NORM_START, NORM_END>,
}

impl Bsa {
    pub fn new() -> Self {
        info!("BSA::new: creating zones");
        Self {
            dma: Bsu::new(),
            dma32: Bsu::new(),
            norm: Bsu::new(),
        }
    }

    pub fn usage(&self) -> [usize; 3]
    {
        [self.dma.usage(), self.dma32.usage(), self.norm.usage()]
    }

    pub fn init(&self) {
        info!("BSA: initialising DMA zone");
        self.dma.init();
        info!("BSA: initialising DMA32 zone");
        self.dma32.init();
        info!("BSA: initialising NORM zone");
        self.norm.init();
        info!("BSA: all zones initialised");
    }

    pub fn alloc_pages(&mut self, order: usize, gfp: GfpFlags) -> Option<Paddr> {
        trace!("BSA::alloc_pages: order={}, flags={:?}", order, gfp);
        if order > MAX_ORDER {
            error!("BSA: allocation order {} exceeds MAX_ORDER", order);
            return None;
        }
        if gfp == GfpFlags::GFP_NORMAL || gfp == GfpFlags::GFP_KERNEL {
            if let Some(pfn) = self.norm.alloc_pages(order) {
                debug!("BSA: allocated from NORM zone, PFN {}", pfn);
                return Some(Paddr::from_raw(pfn * PAGE_SIZE));
            }
            debug!("BSA: NORM zone failed, trying lower zones");
        }
        if gfp == GfpFlags::GFP_DMA32 || gfp == GfpFlags::GFP_KERNEL {
            if let Some(pfn) = self.dma32.alloc_pages(order) {
                debug!("BSA: allocated from DMA32 zone, PFN {}", pfn);
                return Some(Paddr::from_raw(pfn * PAGE_SIZE));
            }
            debug!("BSA: DMA32 zone failed");
        }
        if gfp == GfpFlags::GFP_DMA || gfp == GfpFlags::GFP_KERNEL {
            if let Some(pfn) = self.dma.alloc_pages(order) {
                debug!("BSA: allocated from DMA zone, PFN {}", pfn);
                return Some(Paddr::from_raw(pfn * PAGE_SIZE));
            }
            debug!("BSA: DMA zone failed");
        }
        warn!("BSA: failed to allocate order {} with flags {:?}", order, gfp);
        None
    }

    pub fn free_pages(&mut self, paddr: Paddr) {
        let pfn = paddr.to_raw() / PAGE_SIZE;
        trace!("BSA::free_pages: PFN {}", pfn);
        let order = pfn_to_page(pfn).order() as usize;
        if order > MAX_ORDER {
            error!("BSA: invalid order {} at PFN {:#X}", order, pfn);
            return;
        }
        if pfn < DMA_END {
            trace!("BSA: freeing in DMA zone");
            self.dma.free_pages(pfn);
        } else if pfn < DMA32_END {
            trace!("BSA: freeing in DMA32 zone");
            self.dma32.free_pages(pfn);
        } else {
            trace!("BSA: freeing in NORM zone");
            self.norm.free_pages(pfn);
        }
    }
}

// Global instance
static mut BSA: Option<Bsa> = None;
use core::sync::atomic::{AtomicBool, Ordering};
static BSA_INIT: AtomicBool = AtomicBool::new(false);

pub fn init() {
    info!("BSA: initialising global instance");
    if BSA_INIT.swap(true, Ordering::SeqCst) {
        panic!("BSA already initialised");
    }
    let bsa = Bsa::new();
    bsa.init();
    unsafe { BSA = Some(bsa); }
    info!("BSA: ready");
}

#[allow(static_mut_refs)]
pub fn alloc_pages(order: usize, gfp: GfpFlags) -> Option<Paddr> {
    trace!("BSA::alloc_pages (global): order={}", order);
    unsafe { BSA.as_mut().unwrap().alloc_pages(order, gfp) }
}

#[allow(static_mut_refs)]
pub fn free_pages(paddr: Paddr) {
    trace!("BSA::free_pages (global): paddr={:#X}", paddr.to_raw());
    unsafe { BSA.as_mut().unwrap().free_pages(paddr) }
}

#[allow(static_mut_refs)]
pub fn usage() -> [usize; 3] {
    unsafe { BSA.as_ref().unwrap().usage() }
}
