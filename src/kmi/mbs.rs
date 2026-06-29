use alloc::vec::Vec;
use crate::mem::leak::Leak;
use alloc::borrow::ToOwned;
use alloc::string::{ToString as _, String};
use alloc::sync::Arc;
use crate::mem::kdm::Vaddr;
use crate::sched::proc::Process;
use crate::arch::paging::EntryFlags;
use crate::sched::task::TaskId;

pub struct Module<'a> {
    pub bytes   : &'a [u8],
    pub elf     : elf::ElfBytes<'a, elf::endian::NativeEndian>,
    pub offset  : usize,
    pub entry   : fn(),
    pub name    : &'a str,
    pub size    : usize,
}

impl<'a> Module<'a> {
    #[inline]
    pub fn new(
        bytes: &'a [u8],
        elf: elf::ElfBytes<'a, elf::endian::NativeEndian>,
        offset: usize,
        entry: fn(),
        name: &'a str,
        size: usize,
    ) -> Self { Self { bytes, elf, offset, entry, name, size } }

    pub fn load(elf: &'a [u8]) -> Result<Self, alloc::string::String> {
        let bytes
        =   elf::ElfBytes
        ::  <elf::endian::NativeEndian>
        ::  minimal_parse(elf)
        .   map_err(
            |e|
            e   .to_string()
        )?;

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
        };

        if min_vaddr == usize::MAX { return Err("No PT_LOAD segments found in module".to_owned()) }
        
        // Align boundaries to page size
        let aligned_min = min_vaddr & !0xFFF;
        let aligned_max = (max_vaddr + 0xFFF) & !0xFFF;
        let total_size = aligned_max - aligned_min;
        let total_pages = total_size / 4096;

        let paddr = crate::mem::upa::alloc(total_pages);
        if paddr.to_raw() == 0 { return Err("Out of memory".to_owned()) }

        let hhdm_base = paddr.to_virt().to_raw();

        debug!("hhdm_base {:p}", hhdm_base as *const ());

        unsafe { core::ptr::write_bytes(hhdm_base as *mut u8, 0, total_size); }

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
        };

        let mut entry_vaddr = 0;
        let mut found = false;
        let mut modname = "unknown";
        
        if let Ok(Some((syms, strtab))) = bytes.symbol_table() {
            for sym in syms.iter() {
                if let Ok(dirty_name) = strtab.get(sym.st_name as usize) {
                    let name = dirty_name.trim();
                    if name == "_start" {
                        entry_vaddr = hhdm_base.wrapping_add(sym.st_value as usize);
                        found = true;
                    } else if name == "MODNAME" {
                        modname = *Vaddr
                            ::from_raw(
                                hhdm_base
                                    .wrapping_add(sym.st_value as usize)
                            )
                                .to_ref();
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
            return Err("Module entry point is outside loaded bounds".to_owned())
        }
        
        let entry_fn: fn() = unsafe { core::mem::transmute(entry_vaddr) };

        Ok(Self::new(elf, bytes, hhdm_base, entry_fn, modname, max_vaddr - min_vaddr))
    }

    pub fn symbols(&self) -> Result<(elf::parse::ParsingIterator<'_, elf::endian::LittleEndian, elf::symbol::Symbol>, elf::string_table::StringTable<'_>), alloc::string::String> {
        match self.elf.symbol_table() {
            Ok(Some((syms, strtab))) => Ok((syms.iter(), strtab)),
            Err(e) => Err(e.to_string()),
            Ok(None) => Err("No symbol table".to_string()),
        }
    }

    pub fn getaddr<T>(&self, sym: &elf::symbol::Symbol) -> Vaddr {
        let va = self.offset.wrapping_add(sym.st_value as usize);
        Vaddr::from_raw(va)
    }

    pub fn dive<T>(&self, sym: &elf::symbol::Symbol) -> Option<&mut T> {
        if let Some(va) = self.offset.checked_add(sym.st_value as usize) {
            return Some(Vaddr::from_raw(va).to_ref_mut())
        } 
        None
    }

    pub fn run(&self) -> TaskId {
        let proc = Arc::new(Process::new());
    
        let stack_top = crate::sched::alloc_kestack(32 << 10);
        
        let mut task = crate::sched::task::Task::new_kernel(
            self.entry.clone(),
            stack_top,
            crate::sched::task::Priority(0),
            self.name.to_string(),
        );
        
        task.process = proc.clone();
        
        let id = task.id;
        crate::sched::rq::RUNQUEUES[0].lock().spawn_task(task);
        
        id
    }
}

pub mod safe {
    use super::*;
    
    // compatible with `nk::elf::ElfSym`
    pub type Symbol = elf::symbol::Elf64_Sym;

    pub type ModuleHandle<'a> = Leak<Module<'a>>;

    pub fn load_module(data: &'_[u8]) -> Result<ModuleHandle<'_>, String> {
        match Module::load(data) {
            Ok(m) => Ok(Leak::new(m)),
            Err(e) => Err(e)
        }
    }

    pub fn get_symbols(m: ModuleHandle<'_>) -> Result<Vec<Symbol>, String> {
        match m.symbols() {
            Ok((syms, _)) => {
                let mut rv = Vec::new();
                for s in syms { rv.push( Symbol {
                    st_info: s.st_info,
                    st_name: s.st_name,
                    st_other: s.st_other,
                    st_shndx: s.st_shndx,
                    st_size: s.st_size,
                    st_value: s.st_value,
                }) }
                Ok(rv)
            },
            Err(e) => Err(e),
        }
    }

    pub fn get_string(m: ModuleHandle<'_>, st_name: u32) -> Result<String, String> {
        match m.symbols() {
            Ok((_, strs)) => {
                match strs.get(st_name as usize) {
                    Ok(s) => Ok(s.to_string()),
                    Err(e) => Err(e.to_string()),
                }
            },
            Err(e) => Err(e),
        }
    }

    pub fn sym_get_ptr(m: ModuleHandle<'_>, sym: Symbol) -> *const () {
        m.offset.wrapping_add(sym.st_value as usize) as *const ()
    }

    pub fn run_module(m: ModuleHandle<'_>) -> TaskId {
        m.run()
    }
}
