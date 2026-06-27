//! Kernel System Table
//! 
//! Static table of kernel functions which is not mutable.

use core::{alloc::Layout, fmt::Arguments, mem::transmute as to, panic::PanicInfo, ptr::addr_of, sync::atomic::{AtomicUsize, Ordering::{Relaxed, Release}}};

use alloc::collections::btree_map::BTreeMap;

use crate::{kmi::kmdl::{Symbol, SymbolGuard, SymbolHandle}, sched::{current_process, task::TaskId}, sync::RwLock};

#[repr(C)]
pub struct KeSysTab {
    pub link            :   fn(u64) ->  Option<SymbolHandle>,
    pub link_guard      :   fn(&SymbolHandle) -> SymbolGuard,
    pub link_guard_get  :   fn(&SymbolGuard) -> &fn(),
    pub export          :   fn(u64, &'static fn()) -> Option<Symbol>,
    pub suicide         :   fn(i32) -> !,
    pub log             :   fn(u8, &'static str, &'static str, u32, &Arguments) -> (),
    pub panic           :   fn(&PanicInfo) -> !,
    pub alloc           :   fn(Layout) -> *mut u8,
    pub free            :   fn(*mut u8, Layout) -> (),
    pub run_module      :   fn(elf: &[u8]) -> Result<TaskId, usize>,
    pub cprc_inc        :   fn() -> (),
    pub cprc_dec        :   fn() -> (),
    pub cprc_load       :   fn() -> usize,
    pub cprc_store      :   fn(usize) -> (),
    pub cprc_ref        :   fn() -> &'static AtomicUsize,

    pub gstab           :   &'static RwLock<BTreeMap<u64, Symbol>>,
}

fn log_wrapper(lv: u8, mp: &'static str, f: &'static str, l: u32, a: &core::fmt::Arguments) -> () {
    crate::kmsg::log(unsafe { to(lv) }, mp, f, l, *a);
}

lazy_static! {
    pub static ref KST: KeSysTab = KeSysTab {
        link            :   crate::kmi::kmdl::link,
        link_guard      :   SymbolHandle::get,
        link_guard_get  :   SymbolGuard::get::<fn()->()>,
        export          :   crate::kmi::kmdl::export::<fn()->()>,
        suicide         :   crate::kmi::kmdl::suicide,
        log             :   log_wrapper,
        panic           :   crate::rt::panic::panic,
        alloc           :   crate::mem::soa::alloc,
        free            :   crate::mem::soa::free,
        gstab           :   &super::kmdl::GSTAB,
        run_module      :   super::mbs::run_module,
        cprc_inc        :   || { current_process().expect("Unknown context").rc.fetch_add(1, Release); },
        cprc_dec        :   || { current_process().expect("Unknown context").rc.fetch_sub(1, Release); },
        cprc_load       :   || { current_process().expect("Unknown context").rc.load(Relaxed) },
        cprc_store      :   |x| { current_process().expect("Unknown context").rc.store(x, Relaxed); },
        cprc_ref        :   || unsafe { addr_of!(current_process().expect("Unknown context").rc).as_ref_unchecked() },
    };
}
