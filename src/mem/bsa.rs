use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem::MaybeUninit;
use crate::mem::{pfm, pmr, kdm::Paddr, ema};
use crate::sync::{Nutex, Nitex};
use crate::arch;
use heapless::Vec;

pub const MAX_ORDER: usize = 10;

const PCP_SIZE: usize = 32;

const DMA_END: usize = 16 * 1024 * 1024;      // 16 MiB
const DMA32_END: usize = 4 * 1024 * 1024 * 1024; // 4 GiB

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Dma,
    Dma32,
    Normal,
}

impl Zone {
    #[inline]
    pub fn from_pfn(pfn: usize) -> Self {
        let paddr = pfn * 4096;
        if paddr < DMA_END {
            Zone::Dma
        } else if paddr < DMA32_END {
            Zone::Dma32
        } else {
            Zone::Normal
        }
    }

    #[inline]
    pub const fn index(self) -> usize {
        match self {
            Zone::Dma => 0,
            Zone::Dma32 => 1,
            Zone::Normal => 2,
        }
    }
}

struct FreeArea {
    head: Option<usize>,
    count: usize,
}

impl FreeArea {
    const fn new() -> Self {
        Self {
            head: None,
            count: 0,
        }
    }

    fn add(&mut self, pfn: usize) {
        if let Some(page) = pfm::get_page(pfn) {
            page.private.store(
                self.head.unwrap_or(0) as u32,
                Ordering::Release
            );
            self.head = Some(pfn);
            self.count += 1;
        }
    }

    fn remove(&mut self) -> Option<usize> {
        if let Some(pfn) = self.head {
            if let Some(page) = pfm::get_page(pfn) {
                let next = page.private.load(Ordering::Acquire) as usize;
                self.head = if next == 0 { None } else { Some(next) };
                self.count -= 1;
                Some(pfn)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

#[repr(align(64))]
struct PerCpuCache {
    pages: [Vec<usize, PCP_SIZE>; MAX_ORDER],
}

impl PerCpuCache {
    const fn new() -> Self {
        Self {
            pages: [
                Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(),
            ],
        }
    }

    fn try_alloc(&mut self, order: usize) -> Option<usize> {
        if order < MAX_ORDER {
            self.pages[order].pop()
        } else {
            None
        }
    }

    fn try_free(&mut self, order: usize, pfn: usize) -> bool {
        if order < MAX_ORDER && !self.pages[order].is_full() {
            let _ = self.pages[order].push(pfn);
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn is_empty(&self, order: usize) -> bool {
        order >= MAX_ORDER || self.pages[order].is_empty()
    }

    #[allow(dead_code)]
    fn is_full(&self, order: usize) -> bool {
        order < MAX_ORDER && self.pages[order].is_full()
    }

    #[allow(dead_code)]
    fn clear(&mut self, order: usize) {
        if order < MAX_ORDER {
            self.pages[order].clear();
        }
    }
}

struct ZoneInner {
    free_areas: [FreeArea; MAX_ORDER],
    start_pfn: usize,
    end_pfn: usize,
}

impl ZoneInner {
    const fn new(start_pfn: usize, end_pfn: usize) -> Self {
        Self {
            free_areas: [ const { FreeArea::new() }; MAX_ORDER ],
            start_pfn,
            end_pfn,
        }
    }
}

struct ZoneData {
    lock: Nutex<ZoneInner>,
    pcp: [MaybeUninit<Nitex<PerCpuCache>>; arch::MAX_CPUS],
    free_pages: AtomicUsize,
    pcp_pages: AtomicUsize,
}

impl ZoneData {
    const fn new(start_pfn: usize, end_pfn: usize) -> Self {
        Self {
            lock: Nutex::new(ZoneInner::new(start_pfn, end_pfn)),
            pcp: [ const { MaybeUninit::uninit() }; arch::MAX_CPUS ],
            free_pages: AtomicUsize::new(0),
            pcp_pages: AtomicUsize::new(0),
        }
    }

    #[inline]
    unsafe fn pcp(&self, cpu_id: usize) -> &Nitex<PerCpuCache> {
        unsafe { self.pcp[cpu_id].assume_init_ref() }
    }

    fn init_pcp(&self) {
        for i in 0..arch::MAX_CPUS {
            unsafe {
                let ptr = self.pcp[i].as_ptr();
                *(ptr as *mut Nitex<PerCpuCache>).as_mut_unchecked() = Nitex::new(PerCpuCache::new());
            }
        }
    }
}

static ZONES: [ZoneData; 3] = [
    ZoneData::new(0, 0),
    ZoneData::new(0, 0),
    ZoneData::new(0, 0),
];

#[inline]
fn log2_ceil(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let mut order = 0;
    let mut size = 1;
    while size < n {
        size <<= 1;
        order += 1;
    }
    order
}

pub fn init() {
    info!("Initializing BSA");

    for zone in &ZONES {
        zone.init_pcp();
    }

    let (ema_start, ema_end) = ema::get_allocated_range();
    let ema_start_pfn = ema_start / 4096;
    let ema_end_pfn = ema_end.div_ceil(4096);

    let mut zone_boundaries = [(0usize, 0usize); 3];

    let mut max_pfn = 0;
    for region in pmr::iter() {
        let end_pfn = (region.base + region.len) / 4096;
        if end_pfn > max_pfn {
            max_pfn = end_pfn;
        }
    }

    let dma_end_pfn = DMA_END / 4096;
    let dma32_end_pfn = DMA32_END / 4096;

    zone_boundaries[0] = (0, core::cmp::min(max_pfn, dma_end_pfn));
    zone_boundaries[1] = (
        core::cmp::min(max_pfn, dma_end_pfn),
        core::cmp::min(max_pfn, dma32_end_pfn)
    );
    zone_boundaries[2] = (core::cmp::min(max_pfn, dma32_end_pfn), max_pfn);

    for (i, &(start, end)) in zone_boundaries.iter().enumerate() {
        let zone = &ZONES[i];
        let mut inner = zone.lock.lock();
        inner.start_pfn = start;
        inner.end_pfn = end;
        drop(inner);

        let zone_name = match i {
            0 => "DMA",
            1 => "DMA32",
            2 => "Normal",
            _ => "Unknown",
        };

        info!(
            "Zone {} initialized: PFN {} - {} ({} pages)",
            zone_name,
            start,
            end,
            end.saturating_sub(start)
        );
    }

    for region in pmr::iter() {
        let start_pfn = region.base / 4096;
        let end_pfn = (region.base + region.len) / 4096;

        for pfn in start_pfn..end_pfn {
            if let Some(page) = pfm::get_page(pfn) {
                page.flags.store(
                    pfm::PageFlags::ALLOCATED.bits(),
                    Ordering::Release
                );
            }
        }
    }

    if ema_end > ema_start {
        for pfn in ema_start_pfn..ema_end_pfn {
            if let Some(page) = pfm::get_page(pfn) {
                page.flags.store(
                    pfm::PageFlags::RESERVED.bits(),
                    Ordering::Release
                );
            }
        }
        info!(
            "Marked {} EMA pages as RESERVED (PFN {} - {})",
            ema_end_pfn - ema_start_pfn,
            ema_start_pfn,
            ema_end_pfn
        );
    }

    for region in pmr::iter() {
        if region.kind == pmr::Kind::USABLE {
            let start_pfn = region.base / 4096;
            let end_pfn = (region.base + region.len) / 4096;

            let align_mask = (1 << MAX_ORDER) - 1;
            let aligned_start = (start_pfn + align_mask) & !align_mask;
            let aligned_end = end_pfn & !align_mask;

            let mut pfn = aligned_start;
            while pfn < aligned_end {
                let mut order = MAX_ORDER - 1;
                while order > 0 {
                    let block_size = 1 << order;
                    if pfn.is_multiple_of(block_size) && pfn + block_size <= aligned_end {
                        break;
                    }
                    order -= 1;
                }

                let block_size = 1 << order;
                if pfn + block_size <= aligned_end {
                    for i in 0..block_size {
                        if let Some(page) = pfm::get_page(pfn + i) {
                            page.flags.store(
                                pfm::PageFlags::FREE.bits(),
                                Ordering::Release
                            );
                            if i == 0 {
                                page.flags.fetch_or(pfm::PageFlags::BUDDY_HEAD.bits(), Ordering::Release);
                                page.order.store(order as u8, Ordering::Release);
                            }
                        }
                    }

                    let zone = Zone::from_pfn(pfn);
                    let zone_data = &ZONES[zone.index()];
                    let mut inner = zone_data.lock.lock();
                    inner.free_areas[order].add(pfn);
                    zone_data.free_pages.fetch_add(block_size, Ordering::Release);
                    drop(inner);

                    pfn += block_size;
                } else {
                    break;
                }
            }
        }
    }

    let stats = usage();
    info!(
        "Initialized. Free pages: DMA={}, DMA32={}, Normal={}",
        stats[0], stats[1], stats[2]
    );
}

pub fn alloc(count: usize) -> Paddr {
    if count == 0 {
        return Paddr::from_raw(0);
    }

    let order = log2_ceil(count);
    if order >= MAX_ORDER {
        error!("Allocation too large ({} pages, order {})", count, order);
        return Paddr::from_raw(0);
    }

    let cpu_id = arch::current_cpu();

    for zone_idx in [1, 2, 0] {
        let zone = &ZONES[zone_idx];
        
        {
            let pcp = unsafe { zone.pcp(cpu_id) };
            let mut pcp_guard = pcp.lock();
            if let Some(pfn) = pcp_guard.try_alloc(order) {
                zone.pcp_pages.fetch_sub(1 << order, Ordering::Release);
                return Paddr::from_raw(pfn * 4096);
            }
        }

        if let Some(pfn) = alloc_from_zone(zone_idx, order) {
            refill_pcp(zone_idx, order, cpu_id);
            return Paddr::from_raw(pfn * 4096);
        }
    }

    error!("Out of memory (requested {} pages)", count);
    Paddr::from_raw(0)
}

fn refill_pcp(zone_idx: usize, order: usize, cpu_id: usize) {
    let zone = &ZONES[zone_idx];
    let mut inner = zone.lock.lock();
    let pcp = unsafe { zone.pcp(cpu_id) };
    let mut pcp_guard = pcp.lock();

    let target_count = PCP_SIZE / 2;
    let mut count = 0;

    while count < target_count {
        let mut found_order = order;
        while found_order < MAX_ORDER {
            if !inner.free_areas[found_order].is_empty() {
                break;
            }
            found_order += 1;
        }

        if found_order >= MAX_ORDER {
            break;
        }

        let pfn = match inner.free_areas[found_order].remove() {
            Some(pfn) => pfn,
            None => break,
        };

        while found_order > order {
            found_order -= 1;
            let buddy_pfn = pfn + (1 << found_order);

            for i in 0..(1 << found_order) {
                if let Some(page) = pfm::get_page(buddy_pfn + i) {
                    if i == 0 {
                        page.flags.store(
                            (pfm::PageFlags::FREE | pfm::PageFlags::BUDDY_HEAD).bits(),
                            Ordering::Release,
                        );
                        page.order.store(found_order as u8, Ordering::Release);
                    } else {
                        page.flags.store(pfm::PageFlags::FREE.bits(), Ordering::Release);
                    }
                }
            }

            inner.free_areas[found_order].add(buddy_pfn);
        }

        if pcp_guard.try_free(order, pfn) {
            if let Some(page) = pfm::get_page(pfn) {
                page.flags.store(
                    (pfm::PageFlags::ALLOCATED | pfm::PageFlags::BUDDY_HEAD).bits(),
                    Ordering::Release,
                );
                page.order.store(order as u8, Ordering::Release);
            }
            zone.free_pages.fetch_sub(1 << order, Ordering::Release);
            zone.pcp_pages.fetch_add(1 << order, Ordering::Release);
            count += 1;
        } else {
            if let Some(page) = pfm::get_page(pfn) {
                page.flags.store(
                    (pfm::PageFlags::FREE | pfm::PageFlags::BUDDY_HEAD).bits(),
                    Ordering::Release,
                );
                page.order.store(order as u8, Ordering::Release);
            }
            inner.free_areas[order].add(pfn);
            break;
        }
    }
}

fn free_to_zone(zone_idx: usize, mut pfn: usize, mut order: usize) {
    let zone = &ZONES[zone_idx];
    let mut inner = zone.lock.lock();

    if let Some(page) = pfm::get_page(pfn) {
        page.flags.store(
            (pfm::PageFlags::FREE | pfm::PageFlags::BUDDY_HEAD).bits(),
            Ordering::Release,
        );
        page.order.store(order as u8, Ordering::Release);
    }

    while order < MAX_ORDER - 1 {
        let buddy_pfn = pfn ^ (1 << order);

        if let Some(buddy_page) = pfm::get_page(buddy_pfn) {
            let buddy_flags = pfm::PageFlags::from_bits_truncate(
                buddy_page.flags.load(Ordering::Acquire),
            );
            let buddy_order = buddy_page.order.load(Ordering::Acquire) as usize;

            if buddy_flags.contains(pfm::PageFlags::FREE)
                && buddy_flags.contains(pfm::PageFlags::BUDDY_HEAD)
                && buddy_order == order
            {
                remove_from_free_list(&mut inner.free_areas[order], buddy_pfn);

                if buddy_pfn < pfn {
                    pfn = buddy_pfn;
                }

                order += 1;
                continue;
            }
        }

        break;
    }

    if let Some(page) = pfm::get_page(pfn) {
        page.flags.store(
            (pfm::PageFlags::FREE | pfm::PageFlags::BUDDY_HEAD).bits(),
            Ordering::Release,
        );
        page.order.store(order as u8, Ordering::Release);
    }

    inner.free_areas[order].add(pfn);
    zone.free_pages.fetch_add(1 << order, Ordering::Release);
}

fn remove_from_free_list(free_area: &mut FreeArea, target_pfn: usize) {
    if free_area.head == Some(target_pfn) {
        free_area.remove();
        return;
    }

    let mut current = free_area.head;
    let max_steps = free_area.count + 2;
    let mut steps = 0;

    while let Some(pfn) = current {
        steps += 1;
        if steps > max_steps {
            error!(
                "remove_from_free_list: CYCLE DETECTED! target_pfn={}, cache free_area count={}",
                target_pfn, free_area.count
            );
            return;
        }

        if let Some(page) = pfm::get_page(pfn) {
            let next = page.private.load(Ordering::Acquire) as usize;
            if next == target_pfn {
                if let Some(target_page) = pfm::get_page(target_pfn) {
                    let target_next = target_page.private.load(Ordering::Acquire);
                    page.private.store(target_next, Ordering::Release);
                }
                free_area.count -= 1;
                return;
            }
            current = if next == 0 { None } else { Some(next) };
        } else {
            break;
        }
    }

    warn!(
        "remove_from_free_list: PFN {} not found in free list (searched {} entries)",
        target_pfn, steps
    );
}

fn drain_pcp(zone_idx: usize, order: usize, cpu_id: usize) {
    let zone = &ZONES[zone_idx];
    let mut to_free = Vec::<usize, 32>::new();
    
    {
        let pcp = unsafe { zone.pcp(cpu_id) };
        let mut pcp_guard = pcp.lock();
        let drain_count = pcp_guard.pages[order].len() / 2;
        for _ in 0..drain_count {
            if let Some(pfn) = pcp_guard.pages[order].pop() {
                zone.pcp_pages.fetch_sub(1 << order, Ordering::Release);
                let _ = to_free.push(pfn);
            }
        }
    }
    
    for pfn in to_free {
        free_to_zone(zone_idx, pfn, order);
    }
}

pub fn usage() -> [usize; 3] {
    [
        ZONES[0].free_pages.load(Ordering::Acquire) + ZONES[0].pcp_pages.load(Ordering::Acquire),
        ZONES[1].free_pages.load(Ordering::Acquire) + ZONES[1].pcp_pages.load(Ordering::Acquire),
        ZONES[2].free_pages.load(Ordering::Acquire) + ZONES[2].pcp_pages.load(Ordering::Acquire),
    ]
}

#[allow(dead_code)]
pub fn alloc_from_zone_direct(zone: Zone, count: usize) -> Paddr {
    if count == 0 {
        return Paddr::from_raw(0);
    }

    let order = log2_ceil(count);
    if order >= MAX_ORDER {
        return Paddr::from_raw(0);
    }

    let zone_data = &ZONES[zone.index()];
    let cpu_id = arch::current_cpu();

    {
        let pcp = unsafe { zone_data.pcp(cpu_id) };
        let mut pcp_guard = pcp.lock();
        if let Some(pfn) = pcp_guard.try_alloc(order) {
            zone_data.pcp_pages.fetch_sub(1 << order, Ordering::Release);
            return Paddr::from_raw(pfn * 4096);
        }
    }

    if let Some(pfn) = alloc_from_zone(zone.index(), order) {
        refill_pcp(zone.index(), order, cpu_id);
        Paddr::from_raw(pfn * 4096)
    } else {
        Paddr::from_raw(0)
    }
}

fn alloc_from_zone(zone_idx: usize, order: usize) -> Option<usize> {
    let zone = &ZONES[zone_idx];
    let mut inner = zone.lock.lock();

    let mut found_order = order;
    while found_order < MAX_ORDER {
        if !inner.free_areas[found_order].is_empty() {
            break;
        }
        found_order += 1;
    }

    if found_order >= MAX_ORDER {
        return None;
    }

    let pfn = inner.free_areas[found_order].remove()?;

    while found_order > order {
        found_order -= 1;
        let buddy_pfn = pfn + (1 << found_order);

        for i in 0..(1 << found_order) {
            if let Some(page) = pfm::get_page(buddy_pfn + i) {
                if i == 0 {
                    page.flags.store(
                        (pfm::PageFlags::FREE | pfm::PageFlags::BUDDY_HEAD).bits(),
                        Ordering::Release,
                    );
                    page.order.store(found_order as u8, Ordering::Release);
                } else {
                    page.flags.store(pfm::PageFlags::FREE.bits(), Ordering::Release);
                }
            }
        }

        inner.free_areas[found_order].add(buddy_pfn);
    }

    if let Some(page) = pfm::get_page(pfn) {
        page.flags.store(
            (pfm::PageFlags::ALLOCATED | pfm::PageFlags::BUDDY_HEAD).bits(),
            Ordering::Release,
        );
        page.order.store(order as u8, Ordering::Release);
    }

    let block_size = 1 << order;
    zone.free_pages.fetch_sub(block_size, Ordering::Release);

    drop(inner);

    Some(pfn)
}

pub fn free(paddr: Paddr) {
    let pfn = paddr.to_raw() / 4096;

    if let Some(page) = pfm::get_page(pfn) {
        let order = page.order.load(Ordering::Acquire) as usize;
        let zone = Zone::from_pfn(pfn);
        let zone_data = &ZONES[zone.index()];
        let cpu_id = arch::current_cpu();

        {
            let pcp = unsafe { zone_data.pcp(cpu_id) };
            let mut pcp_guard = pcp.lock();
            if pcp_guard.try_free(order, pfn) {
                page.flags.store(
                    (pfm::PageFlags::ALLOCATED | pfm::PageFlags::BUDDY_HEAD).bits(),
                    Ordering::Release,
                );
                zone_data.pcp_pages.fetch_add(1 << order, Ordering::Release);
                return;
            }
        }

        free_to_zone(zone.index(), pfn, order);
        drain_pcp(zone.index(), order, cpu_id);
    }
}
