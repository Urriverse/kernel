//! ptm policy engine
//!
//! Provides high-level PTM API: map / remap / unmap with smart policy decisions.
//!
//! The policy engine sits on top of [`Exco`](super::exco::Exco) and automates:
//!
//! * **map** — automatically chooses the largest possible page size (4 KiB,
//!   2 MiB or 1 GiB) based on alignment and physically contiguous backing.
//!   If the target range is already partially mapped or blocked by huge pages,
//!   it splits them transparently.
//!
//! * **remap** — changes flags on an existing mapping.  If the region is a
//!   mix of page sizes it splits everything down to 4 KiB first.
//!
//! * **unmap** — unmaps a range, automatically splitting huge pages at the
//!   boundaries so that no adjacent pages are accidentally removed.

use heapless::Vec;

use super::exco::{self, is_huge, walk_entry, walk_entry_mut, Area, Exco};
use super::*;
use crate::mem::kdm::Paddr;
use crate::mem::pmr::{self, Kind};

// ---------------------------------------------------------------------------
// Helper: check physical contiguity for a range of bytes
// ---------------------------------------------------------------------------

/// Returns `true` if the physical memory range `[paddr, paddr + size)` lies
/// entirely inside a single memory region that is usable for mapping.
fn is_phys_range_contiguous(paddr: Paddr, size: usize) -> bool {
    let start = paddr.to_raw();
    let end = start + size;

    for region in pmr::iter() {
        // Only consider regions that can be mapped (USABLE, KERNEL, BOOTLOADER)
        match region.kind {
            Kind::USABLE | Kind::KERNEL | Kind::BOOTLOADER => {
                let reg_start = region.base;
                let reg_end = region.base + region.len;
                if start >= reg_start && end <= reg_end {
                    return true;
                }
            }
            _ => continue,
        }
    }
    warn!(
        "Physical range {:#X}..{:#X} is not contiguous in a usable region",
        start, end
    );
    false
}

/// Returns `true` if the physical memory starting at `paddr` is contiguous
/// over `page_size` bytes (alignment already checked).
fn is_phys_contiguous(paddr: Paddr, page_size: usize) -> bool {
    // Alignment is already verified by caller.
    is_phys_range_contiguous(paddr, page_size)
}

// ---------------------------------------------------------------------------
// Map policy — auto‑select page size
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PageSize {
    Size4K,
    Size2M,
    Size1G,
}

impl PageSize {
    fn bytes(self) -> usize {
        match self {
            PageSize::Size4K => 4096,
            PageSize::Size2M => 2 << 20,
            PageSize::Size1G => 1 << 30,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            PageSize::Size4K => "4K",
            PageSize::Size2M => "2M",
            PageSize::Size1G => "1G",
        }
    }
}

/// Choose the largest page size that can be used to map `size` bytes starting
/// at `vaddr` → `paddr`.
fn select_page_size(vaddr: usize, paddr: Paddr, size: usize) -> PageSize {
    let align_1g = 1_usize << 30;
    let align_2m = 2_usize << 20;

    if size >= align_1g
        && vaddr & (align_1g - 1) == 0
        && paddr.to_raw() & (align_1g - 1) == 0
        && is_phys_contiguous(paddr, align_1g)
    {
        PageSize::Size1G
    } else if size >= align_2m
        && vaddr & (align_2m - 1) == 0
        && paddr.to_raw() & (align_2m - 1) == 0
        && is_phys_contiguous(paddr, align_2m)
    {
        PageSize::Size2M
    } else {
        PageSize::Size4K
    }
}

// ---------------------------------------------------------------------------
// Polen — the policy engine
// ---------------------------------------------------------------------------

pub struct Polen {
    /// The underlying page‑table executor.
    pub exco: Exco,
}

impl Polen {
    /// Create a new `Polen` wrapping a fresh, empty page‑table hierarchy.
    pub fn new() -> Self {
        debug!("Polen::new");
        Polen { exco: Exco::new() }
    }

    /// Wrap an existing `Exco`.
    pub fn from_exco(exco: Exco) -> Self {
        info!("Polen::from_exco CR3 {:#018X}", exco.cr3);
        Polen { exco }
    }

    /// Create a reference page table hierarchy by:
    /// - Allocating a **new PML4** (owned by this `Polen`).
    /// - Copying **only the PML4 entries** from the current (Limine) tables,
    ///   pointing to the same child tables (no deep copy).
    /// - Then clearing all lower‑half PML4 entries (indices 0..255).
    ///
    /// The resulting `Polen` shares the bootloader’s page tables for higher‑half
    /// mappings (including stack, kernel, ACPI, etc.), but owns its own PML4.
    /// This is very memory‑efficient and avoids deep copies.
    pub fn reference() -> Self {
        info!("Polen::reference: creating shallow copy of current tables (new PML4 only)");
        let current = Exco::current();

        // Allocate a new PML4 table (owned)
        let new_root = exco::alloc_tab_zeroed();
        let new_cr3 = crate::mem::kdm::Vaddr::from_ref(new_root).to_phys().to_raw() as u64;

        // Copy all 512 PML4 entries from current root to new root
        for i in 0..512 {
            new_root.0[i] = current.root.0[i];
        }

        // Clear lower half entries (indices 0..255)
        for i in 0..256 {
            new_root.0[i] = Entry::default();
        }

        // Flush TLB? Not needed yet because we haven't activated.

        let new_exco = Exco {
            cr3: new_cr3,
            root: new_root,
            owned: true, // We own the PML4
        };

        info!("Polen::reference: new CR3 {:#018X} (shallow, lower half cleared)", new_exco.cr3);
        Polen { exco: new_exco }
    }

    // ------------------------------------------------------------------
    //  Map
    // ------------------------------------------------------------------

    /// Map a range of virtual addresses `[vaddr, vaddr + size)` to the
    /// physical range starting at `paddr` with the given `flags`.
    ///
    /// The function automatically splits the region into:
    /// - an unaligned head (mapped with 4 KiB pages),
    /// - an aligned middle (mapped with the largest possible page sizes),
    /// - an unaligned tail (mapped with 4 KiB pages).
    ///
    /// Huge pages in the way are transparently split.
    ///
    /// # Note
    /// The `size` is rounded up to the next page (4 KiB) boundary if not already aligned.
    /// A warning is logged in that case.
    ///
    /// # Panics
    ///
    /// Panics on allocation failure or internal inconsistency.
    pub fn map(&mut self, vaddr: usize, paddr: Paddr, size: usize, flags: EntryFlags) {
        self.try_map(vaddr, paddr, size, flags)
            .expect("Polen::map failed");
    }

    /// Fallible version of [`map`](Self::map).
    pub fn try_map(
        &mut self,
        vaddr: usize,
        paddr: Paddr,
        mut size: usize,
        flags: EntryFlags,
    ) -> Result<(), &'static str> {
        if vaddr & 0xfff != 0 {
            return Err("misaligned virtual address");
        }
        if size == 0 {
            return Err("zero-size mapping");
        }

        // Round up size to page boundary (4 KiB)
        let aligned_size = (size + 4095) & !4095;
        if aligned_size != size {
            warn!(
                "Polen::map: size {:#X} not page-aligned, rounding up to {:#X}",
                size, aligned_size
            );
            size = aligned_size;
        }

        // ------------------------------------------------------------------
        // 1. Compute unaligned head and tail, and the aligned middle part.
        // ------------------------------------------------------------------
        const ALIGN_2M: usize = 2 << 20;
        let first_2m_aligned = (vaddr + ALIGN_2M - 1) & !(ALIGN_2M - 1);
        let head_size = if first_2m_aligned > vaddr {
            let hs = first_2m_aligned - vaddr;
            if hs > size { size } else { hs }
        } else {
            0
        };

        let last_2m_aligned = (vaddr + size) & !(ALIGN_2M - 1);
        let tail_size = if last_2m_aligned > vaddr && last_2m_aligned < vaddr + size {
            (vaddr + size) - last_2m_aligned
        } else {
            0
        };

        let middle_start = vaddr + head_size;
        let middle_size = size - head_size - tail_size;

        // ------------------------------------------------------------------
        // 2. Map head (4K pages) – head is always < 2M, so no huge page possible.
        // ------------------------------------------------------------------
        if head_size > 0 {
            let mut head_vaddr = vaddr;
            let mut head_paddr = paddr;
            let head_bytes = head_size;
            let mut remaining = head_bytes;
            while remaining > 0 {
                let step = 4096;
                self.map_4k_block(head_vaddr, head_paddr, flags)?;
                head_vaddr += step;
                head_paddr = Paddr::from_raw(head_paddr.to_raw() + step);
                remaining -= step;
            }
        }

        // ------------------------------------------------------------------
        // 3. Map aligned middle (auto‑selects 4K/2M/1G)
        // ------------------------------------------------------------------
        if middle_size > 0 {
            let mut mid_vaddr = middle_start;
            let mut mid_paddr = Paddr::from_raw(paddr.to_raw() + head_size);
            let mut mid_rem = middle_size;
            while mid_rem > 0 {
                let ps = select_page_size(mid_vaddr, mid_paddr, mid_rem);
                let ps_bytes = ps.bytes();
                // Step is min(ps_bytes, mid_rem) – but for huge pages we require full size.
                let step = if ps_bytes > mid_rem { mid_rem } else { ps_bytes };

                match ps {
                    PageSize::Size1G => {
                        if step == ps_bytes {
                            if let Ok(entry) = walk_entry_mut(&mut self.exco.root, mid_vaddr, 2, false) {
                                if entry.is_present() && is_huge(entry) {
                                    debug!("  split 1G (blocking 1G map)");
                                    split_and_retry(self, mid_vaddr, ps)?;
                                    continue;
                                }
                            }
                            self.exco.try_map1g(mid_vaddr, mid_paddr, flags)?;
                            mid_vaddr += step;
                            mid_paddr = Paddr::from_raw(mid_paddr.to_raw() + step);
                            mid_rem -= step;
                            continue;
                        } else {
                            // Not enough for 1G, fall through to 2M or 4K.
                        }
                    }
                    PageSize::Size2M => {
                        if step == ps_bytes {
                            if let Ok(entry) = walk_entry_mut(&mut self.exco.root, mid_vaddr, 1, false) {
                                if entry.is_present() && is_huge(entry) {
                                    debug!("  split 2M (blocking 2M map)");
                                    split_and_retry(self, mid_vaddr, ps)?;
                                    continue;
                                }
                            }
                            self.exco.try_map2m(mid_vaddr, mid_paddr, flags)?;
                            mid_vaddr += step;
                            mid_paddr = Paddr::from_raw(mid_paddr.to_raw() + step);
                            mid_rem -= step;
                            continue;
                        } else {
                            // Not enough for 2M, fall through to 4K.
                        }
                    }
                    PageSize::Size4K => {
                        // Always map 4K, step is min(4096, mid_rem)
                        let step_4k = if step > 4096 { 4096 } else { step };
                        self.map_4k_block(mid_vaddr, mid_paddr, flags)?;
                        mid_vaddr += step_4k;
                        mid_paddr = Paddr::from_raw(mid_paddr.to_raw() + step_4k);
                        mid_rem -= step_4k;
                        continue;
                    }
                }

                // If we fall through (insufficient size for huge page), map as 4K.
                let step_4k = if step > 4096 { 4096 } else { step };
                self.map_4k_block(mid_vaddr, mid_paddr, flags)?;
                mid_vaddr += step_4k;
                mid_paddr = Paddr::from_raw(mid_paddr.to_raw() + step_4k);
                mid_rem -= step_4k;
            }
        }

        // ------------------------------------------------------------------
        // 4. Map tail (4K pages) – tail is always < 2M.
        // ------------------------------------------------------------------
        if tail_size > 0 {
            let mut tail_vaddr = last_2m_aligned;
            let mut tail_paddr = Paddr::from_raw(paddr.to_raw() + size - tail_size);
            let mut tail_rem = tail_size;
            while tail_rem > 0 {
                let step = 4096;
                trace!("  tail 4K@{:#X} phys {:#016X}", tail_vaddr, tail_paddr.to_raw());
                self.map_4k_block(tail_vaddr, tail_paddr, flags)?;
                tail_vaddr += step;
                tail_paddr = Paddr::from_raw(tail_paddr.to_raw() + step);
                tail_rem -= step;
            }
        }

        Ok(())
    }

    // Helper that maps a single 4K page, splitting any blocking huge page.
    fn map_4k_block(
        &mut self,
        vaddr: usize,
        paddr: Paddr,
        flags: EntryFlags,
    ) -> Result<(), &'static str> {
        loop {
            match self.exco.try_map4k(vaddr, paddr, flags) {
                Ok(_) => return Ok(()),
                Err(_) => {
                    // A huge page is blocking — try split.
                    if let Ok(_entry) = walk_entry_mut(&mut self.exco.root, vaddr, 1, false) {
                        let base = vaddr & !(0x1f_ffff);
                        debug!("  map_4k_block: split 2M at {:#X}", base);
                        self.exco.try_split2m(base)?;
                    } else if let Ok(_entry) = walk_entry_mut(&mut self.exco.root, vaddr, 2, false) {
                        let base = vaddr & !(0x3fff_ffff);
                        debug!("  map_4k_block: split 1G at {:#X}", base);
                        self.exco.try_split1g(base)?;
                    } else {
                        return Err("map_4k_block: cannot resolve blocking page");
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------------
    //  Remap  –  change flags on an existing mapping
    // ------------------------------------------------------------------

    /// Change the flags on an existing mapping covering `[vaddr, vaddr + size)`.
    ///
    /// If the region contains huge pages they are split to 4 KiB granularity.
    pub fn remap(&mut self, vaddr: usize, size: usize, new_flags: EntryFlags) {
        self.try_remap(vaddr, size, new_flags)
            .expect("Polen::remap failed");
    }

    /// Fallible version of [`remap`](Self::remap).
    pub fn try_remap(
        &mut self,
        mut vaddr: usize,
        mut size: usize,
        new_flags: EntryFlags,
    ) -> Result<(), &'static str> {
        if vaddr & 0xfff != 0 || size == 0 {
            return Err("misaligned or zero-size remap");
        }

        debug!("Polen::remap [ {:#X} .. {:#X} ) -> flags {:?}", vaddr, vaddr + size, new_flags);

        while size > 0 {
            if let Ok(entry) = walk_entry_mut(&mut self.exco.root, vaddr, 2, false) {
                if entry.is_present() && is_huge(entry) {
                    let base = vaddr & !(0x3fff_ffff);
                    debug!("  remap: split 1G at {:#X}", base);
                    self.exco.try_split1g(base)?;
                    continue;
                }
            }
            if let Ok(entry) = walk_entry_mut(&mut self.exco.root, vaddr, 1, false) {
                if entry.is_present() && is_huge(entry) {
                    let base = vaddr & !(0x1f_ffff);
                    debug!("  remap: split 2M at {:#X}", base);
                    self.exco.try_split2m(base)?;
                    continue;
                }
            }

            let entry = walk_entry_mut(&mut self.exco.root, vaddr, 0, false)?;
            if !entry.is_present() {
                return Err("address not mapped");
            }
            let paddr = entry.address();
            *entry = Entry::new(paddr, new_flags | EntryFlags::PRESENT);
            exco::flush_tlb(vaddr);

            vaddr += 4096;
            size = size.saturating_sub(4096);
        }

        Ok(())
    }

    // ------------------------------------------------------------------
    //  Unmap  –  unmap a range, splitting huge pages at boundaries
    // ------------------------------------------------------------------

    /// Unmap `[vaddr, vaddr + size)`.
    ///
    /// Huge pages that partially overlap the range are split so that only the
    /// requested portion is unmapped.
    pub fn unmap(&mut self, vaddr: usize, size: usize) {
        self.try_unmap(vaddr, size).expect("Polen::unmap failed");
    }

    /// Fallible version of [`unmap`](Self::unmap).
    pub fn try_unmap(
        &mut self,
        mut vaddr: usize,
        mut size: usize,
    ) -> Result<(), &'static str> {
        if vaddr & 0xfff != 0 || size == 0 {
            return Err("misaligned or zero-size unmap");
        }

        info!("Polen::unmap [ {:#X} .. {:#X} ) ({} bytes)", vaddr, vaddr + size, size);

        while size > 0 {
            if let Ok(entry) = walk_entry_mut(&mut self.exco.root, vaddr, 2, false) {
                if entry.is_present() && is_huge(entry) {
                    let base = vaddr & !(0x3fff_ffff);
                    if base != vaddr || size < (1 << 30) {
                        debug!("  unmap: partial split 1G at {:#X}", base);
                        self.exco.try_split1g(base)?;
                        continue;
                    }
                }
            }
            if let Ok(entry) = walk_entry_mut(&mut self.exco.root, vaddr, 1, false) {
                if entry.is_present() && is_huge(entry) {
                    let base = vaddr & !(0x1f_ffff);
                    if base != vaddr || size < (2 << 20) {
                        debug!("  unmap: partial split 2M at {:#X}", base);
                        self.exco.try_split2m(base)?;
                        continue;
                    }
                }
            }

            self.exco.try_unmap(vaddr)?;

            vaddr += 4096;
            size = size.saturating_sub(4096);
        }

        Ok(())
    }

    // ------------------------------------------------------------------
    //  Merge  –  try to coalesce 4 KiB pages into huge pages
    // ------------------------------------------------------------------

    /// Try to merge all 4 KiB pages inside `[vaddr, vaddr + size)` into
    /// 2 MiB (and then 1 GiB) huge pages where possible.
    ///
    /// Only blocks that are **fully contained** in the region are considered.
    /// Unaligned head and tail parts remain as 4 KiB pages.
    pub fn merge_range(&mut self, start: usize, size: usize) {
        let end = start + size;
        debug!("Polen::merge_range [ {:#X} .. {:#X} )", start, end);

        // If the region is smaller than 2 MiB, no merging possible.
        if size < (2 << 20) {
            return;
        }

        // ----- 2 MiB merging -----
        let two_m = 2 << 20;
        let two_m_mask = !(two_m - 1);
        let first_2m = (start + two_m - 1) & two_m_mask;
        let last_2m = (end - 1) & two_m_mask;

        let mut vaddr = first_2m;
        while vaddr <= last_2m {
            let _ = self.exco.try_merge2m(vaddr);
            vaddr += two_m;
        }

        // ----- 1 GiB merging (only after 2M merging) -----
        if size >= (1 << 30) {
            let one_g = 1 << 30;
            let one_g_mask = !(one_g - 1);
            let first_1g = (start + one_g - 1) & one_g_mask;
            let last_1g = (end - 1) & one_g_mask;

            let mut vaddr_gb = first_1g;
            while vaddr_gb <= last_1g {
                let _ = self.exco.try_merge1g(vaddr_gb);
                vaddr_gb += one_g;
            }
        }
    }

    // ------------------------------------------------------------------
    //  Query
    // ------------------------------------------------------------------

    /// Return the physical address and flags of the mapping at `vaddr`, or
    /// `None` if unmapped.
    pub fn query(&self, vaddr: usize) -> Option<(Paddr, EntryFlags)> {
        trace!("Polen::query {:#X}", vaddr);

        if let Ok(entry) = walk_entry(&self.exco.root, vaddr, 0) {
            if entry.is_present() {
                let offset = vaddr & 0xfff;
                let paddr = Paddr::from_raw(entry.address().to_raw() + offset);
                trace!("  -> 4K page: phys {:#016X} flags {:?}", paddr.to_raw(), entry.flags());
                return Some((paddr, entry.flags()));
            }
        }

        if let Ok(entry) = walk_entry(&self.exco.root, vaddr, 1) {
            if entry.is_present() && is_huge(entry) {
                let offset = vaddr & 0x1f_ffff;
                let paddr = Paddr::from_raw(entry.address().to_raw() + offset);
                trace!("  -> 2M page: phys {:#016X} flags {:?}", paddr.to_raw(), entry.flags());
                return Some((paddr, entry.flags()));
            }
        }

        if let Ok(entry) = walk_entry(&self.exco.root, vaddr, 2) {
            if entry.is_present() && is_huge(entry) {
                let offset = vaddr & 0x3fff_ffff;
                let paddr = Paddr::from_raw(entry.address().to_raw() + offset);
                trace!("  -> 1G page: phys {:#016X} flags {:?}", paddr.to_raw(), entry.flags());
                return Some((paddr, entry.flags()));
            }
        }

        trace!("  -> not mapped");
        None
    }

    /// Report all mapped areas.  See [`Exco::report`].
    pub fn report<const N: usize>(&self) -> Vec<Area, N> {
        self.exco.report()
    }

    /// Switch the CPU to this page‑table hierarchy by writing CR3.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the current instruction stream, stack, and
    /// all data needed for the remainder of execution are mapped in the new
    /// page tables.
    #[inline(always)]
    pub unsafe fn activate(&self) {
        unsafe {
            self.exco.activate()
        }
    }
}

// ---------------------------------------------------------------------------
// Helper: split a huge page at `vaddr` aligned to the given page size, then
// retry the outer loop.
// ---------------------------------------------------------------------------

fn split_and_retry(polen: &mut Polen, vaddr: usize, ps: PageSize) -> Result<(), &'static str> {
    match ps {
        PageSize::Size1G => {
            polen.exco.try_split1g(vaddr)?;
        }
        PageSize::Size2M => {
            polen.exco.try_split2m(vaddr)?;
        }
        PageSize::Size4K => {}
    }
    Ok(())
}
