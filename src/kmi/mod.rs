pub mod mbs;

// here type erasure is safe as we save contract on module's side.
lazy_static! {
    static ref KESYMTAB: alloc::collections::BTreeMap<&'static str, usize> = auto_btm!
    {   // 48 most required kernel functions for modules!
        "k_mon_log"             =>  crate::kmsg::log                    as *const () as usize,
        "k_mon_add_sink"        =>  crate::kmsg::add                    as *const () as usize,

        "k_mem_alloc"           =>  crate::mem::soa::alloc              as *const () as usize,
        "k_mem_free"            =>  crate::mem::soa::free               as *const () as usize,
        "k_mem_alloc_kestack"   =>  crate::sched::alloc_kestack         as *const () as usize,

        "k_event_subscribe"     => crate::ebus::subscribe               as *const () as usize,
        "k_event_unsubscribe"   => crate::ebus::unsubscribe             as *const () as usize,
        "k_event_publish"       => crate::ebus::publish                 as *const () as usize,

        "k_pag_map"             => crate::mem::ptm::cur_try_map         as *const () as usize,
        "k_pag_remap"           => crate::mem::ptm::cur_try_remap       as *const () as usize,
        "k_pag_unmap"           => crate::mem::ptm::cur_try_unmap       as *const () as usize,
        "k_pag_merge"           => crate::mem::ptm::merge_range         as *const () as usize,
        "k_pag_query"           => crate::mem::ptm::cur_query           as *const () as usize,

        "k_mod_load"            =>  mbs::safe::load_module              as *const () as usize,
        "k_mod_getsym"          =>  mbs::safe::get_symbols              as *const () as usize,
        "k_mod_getstr"          =>  mbs::safe::get_string               as *const () as usize,
        "k_mod_getptr"          =>  mbs::safe::sym_get_ptr              as *const () as usize,
        "k_mod_run"             =>  mbs::safe::run_module               as *const () as usize,

        "k_device_add_method"   => crate::dev::Device::add_method       as *const () as usize,
        "k_device_get_method"   => crate::dev::Device::get_method       as *const () as usize,
        "k_device_reg"          => crate::dev::register_device          as *const () as usize,
        "k_device_unreg"        => crate::dev::unregister_device        as *const () as usize,
        "k_device_get"          => crate::dev::get_driver_data          as *const () as usize,
        "k_device_set"          => crate::dev::set_driver_data          as *const () as usize,
        "k_device_use"          => crate::dev::call_method              as *const () as usize,

        "k_exec_panic"          =>  crate::rt::panic::panic             as *const () as usize,
        "k_exec_exit"           =>  crate::sched::exit                  as *const () as usize,
        "k_exec_yield"          =>  crate::sched::yield_now             as *const () as usize,
        "k_exec_sleep"          =>  crate::sched::sleep                 as *const () as usize,
        "k_exec_spawn"          =>  crate::sched::spawn                 as *const () as usize,
        "k_exec_spawn_with_arg" =>  crate::sched::spawn_with_arg        as *const () as usize,
        "k_exec_wait_child"     =>  crate::sched::wait_child            as *const () as usize,
        "k_exec_set_rt_deadline"=>  crate::sched::set_rt_deadline       as *const () as usize,

        "k_vfs_lookup"          =>  crate::vfs::lookup                  as *const () as usize,
        "k_vfs_readdir"         =>  crate::vfs::readdir                 as *const () as usize,
        "k_vfs_read"            =>  crate::vfs::read                    as *const () as usize,
        "k_vfs_write"           =>  crate::vfs::write                   as *const () as usize,
        "k_vfs_trunc"           =>  crate::vfs::truncate                as *const () as usize,
        "k_vfs_link"            =>  crate::vfs::link                    as *const () as usize,
        "k_vfs_unlink"          =>  crate::vfs::unlink                  as *const () as usize,
        "k_vfs_new"             =>  crate::vfs::new                     as *const () as usize,
        "k_vfs_stat"            =>  crate::vfs::stat                    as *const () as usize,
        "k_vfs_ismp"            =>  crate::vfs::is_mount_point          as *const () as usize,
        "k_vfs_resolve"         =>  crate::vfs::resolve                 as *const () as usize,
        "k_vfs_listdir"         =>  crate::vfs::listdir                 as *const () as usize,
        "k_vfs_rdtstr"          =>  crate::vfs::read_to_string          as *const () as usize,
        "k_vfs_regmb"           =>  crate::vfs::register_mblock         as *const () as usize,
        "k_vfs_mount"           =>  crate::vfs::mount                   as *const () as usize,
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
            if KESYMTAB.contains_key(&name) {
                trace!("Linked {}", name);
                *unsafe { module.dive(&sym) } = KESYMTAB[&name];
            }
        }
    }

    // run module
    let _ = module.run();
    trace!("module started");
}
