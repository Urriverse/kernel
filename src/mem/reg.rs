//! Physical memory map from the bootloader (Limine).
//!
//! This module provides access to the memory map entries, a typed [`Kind`]
//! enumeration for region types, and an iterator over [`Region`]s.

/// Type of a memory region as reported by the bootloader.
extrum! {
    #[derive(Clone, Copy, PartialEq, Default)]
    pub enum Kind: u64
    {
        USABLE      = 0,
        RESERVED    = 1,
        ACPI        = 2,
        ACPI_NVS    = 3,
        BAD         = 4,
        BOOTLOADER  = 5,
        KERNEL      = 6,
        FRAMEBUF    = 7,
        MAPRESERVED = 8,
    }
}

implement_display![Kind];

/// A contiguous memory region with a base address, length, and kind.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Region
{
    pub base: usize,
    pub len: usize,
    pub kind: Kind,
}

/// Iterator over memory map regions.
pub struct Iter
{
    next: usize,
}

/// Limine request for the memory map.
#[unsafe(link_section = ".requests")]
pub static MEMMAP: limine::request::MemmapRequest = limine::request::MemmapRequest::new();

lazy_static!
{
    static ref MMAP: &'static [&'static limine::memmap::Entry] = MEMMAP.response().expect("Can't obtain memory regions info.").entries();
}

impl Iter
{
    /// Create a new iterator starting at the first region.
    pub(super) const fn new() -> Self { Self { next: 0 } }

    /// Return the next region, or `None` if at the end.
    pub fn next(&mut self) -> Option<Region>
    {
        if self.next < MMAP.len()
        {
            let e = MMAP[self.next];
            self.next += 1;
            Some
            (
                Region
                {
                    base: e.base as usize,
                    len: e.length as usize,
                    kind: Kind(e.type_),
                }
            )
        }
        else
        {
            self.next = 0;
            None
        }
    }
}

impl Iterator for Iter
{
    type Item = Region;
    fn next(&mut self) -> Option<Self::Item>
    {
        Iter::next(self)
    }
}

/// Obtain an iterator over all memory regions.
pub fn iter() -> Iter
{
    Iter::new()
}

/// Dump all memory regions to the log (debug level).
pub fn dump()
{
    debug!("Memory regions:");
    for r in iter()
    {
        debug!("~ base {:-12X} of {:>12} KiB, {:<16}", r.base, (r.len + 1023) >> 10, r.kind);
    }
}
