//! Kernel Direct Mapper (KDM).
//!
//! This module provides the offset between physical addresses and their
//! directly‑mapped virtual addresses (the HHDM). It also defines the
//! [`Paddr`] and [`Vaddr`] newtypes for physical and virtual addresses.

/// Limine request for the HHDM offset.
#[unsafe(link_section = ".requests")]
pub static HHDMR: limine::request::HhdmRequest = limine::request::HhdmRequest::new();

lazy_static! {
    /// The HHDM offset: `virtual = physical + *HHDM`.
    ///
    /// This is made `pub(crate)` so that the page table module can use it.
    pub(crate) static ref HHDM: usize = HHDMR.response().expect("Can't obtain HHDM offset.").offset as usize;
}

/// Initialise the KDM module (forces lazy evaluation of `HHDM`).
pub fn init()
{
    let _ = HHDM;
}

/// A physical address.
#[derive(Clone, Copy, Debug)]
pub struct Paddr(usize);

/// A virtual address (in the direct‑map region or elsewhere).
#[derive(Clone, Copy, Debug)]
pub struct Vaddr(usize);

impl Paddr
{
    /// Create a `Paddr` from a raw integer.
    pub fn from_raw(r: usize) -> Self
    {
        Self(r)
    }

    /// Convert to a raw integer.
    pub fn to_raw(self) -> usize
    {
        self.0
    }

    /// Convert this physical address to a virtual address in the direct map.
    pub fn to_virt(self) -> Vaddr
    {
        Vaddr(self.0 + *HHDM)
    }
}

impl Vaddr
{
    /// Create a `Vaddr` from a raw integer.
    pub fn from_raw(r: usize) -> Self
    {
        Self(r)
    }

    /// Create a `Vaddr` from a const pointer.
    pub fn from_ptr<T>(ptr: *const T) -> Self
    {
        Self::from_raw(ptr as usize)
    }

    /// Create a `Vaddr` from a mutable pointer.
    pub fn from_ptr_mut<T>(ptr: *mut T) -> Self
    {
        Self::from_raw(ptr as usize)
    }

    /// Create a `Vaddr` from a reference.
    pub fn from_ref<T>(r: &'_ T) -> Self
    {
        Self::from_ptr(r)
    }

    /// Create a `Vaddr` from a mutable reference.
    pub fn from_ref_mut<T>(r: &'_ mut T) -> Self
    {
        Self::from_ptr_mut(r)
    }

    /// Convert to a raw integer.
    pub fn to_raw(self) -> usize
    {
        self.0
    }

    /// Convert this virtual address (must be in the direct map) to a physical address.
    pub fn to_phys(self) -> Paddr
    {
        Paddr(self.0 - *HHDM)
    }

    /// Convert to a const pointer.
    #[inline(always)]
    pub fn to_ptr<T>(self) -> *const T
    {
        self.0 as *const T
    }

    /// Convert to a mutable pointer.
    #[inline(always)]
    pub fn to_ptr_mut<T>(self) -> *mut T
    {
        self.0 as *mut T
    }

    /// Convert to a reference (unsafe).
    ///
    /// # Safety
    /// The caller must ensure the address is valid and points to a `T`.
    #[inline(always)]
    pub fn to_ref<'a, T>(self) -> &'a T
    {
        unsafe
        {
            self.to_ptr::<T>().as_ref_unchecked()
        }
    }

    /// Convert to a mutable reference (unsafe).
    ///
    /// # Safety
    /// The caller must ensure the address is valid, points to a `T`, and
    /// that no aliasing rules are violated.
    #[inline(always)]
    pub fn to_ref_mut<'a, T>(self) -> &'a mut T
    {
        unsafe
        {
            self.to_ptr_mut::<T>().as_mut_unchecked()
        }
    }
}
