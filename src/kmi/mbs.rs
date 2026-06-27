use alloc::sync::Arc;
use crate::sched::proc::Process;
use crate::arch::paging::EntryFlags;

pub fn run_module(elf: &[u8]) -> Result<Arc<Process>, usize> {
    debug!("Loading module (single-alloc HHDM mode)...");
    
    // 1. Parse the ELF file
    let bytes = elf::ElfBytes::<elf::endian::NativeEndian>::minimal_parse(elf).map_err(|_| 1usize)?;
    
    // 2. Calculate the total virtual memory bounds required by all PT_LOAD segments
    let mut min_vaddr = usize::MAX;
    let mut max_vaddr = 0;
    
    for pt in bytes.segments().iter() {
        for phdr in pt.iter() {
            if phdr.p_type == 1 { // PT_LOAD
                let start = phdr.p_vaddr as usize;
                let end = start + phdr.p_memsz as usize;
                if start < min_vaddr { min_vaddr = start; }
                if end > max_vaddr { max_vaddr = end; }
            }
        }
    }
    
    if min_vaddr == usize::MAX {
        error!("No PT_LOAD segments found in module");
        return Err(2);
    }
    
    // Align boundaries to page size to ensure clean mapping
    let aligned_min = min_vaddr & !0xFFF;
    let aligned_max = (max_vaddr + 0xFFF) & !0xFFF;
    let total_size = aligned_max - aligned_min;
    let total_pages = total_size / 4096;
    
    // 3. Single UPA allocation for the entire module
    let paddr = crate::mem::upa::alloc(total_pages);
    if paddr.to_raw() == 0 {
        error!("OOM: Failed to allocate {} pages for module", total_pages);
        return Err(3);
    }
    
    // The unique base offset in HHDM. Because UPA allocates unique physical frames,
    // this HHDM address is guaranteed to be globally unique and will never clash.
    let hhdm_base = paddr.to_virt().to_raw();
    
    // 4. Load segments into the HHDM region
    // Zero out the entire allocation first (handles BSS and gaps between segments)
    unsafe {
        core::ptr::write_bytes(hhdm_base as *mut u8, 0, total_size);
    }
    
    for pt in bytes.segments().iter() {
        for phdr in pt.iter() {
            if phdr.p_type == 1 { // PT_LOAD
                let offset_in_module = (phdr.p_vaddr as usize) - aligned_min;
                let dst_vaddr = hhdm_base + offset_in_module;
                
                if phdr.p_filesz > 0 {
                    let src = &elf[(phdr.p_offset as usize)..((phdr.p_offset + phdr.p_filesz) as usize)];
                    unsafe {
                        core::ptr::copy_nonoverlapping(src.as_ptr(), dst_vaddr as *mut u8, phdr.p_filesz as usize);
                    }
                }
            }
        }
    }
    
    // 5. Reprotect existing HHDM pages if needed
    // HHDM is RW by default. We just update the existing PTEs to add NO_EXECUTE 
    // for non-executable segments, saving memory by not allocating new page tables.
    {
        let mut ptm = crate::mem::PTM.lock();
        for pt in bytes.segments().iter() {
            for phdr in pt.iter() {
                if phdr.p_type == 1 { // PT_LOAD
                    let offset_in_module = (phdr.p_vaddr as usize) - aligned_min;
                    let dst_vaddr = hhdm_base + offset_in_module;
                    let size = (phdr.p_memsz as usize + 0xFFF) & !0xFFF;
                    
                    // Keep HHDM's default PRESENT | WRITABLE, but enforce W^X
                    let mut flags = EntryFlags::PRESENT | EntryFlags::WRITABLE;
                    if phdr.p_flags & 0x1 == 0 { // PF_X == 0
                        flags |= EntryFlags::NO_EXECUTE;
                    }
                    
                    // try_remap walks the existing HHDM page tables and updates the flags.
                    // If it hits a huge page (2M/1G), it will split it only if necessary.
                    let _ = ptm.try_remap(dst_vaddr, size, flags);
                }
            }
        }
    }
    
    // 6. Resolve the Entry Point
    let entry_offset = (bytes.ehdr.e_entry as usize) - aligned_min;
    let entry_vaddr = hhdm_base + entry_offset;
    
    if entry_vaddr < hhdm_base || entry_vaddr >= hhdm_base + total_size {
        error!("Module entry point {:#X} is outside loaded bounds", bytes.ehdr.e_entry);
        return Err(4);
    }
    
    // Cast the HHDM entry point to a function pointer
    let entry_fn: fn(usize) = unsafe { core::mem::transmute(entry_vaddr) };
    // Pass the address of the Kernel System Table (KST) as the first parameter
    let arg = &crate::kmi::kst::KST as *const _ as usize;
    
    // 7. Create a separated process and spawn the task
    let proc = Arc::new(Process::new());
    
    // Allocate a kernel stack for the module task
    let stack_size = 32 * 1024;
    let stack_pages = stack_size / 4096;
    let stack_paddr = crate::mem::upa::alloc(stack_pages);
    if stack_paddr.to_raw() == 0 {
        error!("OOM while allocating module stack");
        return Err(5);
    }
    let stack_top = stack_paddr.to_virt().to_raw() - 1 + stack_size;
    let stack_top = stack_top - stack_top % 8;
    
    let mut task = crate::sched::task::Task::new_kernel_with_arg(
        entry_fn,
        arg,
        stack_top,
        crate::sched::task::Priority(0),
        "km-init"
    );
    
    // Assign the separated process to the task
    task.process = proc.clone();
    
    // Spawn the task on CPU 0
    let id = task.id;
    crate::sched::rq::RUNQUEUES[0].lock().spawn_task(task);
    
    info!("Module loaded at HHDM {:#X} (size {} KiB), task ID: {:?}", hhdm_base, total_size / 1024, id);
    
    Ok(proc)
}
