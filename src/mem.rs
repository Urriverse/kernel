pub mod pmr;
pub mod kdm;
pub mod ema;
pub mod ptm;
pub mod upa;
pub mod pfm;
pub mod bsa;
pub mod soa;
pub mod vma;

use crate::sync::Nutex;

// absolutely safe to modify, empty PML4.
static mut SPURIOUS: crate::arch::paging::Tab = crate::arch::paging::Tab::new();

// one shared PTM until we get multitasking. after that, kernel mapping changes are prohibited, so no high-half changes inconsency will occur later
#[allow(static_mut_refs)]
pub static PTM: Nutex<ptm::Polen> = Nutex::new(
    ptm::Polen::from_exco(
        crate::arch::paging::Exco::from_root(
            unsafe { &mut SPURIOUS },
            0u64,
            false
        )
    )
);

pub fn init_bsp() {
    ema::init();
    pfm::init();
    kdm::init();
    bsa::init();
    upa::migrate();
    soa::init();
    crate::rt::gall::set_soa();
    *PTM.lock() = ptm::Polen::reference();
    unsafe { PTM.lock().activate() };
}

pub fn init_ap() {
    unsafe { PTM.lock().activate() };
}
