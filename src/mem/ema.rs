//! Early Memory Allocator (EMA).
//!
//! The EMA is a simple bump allocator that runs before a proper page allocator
//! is available. It allocates memory from the top of the largest usable memory
//! region, moving downwards. All allocations are page‑aligned (4 KiB).

use crate::{mem::pmr::Kind, sync::Nutex};

use super::{pmr::{self, Region}, kdm::Paddr};

/// Early memory allocator state.
pub struct EarlyMemAlloc
{
    /// Current top of the free area (allocations are taken from here).
    pub(super) top     : usize,
    /// Bottom of the usable region (allocations cannot go below this).
    pub(super) bottom  : usize,
    /// Limit – the lowest address that can be allocated (initial top when empty).
    pub(super) limit   : usize,
}

impl EarlyMemAlloc
{
    /// Create a new early allocator by finding the largest usable memory region.
    ///
    /// # Panics
    /// Panics if there is no usable memory region.
    pub(super) fn new() -> Self
    {
        // Find the largest usable region.
        let mut largest: Region = Region::default();

        for region in pmr::iter()
        {
            if region.kind == Kind::USABLE && region.len > largest.len
            {
                largest = region;
            }
        }

        if largest == Region::default()
        {
            panic!("Can't initialize EMA: no usable memory")
        }

        // bottom is aligned down to 4 KiB.
        let bottom = (largest.base + largest.len - 4097) >> 12 << 12;
        let top = (bottom - 4097) >> 12 << 12;
        // limit is aligned up to 4 KiB.
        let limit = (largest.base + 4095) >> 12 << 12;

        info!("EMA initialized.");
        debug!("~ top      = {:#X}", top   );
        debug!("~ bottom   = {:#X}", bottom);
        debug!("~ limit    = {:#X}", limit );

        Self { top, bottom, limit }
    }

    /// A no‑op method used to “touch” the allocator (ensure it is initialised).
    #[inline]
    pub(super) fn touch(&self) {}

    /// Allocate `count` bytes from the allocator.
    ///
    /// Returns the physical address of the allocated block, or `0` on failure.
    pub(super) fn alloc(&mut self, count: usize) -> usize
    {
        if self.top - count < self.limit
        {
            error!("EMA failed to allocate {} bytes. Reason: out of memory.", count);
            return 0 as _;
        }
        self.top -= count;
        self.top
    }
}

lazy_static! {
    /// Global early memory allocator, protected by a [`Nutex`].
    pub static ref EMA: Nutex<EarlyMemAlloc> = Nutex::new(EarlyMemAlloc::new());
}

/// Initialise the EMA (forces lazy initialisation).
pub fn init()
{
    EMA.lock().touch();
}

/// Allocate `count` pages (each 4 KiB) from the EMA.
///
/// Returns a physical address (page‑aligned).
pub fn alloc(count: usize) -> Paddr
{
    Paddr::from_raw(EMA.lock().alloc(count << 12))
}

pub fn usage() -> usize
{
    let ema = EMA.lock();
    (ema.bottom - ema.top) >> 12
}
