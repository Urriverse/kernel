//! ptm execution core — x86_64 4‑level paging implementation.
//!
//! This module provides the `Exco` struct which manages a page‑table hierarchy.
//! It is split into several `impl` blocks for clarity:
//!
//! 1. **Construction and destruction** — `new`, `dup`, `current`, `clean`.
//! 2. **Report** — `report` walks the entire hierarchy and collects mapped areas.
//! 3. **Mapping** — `map4k`, `map2m`, `map1g`.
//! 4. **Unmapping** — `unmap`.
//! 5. **Splitting** — `split2m`, `split1g`.
//! 6. **Merging** — `merge2m`, `merge1g`.
//! 7. **Fallible (`try_*`) variants** — each returns a `Result`.

use core::ops::{Index, IndexMut};
use heapless::Vec;

use super::*;
use crate::mem::kdm::{Paddr, Vaddr};
use crate::mem::upa;

// ============================================================================
// Address → index helpers  (4‑level paging)
// ============================================================================

#[inline]
fn pml4_i(vaddr: usize) -> usize {
    (vaddr >> 39) & 0x1ff
}

#[inline]
fn pdpt_i(vaddr: usize) -> usize {
    (vaddr >> 30) & 0x1ff
}

#[inline]
fn pd_i(vaddr: usize) -> usize {
    (vaddr >> 21) & 0x1ff
}

#[inline]
fn pt_i(vaddr: usize) -> usize {
    (vaddr >> 12) & 0x1ff
}

// ============================================================================
// Page masks
// ============================================================================

const MASK_4K: usize = 0xfff;
const MASK_2M: usize = 0x1f_ffff;
const MASK_1G: usize = 0x3fff_ffff;

// ============================================================================
// Table allocation / deallocation helpers
// ============================================================================

pub(crate) fn alloc_tab_zeroed() -> &'static mut Tab {
    let paddr = upa::alloc(1);
    let vaddr = paddr.to_virt();
    let tab: &'static mut Tab = vaddr.to_ref_mut();
    for e in tab.0.iter_mut() {
        *e = Entry::default();
    }
    tab
}

fn free_tab(paddr: Paddr) {
    upa::free(paddr);
}

pub(crate) fn tab_from_entry(entry: &Entry) -> &'static mut Tab {
    entry.address().to_virt().to_ref_mut()
}

pub(crate) fn tab_from_entry_const(entry: &Entry) -> &Tab {
    entry.address().to_virt().to_ref()
}

/// Check whether an entry describes a huge page (2 MiB or 1 GiB).
pub(super) fn is_huge(entry: &Entry) -> bool {
    entry.flags().contains(EntryFlags::HUGE_PAGE)
}

// ============================================================================
// TLB maintenance — free function so anyone can use it
// ============================================================================

/// Flush a single page from the TLB using `invlpg`.
pub(super) fn flush_tlb(vaddr: usize) {
    unsafe {
        core::arch::asm!("invlpg [{}]", in(reg) vaddr, options(nostack, preserves_flags));
    }
}

/// Reload CR3 to flush the entire TLB.
#[allow(dead_code)]
pub(super) fn flush_all(cr3: u64) {
    debug!("TLB flush-all CR3 {:#018X}", cr3);
    unsafe {
        core::arch::asm!("mov cr3, {}", in(reg) cr3, options(nostack));
    }
}

// ============================================================================
// Walking helpers — public(super) so the policy engine can use them.
// ============================================================================

/// Mutable walk — create intermediate tables if `create == true`.
///
/// `level_hint`: 0 → PT (4 KiB), 1 → PD (2 MiB), 2 → PDPT (1 GiB).
pub(super) fn walk_entry_mut<'a>(
    root: &'a mut Tab,
    vaddr: usize,
    level_hint: u8,
    create: bool,
) -> Result<&'a mut Entry, &'static str> {
    let pml4_idx = pml4_i(vaddr);
    let pdpt_idx = pdpt_i(vaddr);
    let pd_idx   = pd_i(vaddr);
    let pt_idx   = pt_i(vaddr);

    // ---- PML4
    let pml4_entry = &mut root.0[pml4_idx];
    if !pml4_entry.is_present() {
        if !create {
            return Err("PML4 entry not present");
        }
        let new_tab = alloc_tab_zeroed();
        *pml4_entry = Entry::new(
            Vaddr::from_ref(new_tab).to_phys(),
            EntryFlags::PRESENT | EntryFlags::WRITABLE,
        );
        debug!("Created PML4[{}] at {:#018X}", pml4_idx, pml4_entry.address().to_raw());
    }

    // ---- PDPT
    let pdpt = tab_from_entry(pml4_entry);
    let pdpt_entry = &mut pdpt.0[pdpt_idx];

    if level_hint == 2 {
        return Ok(pdpt_entry);
    }

    if !pdpt_entry.is_present() {
        if !create {
            return Err("PDPT entry not present");
        }
        let new_tab = alloc_tab_zeroed();
        *pdpt_entry = Entry::new(
            Vaddr::from_ref(new_tab).to_phys(),
            EntryFlags::PRESENT | EntryFlags::WRITABLE,
        );
        debug!("Created PDPT[{}] for vaddr {:#X}", pdpt_idx, vaddr);
    }
    if is_huge(pdpt_entry) {
        return Err("1 GiB huge page encountered - split first");
    }

    // ---- PD
    let pd = tab_from_entry(pdpt_entry);
    let pd_entry = &mut pd.0[pd_idx];

    if level_hint == 1 {
        trace!("walk_mut hit PD entry for 2M page");
        return Ok(pd_entry);
    }

    if !pd_entry.is_present() {
        if !create {
            return Err("PD entry not present");
        }
        let new_tab = alloc_tab_zeroed();
        *pd_entry = Entry::new(
            Vaddr::from_ref(new_tab).to_phys(),
            EntryFlags::PRESENT | EntryFlags::WRITABLE,
        );
        debug!("Created PD[{}] for vaddr {:#X}", pd_idx, vaddr);
    }
    if is_huge(pd_entry) {
        return Err("2 MiB huge page encountered - split first");
    }

    // ---- PT
    let pt = tab_from_entry(pd_entry);
    let pt_entry = &mut pt.0[pt_idx];

    if !pt_entry.is_present() && !create {
        return Err("PT entry not present");
    }

    Ok(pt_entry)
}

/// Immutable walk — never creates tables, only reads.
pub(super) fn walk_entry<'a>(
    root: &'a Tab,
    vaddr: usize,
    level_hint: u8,
) -> Result<&'a Entry, &'static str> {
    let pml4_idx = pml4_i(vaddr);
    let pdpt_idx = pdpt_i(vaddr);
    let pd_idx   = pd_i(vaddr);
    let pt_idx   = pt_i(vaddr);

    // ---- PML4
    let pml4_entry = &root.0[pml4_idx];
    if !pml4_entry.is_present() {
        return Err("PML4 entry not present");
    }
    if is_huge(pml4_entry) {
        return Err("PML4 huge - unsupported");
    }

    // ---- PDPT
    let pdpt = tab_from_entry_const(pml4_entry);
    let pdpt_entry = &pdpt.0[pdpt_idx];
    if !pdpt_entry.is_present() {
        return Err("PDPT entry not present");
    }
    if level_hint == 2 {
        return Ok(pdpt_entry);
    }
    if is_huge(pdpt_entry) {
        return Err("1 GiB huge");
    }

    // ---- PD
    let pd = tab_from_entry_const(pdpt_entry);
    let pd_entry = &pd.0[pd_idx];
    if !pd_entry.is_present() {
        return Err("PD entry not present");
    }
    if level_hint == 1 {
        return Ok(pd_entry);
    }
    if is_huge(pd_entry) {
        return Err("2 MiB huge");
    }

    // ---- PT
    let pt = tab_from_entry_const(pd_entry);
    let pt_entry = &pt.0[pt_idx];
    if !pt_entry.is_present() {
        return Err("PT entry not present");
    }
    Ok(pt_entry)
}

// ============================================================================
// Tab definition
// ============================================================================

#[repr(align(4096))]
pub struct Tab(pub [Entry; 512]);

impl Index<usize> for Tab {
    type Output = Entry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Tab {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

// ============================================================================
// Area  – a single contiguous mapped region with uniform flags
// ============================================================================

#[derive(Clone, Copy)]
pub struct Area {
    pub start: usize,
    pub count: usize,
    pub flags: EntryFlags,
}

impl core::fmt::Display for Area
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("Area at {:X} of {} KiB: {:?}", self.start, (self.count + 1023) >> 10, self.flags))
    }
}

// ============================================================================
// Exco – the page‑table executor
// ============================================================================

pub struct Exco {
    pub cr3: u64,
    pub root: &'static mut Tab,
    pub owned: bool,
}

// --------------------------------------------------------------------
// 1. Construction and destruction
// --------------------------------------------------------------------

impl Exco {
    pub fn from_root(root: &'static mut Tab, cr3: u64, owned: bool) -> Self {
        Exco { cr3, root, owned }
    }

    pub fn new() -> Self {
        let root = alloc_tab_zeroed();
        let cr3 = Vaddr::from_ref(&*root).to_phys().to_raw() as u64;
        info!("Exco::new   CR3 {:#018X} owned", cr3);
        Exco { cr3, root, owned: true }
    }

    pub fn dup(&self) -> Self {
        let (root, cr3) = Self::dup_table(self.root);
        info!("Exco::dup   CR3 {:#018X} (src CR3 {:#018X})", cr3 as u64, self.cr3);
        Exco { cr3: cr3 as u64, root, owned: true }
    }

    fn dup_table(table: &Tab) -> (&'static mut Tab, usize) {
        trace!("Exco::dup_table start");
        let new = alloc_tab_zeroed();
        for (i, entry) in table.0.iter().enumerate() {
            if entry.is_present() && !is_huge(entry) {
                let child = tab_from_entry(entry);
                let (new_child, child_paddr) = Self::dup_table(child);
                new.0[i] = Entry::new(
                    Paddr::from_raw(child_paddr),
                    entry.flags(),
                );
            } else if entry.is_present() {
                new.0[i] = *entry;
            }
        }
        let paddr = Vaddr::from_ref(&*new).to_phys().to_raw();
        trace!("Exco::dup_table -> {:#X}", paddr);
        (new, paddr)
    }

    pub fn current() -> Self {
        let cr3_raw: u64;
        unsafe {
            core::arch::asm!("mov {}, cr3", out(reg) cr3_raw);
        }
        let phys = (cr3_raw & 0x000f_ffff_ffff_f000) as usize;
        let vaddr = Paddr::from_raw(phys).to_virt();
        let root: &'static mut Tab = vaddr.to_ref_mut();
        info!("Exco::current CR3 {:#018X} (phys {:#X})", cr3_raw, phys);
        Exco { cr3: cr3_raw, root, owned: false }
    }

    pub fn clean(&mut self) {
        let cr3 = self.cr3;
        debug!("Exco::clean started for CR3 {:#018X}", cr3);
        if self.owned {
            Self::free_table(self.root, true);
        }
        for e in self.root.0.iter_mut() {
            *e = Entry::default();
        }
        self.cr3 = Vaddr::from_ref(&*self.root).to_phys().to_raw() as u64;
        debug!("Exco::clean finished (new CR3 {:#018X})", self.cr3);
    }

    fn free_table(table: &mut Tab, free_self: bool) {
        let paddr = Vaddr::from_ref(&*table).to_phys();
        for entry in table.0.iter_mut() {
            if entry.is_present() && !is_huge(entry) {
                Self::free_table(tab_from_entry(entry), true);
            }
            *entry = Entry::default();
        }
        if free_self {
            free_tab(paddr);
        }
    }
}

// --------------------------------------------------------------------
// 2. Report
// --------------------------------------------------------------------

impl Exco {
    pub fn report<const N: usize>(&self) -> Vec<Area, N> {
        debug!("Exco::report<{}> start", N);
        let mut areas: Vec<Area, N> = Vec::new();
        let mut total_pages: usize = 0;
        for pml4_idx in 0..512 {
            let pml4_entry = &self.root.0[pml4_idx];
            if !pml4_entry.is_present() {
                continue;
            }
            let base_pml4 = (pml4_idx as usize) << 39;
            if is_huge(pml4_entry) {
                continue;
            }
            let pdpt = tab_from_entry(pml4_entry);
            for pdpt_idx in 0..512 {
                let pdpt_entry = &pdpt.0[pdpt_idx];
                if !pdpt_entry.is_present() {
                    continue;
                }
                let base_pdpt = base_pml4 | (pdpt_idx << 30);
                if is_huge(pdpt_entry) {
                    try_push_area(&mut areas, base_pdpt, 1 << 20, pdpt_entry.flags());
                    total_pages += 1 << 20;
                    continue;
                }
                let pd = tab_from_entry(pdpt_entry);
                for pd_idx in 0..512 {
                    let pd_entry = &pd.0[pd_idx];
                    if !pd_entry.is_present() {
                        continue;
                    }
                    let base_pd = base_pdpt | (pd_idx << 21);
                    if is_huge(pd_entry) {
                        try_push_area(&mut areas, base_pd, 512, pd_entry.flags());
                        total_pages += 512;
                        continue;
                    }
                    let pt = tab_from_entry(pd_entry);
                    for pt_idx in 0..512 {
                        let pt_entry = &pt.0[pt_idx];
                        if !pt_entry.is_present() {
                            continue;
                        }
                        let base_pt = base_pd | (pt_idx << 12);
                        try_push_area(&mut areas, base_pt, 1, pt_entry.flags());
                        total_pages += 1;
                    }
                }
            }
        }
        debug!("Exco::report done: {} areas, {} 4K pages mapped", areas.len(), total_pages);
        areas
    }
}

fn try_push_area<const N: usize>(
    areas: &mut Vec<Area, N>,
    vaddr: usize,
    count: usize,
    flags: EntryFlags,
) {
    if let Some(last) = areas.last_mut() {
        if last.flags == flags && last.start + last.count * 4096 == vaddr {
            last.count += count;
            return;
        }
    }
    let _ = areas.push(Area { start: vaddr, count, flags });
}

// --------------------------------------------------------------------
// 3. Mapping
// --------------------------------------------------------------------

impl Exco {
    pub fn map4k(&mut self, vaddr: usize, paddr: Paddr, flags: EntryFlags) {
        self.try_map4k(vaddr, paddr, flags).expect("map4k failed");
    }

    pub fn map2m(&mut self, vaddr: usize, paddr: Paddr, flags: EntryFlags) {
        self.try_map2m(vaddr, paddr, flags).expect("map2m failed");
    }

    pub fn map1g(&mut self, vaddr: usize, paddr: Paddr, flags: EntryFlags) {
        self.try_map1g(vaddr, paddr, flags).expect("map1g failed");
    }

    pub fn try_map4k(
        &mut self,
        vaddr: usize,
        paddr: Paddr,
        flags: EntryFlags,
    ) -> Result<(), &'static str> {
        if vaddr & MASK_4K != 0 {
            return Err("vaddr not 4 KiB-aligned");
        }
        let entry = walk_entry_mut(&mut self.root, vaddr, 0, true)?;
        *entry = Entry::new(paddr, flags | EntryFlags::PRESENT);
        flush_tlb(vaddr);
        Ok(())
    }

    pub fn try_map2m(
        &mut self,
        vaddr: usize,
        paddr: Paddr,
        flags: EntryFlags,
    ) -> Result<(), &'static str> {
        if vaddr & MASK_2M != 0 {
            return Err("vaddr not 2 MiB-aligned");
        }
        if paddr.to_raw() & MASK_2M != 0 {
            return Err("paddr not 2 MiB-aligned");
        }
        debug!("map2m vaddr {:#X} -> phys {:#016X} flags {:?}", vaddr, paddr.to_raw(), flags);
        let entry = walk_entry_mut(&mut self.root, vaddr, 1, true)?;
        *entry = Entry::new(paddr, flags | EntryFlags::PRESENT | EntryFlags::HUGE_PAGE);
        flush_tlb(vaddr);
        Ok(())
    }

    pub fn try_map1g(
        &mut self,
        vaddr: usize,
        paddr: Paddr,
        flags: EntryFlags,
    ) -> Result<(), &'static str> {
        if vaddr & MASK_1G != 0 {
            return Err("vaddr not 1 GiB-aligned");
        }
        if paddr.to_raw() & MASK_1G != 0 {
            return Err("paddr not 1 GiB-aligned");
        }
        debug!("map1g vaddr {:#X} -> phys {:#016X} flags {:?}", vaddr, paddr.to_raw(), flags);
        let entry = walk_entry_mut(&mut self.root, vaddr, 2, true)?;
        *entry = Entry::new(paddr, flags | EntryFlags::PRESENT | EntryFlags::HUGE_PAGE);
        flush_tlb(vaddr);
        Ok(())
    }
}

// --------------------------------------------------------------------
// 4. Unmapping
// --------------------------------------------------------------------

impl Exco {
    pub fn unmap(&mut self, vaddr: usize) {
        self.try_unmap(vaddr).expect("unmap failed");
    }

    pub fn try_unmap(&mut self, vaddr: usize) -> Result<(), &'static str> {
        if let Ok(entry) = walk_entry_mut(&mut self.root, vaddr, 0, false) {
            if entry.is_present() {
                *entry = Entry::default();
                flush_tlb(vaddr);
                return Ok(());
            }
        }

        if let Ok(entry) = walk_entry_mut(&mut self.root, vaddr, 1, false) {
            if entry.is_present() && is_huge(entry) {
                debug!("unmap 2M huge page at {:#X}", vaddr & !MASK_2M);
                *entry = Entry::default();
                flush_tlb(vaddr & !MASK_2M);
                return Ok(());
            }
        }

        if let Ok(entry) = walk_entry_mut(&mut self.root, vaddr, 2, false) {
            if entry.is_present() && is_huge(entry) {
                debug!("unmap 1G huge page at {:#X}", vaddr & !MASK_1G);
                *entry = Entry::default();
                flush_tlb(vaddr & !MASK_1G);
                return Ok(());
            }
        }

        Err("address not mapped")
    }
}

// --------------------------------------------------------------------
// 5. Splitting huge pages
// --------------------------------------------------------------------

impl Exco {
    pub fn split2m(&mut self, vaddr: usize) {
        self.try_split2m(vaddr).expect("split2m failed");
    }

    pub fn split1g(&mut self, vaddr: usize) {
        self.try_split1g(vaddr).expect("split1g failed");
    }

    pub fn try_split2m(&mut self, vaddr: usize) -> Result<(), &'static str> {
        if vaddr & MASK_2M != 0 {
            return Err("vaddr not 2 MiB-aligned");
        }

        let entry = walk_entry_mut(&mut self.root, vaddr, 1, false)?;
        if !entry.is_present() || !is_huge(entry) {
            return Err("not a 2 MiB huge page");
        }

        let base_paddr = entry.address();
        let flags = entry.flags() - EntryFlags::HUGE_PAGE;

        debug!("split2m {:#X} (phys {:#016X}) -> 512x4K", vaddr, base_paddr.to_raw());

        let pt = alloc_tab_zeroed();
        let pt_paddr = Vaddr::from_ref(&*pt).to_phys();

        for i in 0..512 {
            let page_paddr = Paddr::from_raw(base_paddr.to_raw() + (i << 12));
            pt.0[i] = Entry::new(page_paddr, flags);
        }

        *entry = Entry::new(pt_paddr, EntryFlags::PRESENT | EntryFlags::WRITABLE);
        flush_tlb(vaddr);
        Ok(())
    }

    pub fn try_split1g(&mut self, vaddr: usize) -> Result<(), &'static str> {
        if vaddr & MASK_1G != 0 {
            return Err("vaddr not 1 GiB-aligned");
        }

        let entry = walk_entry_mut(&mut self.root, vaddr, 2, false)?;
        if !entry.is_present() || !is_huge(entry) {
            return Err("not a 1 GiB huge page");
        }

        let base_paddr = entry.address();
        let flags = entry.flags() - EntryFlags::HUGE_PAGE;

        debug!("split1g {:#X} (phys {:#016X}) -> 512x2M", vaddr, base_paddr.to_raw());

        let pd = alloc_tab_zeroed();
        let pd_paddr = Vaddr::from_ref(&*pd).to_phys();

        for i in 0..512 {
            let page_paddr = Paddr::from_raw(base_paddr.to_raw() + (i << 21));
            pd.0[i] = Entry::new(page_paddr, flags | EntryFlags::HUGE_PAGE);
        }

        *entry = Entry::new(pd_paddr, EntryFlags::PRESENT | EntryFlags::WRITABLE);
        flush_tlb(vaddr);
        Ok(())
    }
}

// --------------------------------------------------------------------
// 6. Merging into huge pages
// --------------------------------------------------------------------

impl Exco {
    pub fn merge2m(&mut self, vaddr: usize) {
        self.try_merge2m(vaddr).expect("merge2m failed");
    }

    pub fn merge1g(&mut self, vaddr: usize) {
        self.try_merge1g(vaddr).expect("merge1g failed");
    }

    pub fn try_merge2m(&mut self, vaddr: usize) -> Result<(), &'static str> {
        if vaddr & MASK_2M != 0 {
            return Err("vaddr not 2 MiB-aligned");
        }

        let pd_entry = walk_entry_mut(&mut self.root, vaddr, 1, false)?;
        if !pd_entry.is_present() || is_huge(pd_entry) {
            return Err("PD entry is not a table pointer");
        }

        let pt = tab_from_entry(pd_entry);
        try_coalesce_into_2m(pt, vaddr, pd_entry)
    }

    pub fn try_merge1g(&mut self, vaddr: usize) -> Result<(), &'static str> {
        if vaddr & MASK_1G != 0 {
            return Err("vaddr not 1 GiB-aligned");
        }

        let pdpt_entry = walk_entry_mut(&mut self.root, vaddr, 2, false)?;
        if !pdpt_entry.is_present() || is_huge(pdpt_entry) {
            return Err("PDPT entry is not a table pointer");
        }

        let pd = tab_from_entry(pdpt_entry);
        try_coalesce_into_1g(pd, vaddr, pdpt_entry)
    }
}

fn try_coalesce_into_2m(
    pt: &mut Tab,
    vaddr: usize,
    pd_entry: &mut Entry,
) -> Result<(), &'static str> {
    let first_flags = pt.0[0].flags() - EntryFlags::ACCESSED - EntryFlags::DIRTY;
    let first_paddr = pt.0[0].address();

    if !pt.0[0].is_present() {
        return Err("first PT entry not present");
    }

    for i in 1..512 {
        if !pt.0[i].is_present() {
            return Err("PT entry not present");
        }
        let flags = pt.0[i].flags() - EntryFlags::ACCESSED - EntryFlags::DIRTY;
        if flags != first_flags {
            return Err("inconsistent flags across PT entries");
        }
        let expected_paddr = Paddr::from_raw(first_paddr.to_raw() + (i << 12));
        if pt.0[i].address().to_raw() != expected_paddr.to_raw() {
            return Err("non-contiguous physical addresses");
        }
    }

    let huge_flags = first_flags | EntryFlags::HUGE_PAGE | EntryFlags::PRESENT;
    *pd_entry = Entry::new(first_paddr, huge_flags);

    let pt_paddr = Vaddr::from_ref(&*pt).to_phys();
    free_tab(pt_paddr);

    info!("Merged 2 MiB at {:#X} (phys {:#016X})", vaddr, first_paddr.to_raw());
    Ok(())
}

fn try_coalesce_into_1g(
    pd: &mut Tab,
    vaddr: usize,
    pdpt_entry: &mut Entry,
) -> Result<(), &'static str> {
    let first_flags = pd.0[0].flags() - EntryFlags::ACCESSED - EntryFlags::DIRTY;
    let first_paddr = pd.0[0].address();

    if !pd.0[0].is_present() || !is_huge(&pd.0[0]) {
        return Err("first PD entry not a 2 MiB huge page");
    }

    for i in 1..512 {
        if !pd.0[i].is_present() || !is_huge(&pd.0[i]) {
            return Err("PD entry not a 2 MiB huge page");
        }
        let flags = pd.0[i].flags() - EntryFlags::ACCESSED - EntryFlags::DIRTY;
        if flags != first_flags {
            return Err("inconsistent flags across PD entries");
        }
        let expected_paddr = Paddr::from_raw(first_paddr.to_raw() + (i << 21));
        if pd.0[i].address().to_raw() != expected_paddr.to_raw() {
            return Err("non-contiguous physical addresses");
        }
    }

    let huge_flags = first_flags | EntryFlags::HUGE_PAGE | EntryFlags::PRESENT;
    *pdpt_entry = Entry::new(first_paddr, huge_flags);

    let pd_paddr = Vaddr::from_ref(&*pd).to_phys();
    free_tab(pd_paddr);

    info!("Merged 1 GiB at {:#X} (phys {:#016X})", vaddr, first_paddr.to_raw());
    Ok(())
}

// --------------------------------------------------------------------
// 7. Fallible (`try_*`) variants
// --------------------------------------------------------------------

impl Exco {
    pub fn try_new() -> Result<Self, &'static str> {
        Ok(Self::new())
    }

    pub fn try_dup(&self) -> Result<Self, &'static str> {
        Ok(self.dup())
    }

    pub fn try_current() -> Result<Self, &'static str> {
        Ok(Self::current())
    }

    pub fn try_clean(&mut self) -> Result<(), &'static str> {
        self.clean();
        Ok(())
    }

    pub fn try_report<const N: usize>(&self) -> Result<Vec<Area, N>, &'static str> {
        let areas = self.report();
        if areas.is_full() {
            error!("Exco::try_report vec of size {} exhausted", N);
            Err("report: Vec capacity exhausted - some areas may be missing")
        } else {
            Ok(areas)
        }
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
        trace!("Exco::activate -> CR3 {:#018X}", self.cr3);
        unsafe {
            core::arch::asm!("mov cr3, {}", in(reg) self.cr3, options(nostack, preserves_flags));
        }
    }
}
