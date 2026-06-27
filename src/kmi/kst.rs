//! Kernel System Table
//! 
//! Static table of kernel functions which is not mutable.

use core::mem::transmute as to;

use crate::kmi::kmdl::{SymbolHandle, Symbol, SymbolGuard};

#[repr(C)]
pub struct KeSysTab {
    pub link            :   fn(u64) ->  Option<SymbolHandle>,
    pub link_guard      :   fn(&SymbolHandle) -> SymbolGuard,
    pub link_guard_get  :   fn(&SymbolGuard) -> &fn(),
    pub export          :   fn(u64, &'static fn()) -> Option<Symbol>,
    pub suicide         :   fn(i32) -> !,
    pub log             :   fn(u8, &'static str, &'static str, u32, &core::fmt::Arguments) -> (),
    pub panic           :   fn(&core::panic::PanicInfo) -> !,
    pub alloc           :   fn(core::alloc::Layout) -> *mut u8,
    pub free            :   fn(*mut u8, core::alloc::Layout) -> (),
}

fn log_wrapper(lv: u8, mp: &'static str, f: &'static str, l: u32, a: &core::fmt::Arguments) -> () {
    crate::kmsg::log(unsafe { to(lv) }, mp, f, l, *a);
}

pub static KST: KeSysTab = KeSysTab {
    link            :   crate::kmi::kmdl::link,
    link_guard      :   SymbolHandle::get,
    link_guard_get  :   SymbolGuard::get::<fn()->()>,
    export          :   crate::kmi::kmdl::export::<fn()->()>,
    suicide         :   crate::kmi::kmdl::suicide,
    log             :   log_wrapper,
    panic           :   crate::rt::panic::panic,
    alloc           :   crate::mem::soa::alloc,
    free            :   crate::mem::soa::free,
};
