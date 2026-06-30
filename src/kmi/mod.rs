// use nopaque::*;

use crate::dev::Device;

pub mod mbs;
pub mod front;
pub mod testbox;

use testbox::{Box, boxed};

pub fn kvdn(name: &'static str) -> Box![&Device] {
    trace!("Hey!");
    let device = crate::dev::Device::new(name);
    trace!("Point");
    let rv: Box<_, Device> = <boxed![&Device]>::new(device);
    trace!("Bye!");
    rv
}

// here type erasure is safe as we save contract on module's side.
lazy_static! {
    static ref KESYMTAB: alloc::collections::BTreeMap<&'static str, usize> = auto_btm!
    {
        // "KeTest"                    => front::KeTest                        as *const () as usize,

        "KeVtDeviceNew"             => kvdn       as *const () as usize,

        // "KeDeviceAddMethod"         => crate::dev::Device::add_method       as *const () as usize,
        // "KeDeviceGetMethod"         => crate::dev::Device::get_method       as *const () as usize,
        // "KeDeviceRegister"          => crate::dev::register_device          as *const () as usize,
        // "KeDeviceUnregister"        => crate::dev::unregister_device        as *const () as usize,
        // "KeDeviceDataGet"           => crate::dev::get_driver_data          as *const () as usize,
        // "KeDeviceDataSet"           => crate::dev::set_driver_data          as *const () as usize,
        // "KeDeviceMethodInvoke"      => crate::dev::call_method              as *const () as usize,

        // "KeEventSubscribe"          => crate::ebus::subscribe               as *const () as usize,
        // "KeEventUnsubscribe"        => crate::ebus::unsubscribe             as *const () as usize,
        // "KeEventPublish"            => crate::ebus::publish                 as *const () as usize,

        "KeExecPanic"               =>  crate::rt::panic::panic             as *const () as usize,
        "KeExecExit"                =>  crate::sched::exit                  as *const () as usize,
        "KeExecYield"               =>  crate::sched::yield_now             as *const () as usize,
        // "KeExecSleep"               =>  crate::sched::sleep                 as *const () as usize,
        // "KeExecSpawn"               =>  crate::sched::spawn                 as *const () as usize,
        // "KeExecArgumentedSpawn"     =>  crate::sched::spawn_with_arg        as *const () as usize,
        // "KeExecWaitChild"           =>  crate::sched::wait_child            as *const () as usize,
        // "KeExecSetDeadline"         =>  crate::sched::set_rt_deadline       as *const () as usize,

        // "KeFsLookup"                =>  crate::vfs::lookup                  as *const () as usize,
        // "KeFsReadDir"               =>  crate::vfs::readdir                 as *const () as usize,
        // "KeFsRead"                  =>  crate::vfs::read                    as *const () as usize,
        // "KeFsWrite"                 =>  crate::vfs::write                   as *const () as usize,
        // "KeFsTruncate"              =>  crate::vfs::truncate                as *const () as usize,
        // "KeFsLink"                  =>  crate::vfs::link                    as *const () as usize,
        // "KeFsUnlink"                =>  crate::vfs::unlink                  as *const () as usize,
        // "KeFsObjectNew"             =>  crate::vfs::new                     as *const () as usize,
        // "KeFsObjectStat"            =>  crate::vfs::stat                    as *const () as usize,
        // "KeFsObjectIsMountPoint"    =>  crate::vfs::is_mount_point          as *const () as usize,
        // "KeFsResolve"               =>  crate::vfs::resolve                 as *const () as usize,
        // "KeFsListDir"               =>  crate::vfs::listdir                 as *const () as usize,
        // "KeFsReadToString"          =>  crate::vfs::read_to_string          as *const () as usize,
        // "KeFsMetaBlockRegister"     =>  crate::vfs::register_mblock         as *const () as usize,
        // "KeFsMount"                 =>  crate::vfs::mount                   as *const () as usize,
        // "KeFsCurrentRoot"           => crate::sched::current_root           as *const () as usize,

        "KeMemAlloc"                =>  crate::mem::soa::alloc              as *const () as usize,
        "KeMemFree"                 =>  crate::mem::soa::free               as *const () as usize,
        "KeMemStack"                =>  crate::sched::alloc_kstack         as *const () as usize,
        "KeMemPhysToVirt"           =>  crate::mem::kdm::Paddr::to_virt     as *const () as usize,
        "KeMemVirtToPhys"           =>  crate::mem::kdm::Vaddr::to_phys     as *const () as usize,

        // "KeModuleLoad"              =>  mbs::safe::load_module              as *const () as usize,
        // "KeModuleSymbols"           =>  mbs::safe::get_symbols              as *const () as usize,
        // "KeModuleString"            =>  mbs::safe::get_string               as *const () as usize,
        // "KeModulePointer"           =>  mbs::safe::sym_get_ptr              as *const () as usize,
        // "KeModuleExecute"           =>  mbs::safe::run_module               as *const () as usize,

        "KeMonLog"                  =>  crate::kmsg::log                    as *const () as usize,
        // "KeMonAddSink"              =>  crate::kmsg::add                    as *const () as usize,

        // "KePagingPap"               => crate::mem::ptm::cur_try_map         as *const () as usize,
        // "KePagingRemap"             => crate::mem::ptm::cur_try_remap       as *const () as usize,
        // "KePagingUnmap"             => crate::mem::ptm::cur_try_unmap       as *const () as usize,
        // "KePagingMerge"             => crate::mem::ptm::merge_range         as *const () as usize,
        // "KePagingQuery"             => crate::mem::ptm::cur_query           as *const () as usize,
    };
}

pub fn init(elf: &[u8]) {
    // parse and load module
    let module = mbs::Module::load(elf).expect("Unable to start bootstrap module");

    trace!("Module loaded");

    let (symtab, strtab) = module.symbols().expect("No symtab");

    // link it to the kernel
    for sym in symtab {
        if let Ok(name) = strtab.get(sym.st_name as usize) {
            trace!("Found symbol `{}`", name);
            if KESYMTAB.contains_key(&name) {
                trace!("Linking `{}` -> {:p}...", name, KESYMTAB[&name] as *const ());
                if let Some(r) = module.dive(&sym) {
                    *r = KESYMTAB[&name];
                } else {
                    error!("Failed to resolve address of symbol `{}`", name);
                }
                trace!("Linked {}", name);
            } else if name.len() > 2 && name.starts_with("Ke") {
                warn!("Symbol `{}` looks like Kexport, but unknown for kernel", name);
            }
        }
    }

    // run module
    let id = module.run();
    trace!("module started with task id {}", id.0);
}
