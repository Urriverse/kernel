use core::marker::PhantomData;

#[repr(C)] pub struct Leak<T>(*mut (), PhantomData<T>);

impl<T: 'static> core::ops::Deref for Leak<T> {
    type Target = T;

    fn deref(&self) -> &'static Self::Target {
        unsafe { (self.0 as *const T).as_ref_unchecked() }
    }
}

impl<T: 'static> core::ops::DerefMut for Leak<T> {
    fn deref_mut(&mut self) -> &'static mut Self::Target {
        unsafe { (self.0 as *mut T).as_mut_unchecked() }
    }
}

impl<T> Leak<T> {
    pub fn new(inner: T) -> Self {
        let va = crate::mem::kdm::Vaddr::from_raw(crate::mem::soa::alloc(core::alloc::Layout::for_value(&inner)) as usize);
        let mutref = va.to_ref_mut::<T>();
        *mutref = inner; // move!
        Self(va.to_ptr_mut(), PhantomData)
    }

    pub fn inner(&self) -> &'static mut T { unsafe { (self.0 as *mut T).as_mut_unchecked() } }
}

pub type RwLeak<T> = Leak<crate::sync::RwLock<T>>;
pub type MuLeak<T> = Leak<crate::sync::Mutex<T>>;
pub type NuLeak<T> = Leak<crate::sync::Nutex<T>>;
pub type NiLeak<T> = Leak<crate::sync::Nitex<T>>;
