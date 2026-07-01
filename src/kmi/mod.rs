use core::cell::UnsafeCell;

pub mod mbs;

type SymAddr = usize;

#[derive(Debug, Clone, Copy)] #[allow(unused)]
struct ModSymbol {
    addr: SymAddr,
    vmaj: u32,
    vmin: u32,
}

struct KST (UnsafeCell<alloc::collections::BTreeMap<&'static str, ModSymbol>>);

unsafe impl Sync for KST {}
unsafe impl Send for KST {}

lazy_static! {
    static ref KESYMTAB
    :   KST
    =   KST(UnsafeCell::new(alloc::collections::BTreeMap::new()),);
    // auto_btm!
    // {
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
        // "FsCurrentRoot"           => crate::sched::current_root           as *const () as usize,

        // "MemAlloc"                =>  crate::mem::soa::alloc              as *const () as usize,
        // "MemFree"                 =>  crate::mem::soa::free               as *const () as usize,
        // "MemStack"                =>  crate::sched::alloc_kstack         as *const () as usize,
        // "MemPhysToVirt"           =>  crate::mem::kdm::Paddr::to_virt     as *const () as usize,
        // "MemVirtToPhys"           =>  crate::mem::kdm::Vaddr::to_phys     as *const () as usize,

        // "ModuleLoad"              =>  mbs::safe::load_module              as *const () as usize,
        // "ModuleSymbols"           =>  mbs::safe::get_symbols              as *const () as usize,
        // "ModuleString"            =>  mbs::safe::get_string               as *const () as usize,
        // "ModulePointer"           =>  mbs::safe::sym_get_ptr              as *const () as usize,
        // "ModuleExecute"           =>  mbs::safe::run_module               as *const () as usize,

        // "MonLog"                  =>  crate::kmsg::log  as *const () as usize,
        // "MonAddSink"              =>  crate::kmsg::add                    as *const () as usize,

        // "PagingPap"               => crate::mem::ptm::cur_try_map         as *const () as usize,
        // "PagingRemap"             => crate::mem::ptm::cur_try_remap       as *const () as usize,
        // "PagingUnmap"             => crate::mem::ptm::cur_try_unmap       as *const () as usize,
        // "PagingMerge"             => crate::mem::ptm::merge_range         as *const () as usize,
        // "PagingQuery"             => crate::mem::ptm::cur_query           as *const () as usize,
    // };
}

pub fn init(elf: &[u8]) {
    let kst = unsafe { (*KESYMTAB).0.get().as_mut_unchecked() };

    trace!("Analyzing kernel");
    
    for entt in crate::KMI_TABLE {
        info!("Oh, wow: kernel exports `{}`!", unsafe { entt.2.as_ref_unchecked() });
        kst.insert(
            unsafe { entt.2.as_ref_unchecked() },
            ModSymbol {
                addr: entt.0 as _,
                vmaj: (entt.1 >> 32) as u32,
                vmin: entt.1 as u32 & u32::MAX,
            }
        );
    }

    // parse and load module
    let module = mbs::Module::load(elf).expect("Unable to start initial module");

    trace!("Module loaded at {:p}", module.offset as *const ());

    let (symtab, strtab) = module.symbols().expect("No symtab");

    trace!("Linking module");

    // link it to the kernel
    for sym in symtab {
        if let Ok(name) = strtab.get(sym.st_name as usize) {
            trace!("Found symbol `{}`", name);
            if name.starts_with("Ke") {
                panic!("Module tried to export kernel symbol");
            }

            if name.starts_with("Ki") {
                let name = &name[2..];
                trace!("Linking `{}`", name);
                if kst.contains_key(name) {
                    if let Some(r) = module.dive(&sym) {
                        trace!("Assigning {:p} (address behind {}) to {:?}", r, name, kst[name]);
                        r.0 = kst[name].addr as _;
                    } else {
                        error!("Failed to resolve address of symbol `{}`", name);
                    }
                    trace!("Linked {}", name);
                } else if name.len() > 2 && name.starts_with("Ke") {
                    panic!("Symbol `{}` looks like kernel import, but unknown for kernel", name);
                }
            }

            if name.starts_with("Mi") {
                warn!("Initial module has modular imports which is unexpected");
                warn!("This will be panic later");
            }
        }
    }

    // run module
    let id = module.run();
    trace!("Initial module started with task id {}", id.0);
}
