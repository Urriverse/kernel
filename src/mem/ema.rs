use core::hint::unlikely;

use crate::{mem::pmr::Kind, sync::Nutex};

use super::{pmr::{self, Region}, kdm::Paddr};

pub struct EarlyMemAlloc
{
    pub(super) top     : usize,
    pub(super) bottom  : usize,
    pub(super) limit   : usize,
}

impl EarlyMemAlloc
{
    pub(super) fn new() -> Self
    {
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

        let bottom = (largest.base + largest.len) & !0xfff;
        let top = bottom;
        let limit = (largest.base + 4095) & !0xfff;

        info!("Initialized");
        // debug!("~ top      = {:#X}", top   );
        // debug!("~ bottom   = {:#X}", bottom);
        // debug!("~ limit    = {:#X}", limit );

        Self { top, bottom, limit }
    }

    #[inline]
    pub(super) fn touch(&self) {}

    pub(super) fn alloc(&mut self, count: usize) -> usize {
        let count = (count + 4095) & !4095;
        if unlikely(self.top < self.limit || self.top - count < self.limit ){
            error!("EMA: out of memory (requested {} bytes)", count);
            return 0;
        }
        self.top -= count;
        self.top
    }
    
    pub fn allocated_range(&self) -> (usize, usize) {
        if self.top >= self.bottom {
            (0, 0)
        } else {
            (self.top, self.bottom)
        }
    }
}

lazy_static! {
    pub static ref EMA: Nutex<EarlyMemAlloc> = Nutex::new(EarlyMemAlloc::new());
}

pub fn init()
{
    EMA.lock().touch();
}

pub fn alloc(count: usize) -> Paddr
{
    Paddr::from_raw(EMA.lock().alloc(count << 12))
}

pub fn usage() -> usize
{
    let ema = EMA.lock();
    (ema.bottom - ema.top) >> 12
}

pub fn get_allocated_range() -> (usize, usize) {
    EMA.lock().allocated_range()
}
