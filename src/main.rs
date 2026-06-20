#![no_std]
#![no_main]

#![feature(unsafe_cell_access)]
#![feature(abi_x86_interrupt)]
#![feature(const_trait_impl)]
#![feature(likely_unlikely)]
#![feature(const_destruct)]
#![feature(const_cmp)]

#![cfg_attr(not(debug_assertions), allow(unused_assignments))]

use alloc::string::ToString;

use crate::sched::current_process;

#[allow(unused)] #[macro_use] pub extern crate extrum;
#[allow(unused)] #[macro_use] pub extern crate bitflags;
#[allow(unused)] #[macro_use] pub extern crate lazy_static;
#[allow(unused)] #[macro_use] pub extern crate alloc;

#[macro_use] mod macros;
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

        LATE_INIT.open();

        dev::init();

        DEV_INIT.open();

        let ticks_per_10ms = crate::arch::timer::get_ticks_per_10ms();
        crate::sched::init(ticks_per_10ms);

        crate::sched::spawn_kernel_task(reaper, crate::sched::task::Priority(-1), "reaper", Some(vfs::RootRef::new(vfs::RootReg::new())));
        crate::sched::spawn_kernel_task(test, crate::sched::task::Priority(0), "test", Some(vfs::RootRef::new(vfs::RootReg::new())));

        // sched::exit(0);
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

        // sched::exit(0);
    }
}

fn test() {
    let roots = current_process().expect("NOPID").roots.clone();
    debug!("Got roots");
    let inode = vfs::Inode::new();
    inode.vtable = &vfs::PVFS_VTABLE;
    debug!("Inode created");
    if let Err(e) = roots.add_new_root("pv".to_string(), inode.id) { panic!("{:?}", e); }
    debug!("Root created");
    if let Err(e) = (inode.vtable.new)(inode, vfs::Kind::File) { panic!("{:?}", e); }
    debug!("File created");
    if let Err(e) = (inode.vtable.write)(inode, 0, &[b'[', b'N', b'O', b'T', b' ', b'F', b'A', b'I', b'L', b'E', b'D', b']']) { panic!("{:?}", e); }
    debug!("File written");
    let mut buf: &mut [u8] = &mut [b'[', b'F', b'A', b'I', b'L', b'E', b'D', b']', b' ', b' ', b' ', b' '];
    if let Err(e) = (inode.vtable.read)(inode, 0, &mut buf) { panic!("{:?}", e); }
    debug!("vfs test: {}", str::from_utf8(buf).unwrap());
    sched::exit(0);
}

fn reaper() {
    loop {
        crate::info!("waiting for any child to exit...");
        if let Some((id, code)) = crate::sched::wait_any() {
            crate::info!("reaped zombie task {:?}, exit code: {}", id, code);
        }
    }
}
