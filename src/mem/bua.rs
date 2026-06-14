//! BSA‑UPA Adapter – bridges the UPA interface to the BSA.

use crate::mem::bsa::{self, GfpFlags};
use crate::mem::kdm::Paddr;
use crate::mem::pfm::pfn_to_page;

/// Allocate `count` physical pages.
pub fn alloc(count: usize) -> Paddr {
    trace!("BUA::alloc: count={}", count);
    if count == 0 {
        trace!("BUA: alloc(0) -> null");
        return Paddr::from_raw(0);
    }
    let order = count.next_power_of_two().trailing_zeros() as usize;
    trace!("BUA: count {} -> order {}", count, order);
    if let Some(paddr) = bsa::alloc_pages(order, GfpFlags::GFP_KERNEL) {
        let pfn = paddr.to_raw() / 4096;
        pfn_to_page(pfn).set_order(order as u8);
        debug!("BUA: allocated {} pages (order {}) at PFN {}", count, order, pfn);
        paddr
    } else {
        error!("BUA: failed to allocate {} pages (order {})", count, order);
        Paddr::from_raw(0)
    }
}

/// Free a block previously allocated.
pub fn free(paddr: Paddr) {
    if paddr.to_raw() == 0 {
        trace!("BUA: free(null) ignored");
        return;
    }
    let pfn = paddr.to_raw() / 4096;
    let order = pfn_to_page(pfn).order() as usize;
    trace!("BUA: freeing PFN {} (order {})", pfn, order);
    bsa::free_pages(paddr);
}
