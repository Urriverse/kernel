//! Buddy System Unit – per‑zone physical page allocator.

use crate::mem::pfm::{pfn_to_page, PAGE_SIZE, is_reserved};
use crate::sync::Mutex;
use core::sync::atomic::Ordering;

pub const MAX_ORDER: usize = 10;  // 2^10 * 4 KiB = 4 MiB

pub struct Bsu<const START_PFN: usize, const END_PFN: usize> {
    free_lists: [Mutex<Option<usize>>; MAX_ORDER + 1],
}

impl<const START: usize, const END: usize> Bsu<START, END> {
    pub const fn new() -> Self {
        const INIT: Mutex<Option<usize>> = Mutex::new(None);
        Self { free_lists: [INIT; MAX_ORDER + 1] }
    }

    pub fn init(&self) {
        info!("BSU[{:#X}..{:#X}]: initialising", START << 12, END << 12);
        let mut total_added = 0;
        for region in crate::mem::reg::iter() {
            if region.kind != crate::mem::reg::Kind::USABLE {
                trace!("BSU: skipping non‑usable region kind {:?}", region.kind);
                continue;
            }
            let start_pfn = (region.base + PAGE_SIZE - 1) / PAGE_SIZE;
            let end_pfn = (region.base + region.len) / PAGE_SIZE;
            let zone_start = START.max(start_pfn);
            let zone_end = END.min(end_pfn);
            if zone_start >= zone_end {
                trace!("BSU: region PFN [{},{}) outside zone", start_pfn, end_pfn);
                continue;
            }
            // Scan the zone for contiguous non‑reserved runs
            let mut pfn = zone_start;
            while pfn < zone_end {
                if is_reserved(pfn) {
                    pfn += 1;
                    continue;
                }
                let run_start = pfn;
                while pfn < zone_end && !is_reserved(pfn) {
                    pfn += 1;
                }
                let run_len = pfn - run_start;
                total_added += run_len;
                debug!("BSU: adding contiguous run PFN {}..+{}", run_start, run_len);
                self.add_contiguous_range(run_start, run_len);
            }
        }
        info!("BSU: initialised, added {} pages total", total_added);
    }

    /// Insert a contiguous range of free pages efficiently.
    /// No recursion – splits the range into the largest possible aligned blocks.
    fn add_contiguous_range(&self, start_pfn: usize, len: usize) {
        trace!("BSU::add_contiguous_range: start={}, len={}", start_pfn, len);
        let mut pfn = start_pfn;
        let mut remaining = len;
        while remaining > 0 {
            // Find the largest order that satisfies:
            //  - block_size <= remaining
            //  - pfn is aligned to block_size
            let mut order = MAX_ORDER;
            let block_size = loop {
                let size = 1 << order;
                if size <= remaining && (pfn & (size - 1)) == 0 {
                    break size;
                }
                if order == 0 {
                    break 1;
                }
                order -= 1;
            };
            trace!("BSU: freeing block order {} at PFN {} (size {})", order, pfn, block_size);
            self.free_pages_of_order(pfn, order);
            pfn += block_size;
            remaining -= block_size;
        }
    }

    fn free_pages_of_order(&self, pfn: usize, order: usize) {
        trace!("BSU::free_pages_of_order: pfn={}, order={}", pfn, order);
        debug_assert!(order <= MAX_ORDER);
        let page = pfn_to_page(pfn);
        for i in 0..(1 << order) {
            let pg = pfn_to_page(pfn + i);
            pg.set_refcount(0);
            pg.set_flags(0);
            pg.set_order(0);
        }
        let mut list = self.free_lists[order].lock();
        page.set_next_free(pfn_to_next_pfn(*list));
        *list = Some(pfn);
        drop(list);
        self.coalesce(pfn, order);
    }

    fn coalesce(&self, pfn: usize, order: usize) {
        trace!("BSU::coalesce: pfn={}, order={}", pfn, order);
        if order == MAX_ORDER {
            trace!("BSU: max order reached, no coalesce");
            return;
        }
        let buddy = pfn ^ (1 << order);
        if buddy < START || buddy >= END {
            trace!("BSU: buddy {} out of zone", buddy);
            return;
        }
        let buddy_free = {
            let list = self.free_lists[order].lock();
            let mut cur = *list;
            while let Some(block) = cur {
                if block == buddy { break; }
                cur = pfn_to_page(block).next_free();
            }
            cur.is_some()
        };
        if !buddy_free {
            trace!("BSU: buddy {} not free", buddy);
            return;
        }
        debug!("BSU: coalescing order {} blocks PFN {} and {} into order {}",
               order, pfn, buddy, order + 1);
        self.remove_from_list(pfn, order);
        self.remove_from_list(buddy, order);
        let merged = pfn.min(buddy);
        self.free_pages_of_order(merged, order + 1);
    }

    fn remove_from_list(&self, pfn: usize, order: usize) {
        trace!("BSU::remove_from_list: pfn={}, order={}", pfn, order);
        let mut list = self.free_lists[order].lock();
        let mut prev = None;
        let mut cur = *list;
        while let Some(block) = cur {
            if block == pfn {
                let next = pfn_to_page(block).next_free();
                if let Some(prev_pfn) = prev {
                    pfn_to_page(prev_pfn).set_next_free(pfn_to_next_pfn(next));
                } else {
                    *list = next;
                }
                pfn_to_page(pfn).set_next_free(INVALID_PFN);
                trace!("BSU: removed PFN {} from order {} free list", pfn, order);
                break;
            }
            prev = cur;
            cur = pfn_to_page(block).next_free();
        }
    }

    pub fn alloc_pages(&self, order: usize) -> Option<usize> {
        trace!("BSU::alloc_pages: order={}", order);
        if order > MAX_ORDER {
            error!("BSU: allocation order {} exceeds MAX_ORDER", order);
            return None;
        }
        for o in order..=MAX_ORDER {
            let mut list = self.free_lists[o].lock();
            if let Some(pfn) = *list {
                *list = pfn_to_page(pfn).next_free();
                drop(list);
                debug!("BSU: allocated order {} block at PFN {} (from order {} list)", order, pfn, o);
                let mut block_pfn = pfn;
                for split_order in (order..o).rev() {
                    let buddy = block_pfn + (1 << split_order);
                    debug!("BSU: splitting, freeing buddy PFN {} at order {}", buddy, split_order);
                    self.free_pages_of_order(buddy, split_order);
                }
                for i in 0..(1 << order) {
                    let pg = pfn_to_page(block_pfn + i);
                    pg.set_refcount(1);
                    pg.set_flags(0);
                    pg.set_order(order as u8);
                }
                return Some(block_pfn);
            }
        }
        warn!("BSU: failed to allocate order {}", order);
        None
    }

    pub fn free_pages(&self, pfn: usize) {
        trace!("BSU::free_pages: pfn={}", pfn);
        if is_reserved(pfn) {
            warn!("BSU: attempt to free reserved PFN {}", pfn);
            return;
        }
        let order = pfn_to_page(pfn).order() as usize;
        if order > MAX_ORDER {
            error!("BSU: invalid order {} at PFN {:#X}", order, pfn);
            return;
        }
        let page = pfn_to_page(pfn);
        let old = page.dec_ref();
        if old > 1 {
            trace!("BSU: PFN {} refcount now {} (not freeing)", pfn, old - 1);
            return;
        }
        debug_assert_eq!(old, 1);
        debug!("BSU: freeing order {} block at PFN {}", order, pfn);
        self.free_pages_of_order(pfn, order);
    }
}

const INVALID_PFN: u32 = 0xffff_ffff;
fn pfn_to_next_pfn(opt: Option<usize>) -> u32 {
    opt.map(|p| p as u32).unwrap_or(INVALID_PFN)
}
