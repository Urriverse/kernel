#[repr(C)] pub struct Leak<T>(*mut T);

impl<T> core::ops::Deref for Leak<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref_unchecked() }
    }
}

impl<T> core::ops::DerefMut for Leak<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut_unchecked() }
    }
}

impl<T> Leak<T> {
    pub fn new(inner: T) -> Self {
        let va = crate::mem::kdm::Vaddr::from_raw(crate::mem::soa::alloc(core::alloc::Layout::for_value(&inner)) as usize);
        let mutref = va.to_ref_mut::<T>();
        *mutref = inner; // move!
        Self(va.to_ptr_mut())
    }
}

pub type RwLeak<T> = Leak<crate::sync::RwLock<T>>;
pub type MuLeak<T> = Leak<crate::sync::Mutex<T>>;
pub type NuLeak<T> = Leak<crate::sync::Nutex<T>>;
pub type NiLeak<T> = Leak<crate::sync::Nitex<T>>;
