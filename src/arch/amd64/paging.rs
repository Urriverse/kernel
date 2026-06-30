//! # x86_64 Paging (4‑Level Page Tables)
//!
//! This module implements the kernel's 4‑level paging infrastructure for the x86_64
//! architecture. It provides low‑level page table manipulation, including mapping,
//! unmapping, and the use of huge pages (2 MiB and 1 GiB) for performance.
//!
//! ## Overview
//!
//! The x86_64 architecture uses 4 levels of page tables:
//! - **PML4** (Page Map Level 4) – top level, 512 entries, each pointing to a PDPT.
//! - **PDPT** (Page Directory Pointer Table) – second level, 512 entries, each
//!   pointing to a PD (or a 1 GiB huge page).
//! - **PD** (Page Directory) – third level, 512 entries, each pointing to a PT
//!   (or a 2 MiB huge page).
//! - **PT** (Page Table) – fourth level, 512 entries, each pointing to a 4 KiB page.
//!
//! Each entry is a 64‑bit value that contains the physical address of the next
//! level table (or the page frame) and various flags (present, writable, user,
//! no‑execute, etc.).
//!
//! ## Key Abstractions
//!
//! - **`Entry`**: A 64‑bit page table entry with methods to read/write address and flags.
//! - **`Tab`**: A page table (array of 512 `Entry`s), aligned to 4 KiB.
//! - **`Exco`**: An execution context that holds a CR3 value and a reference to
//!   the root PML4 table. It provides methods for mapping, unmapping, splitting,
//!   merging, and reporting the page table structure.
//! - **`Area`**: A contiguous range of virtual addresses with the same flags,
//!   used for reporting mapped regions.
//!
//! ## Huge Pages
//!
//! The kernel supports 2 MiB and 1 GiB huge pages to reduce TLB pressure and
//! improve performance. Huge pages are used when:
//! - The virtual address is aligned to the huge page size.
//! - The physical address is aligned to the huge page size.
//! - The memory region is contiguous (checked via PMR).
//!
//! The `Exco` provides `map2m`/`map1g` and `try_merge2m`/`try_merge1g` methods
//! to manage huge pages. The `split2m`/`split1g` methods break huge pages into
//! smaller pages when needed (e.g., for partial unmapping or changing permissions).
//!
//! ## Allocation and Freeing of Page Tables
//!
//! Page tables are allocated from the physical memory allocator (`upa`) and are
//! zero‑initialised. The `alloc_tab_zeroed()` function allocates a 4 KiB page
//! and returns a mutable reference to it as a `Tab`. The `free_tab()` function
//! frees a page table back to the allocator.
//!
//! ## Walking the Page Tables
//!
//! The `walk_entry` and `walk_entry_mut` functions traverse the page tables for
//! a given virtual address, returning the entry at the specified level. The
//! `walk_entry_mut` variant can optionally create missing intermediate tables.
//!
//! ## Execution Context (`Exco`)
//!
//! An `Exco` represents a page table context (an address space). It contains:
//! - `cr3`: The physical address of the root PML4 (loaded into CR3).
//! - `root`: A mutable reference to the PML4 table.
//! - `owned`: Whether this context owns the page tables (for cleanup).
//!
//! The `Exco` provides safe methods to manipulate the page tables and to
//! activate the context (load CR3). It also supports duplication (`dup()`)
//! for creating a new address space (copy‑on‑write is handled separately).
//!
//! ## Reporting and Debugging
//!
//! The `report<const N>` method traverses the page tables and returns a
//! `Vec<Area, N>` of contiguous mapped regions, grouped by flags. This is
//! useful for debugging and for the `Vmm` module to track memory usage.
//!
//! ## Safety
//!
//! - Most functions in this module are `unsafe` because they manipulate
//!   page tables, which directly affect memory access and CPU behaviour.
//! - The `Exco::activate()` function uses inline assembly to load CR3.
//! - The `walk_entry_mut` function uses raw pointers to modify page tables.
//! - The `alloc_tab_zeroed` and `free_tab` functions interact with the
//!   physical memory allocator.
//! - The `transmute` in `try_merge` is safe because the functions are
//!   called with the correct arguments.

use crate::mem::kdm::Paddr;

// ============================================================================
// PAGE TABLE ENTRY FLAGS
// ============================================================================

bitflags! {
    /// Flags for page table entries.
    ///
    /// These bits control access permissions, caching, and other attributes
    /// of the page or page table.
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct EntryFlags: u64 {
        /// The page is present in memory.
        const PRESENT         = 1 <<  0;
        /// The page is writable (for kernel mode, or user if `USER_ACCESSIBLE`).
        const WRITABLE        = 1 <<  1;
        /// The page is accessible from user mode (CPL 3).
        const USER_ACCESSIBLE = 1 <<  2;
        /// Write‑through caching (vs. write‑back).
        const WRITE_THROUGH   = 1 <<  3;
        /// Cache disabled for this page.
        const CACHE_DISABLE   = 1 <<  4;
        /// The page has been accessed (set by hardware).
        const ACCESSED        = 1 <<  5;
        /// The page has been written to (set by hardware).
        const DIRTY           = 1 <<  6;
        /// The entry points to a huge page (2 MiB or 1 GiB).
        const HUGE_PAGE       = 1 <<  7;
        /// The page is global (not flushed on CR3 switch).
        const GLOBAL          = 1 <<  8;
        /// Execute disable (NX bit) – the page cannot be executed.
        const NO_EXECUTE      = 1 << 63;

        // Kernel‑specific software‑managed flags (stored in available bits).
        /// Copy‑on‑write flag (used by the scheduler).
        const COPY_ON_WRITE   = 1 << 52;
        /// File‑mapped flag (for mmap).
        const FILE_MAPPED     = 1 << 53;
        /// Swapped flag (page is swapped out).
        const SWAPPED         = 1 << 54;
    }
}

// ============================================================================
// PAGE TABLE ENTRY
// ============================================================================

/// A 64‑bit page table entry.
///
/// This struct wraps a `u64` and provides methods to manipulate the address
/// and flags of the entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Entry(u64);

impl Entry {
    /// Mask for the physical address bits (bits 12‑51).
    pub const ADDRESS_MASK: u64 = 0x000f_ffff_ffff_f000;

    /// Mask for the available bits (bits 52‑62).
    pub const AVAILABLE_MASK: u64 = 0x7ff0_0000_0000_0e00;

    /// Shift for the physical address (12 bits).
    pub const ADDRESS_SHIFT: u32 = 12;

    /// Creates a new entry with the given physical address and flags.
    #[inline]
    pub const fn new(physical_address: Paddr, flags: EntryFlags) -> Self {
        let addr_part = physical_address.to_raw() as u64 & Self::ADDRESS_MASK;
        Self(addr_part | flags.bits())
    }

    /// Returns the raw `u64` value of the entry.
    #[inline]
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    /// Returns the flags of the entry.
    #[inline]
    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.0)
    }

    /// Sets the flags of the entry, preserving the address bits.
    #[inline]
    pub fn set_flags(&mut self, flags: EntryFlags) {
        self.0 = (self.0 & !EntryFlags::all().bits()) | flags.bits();
    }

    /// Returns the physical address stored in the entry.
    #[inline]
    pub fn address(&self) -> Paddr {
        Paddr::from_raw((self.0 & Self::ADDRESS_MASK) as usize)
    }

    /// Sets the physical address of the entry, preserving the flags.
    #[inline]
    pub fn set_address(&mut self, paddr: Paddr) {
        let addr = paddr.to_raw() as u64;
        self.0 = (self.0 & !Self::ADDRESS_MASK) | (addr & Self::ADDRESS_MASK);
    }

    /// Returns `true` if the entry is present.
    #[inline]
    pub fn is_present(&self) -> bool {
        self.flags().contains(EntryFlags::PRESENT)
    }

    /// Returns `true` if the entry is writable.
    #[inline]
    pub fn is_writable(&self) -> bool {
        self.flags().contains(EntryFlags::WRITABLE)
    }

    /// Returns `true` if the page is executable (NX bit is not set).
    #[inline]
    pub fn is_executable(&self) -> bool {
        !self.flags().contains(EntryFlags::NO_EXECUTE)
    }
}

impl From<u64> for Entry {
    #[inline]
    fn from(raw: u64) -> Self {
        Entry(raw)
    }
}

impl From<Entry> for u64 {
    #[inline]
    fn from(entry: Entry) -> Self {
        entry.0
    }
}

impl Default for Entry {
    #[inline]
    fn default() -> Self {
        Entry(0)
    }
}

// ============================================================================
// PAGE TABLE STRUCTURE
// ============================================================================

use core::hint::{likely, unlikely};
use core::ops::{Index, IndexMut};

use crate::mem::kdm::Vaddr;
use crate::mem::upa;

// ----------------------------------------------------------------------------
// HELPERS: INDEX FUNCTIONS
// ----------------------------------------------------------------------------

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

// ----------------------------------------------------------------------------
// MASK CONSTANTS
// ----------------------------------------------------------------------------

const MASK_4K: usize = 0xfff;
const MASK_2M: usize = 0x1f_ffff;
const MASK_1G: usize = 0x3fff_ffff;

// ----------------------------------------------------------------------------
// PAGE TABLE ALLOCATION / DEALLOCATION
// ----------------------------------------------------------------------------

/// Allocates a zero‑initialised page table (4 KiB).
///
/// This function allocates a physical page from the UPA, maps it via the HHDM,
/// and returns a mutable reference to it as a `Tab`. The table is zeroed.
///
/// # Panics
/// Panics if the allocation fails.
pub fn alloc_tab_zeroed() -> &'static mut Tab {
    let paddr = upa::alloc(1);
    let vaddr = paddr.to_virt();
    let tab: &'static mut Tab = vaddr.to_ref_mut();
    for e in tab.0.iter_mut() {
        *e = Entry::default();
    }
    tab
}

/// Frees a page table by physical address.
///
/// # Arguments
/// * `paddr` – The physical address of the table to free.
fn free_tab(paddr: Paddr) {
    upa::free(paddr);
}

/// Returns a mutable reference to the table pointed to by an entry.
///
/// # Safety
/// The entry must point to a valid, mapped table.
pub fn tab_from_entry(entry: &Entry) -> &'static mut Tab {
    entry.address().to_virt().to_ref_mut()
}

/// Returns an immutable reference to the table pointed to by an entry.
///
/// # Safety
/// The entry must point to a valid, mapped table.
pub fn tab_from_entry_const(entry: &Entry) -> &Tab {
    entry.address().to_virt().to_ref()
}

/// Returns `true` if the entry is a huge page (2 MiB or 1 GiB).
#[inline]
pub fn is_huge(entry: &Entry) -> bool {
    entry.flags().contains(EntryFlags::HUGE_PAGE)
}

// ----------------------------------------------------------------------------
// TLB FUNCTIONS
// ----------------------------------------------------------------------------

/// Invalidates a single TLB entry for the given virtual address.
#[inline]
pub fn flush_tlb(vaddr: usize) {
    unsafe {
        core::arch::asm!("invlpg [{}]", in(reg) vaddr, options(nostack, preserves_flags));
    }
}

/// Invalidates all TLB entries (by reloading CR3).
pub fn flush_all(cr3: u64) {
    debug!("TLB flush-all CR3 {:#018X}", cr3);
    unsafe {
        core::arch::asm!("mov cr3, {}", in(reg) cr3, options(nostack));
    }
}

// ============================================================================
// PAGE TABLE WALKING
// ============================================================================

/// Walks the page tables and returns a mutable reference to the entry
/// at the given virtual address and level.
///
/// # Arguments
/// * `root` – The root PML4 table.
/// * `vaddr` – The virtual address to walk.
/// * `level_hint` – The level to return: 0 = PT entry, 1 = PD entry, 2 = PDPT entry.
/// * `create` – Whether to create missing intermediate tables.
///
/// # Returns
/// `Ok(&mut Entry)` if the entry exists (or was created), `Err` otherwise.
///
/// # Errors
/// - `PML4 entry not present` (if `create` is false and the PML4 entry is missing).
/// - `PDPT entry not present` (if `create` is false and the PDPT entry is missing).
/// - `PD entry not present` (if `create` is false and the PD entry is missing).
/// - `1 GiB huge page encountered` (if a 1 GiB page blocks the walk).
/// - `2 MiB huge page encountered` (if a 2 MiB page blocks the walk).
pub fn walk_entry_mut(
    root: &mut Tab,
    vaddr: usize,
    level_hint: u8,
    create: bool,
) -> Result<&mut Entry, &'static str> {
    let pml4_idx = pml4_i(vaddr);
    let pdpt_idx = pdpt_i(vaddr);
    let pd_idx   = pd_i(vaddr);
    let pt_idx   = pt_i(vaddr);

    let pml4_entry = &mut root.0[pml4_idx];
    if !pml4_entry.is_present() {
        if unlikely(!create) {
            return Err("PML4 entry not present");
        }
        let new_tab = alloc_tab_zeroed();
        *pml4_entry = Entry::new(
            Vaddr::from_ref(new_tab).to_phys(),
            EntryFlags::PRESENT | EntryFlags::WRITABLE,
        );
        debug!("Created PML4[{}] at {:#018X}", pml4_idx, pml4_entry.address().to_raw());
    }

    let pdpt = tab_from_entry(pml4_entry);
    let pdpt_entry = &mut pdpt.0[pdpt_idx];

    if level_hint == 2 {
        return Ok(pdpt_entry);
    }

    if !pdpt_entry.is_present() {
        if unlikely(!create) {
            return Err("PDPT entry not present");
        }
        let new_tab = alloc_tab_zeroed();
        *pdpt_entry = Entry::new(
            Vaddr::from_ref(new_tab).to_phys(),
            EntryFlags::PRESENT | EntryFlags::WRITABLE,
        );
        // debug!("Created PDPT[{}] for vaddr {:#X}", pdpt_idx, vaddr);
    }
    if unlikely(is_huge(pdpt_entry)) {
        return Err("1 GiB huge page encountered - split first");
    }

    let pd = tab_from_entry(pdpt_entry);
    let pd_entry = &mut pd.0[pd_idx];

    if level_hint == 1 {
        return Ok(pd_entry);
    }

    if !pd_entry.is_present() {
        if unlikely(!create) {
            return Err("PD entry not present");
        }
        let new_tab = alloc_tab_zeroed();
        *pd_entry = Entry::new(
            Vaddr::from_ref(new_tab).to_phys(),
            EntryFlags::PRESENT | EntryFlags::WRITABLE,
        );
    }
    if unlikely(is_huge(pd_entry)) {
        return Err("2 MiB huge page encountered - split first");
    }

    let pt = tab_from_entry(pd_entry);
    let pt_entry = &mut pt.0[pt_idx];

    if unlikely(!pt_entry.is_present() && !create) {
        return Err("PT entry not present");
    }

    Ok(pt_entry)
}

/// Walks the page tables and returns an immutable reference to the entry
/// at the given virtual address and level.
///
/// # Arguments
/// * `root` – The root PML4 table.
/// * `vaddr` – The virtual address to walk.
/// * `level_hint` – The level to return: 0 = PT entry, 1 = PD entry, 2 = PDPT entry.
///
/// # Returns
/// `Ok(&Entry)` if the entry exists, `Err` otherwise.
pub fn walk_entry(
    root: &Tab,
    vaddr: usize,
    level_hint: u8,
) -> Result<&Entry, &'static str> {
    let pml4_idx = pml4_i(vaddr);
    let pdpt_idx = pdpt_i(vaddr);
    let pd_idx   = pd_i(vaddr);
    let pt_idx   = pt_i(vaddr);

    let pml4_entry = &root.0[pml4_idx];
    if unlikely(!pml4_entry.is_present()) {
        return Err("PML4 entry not present");
    }
    if unlikely(is_huge(pml4_entry)) {
        return Err("PML4 huge - unsupported");
    }

    let pdpt = tab_from_entry_const(pml4_entry);
    let pdpt_entry = &pdpt.0[pdpt_idx];
    if unlikely(!pdpt_entry.is_present()) {
        return Err("PDPT entry not present");
    }
    if level_hint == 2 {
        return Ok(pdpt_entry);
    }
    if is_huge(pdpt_entry) {
        return Err("1 GiB huge");
    }

    let pd = tab_from_entry_const(pdpt_entry);
    let pd_entry = &pd.0[pd_idx];
    if unlikely(!pd_entry.is_present()) {
        return Err("PD entry not present");
    }
    if level_hint == 1 {
        return Ok(pd_entry);
    }
    if is_huge(pd_entry) {
        return Err("2 MiB huge");
    }

    let pt = tab_from_entry_const(pd_entry);
    let pt_entry = &pt.0[pt_idx];
    if unlikely(!pt_entry.is_present()) {
        return Err("PT entry not present");
    }
    Ok(pt_entry)
}

// ============================================================================
// PAGE TABLE STRUCTURE (Tab)
// ============================================================================

/// A page table (4 KiB aligned array of 512 entries).
#[repr(align(4096))]
#[derive(Debug)]
pub struct Tab(pub [Entry; 512]);

const impl Default for Tab {
    /// Creates a new, zero‑initialised page table.
    fn default() -> Self {
        Self([
            const {
                Entry::new(
                    Paddr::from_raw(0),
                    EntryFlags::empty()
                )
            };
            512
        ])
    }
}

impl Tab {
    /// Creates a new, zero‑initialised page table.
    pub const fn new() -> Self { Self::default() }
}

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
// AREA (for reporting mapped regions)
// ============================================================================

/// A contiguous virtual address range with the same flags.
#[derive(Clone, Copy)]
pub struct Area {
    pub start: usize,
    pub count: usize,
    pub flags: EntryFlags,
}

impl core::fmt::Display for Area {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("Area at {:X} of {} KiB: {:?}", self.start, (self.count + 1023) >> 10, self.flags))
    }
}

// ============================================================================
// EXECUTION CONTEXT (Exco)
// ============================================================================

/// An execution context for a set of page tables.
///
/// This struct holds the CR3 value and a reference to the root PML4 table.
/// It provides methods for mapping, unmapping, and other page table operations.
#[derive(Debug)]
pub struct Exco {
    pub cr3: u64,
    pub root: &'static mut Tab,
    pub owned: bool,
}

impl Default for Exco {
    fn default() -> Self {
        let root = alloc_tab_zeroed();
        let cr3 = Vaddr::from_ref(&*root).to_phys().to_raw() as u64;
        info!("Exco::new   CR3 {:#018X} owned", cr3);
        Exco { cr3, root, owned: true }
    }
}

impl Exco {
    // ------------------------------------------------------------------------
    // CONSTRUCTORS
    // ------------------------------------------------------------------------

    /// Creates a new `Exco` from a root table, CR3, and ownership flag.
    pub const fn from_root(root: &'static mut Tab, cr3: u64, owned: bool) -> Self {
        Exco { cr3, root, owned }
    }

    /// Creates a new empty address space (allocates a new PML4).
    pub fn new() -> Self { Self::default() }

    /// Duplicates the current address space (copy‑on‑write style).
    ///
    /// This creates a new PML4 and copies all entries from the current table.
    /// Child tables are recursively duplicated.
    pub fn dup(&self) -> Self {
        let (root, cr3) = Self::dup_table(self.root);
        info!("Exco::dup   CR3 {:#018X} (src CR3 {:#018X})", cr3 as u64, self.cr3);
        Exco { cr3: cr3 as u64, root, owned: true }
    }

    /// Internal recursive function to duplicate a table.
    fn dup_table(table: &Tab) -> (&'static mut Tab, usize) {
        let new = alloc_tab_zeroed();
        for (i, entry) in table.0.iter().enumerate() {
            if likely(entry.is_present() && !is_huge(entry)) {
                let child = tab_from_entry(entry);
                let (_, child_paddr) = Self::dup_table(child);
                new.0[i] = Entry::new(Paddr::from_raw(child_paddr), entry.flags());
            } else if entry.is_present() {
                new.0[i] = *entry;
            }
        }
        let paddr = Vaddr::from_ref(&*new).to_phys().to_raw();
        (new, paddr)
    }

    /// Returns the current CPU's page table context.
    pub fn current() -> Self {
        let cr3_raw: u64;
        unsafe {
            core::arch::asm!("mov {}, cr3", out(reg) cr3_raw);
        }
        let phys = (cr3_raw & 0x000f_ffff_ffff_f000) as usize;
        let vaddr = Paddr::from_raw(phys).to_virt();
        let root: &'static mut Tab = vaddr.to_ref_mut();
        Exco { cr3: cr3_raw, root, owned: false }
    }

    /// Clears the page table (replaces it with a zeroed table).
    ///
    /// If the context is owned, the old tables are freed.
    pub fn clean(&mut self) {
        if likely(self.owned) {
            Self::free_table(self.root, true);
            self.root = alloc_tab_zeroed();
        } else {
            for e in self.root.0.iter_mut() {
                *e = Entry::default();
            }
        }
        self.cr3 = Vaddr::from_ref(&*self.root).to_phys().to_raw() as u64;
    }

    /// Recursively frees a page table and its children.
    fn free_table(table: &mut Tab, free_self: bool) {
        let paddr = Vaddr::from_ref(&*table).to_phys();
        for entry in table.0.iter_mut() {
            if likely(entry.is_present() && !is_huge(entry)) {
                Self::free_table(tab_from_entry(entry), true);
            }
            *entry = Entry::default();
        }
        if free_self {
            free_tab(paddr);
        }
    }

    // ------------------------------------------------------------------------
    // MAPPING
    // ------------------------------------------------------------------------

    /// Maps a 4 KiB page (convenience wrapper).
    pub fn map4k(&mut self, vaddr: usize, paddr: Paddr, flags: EntryFlags) {
        self.try_map4k(vaddr, paddr, flags).expect("map4k failed");
    }

    /// Maps a 2 MiB huge page (convenience wrapper).
    pub fn map2m(&mut self, vaddr: usize, paddr: Paddr, flags: EntryFlags) {
        self.try_map2m(vaddr, paddr, flags).expect("map2m failed");
    }

    /// Maps a 1 GiB huge page (convenience wrapper).
    pub fn map1g(&mut self, vaddr: usize, paddr: Paddr, flags: EntryFlags) {
        self.try_map1g(vaddr, paddr, flags).expect("map1g failed");
    }

    /// Tries to map a 4 KiB page.
    ///
    /// # Errors
    /// - If the virtual address is not 4 KiB aligned.
    /// - If the page table walk fails.
    pub fn try_map4k(&mut self, vaddr: usize, paddr: Paddr, flags: EntryFlags) -> Result<(), &'static str> {
        if unlikely(vaddr & MASK_4K != 0) {
            return Err("vaddr not 4 KiB-aligned");
        }
        let entry = walk_entry_mut(self.root, vaddr, 0, true)?;
        *entry = Entry::new(paddr, flags | EntryFlags::PRESENT);
        flush_tlb(vaddr);
        Ok(())
    }

    /// Tries to map a 2 MiB huge page.
    ///
    /// # Errors
    /// - If the virtual address is not 2 MiB aligned.
    /// - If the physical address is not 2 MiB aligned.
    pub fn try_map2m(&mut self, vaddr: usize, paddr: Paddr, flags: EntryFlags) -> Result<(), &'static str> {
        if unlikely(vaddr & MASK_2M != 0) {
            return Err("vaddr not 2 MiB-aligned");
        }
        if unlikely(paddr.to_raw() & MASK_2M != 0) {
            return Err("paddr not 2 MiB-aligned");
        }
        debug!("map2m vaddr {:#X} -> phys {:#016X} flags {:?}", vaddr, paddr.to_raw(), flags);
        let entry = walk_entry_mut(self.root, vaddr, 1, true)?;
        *entry = Entry::new(paddr, flags | EntryFlags::PRESENT | EntryFlags::HUGE_PAGE);
        flush_tlb(vaddr);
        Ok(())
    }

    /// Tries to map a 1 GiB huge page.
    ///
    /// # Errors
    /// - If the virtual address is not 1 GiB aligned.
    /// - If the physical address is not 1 GiB aligned.
    pub fn try_map1g(&mut self, vaddr: usize, paddr: Paddr, flags: EntryFlags) -> Result<(), &'static str> {
        if unlikely(vaddr & MASK_1G != 0) {
            return Err("vaddr not 1 GiB-aligned");
        }
        if unlikely(paddr.to_raw() & MASK_1G != 0) {
            return Err("paddr not 1 GiB-aligned");
        }
        debug!("map1g vaddr {:#X} -> phys {:#016X} flags {:?}", vaddr, paddr.to_raw(), flags);
        let entry = walk_entry_mut(self.root, vaddr, 2, true)?;
        *entry = Entry::new(paddr, flags | EntryFlags::PRESENT | EntryFlags::HUGE_PAGE);
        flush_tlb(vaddr);
        Ok(())
    }

    // ------------------------------------------------------------------------
    // UNMAPPING
    // ------------------------------------------------------------------------

    /// Unmaps a 4 KiB page (convenience wrapper).
    pub fn unmap(&mut self, vaddr: usize) {
        self.try_unmap(vaddr).expect("unmap failed");
    }

    /// Tries to unmap a 4 KiB page.
    ///
    /// # Errors
    /// - If the address is not mapped.
    pub fn try_unmap(&mut self, vaddr: usize) -> Result<(), &'static str> {
        if let Ok(entry) = walk_entry_mut(self.root, vaddr, 0, false)
        && entry.is_present() {
            *entry = Entry::default();
            flush_tlb(vaddr);
            return Ok(());
        }

        if let Ok(entry) = walk_entry_mut(self.root, vaddr, 1, false)
        && entry.is_present() && is_huge(entry) {
            debug!("unmap 2M huge page at {:#X}", vaddr & !MASK_2M);
            *entry = Entry::default();
            flush_tlb(vaddr & !MASK_2M);
            return Ok(());
        }

        if let Ok(entry) = walk_entry_mut(self.root, vaddr, 2, false)
        && entry.is_present() && is_huge(entry) {
            debug!("unmap 1G huge page at {:#X}", vaddr & !MASK_1G);
            *entry = Entry::default();
            flush_tlb(vaddr & !MASK_1G);
            return Ok(());
        }

        Err("address not mapped")
    }

    // ------------------------------------------------------------------------
    // SPLITTING HUGE PAGES
    // ------------------------------------------------------------------------

    /// Splits a 2 MiB huge page (convenience wrapper).
    pub fn split2m(&mut self, vaddr: usize) {
        self.try_split2m(vaddr).expect("split2m failed");
    }

    /// Splits a 1 GiB huge page (convenience wrapper).
    pub fn split1g(&mut self, vaddr: usize) {
        self.try_split1g(vaddr).expect("split1g failed");
    }

    /// Tries to split a 2 MiB huge page into 4 KiB pages.
    ///
    /// # Errors
    /// - If the virtual address is not 2 MiB aligned.
    /// - If the entry is not a 2 MiB huge page.
    pub fn try_split2m(&mut self, vaddr: usize) -> Result<(), &'static str> {
        if unlikely(vaddr & MASK_2M != 0) {
            return Err("vaddr not 2 MiB-aligned");
        }

        let entry = walk_entry_mut(self.root, vaddr, 1, false)?;
        if unlikely(!entry.is_present() || !is_huge(entry)) {
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

    /// Tries to split a 1 GiB huge page into 2 MiB pages.
    ///
    /// # Errors
    /// - If the virtual address is not 1 GiB aligned.
    /// - If the entry is not a 1 GiB huge page.
    pub fn try_split1g(&mut self, vaddr: usize) -> Result<(), &'static str> {
        if unlikely(vaddr & MASK_1G != 0) {
            return Err("vaddr not 1 GiB-aligned");
        }

        let entry = walk_entry_mut(self.root, vaddr, 2, false)?;
        if unlikely(!entry.is_present() || !is_huge(entry)) {
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

    // ------------------------------------------------------------------------
    // MERGING INTO HUGE PAGES
    // ------------------------------------------------------------------------

    /// Merges 4 KiB pages into a 2 MiB huge page (convenience wrapper).
    pub fn merge2m(&mut self, vaddr: usize) {
        self.try_merge2m(vaddr).expect("merge2m failed");
    }

    /// Merges 2 MiB pages into a 1 GiB huge page (convenience wrapper).
    pub fn merge1g(&mut self, vaddr: usize) {
        self.try_merge1g(vaddr).expect("merge1g failed");
    }

    /// Tries to merge 512 consecutive 4 KiB pages into a 2 MiB huge page.
    ///
    /// # Errors
    /// - If the virtual address is not 2 MiB aligned.
    /// - If the PD entry is not a table pointer.
    /// - If the pages are not contiguous or have inconsistent flags.
    pub fn try_merge2m(&mut self, vaddr: usize) -> Result<(), &'static str> {
        if unlikely(vaddr & MASK_2M != 0) {
            return Err("vaddr not 2 MiB-aligned");
        }

        let pd_entry = walk_entry_mut(self.root, vaddr, 1, false)?;
        if unlikely(!pd_entry.is_present() || is_huge(pd_entry)) {
            return Err("PD entry is not a table pointer");
        }

        let pt = tab_from_entry(pd_entry);
        try_coalesce_into_2m(pt, vaddr, pd_entry)
    }

    /// Tries to merge 512 consecutive 2 MiB pages into a 1 GiB huge page.
    ///
    /// # Errors
    /// - If the virtual address is not 1 GiB aligned.
    /// - If the PDPT entry is not a table pointer.
    /// - If the pages are not contiguous or have inconsistent flags.
    pub fn try_merge1g(&mut self, vaddr: usize) -> Result<(), &'static str> {
        if unlikely(vaddr & MASK_1G != 0) {
            return Err("vaddr not 1 GiB-aligned");
        }

        let pdpt_entry = walk_entry_mut(self.root, vaddr, 2, false)?;
        if unlikely(!pdpt_entry.is_present() || is_huge(pdpt_entry)) {
            return Err("PDPT entry is not a table pointer");
        }

        let pd = tab_from_entry(pdpt_entry);
        try_coalesce_into_1g(pd, vaddr, pdpt_entry)
    }

    // ------------------------------------------------------------------------
    // ACTIVATION
    // ------------------------------------------------------------------------

    /// Loads this page table context (writes CR3).
    ///
    /// # Safety
    /// This function uses inline assembly to write CR3. The caller must ensure
    /// that the context is valid and that the CPU supports the CR3 value.
    #[inline(always)]
    pub unsafe fn activate(&self) {
        unsafe {
            core::arch::asm!("mov cr3, {}", in(reg) self.cr3, options(nostack, preserves_flags));
        }
    }

    fn free_table_recursive(table: &mut Tab, level: u8) {
        for i in 0..512 {
            let entry = &mut table.0[i];
            if entry.is_present() {
                if is_huge(entry) {
                    // Huge page: just clear the entry. 
                    // The physical memory is managed by the VMM/Process.
                    *entry = Entry::default();
                } else if level > 1 {
                    // Points to another page table
                    let child = tab_from_entry(entry);
                    Self::free_table_recursive(child, level - 1);
                    free_tab(entry.address());
                    *entry = Entry::default();
                } else {
                    // Level 1 (PT): Points to 4K physical pages.
                    // We only free the PT structure, not the mapped physical pages 
                    // (those are freed by the VMM when the Process drops).
                    *entry = Entry::default();
                }
            }
        }
    }
}

impl Drop for Exco {
    fn drop(&mut self) {
        if !self.owned {
            return; // Do not free kernel/shared page tables
        }

        // Free user-space page tables (PML4 indices 0..256)
        for i in 0..256 {
            let pml4_entry = &mut self.root.0[i];
            if pml4_entry.is_present() && !is_huge(pml4_entry) {
                let pdpt = tab_from_entry(pml4_entry);
                Self::free_table_recursive(pdpt, 3);
                free_tab(pml4_entry.address());
            }
            *pml4_entry = Entry::default();
        }
        
        // Free the root PML4 table itself
        let root_paddr = Vaddr::from_ref(&*self.root).to_phys();
        free_tab(root_paddr);
    }
}

// ============================================================================
// COALESCING FUNCTIONS (FOR MERGING)
// ============================================================================

/// Internal function to coalesce a PT into a 2 MiB huge page.
fn try_coalesce_into_2m(
    pt: &mut Tab,
    vaddr: usize,
    pd_entry: &mut Entry,
) -> Result<(), &'static str> {
    let first_flags = pt.0[0].flags() - EntryFlags::ACCESSED - EntryFlags::DIRTY;
    let first_paddr = pt.0[0].address();

    if unlikely(!pt.0[0].is_present()) {
        return Err("first PT entry not present");
    }

    for i in 1..512 {
        if unlikely(!pt.0[i].is_present() ){
            return Err("PT entry not present");
        }
        let flags = pt.0[i].flags() - EntryFlags::ACCESSED - EntryFlags::DIRTY;
        if unlikely(flags != first_flags) {
            return Err("inconsistent flags across PT entries");
        }
        let expected_paddr = Paddr::from_raw(first_paddr.to_raw() + (i << 12));
        if unlikely(pt.0[i].address().to_raw() != expected_paddr.to_raw()) {
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

/// Internal function to coalesce a PD into a 1 GiB huge page.
fn try_coalesce_into_1g(
    pd: &mut Tab,
    vaddr: usize,
    pdpt_entry: &mut Entry,
) -> Result<(), &'static str> {
    let first_flags = pd.0[0].flags() - EntryFlags::ACCESSED - EntryFlags::DIRTY;
    let first_paddr = pd.0[0].address();

    if unlikely(!pd.0[0].is_present() || !is_huge(&pd.0[0])) {
        return Err("first PD entry not a 2 MiB huge page");
    }

    for i in 1..512 {
        if unlikely(!pd.0[i].is_present() || !is_huge(&pd.0[i])) {
            return Err("PD entry not a 2 MiB huge page");
        }
        let flags = pd.0[i].flags() - EntryFlags::ACCESSED - EntryFlags::DIRTY;
        if unlikely(flags != first_flags) {
            return Err("inconsistent flags across PD entries");
        }
        let expected_paddr = Paddr::from_raw(first_paddr.to_raw() + (i << 21));
        if unlikely(pd.0[i].address().to_raw() != expected_paddr.to_raw()) {
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
