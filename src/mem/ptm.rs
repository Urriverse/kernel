use heapless::Vec;

use crate::arch::paging::{Area, Entry, EntryFlags, Exco, is_huge, tab_from_entry, walk_entry, walk_entry_mut};
use crate::mem::kdm::Paddr;
use crate::mem::pmr::{self, Kind};

fn is_phys_range_contiguous(paddr: Paddr, size: usize) -> bool {
    let start = paddr.to_raw();
    let end = start + size;

    for region in pmr::iter() {
        match region.kind {
            Kind::USABLE | Kind::KERNEL | Kind::BOOTLOADER => {
                let reg_start = region.base;
                let reg_end = region.base + region.len;
                if start >= reg_start && end <= reg_end {
                    return true;
                }
            }
            _ => continue,
        }
    }
    warn!(
        "Physical range {:#X}..{:#X} is not contiguous in a usable region",
        start, end
    );
    false
}

fn is_phys_contiguous(paddr: Paddr, page_size: usize) -> bool {
    is_phys_range_contiguous(paddr, page_size)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PageSize {
    Size4K,
    Size2M,
    Size1G,
}

impl PageSize {
    fn bytes(self) -> usize {
        match self {
            PageSize::Size4K => 4096,
            PageSize::Size2M => 2 << 20,
            PageSize::Size1G => 1 << 30,
        }
    }
}

fn select_page_size(vaddr: usize, paddr: Paddr, size: usize) -> PageSize {
    let align_1g = 1_usize << 30;
    let align_2m = 2_usize << 20;

    if size >= align_1g
        && vaddr & (align_1g - 1) == 0
        && paddr.to_raw() & (align_1g - 1) == 0
        && is_phys_contiguous(paddr, align_1g)
    {
        PageSize::Size1G
    } else if size >= align_2m
        && vaddr & (align_2m - 1) == 0
        && paddr.to_raw() & (align_2m - 1) == 0
        && is_phys_contiguous(paddr, align_2m)
    {
        PageSize::Size2M
    } else {
        PageSize::Size4K
    }
}

pub struct Polen {
    pub exco: Exco,
}

impl Polen {
    pub fn new() -> Self {
        debug!("Polen::new");
        Polen { exco: Exco::new() }
    }

    pub const fn from_exco(exco: Exco) -> Self {
        Polen { exco }
    }

    pub fn reference() -> Self {
        info!("Polen::reference: creating shallow copy of current tables (new PML4 only)");
        let current = Exco::current();

        let new_root = crate::arch::paging::alloc_tab_zeroed();
        let new_cr3 = crate::mem::kdm::Vaddr::from_ref(new_root).to_phys().to_raw() as u64;

        for i in 0..512 {
            new_root.0[i] = current.root.0[i];
        }

        for i in 0..256 {
            new_root.0[i] = Entry::default();
        }

        let new_exco = Exco {
            cr3: new_cr3,
            root: new_root,
            owned: true,
        };

        info!("Polen::reference: new CR3 {:#018X} (shallow, lower half cleared)", new_exco.cr3);
        Polen { exco: new_exco }
    }

    pub fn map(&mut self, vaddr: usize, paddr: Paddr, size: usize, flags: EntryFlags) {
        self.try_map(vaddr, paddr, size, flags)
            .expect("Polen::map failed");
    }

    pub fn try_map(
        &mut self,
        vaddr: usize,
        paddr: Paddr,
        mut size: usize,
        flags: EntryFlags,
    ) -> Result<(), &'static str> {
        if vaddr & 0xfff != 0 {
            return Err("misaligned virtual address");
        }
        if size == 0 {
            return Err("zero-size mapping");
        }

        trace!("try_map(self, {:p}, {:p}, {}, {:?})", vaddr as *const (), paddr.to_raw() as *const (), size, flags);

        let aligned_size = (size + 4095) & !4095;
        if aligned_size != size {
            warn!(
                "Polen::map: size {:#X} not page-aligned, rounding up to {:#X}",
                size, aligned_size
            );
            size = aligned_size;
        }

        const ALIGN_2M: usize = 2 << 20;
        let first_2m_aligned = (vaddr + ALIGN_2M - 1) & !(ALIGN_2M - 1);
        let head_size = if first_2m_aligned > vaddr {
            let hs = first_2m_aligned - vaddr;
            if hs > size { size } else { hs }
        } else {
            0
        };

        let last_2m_aligned = (vaddr + size) & !(ALIGN_2M - 1);
        let tail_size = if last_2m_aligned > vaddr && last_2m_aligned < vaddr + size {
            (vaddr + size) - last_2m_aligned
        } else {
            0
        };

        let middle_start = vaddr + head_size;
        let middle_size = size - head_size - tail_size;

        if head_size > 0 {
            let mut head_vaddr = vaddr;
            let mut head_paddr = paddr;
            let head_bytes = head_size;
            let mut remaining = head_bytes;
            while remaining > 0 {
                let step = 4096;
                self.map_4k_block(head_vaddr, head_paddr, flags)?;
                head_vaddr += step;
                head_paddr = Paddr::from_raw(head_paddr.to_raw() + step);
                remaining -= step;
            }
        }

        if middle_size > 0 {
            let mut mid_vaddr = middle_start;
            let mut mid_paddr = Paddr::from_raw(paddr.to_raw() + head_size);
            let mut mid_rem = middle_size;
            while mid_rem > 0 {
                let ps = select_page_size(mid_vaddr, mid_paddr, mid_rem);
                let ps_bytes = ps.bytes();
                // Step is min(ps_bytes, mid_rem) – but for huge pages we require full size.
                let step = if ps_bytes > mid_rem { mid_rem } else { ps_bytes };

                match ps {
                    PageSize::Size1G => {
                        if step == ps_bytes {
                            if let Ok(entry) = walk_entry_mut(&mut self.exco.root, mid_vaddr, 2, false) {
                                if entry.is_present() && is_huge(entry) {
                                    debug!("  split 1G (blocking 1G map)");
                                    split_and_retry(self, mid_vaddr, ps)?;
                                    continue;
                                }
                            }
                            self.exco.try_map1g(mid_vaddr, mid_paddr, flags)?;
                            mid_vaddr += step;
                            mid_paddr = Paddr::from_raw(mid_paddr.to_raw() + step);
                            mid_rem -= step;
                            continue;
                        }
                    }
                    PageSize::Size2M => {
                        if step == ps_bytes {
                            if let Ok(entry) = walk_entry_mut(&mut self.exco.root, mid_vaddr, 1, false) {
                                if entry.is_present() && is_huge(entry) {
                                    debug!("  split 2M (blocking 2M map)");
                                    split_and_retry(self, mid_vaddr, ps)?;
                                    continue;
                                }
                            }
                            self.exco.try_map2m(mid_vaddr, mid_paddr, flags)?;
                            mid_vaddr += step;
                            mid_paddr = Paddr::from_raw(mid_paddr.to_raw() + step);
                            mid_rem -= step;
                            continue;
                        }
                    }
                    PageSize::Size4K => {
                        let step_4k = if step > 4096 { 4096 } else { step };
                        self.map_4k_block(mid_vaddr, mid_paddr, flags)?;
                        mid_vaddr += step_4k;
                        mid_paddr = Paddr::from_raw(mid_paddr.to_raw() + step_4k);
                        mid_rem -= step_4k;
                        continue;
                    }
                }

                let step_4k = if step > 4096 { 4096 } else { step };
                self.map_4k_block(mid_vaddr, mid_paddr, flags)?;
                mid_vaddr += step_4k;
                mid_paddr = Paddr::from_raw(mid_paddr.to_raw() + step_4k);
                mid_rem -= step_4k;
            }
        }

        if tail_size > 0 {
            let mut tail_vaddr = last_2m_aligned;
            let mut tail_paddr = Paddr::from_raw(paddr.to_raw() + size - tail_size);
            let mut tail_rem = tail_size;
            while tail_rem > 0 {
                let step = 4096;
                self.map_4k_block(tail_vaddr, tail_paddr, flags)?;
                tail_vaddr += step;
                tail_paddr = Paddr::from_raw(tail_paddr.to_raw() + step);
                tail_rem -= step;
            }
        }

        Ok(())
    }

    pub fn map_4k_block(
        &mut self,
        vaddr: usize,
        paddr: Paddr,
        flags: EntryFlags,
    ) -> Result<(), &'static str> {
        loop {
            match self.exco.try_map4k(vaddr, paddr, flags) {
                Ok(_) => return Ok(()),
                Err(_) => {
                    if let Ok(_entry) = walk_entry_mut(&mut self.exco.root, vaddr, 1, false) {
                        let base = vaddr & !(0x1f_ffff);
                        debug!("  map_4k_block: split 2M at {:#X}", base);
                        self.exco.try_split2m(base)?;
                    } else if let Ok(_entry) = walk_entry_mut(&mut self.exco.root, vaddr, 2, false) {
                        let base = vaddr & !(0x3fff_ffff);
                        debug!("  map_4k_block: split 1G at {:#X}", base);
                        self.exco.try_split1g(base)?;
                    } else {
                        return Err("map_4k_block: cannot resolve blocking page");
                    }
                }
            }
        }
    }

    pub fn remap(&mut self, vaddr: usize, size: usize, new_flags: EntryFlags) {
        self.try_remap(vaddr, size, new_flags)
            .expect("Polen::remap failed");
    }

    pub fn try_remap(
        &mut self,
        mut vaddr: usize,
        mut size: usize,
        new_flags: EntryFlags,
    ) -> Result<(), &'static str> {
        if vaddr & 0xfff != 0 || size == 0 {
            return Err("misaligned or zero-size remap");
        }

        debug!("Polen::remap [ {:#X} .. {:#X} ) -> flags {:?}", vaddr, vaddr + size, new_flags);

        while size > 0 {
            if let Ok(entry) = walk_entry_mut(&mut self.exco.root, vaddr, 2, false) {
                if entry.is_present() && is_huge(entry) {
                    let base = vaddr & !(0x3fff_ffff);
                    debug!("  remap: split 1G at {:#X}", base);
                    self.exco.try_split1g(base)?;
                    continue;
                }
            }
            if let Ok(entry) = walk_entry_mut(&mut self.exco.root, vaddr, 1, false) {
                if entry.is_present() && is_huge(entry) {
                    let base = vaddr & !(0x1f_ffff);
                    debug!("  remap: split 2M at {:#X}", base);
                    self.exco.try_split2m(base)?;
                    continue;
                }
            }

            let entry = walk_entry_mut(&mut self.exco.root, vaddr, 0, false)?;
            if !entry.is_present() {
                return Err("address not mapped");
            }
            let paddr = entry.address();
            *entry = Entry::new(paddr, new_flags | EntryFlags::PRESENT);
            crate::arch::paging::flush_tlb(vaddr);

            vaddr += 4096;
            size = size.saturating_sub(4096);
        }

        Ok(())
    }

    pub fn unmap(&mut self, vaddr: usize, size: usize) {
        self.try_unmap(vaddr, size).expect("Polen::unmap failed");
    }

    pub fn try_unmap(
        &mut self,
        mut vaddr: usize,
        mut size: usize,
    ) -> Result<(), &'static str> {
        if vaddr & 0xfff != 0 || size == 0 {
            return Err("misaligned or zero-size unmap");
        }

        info!("Polen::unmap [ {:#X} .. {:#X} ) ({} bytes)", vaddr, vaddr + size, size);

        while size > 0 {
            if let Ok(entry) = walk_entry_mut(&mut self.exco.root, vaddr, 2, false) {
                if entry.is_present() && is_huge(entry) {
                    let base = vaddr & !(0x3fff_ffff);
                    if base != vaddr || size < (1 << 30) {
                        debug!("  unmap: partial split 1G at {:#X}", base);
                        self.exco.try_split1g(base)?;
                        continue;
                    }
                }
            }
            if let Ok(entry) = walk_entry_mut(&mut self.exco.root, vaddr, 1, false) {
                if entry.is_present() && is_huge(entry) {
                    let base = vaddr & !(0x1f_ffff);
                    if base != vaddr || size < (2 << 20) {
                        debug!("  unmap: partial split 2M at {:#X}", base);
                        self.exco.try_split2m(base)?;
                        continue;
                    }
                }
            }

            self.exco.try_unmap(vaddr)?;

            vaddr += 4096;
            size = size.saturating_sub(4096);
        }

        Ok(())
    }

    pub fn merge_range(&mut self, start: usize, size: usize) {
        let end = start + size;
        debug!("Polen::merge_range [ {:#X} .. {:#X} )", start, end);

        if size < (2 << 20) {
            return;
        }

        let two_m = 2 << 20;
        let two_m_mask = !(two_m - 1);
        let first_2m = (start + two_m - 1) & two_m_mask;
        let last_2m = (end - 1) & two_m_mask;

        let mut vaddr = first_2m;
        while vaddr <= last_2m {
            let _ = self.exco.try_merge2m(vaddr);
            vaddr += two_m;
        }

        if size >= (1 << 30) {
            let one_g = 1 << 30;
            let one_g_mask = !(one_g - 1);
            let first_1g = (start + one_g - 1) & one_g_mask;
            let last_1g = (end - 1) & one_g_mask;

            let mut vaddr_gb = first_1g;
            while vaddr_gb <= last_1g {
                let _ = self.exco.try_merge1g(vaddr_gb);
                vaddr_gb += one_g;
            }
        }
    }

    pub fn query(&self, vaddr: usize) -> Option<(Paddr, EntryFlags)> {
        trace!("Polen::query {:#X}", vaddr);

        if let Ok(entry) = walk_entry(&self.exco.root, vaddr, 0) {
            if entry.is_present() {
                let offset = vaddr & 0xfff;
                let paddr = Paddr::from_raw(entry.address().to_raw() + offset);
                trace!("  -> 4K page: phys {:#016X} flags {:?}", paddr.to_raw(), entry.flags());
                return Some((paddr, entry.flags()));
            }
        }

        if let Ok(entry) = walk_entry(&self.exco.root, vaddr, 1) {
            if entry.is_present() && is_huge(entry) {
                let offset = vaddr & 0x1f_ffff;
                let paddr = Paddr::from_raw(entry.address().to_raw() + offset);
                trace!("  -> 2M page: phys {:#016X} flags {:?}", paddr.to_raw(), entry.flags());
                return Some((paddr, entry.flags()));
            }
        }

        if let Ok(entry) = walk_entry(&self.exco.root, vaddr, 2) {
            if entry.is_present() && is_huge(entry) {
                let offset = vaddr & 0x3fff_ffff;
                let paddr = Paddr::from_raw(entry.address().to_raw() + offset);
                trace!("  -> 1G page: phys {:#016X} flags {:?}", paddr.to_raw(), entry.flags());
                return Some((paddr, entry.flags()));
            }
        }

        trace!("  -> not mapped");
        None
    }

    pub fn report<const N: usize>(&self) -> Vec<Area, N> {
        self.exco.report()
    }

    #[inline(always)]
    pub unsafe fn activate(&self) {
        unsafe {
            self.exco.activate()
        }
    }

    pub fn mark_user_cow(&mut self) {
        for pml4_idx in 0..256 {
            let pml4_entry = &mut self.exco.root.0[pml4_idx];
            if !pml4_entry.is_present() || is_huge(pml4_entry) { continue; }
            
            let pdpt = tab_from_entry(pml4_entry);
            for pdpt_idx in 0..512 {
                let pdpt_entry = &mut pdpt.0[pdpt_idx];
                if !pdpt_entry.is_present() || is_huge(pdpt_entry) { continue; }
                
                let pd = tab_from_entry(pdpt_entry);
                for pd_idx in 0..512 {
                    let pd_entry = &mut pd.0[pd_idx];
                    if !pd_entry.is_present() || is_huge(pd_entry) { continue; }
                    
                    let pt = tab_from_entry(pd_entry);
                    for pt_idx in 0..512 {
                        let pt_entry = &mut pt.0[pt_idx];
                        // Если страница присутствует и доступна для записи
                        if pt_entry.is_present() && pt_entry.is_writable() {
                            let mut flags = pt_entry.flags();
                            flags.remove(EntryFlags::WRITABLE);
                            flags.insert(EntryFlags::COPY_ON_WRITE);
                            pt_entry.set_flags(flags);
                        }
                    }
                }
            }
        }
        unsafe { self.exco.activate(); } 
    }
}

fn split_and_retry(polen: &mut Polen, vaddr: usize, ps: PageSize) -> Result<(), &'static str> {
    match ps {
        PageSize::Size1G => {
            polen.exco.try_split1g(vaddr)?;
        }
        PageSize::Size2M => {
            polen.exco.try_split2m(vaddr)?;
        }
        PageSize::Size4K => {}
    }
    Ok(())
}
