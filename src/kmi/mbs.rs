use core::ptr::addr_of;

use alloc::sync::Arc;
use crate::kmi::kst::KST;
use crate::mem::kdm::Vaddr;
use crate::sched::proc::Process;
use crate::arch::paging::EntryFlags;
use crate::sched::task::TaskId;

pub fn run_module(elf: &[u8]) -> Result<TaskId, usize> {
    debug!("Loading module");
    
    // 1. Parse the ELF file
    let bytes = elf::ElfBytes::<elf::endian::NativeEndian>::minimal_parse(elf).map_err(|_| 1usize)?;
    
    // 2. Calculate the total virtual memory bounds required by all PT_LOAD segments
    let mut min_vaddr = usize::MAX;
    let mut max_vaddr = 0;
    
    if let Some(segments) = bytes.segments() {
        for phdr in segments.iter() {
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
    
    // Align boundaries to page size
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
    
    let hhdm_base = paddr.to_virt().to_raw();
    
    // 4. Load segments into the HHDM region
    unsafe {
        core::ptr::write_bytes(hhdm_base as *mut u8, 0, total_size);
    }
    
    if let Some(segments) = bytes.segments() {
        for phdr in segments.iter() {
            if phdr.p_type == 1 { // PT_LOAD
                let offset_in_module = (phdr.p_vaddr as usize).wrapping_sub(aligned_min);
                let dst_vaddr = hhdm_base.wrapping_add(offset_in_module);
                
                if phdr.p_filesz > 0 {
                    let src = &elf[(phdr.p_offset as usize)..((phdr.p_offset + phdr.p_filesz) as usize)];
                    unsafe {
                        core::ptr::copy_nonoverlapping(src.as_ptr(), dst_vaddr as *mut u8, phdr.p_filesz as usize);
                    }
                }
            }
        }
    }
    
    // 5. Apply Relocations (Handles PIE binaries)
    if let Some(section_headers) = bytes.section_headers() {
        for shdr in section_headers.iter() {
            if shdr.sh_type == 4 { // SHT_RELA
                if let Ok(relas) = bytes.section_data_as_relas(&shdr) {
                    for rela in relas {
                        let offset = rela.r_offset as usize;
                        let dst_vaddr = hhdm_base.wrapping_add(offset);
                        
                        match rela.r_type {
                            8 => { // R_X86_64_RELATIVE (Base + Addend)
                                let new_val = hhdm_base.wrapping_add(rela.r_addend as usize);
                                unsafe { *(dst_vaddr as *mut usize) = new_val; }
                            }
                            1 => { // R_X86_64_64 (Symbol + Addend)
                                // For local symbols in PIE, this effectively acts like RELATIVE
                                let new_val = hhdm_base.wrapping_add(rela.r_addend as usize);
                                unsafe { *(dst_vaddr as *mut usize) = new_val; }
                            }
                            _ => {} // Ignore other types for now
                        }
                    }
                }
            }
        }
    }

    // 6. Reprotect existing HHDM pages if needed
    {
        let mut ptm = crate::mem::PTM.lock();
        if let Some(segments) = bytes.segments() {
            for phdr in segments.iter() {
                if phdr.p_type == 1 { // PT_LOAD
                    let offset_in_module = (phdr.p_vaddr as usize).wrapping_sub(aligned_min);
                    let dst_vaddr = hhdm_base.wrapping_add(offset_in_module);
                    
                    let seg_start = dst_vaddr & !0xFFF;
                    let seg_end = (dst_vaddr + phdr.p_memsz as usize + 0xFFF) & !0xFFF;
                    let size = seg_end - seg_start;
                    
                    let mut flags = EntryFlags::PRESENT | EntryFlags::WRITABLE;
                    if phdr.p_flags & 1 == 0 { // PF_X == 0
                        flags |= EntryFlags::NO_EXECUTE;
                    }
                    
                    let _ = ptm.try_remap(seg_start, size, flags);
                }
            }
        }
    }
    
    let mut entry_vaddr = 0;
    let mut found = false;
    let mut modname = "unknown";
    
    if let Ok(Some((syms, strtab))) = bytes.symbol_table() {
        for sym in syms.iter() {
            if let Ok(name) = strtab.get(sym.st_name as usize) {
                // debug!("+ {:?}", name);
                if name == "module_start" {
                    entry_vaddr = hhdm_base.wrapping_add(sym.st_value as usize);
                    found = true;
                    info!("Resolved entry point symbol '{}' at {:#X}", name, entry_vaddr);
                }
                if name == "SYSTAB" {
                    let vaddr = hhdm_base.wrapping_add(sym.st_value as usize);
                    *Vaddr::from_raw(vaddr).to_ref_mut::<*const super::kst::KeSysTab>() = addr_of!(*KST);
                }
                if name == "MODNAME" {
                    let vaddr = hhdm_base.wrapping_add(sym.st_value as usize);
                    modname = *Vaddr::from_raw(vaddr).to_ref::<&'static str>();
                }
            }
        }
    }
    
    if !found {
        // Fallback to e_entry if the symbol wasn't found
        let entry_offset = (bytes.ehdr.e_entry as usize).wrapping_sub(aligned_min);
        entry_vaddr = hhdm_base.wrapping_add(entry_offset);
        warn!("Entry point symbol not found, falling back to e_entry: {:#X}", entry_vaddr);
    }
    
    if entry_vaddr < hhdm_base || entry_vaddr >= hhdm_base + total_size {
        error!("Module entry point {:#X} is outside loaded bounds", entry_vaddr);
        return Err(4);
    }
    
    let entry_fn: fn() = unsafe { core::mem::transmute(entry_vaddr) };
    
    let proc = Arc::new(Process::new());
    
    let stack_top = crate::sched::allocate_kernel_stack(32 << 10);
    
    let mut task = crate::sched::task::Task::new_kernel(
        entry_fn,
        stack_top,
        crate::sched::task::Priority(0),
        modname,
    );
    
    task.process = proc.clone();
    
    let id = task.id;
    crate::sched::rq::RUNQUEUES[0].lock().spawn_task(task);
    
    info!("Module loaded at HHDM {:#X} (size {} KiB), task ID: {:?}", hhdm_base, total_size / 1024, id);
    
    Ok(id)
}
