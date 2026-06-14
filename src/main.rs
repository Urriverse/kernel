//! Kernel entry point and main initialisation.

#![no_std]
#![no_main]
#![feature(ptr_alignment_type)]
#![feature(unboxed_closures)]
#![allow(unused)]

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
    mem::reg::dump();
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

    debug!("EMA usage is {} KiB", mem::ema::usage() << 2);
}
