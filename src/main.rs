//! Kernel entry point and main initialisation.

#![no_std]
#![no_main]
#![feature(ptr_alignment_type)]
#![feature(unboxed_closures)]
#![allow(unused)]

use crate::mem::bsa::GfpFlags;

#[macro_use] pub extern crate extrum;
#[macro_use] pub extern crate bitflags;
#[macro_use] pub extern crate lazy_static;
extern crate alloc;

#[macro_use] mod macros;
pub mod rt;
pub mod sync;
pub mod kmsg;
pub mod mem;

// The kernel entry point.
entry!
{
    info!("Kernel v{} started.", env!("CARGO_PKG_VERSION"));
    mem::pmr::dump();
    mem::ema::init();

    // Simple allocation test using the unified page allocator.
    let x = mem::upa::alloc(4);
    mem::upa::free(x);

    let old = mem::ptm::Polen::from_exco(mem::ptm::Exco::current());

    debug!("Old PTM dump:");
    for r in old.report::<128>()
    {
        trace!("~ {}", r);
    }

    // Build page tables that map HHDM and the kernel.
    let mut ptm = mem::ptm::Polen::reference();

    debug!("PTM dump:");
    for r in ptm.report::<64>()
    {
        trace!("~ {}", r);
    }

    unsafe { ptm.activate(); }

    mem::pfm::init(&mut ptm);

    mem::bsa::init();

    mem::upa::migrate();

    mem::soa::init();

    // let x = mem::bsa::alloc_pages(4, GfpFlags::GFP_KERNEL);
    let x = mem::upa::alloc(1);

    // ── SOA allocation / deallocation smoke test ────────────────────────
    let obj_a = mem::soa::alloc(16).expect("SOA: 16-byte alloc failed");
    let obj_b = mem::soa::alloc(64).expect("SOA: 64-byte alloc failed");
    info!("SOA test: obj_a={:#X}  obj_b={:#X}", obj_a as usize, obj_b as usize);
    unsafe { mem::soa::dealloc(obj_a); }
    unsafe { mem::soa::dealloc(obj_b); }
    // Re‑alloc should reuse the just‑freed slot.
    let _reuse = mem::soa::alloc(16).expect("SOA: 16-byte re-alloc failed");
    unsafe { mem::soa::dealloc(_reuse); }
    info!("SOA: smoke test passed");

    for (i, &(pages, allocs)) in mem::soa::usage().iter().enumerate() {
        if pages > 0 {
            debug!("SOA cache[{}]: {} slab pages, {} allocs",
                   i, pages, allocs);
        }
    }

    debug!("EMA usage is {} KiB", mem::ema::usage() << 2);
    debug!("BSA usage is {} KiB", mem::bsa::usage().iter().sum::<usize>() << 2);
}
