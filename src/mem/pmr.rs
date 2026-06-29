//! # Physical Memory Regions (PMR)
//!
//! This module provides an interface to the physical memory map provided by the
//! bootloader (Limine). It enumerates all memory regions, their types, base addresses,
//! and lengths, and allows iteration over them.
//!
//! ## Overview
//!
//! The PMR module is responsible for:
//!
//! - Parsing the Limine memory map response.
//! - Categorizing regions by their `Kind` (usable, reserved, ACPI, kernel, etc.).
//! - Providing an iterator over all regions (`pmr::iter()`).
//! - Allowing random access to regions by index (`pmr::nth`, `pmr::nth_unchecked`).
//!
//! ## Memory Region Kinds
//!
//! The `Kind` enum mirrors the Limine memory map entry types:
//!
//! - `USABLE` – Normal RAM that can be used by the kernel.
//! - `RESERVED` – Reserved for hardware or firmware; do not use.
//! - `ACPI` – ACPI reclaimable memory.
//! - `ACPI_NVS` – ACPI NVS memory (non‑volatile storage).
//! - `BAD` – Memory with errors; should be avoided.
//! - `BOOTLOADER` – Bootloader‑reserved memory (may be usable after boot).
//! - `KERNEL` – Memory occupied by the kernel image.
//! - `FRAMEBUF` – Framebuffer memory (mapped to the display).
//! - `MAPRESERVED` – Reserved for memory‑mapped I/O.
//!
//! ## Usage
//!
//! The PMR module is used early in the boot process by the memory management
//! subsystem to discover available physical memory for the early allocator (EMA),
//! the page frame manager (PFM), and the buddy allocator (BSA).
//!
//! Example:
//! ```ignore
//! for region in pmr::iter() {
//!     if region.kind == pmr::Kind::USABLE {
//!         // Use this region for memory allocation
//!     }
//! }
//! ```
//!
//! ## Lazy Initialization
//!
//! The memory map is stored in a `lazy_static` (`MMAP`) which fetches the Limine
//! response when first accessed. This ensures that the response is available
//! before any PMR functions are called.
//!
//! ## Safety
//!
//! - The Limine memory map is guaranteed to be valid by the bootloader.
//! - All functions are safe; they just read the static data.
//! - The iterator does not perform bounds checks on `MMAP`; it relies on the
//!   length provided by Limine.

// ============================================================================
// TYPES
// ============================================================================

extrum! {
    /// Physical memory region type.
    ///
    /// This enum corresponds to the `type` field of Limine memory map entries.
    /// The numeric values match the Limine specification.
    #[derive(Clone, Copy, PartialEq, Default)]
    pub enum Kind: u64 {
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

/// A physical memory region.
///
/// Represents a contiguous range of physical memory with a given type.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Region {
    /// Base physical address of the region (in bytes).
    pub base: usize,
    /// Length of the region (in bytes).
    pub len: usize,
    /// Type of the region (usable, reserved, etc.).
    pub kind: Kind,
}

/// Iterator over physical memory regions.
///
/// This struct is returned by `pmr::iter()` and yields `Region` items.
pub struct Iter {
    next: usize,
}

// ============================================================================
// LIMINE REQUEST & GLOBAL STATE
// ============================================================================

// Limine request for the memory map.
//
// This is a static request that the bootloader fills with the memory map.
// The response is accessed via `MEMMAP.response()`.
limine! { pub MEMMAP <= MemmapRequest }

// Lazy‑initialized reference to the Limine memory map entries.
//
// The map is a slice of Limine `Entry` structs, each containing base, length,
// and type information.
lazy_static! {
    static ref MMAP: &'static [&'static limine::memmap::Entry] =
        MEMMAP.response().expect("Can't obtain memory regions info.").entries();
}

// ============================================================================
// PUBLIC API
// ============================================================================

/// Returns the memory region at the given index, if it exists.
///
/// # Arguments
/// * `n` – The index of the region.
///
/// # Returns
/// `Some(Region)` if the index is valid, otherwise `None`.
pub fn nth(n: usize) -> Option<Region> {
    let o = MMAP.get(n);
    o.map(
        |e| Region {
            base: e.base as usize,
            len: e.length as usize,
            kind: Kind(e.type_),
        }
    )
}

/// Returns the memory region at the given index without bounds checking.
///
/// # Safety
/// The caller must ensure that `n < pmr::len()`.
///
/// # Arguments
/// * `n` – The index of the region.
///
/// # Returns
/// A `Region` struct.
pub fn nth_unchecked(n: usize) -> Region {
    let e = MMAP[n];
    Region {
        base: e.base as usize,
        len: e.length as usize,
        kind: Kind(e.type_),
    }
}

/// Returns an iterator over all physical memory regions.
///
/// The iterator yields `Region` structs in the order provided by Limine.
pub fn iter() -> Iter {
    Iter::new()
}

/// Returns the total number of memory regions.
pub fn len() -> usize {
    MMAP.len()
}

/// Dumps all memory regions to the log (for debugging).
///
/// This function logs each region's base, size, and type at the `debug` level.
pub fn dump() {
    debug!("Memory regions:");
    for r in iter() {
        #[cfg(feature = "lowlog")] let _ = r;
        debug!("~ base {:-12X} of {:>12} KiB, {:<16}", r.base, (r.len + 1023) >> 10, r.kind);
    }
}

// ============================================================================
// ITERATOR IMPLEMENTATION
// ============================================================================

impl Iter {
    /// Creates a new iterator starting at index 0.
    pub(super) const fn new() -> Self {
        Self { next: 0 }
    }

    /// Advances the iterator and returns the next region, if any.
    ///
    /// This method is used internally by the `Iterator` trait implementation.
    pub fn next_reg(&mut self) -> Option<Region> {
        if self.next < MMAP.len() {
            let e = MMAP[self.next];
            self.next += 1;
            Some(Region {
                base: e.base as usize,
                len: e.length as usize,
                kind: Kind(e.type_),
            })
        } else {
            self.next = 0;
            None
        }
    }
}

impl Iterator for Iter {
    type Item = Region;

    /// Returns the next region.
    fn next(&mut self) -> Option<Self::Item> {
        Iter::next_reg(self)
    }
}
