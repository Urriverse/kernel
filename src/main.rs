#![no_std]
#![no_main]

#![feature(unsafe_cell_access)]
#![feature(abi_x86_interrupt)]
#![feature(const_trait_impl)]
#![feature(likely_unlikely)]
#![feature(const_cmp)]

#![cfg_attr(not(debug_assertions), allow(unused_assignments))]

#[allow(unused)] #[macro_use] pub extern crate extrum;
#[allow(unused)] #[macro_use] pub extern crate bitflags;
#[allow(unused)] #[macro_use] pub extern crate lazy_static;
#[allow(unused)] #[macro_use] pub extern crate alloc;

#[macro_use] mod macros;
#[macro_use] pub mod driverkit;
pub mod rt;
pub mod sync;
pub mod kmsg;
pub mod mem;
pub mod arch;
pub mod dev;
pub mod sched;
pub mod vfs;

limine! { pub SMPR <= MpRequest: 0 }

lazy_static! {
    static ref SMP: &'static limine::request::Response<limine::request::MpRespData> = SMPR.response().expect("Can't obtain SMP info");
}

fueue! { ARCH_INIT MEM_INIT LATE_INIT DEV_INIT }

entry! {
    for BSP {
        arch::early_init_bs();

        start_aps!();

        arch::init_bsp();

        ARCH_INIT.open();

        mem::init_bsp();

        MEM_INIT.open();

        arch::late_init_bsp();

        arch::late_init();

        let ticks_per_10ms = crate::arch::timer::get_ticks_per_10ms();
        crate::sched::init(ticks_per_10ms);

        LATE_INIT.open();

        dev::init();

        DEV_INIT.open();

        crate::sched::spawn_kernel_task(init_task, crate::sched::task::Priority(0), "init");
    }

    for AP {
        arch::early_init();

        ARCH_INIT.wait();

        arch::init_ap();

        MEM_INIT.wait();

        mem::init_ap();

        LATE_INIT.wait();

        arch::late_init();

        DEV_INIT.wait();
    }
}

fn init_task() {
    crate::info!("init_task started. I am PID 1.");
    
    loop {
        crate::info!("init: waiting for any child to exit...");
        if let Some((id, code)) = crate::sched::wait_any() {
            crate::info!("init: reaped zombie task {:?}, exit code: {}", id, code);
        }
    }
}
