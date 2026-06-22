use heapless::Vec;
use crate::mem::pmr::{self, Region, Kind};

limine! { pub HHDMR <= HhdmRequest }

lazy_static! {
    pub static ref HHDM: usize = HHDMR.response().expect("Can't obtain HHDM offset.").offset as usize;
    
    static ref REGIONS: Vec<Region, 128> = {
        let mut regions = Vec::new();
        
        for region in pmr::iter() {
            match region.kind {
                Kind::USABLE | 
                Kind::KERNEL | 
                Kind::BOOTLOADER |
                Kind::FRAMEBUF |
                Kind::ACPI |
                Kind::ACPI_NVS |
                Kind::RESERVED => {
                    let _ = regions.push(region);
                }
                _ => continue,
            }
        }
        
        info!("KDM: Initialized. HHDM offset: {:#X}", *HHDM);
        
        regions
    };
}

pub fn init() {
    let _ = &*HHDM;
    let _ = &*REGIONS;
}

#[inline]
pub fn regions() -> &'static [Region] {
    &REGIONS
}

#[inline]
pub fn region_count() -> usize {
    REGIONS.len()
}

#[inline]
pub fn is_mapped(paddr: usize) -> bool {
    for region in REGIONS.iter() {
        if paddr >= region.base && paddr < region.base + region.len {
            return true;
        }
    }
    false
}

#[inline]
pub fn is_range_mapped(paddr: usize, size: usize) -> bool {
    let end = paddr + size;
    for region in REGIONS.iter() {
        let region_end = region.base + region.len;
        if paddr >= region.base && end <= region_end {
            return true;
        }
    }
    false
}

pub fn find_region(paddr: usize) -> Option<Region> {
    for region in REGIONS.iter() {
        if paddr >= region.base && paddr < region.base + region.len {
            return Some(*region);
        }
    }
    None
}

#[derive(Clone, Copy, Debug)]
pub struct Paddr(usize);

#[derive(Clone, Copy, Debug)]
pub struct Vaddr(usize);

impl Paddr {
    #[inline(always)]
    pub const fn from_raw(r: usize) -> Self {
        Self(r)
    }

    #[inline(always)]
    pub const fn to_raw(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn to_virt(self) -> Vaddr {
        Vaddr(self.0 + *HHDM)
    }

    #[inline]
    pub fn try_to_virt(self) -> Option<Vaddr> {
        if is_mapped(self.0) {
            Some(Vaddr(self.0 + *HHDM))
        } else {
            None
        }
    }

    #[inline]
    pub fn is_mapped(self) -> bool {
        is_mapped(self.0)
    }
}

impl Vaddr {
    #[inline(always)]
    pub const fn from_raw(r: usize) -> Self {
        Self(r)
    }

    #[inline(always)]
    pub fn from_ptr<T>(ptr: *const T) -> Self {
        Self::from_raw(ptr as usize)
    }

    #[inline(always)]
    pub fn from_ptr_mut<T>(ptr: *mut T) -> Self {
        Self::from_raw(ptr as usize)
    }

    #[inline(always)]
    pub fn from_ref<T>(r: &'_ T) -> Self {
        Self::from_ptr(r)
    }

    #[inline(always)]
    pub fn from_ref_mut<T>(r: &'_ mut T) -> Self {
        Self::from_ptr_mut(r)
    }

    #[inline(always)]
    pub fn to_raw(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn to_phys(self) -> Paddr {
        Paddr(self.0 - *HHDM)
    }

    #[inline(always)]
    pub const fn to_ptr<T>(self) -> *const T {
        self.0 as *const T
    }

    #[inline(always)]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_ptr_mut<T>(self) -> *mut T {
        self.0 as *mut T
    }

    #[inline(always)]
    pub const fn to_ref<'a, T>(self) -> &'a T {
        unsafe {
            self.to_ptr::<T>().as_ref_unchecked()
        }
    }

    #[inline(always)]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_ref_mut<'a, T>(self) -> &'a mut T {
        unsafe {
            self.to_ptr_mut::<T>().as_mut_unchecked()
        }
    }

    #[inline]
    pub fn is_in_hhdm(self) -> bool {
        self.0 >= *HHDM
    }
}
