use core::cell::UnsafeCell;

use alloc::string::{String, ToString};

pub mod mbs;

type SymAddr = usize;

#[derive(Debug, Clone, Copy)] #[allow(unused)]
struct ModSymbol {
    addr: SymAddr,
    vmaj: u32,
    vmin: u32,
}

/// Dirty hack to make UnsafeCell compatible implement Sync + Send.
/// 
/// Note: this struct isn't `pub` intentially. It MUST NOT e accesed
/// from other module or other thread concurrently.
struct KST (UnsafeCell<alloc::collections::BTreeMap<String, ModSymbol>>);
unsafe impl Sync for KST {} unsafe impl Send for KST {}

lazy_static! {
    static ref KESYMTAB: KST
    =   KST(UnsafeCell::new(alloc::collections::BTreeMap::new()),);
}

/// Initialize KMI and start initial module.
/// 
/// Note: this function MUST be called only once.
pub fn init(initm_bytes: &'static [u8]) {
    let kst;

    // safety: access only through this ref and already initialized (thanks lazy_static)
    unsafe {
        kst = (*KESYMTAB).0.get().as_mut_unchecked();
    }

    // collect kernel symbols
    for entt in crate::KMI_TABLE {
        kst.insert(
            entt.name().to_string(),
            ModSymbol {
                addr: entt.address(),
                vmaj: entt.version().0,
                vmin: entt.version().1,
            }
        );
    }

    // parse and load initial module
    let module = mbs::Module::load(initm_bytes).expect("Unable to start initial module");

    trace!("Module loaded at {:p}", module.offset as *const ());

    let (symtab, strtab) = module.symbols().expect("No symtab");

    // link it to the kernel
    for sym in symtab {
        if let Ok(name) = strtab.get(sym.st_name as usize) {
            if name.starts_with("Ke") { panic!("Module tried to export kernel symbol") }
            if name.starts_with("Ki") {
                let name = &name[2..];

                if kst.contains_key(name) {
                    if let Some(r) = module.dive(&sym) {
                        if kst[name].vmaj != r.version().0 || kst[name].vmin < r.version().1 {
                            panic!("Incompatible initial module");
                        }
                        // Ok
                        r.0 = kst[name].addr as _;
                    }
                    else { panic!("Failed to resolve address of symbol `{}`", name) }
                } else if name.len() > 2 && name.starts_with("Ke") {
                    panic!("Symbol `{}` looks like kernel import, but unknown for kernel", name);
                }
            }

            if name.starts_with("Me") {
                // there can't be any conflicts on initial module loading
                if let Some(r) = module.dive(&sym) {
                    // `let _ =`: always None here
                    let _ = kst.insert(
                        name.to_string(),
                        ModSymbol {
                            addr: r.address(),
                            vmaj: r.version().0,
                            vmin: r.version().1,
                        }
                    );
                }
            }

            if name.starts_with("Mi") {
                panic!("Initial module has modular imports")
            }
        }
    }

    // run initial module
    let id = module.run();
    trace!("Initial module started with task id {}", id.0);
}

// "DeviceAddMethod"         => crate::dev::Device::add_method       as *const () as usize,
// "DeviceGetMethod"         => crate::dev::Device::get_method       as *const () as usize,
// "DeviceRegister"          => crate::dev::register_device          as *const () as usize,
// "DeviceUnregister"        => crate::dev::unregister_device        as *const () as usize,
// "DeviceDataGet"           => crate::dev::get_driver_data          as *const () as usize,
// "DeviceDataSet"           => crate::dev::set_driver_data          as *const () as usize,
// "DeviceMethodInvoke"      => crate::dev::call_method              as *const () as usize,

// "EventSubscribe"          => crate::ebus::subscribe               as *const () as usize,
// "EventUnsubscribe"        => crate::ebus::unsubscribe             as *const () as usize,
// "EventPublish"            => crate::ebus::publish                 as *const () as usize,

// "ExecPanic"               =>  crate::rt::panic::panic             as *const () as usize,
// "ExecExit"                =>  crate::sched::exit                  as *const () as usize,
// "ExecYield"               =>  crate::sched::yield_now             as *const () as usize,
// "ExecSleep"               =>  crate::sched::sleep                 as *const () as usize,
// "ExecSpawn"               =>  crate::sched::spawn                 as *const () as usize,
// "ExecArgumentedSpawn"     =>  crate::sched::spawn_with_arg        as *const () as usize,
// "ExecWaitChild"           =>  crate::sched::wait_child            as *const () as usize,
// "ExecSetDeadline"         =>  crate::sched::set_rt_deadline       as *const () as usize,

// "FsLookup"                =>  crate::vfs::lookup                  as *const () as usize,
// "FsReadDir"               =>  crate::vfs::readdir                 as *const () as usize,
// "FsRead"                  =>  crate::vfs::read                    as *const () as usize,
// "FsWrite"                 =>  crate::vfs::write                   as *const () as usize,
// "FsTruncate"              =>  crate::vfs::truncate                as *const () as usize,
// "FsLink"                  =>  crate::vfs::link                    as *const () as usize,
// "FsUnlink"                =>  crate::vfs::unlink                  as *const () as usize,
// "FsObjectNew"             =>  crate::vfs::new                     as *const () as usize,
// "FsObjectStat"            =>  crate::vfs::stat                    as *const () as usize,
// "FsObjectIsMountPoint"    =>  crate::vfs::is_mount_point          as *const () as usize,
// "FsResolve"               =>  crate::vfs::resolve                 as *const () as usize,
// "FsListDir"               =>  crate::vfs::listdir                 as *const () as usize,
// "FsReadToString"          =>  crate::vfs::read_to_string          as *const () as usize,
// "FsMetaBlockRegister"     =>  crate::vfs::register_mblock         as *const () as usize,
// "FsMount"                 =>  crate::vfs::mount                   as *const () as usize,
// "FsCurrentRoot"           =>  crate::sched::current_root          as *const () as usize,

// "MemAlloc"                =>  crate::mem::soa::alloc              as *const () as usize,
// "MemFree"                 =>  crate::mem::soa::free               as *const () as usize,
// "MemStack"                =>  crate::sched::alloc_kstack          as *const () as usize,
// "MemPhysToVirt"           =>  crate::mem::kdm::Paddr::to_virt     as *const () as usize,
// "MemVirtToPhys"           =>  crate::mem::kdm::Vaddr::to_phys     as *const () as usize,

// "ModuleLoad"              =>  mbs::safe::load_module              as *const () as usize,
// "ModuleSymbols"           =>  mbs::safe::get_symbols              as *const () as usize,
// "ModuleString"            =>  mbs::safe::get_string               as *const () as usize,
// "ModulePointer"           =>  mbs::safe::sym_get_ptr              as *const () as usize,
// "ModuleExecute"           =>  mbs::safe::run_module               as *const () as usize,

// "MonLog"                  =>  crate::kmsg::log                    as *const () as usize,
// "MonAddSink"              =>  crate::kmsg::add                    as *const () as usize,

// "PagingPap"               => crate::mem::ptm::cur_try_map         as *const () as usize,
// "PagingRemap"             => crate::mem::ptm::cur_try_remap       as *const () as usize,
// "PagingUnmap"             => crate::mem::ptm::cur_try_unmap       as *const () as usize,
// "PagingMerge"             => crate::mem::ptm::merge_range         as *const () as usize,
// "PagingQuery"             => crate::mem::ptm::cur_query           as *const () as usize,
