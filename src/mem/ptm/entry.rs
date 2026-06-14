//! Page table entry definitions for x86_64 4‑level paging.
//!
//! This module defines the [`EntryFlags`] bitflags and the [`Entry`] struct
//! that represents a single page table entry.

use crate::mem::kdm::Paddr;

bitflags! {
    /// x86_64 page table entry flags.
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct EntryFlags: u64
    {
        /// Present – the page is mapped in memory.
        const PRESENT         = 1 << 0;
        /// Writable – the page may be written to.
        const WRITABLE        = 1 << 1;
        /// User accessible – the page is accessible from ring 3.
        const USER_ACCESSIBLE = 1 << 2;
        /// Write‑through caching enabled.
        const WRITE_THROUGH   = 1 << 3;
        /// Cache disabled.
        const CACHE_DISABLE   = 1 << 4;
        /// Accessed – the page has been read or written.
        const ACCESSED        = 1 << 5;
        /// Dirty – the page has been written to.
        const DIRTY           = 1 << 6;
        /// Huge page – indicates a 2 MiB or 1 GiB page (depends on level).
        const HUGE_PAGE       = 1 << 7;
        /// Global – the page is not invalidated on CR3 write (PGE enabled).
        const GLOBAL          = 1 << 8;

        /// Copy‑on‑write – when set, a write fault should trigger COW handling.
        const COPY_ON_WRITE   = 1 << 60;
        /// File-mapped - when set, read/write operations should be sent to the file subsystem.
        const FILE_MAPPED     = 1 << 61;
        /// Swapped - when set, read/write operations should trigger loading page from storage.
        const SWAPPED         = 1 << 62;
        /// No‑execute – disallow instruction fetch.
        const NO_EXECUTE      = 1 << 63;
    }
}

/// Page table entry for x86_64 (4‑level paging, 4 KiB pages).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Entry(u64);

impl Entry
{
    /// Mask to extract the physical frame number (bits 12..51).
    pub const ADDRESS_MASK: u64 = 0x000f_ffff_ffff_f000;

    /// Mask for the available‑for‑OS bits (bits 9, 10, 11, 52..62).
    pub const AVAILABLE_MASK: u64 = 0x7ff0_0000_0000_0e00;

    /// Number of bits to shift right to obtain the physical frame number.
    pub const ADDRESS_SHIFT: u32 = 12;

    /// Creates a new entry with the given physical address (must be 4 KiB‑aligned)
    /// and the specified flags.
    #[inline]
    pub fn new(physical_address: Paddr, flags: EntryFlags) -> Self
    {
        let addr_part = physical_address.to_raw() as u64 & Self::ADDRESS_MASK;
        Self(addr_part | flags.bits())
    }

    /// Returns the raw `u64` value of the entry.
    #[inline]
    pub fn as_u64(&self) -> u64
    {
        self.0
    }

    /// Returns the flags of this entry.
    #[inline]
    pub fn flags(&self) -> EntryFlags
    {
        EntryFlags::from_bits_truncate(self.0)
    }

    /// Sets the flags of this entry.
    #[inline]
    pub fn set_flags(&mut self, flags: EntryFlags)
    {
        self.0 = (self.0 & !EntryFlags::all().bits()) | flags.bits();
    }

    /// Returns the physical address (4 KiB aligned) of the frame.
    #[inline]
    pub fn address(&self) -> Paddr
    {
        Paddr::from_raw((self.0 & Self::ADDRESS_MASK) as usize)
    }

    /// Sets the physical address (must be 4 KiB aligned).
    #[inline]
    pub fn set_address(&mut self, paddr: Paddr)
    {
        let addr = paddr.to_raw() as u64;
        self.0 = (self.0 & !Self::ADDRESS_MASK) | (addr & Self::ADDRESS_MASK);
    }

    /// Checks whether the page is present.
    #[inline]
    pub fn is_present(&self) -> bool
    {
        self.flags().contains(EntryFlags::PRESENT)
    }

    /// Checks whether the page is writable.
    #[inline]
    pub fn is_writable(&self) -> bool
    {
        self.flags().contains(EntryFlags::WRITABLE)
    }

    /// Checks whether the page is executable (NX bit not set).
    #[inline]
    pub fn is_executable(&self) -> bool
    {
        !self.flags().contains(EntryFlags::NO_EXECUTE)
    }
}

impl From<u64> for Entry
{
    #[inline]
    fn from(raw: u64) -> Self
    {
        Entry(raw)
    }
}

impl From<Entry> for u64
{
    #[inline]
    fn from(entry: Entry) -> Self
    {
        entry.0
    }
}

impl Default for Entry
{
    /// Returns an empty entry (all bits zero, i.e. not present).
    #[inline]
    fn default() -> Self
    {
        Entry(0)
    }
}
