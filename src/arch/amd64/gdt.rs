use super::MAX_CPUS;

pub const KERNEL_CODE_SELECTOR: u16 = 0x08;
pub const KERNEL_DATA_SELECTOR: u16 = 0x10;
pub const USER_CODE_SELECTOR: u16 = 0x18;
pub const USER_DATA_SELECTOR: u16 = 0x20;

#[inline]
pub const fn tss_selector(cpu_id: usize) -> u16 {
    (0x28 + (cpu_id * 16)) as u16
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Tss {
    pub reserved_1: u32,
    pub rsp0: u64,
    pub rsp1: u64,
    pub rsp2: u64,
    pub reserved_2: u64,
    pub ist1: u64,
    pub ist2: u64,
    pub ist3: u64,
    pub ist4: u64,
    pub ist5: u64,
    pub ist6: u64,
    pub ist7: u64,
    pub reserved_3: u64,
    pub reserved_4: u16,
    pub iomap_base: u16,
}

#[repr(C, align(8))]
pub struct Gdt {
    entries: [u64; 5 + (MAX_CPUS * 2)],
}

#[repr(C, packed)]
pub struct Dtr {
    pub limit: u16,
    pub base: u64,
}

impl Gdt {
    pub const fn new() -> Self {
        let mut entries = [0u64; 5 + (MAX_CPUS * 2)];
        
        // 0. Null descriptor
        entries[0] = 0x0000000000000000;
        
        // 1. Kernel Code (64-bit, present, ring 0, readable)
        // Base: 0, Limit: 0, Access: 0x9A (1001 1010), Flags: 0x20 (L=1)
        entries[1] = 0x00209A0000000000;
        
        // 2. Kernel Data (64-bit, present, ring 0, writable)
        // Base: 0, Limit: 0, Access: 0x92 (1001 0010), Flags: 0x00
        entries[2] = 0x0000920000000000;
        
        // 3. User Code (64-bit, present, ring 3, readable)
        // Base: 0, Limit: 0, Access: 0xFA (1111 1010), Flags: 0x20 (L=1)
        entries[3] = 0x0020FA0000000000;
        
        // 4. User Data (64-bit, present, ring 3, writable)
        // Base: 0, Limit: 0, Access: 0xF2 (1111 0010), Flags: 0x00
        entries[4] = 0x0000F20000000000;
        
        Self { entries }
    }

    pub unsafe fn load(&'static self) {
        let dtr = Dtr {
            limit: (core::mem::size_of_val(&self.entries) - 1) as u16,
            base: self as *const _ as u64,
        };

        unsafe {
            core::arch::asm!(
                "lgdt [{0}]",
                in(reg) &dtr,
                options(readonly, nostack, preserves_flags)
            );

            Self::reload_segments();
        }
    }

    #[inline]
    unsafe fn reload_segments() {
        unsafe {
            core::arch::asm!(
                "mov ds, {0:x}",
                "mov es, {0:x}",
                "mov ss, {0:x}",
                "mov fs, {0:x}",
                "mov gs, {0:x}",
                
                "push {1:r}",
                "lea {2}, [rip + 2f]",
                "push {2}",
                "retfq",
                "2:",
                in(reg) KERNEL_DATA_SELECTOR,
                in(reg) KERNEL_CODE_SELECTOR,
                out(reg) _,
                options(nostack, preserves_flags)
            );
        }
    }

    pub fn set_tss(&mut self, cpu_id: usize, tss_ptr: *const Tss) {
        assert!(cpu_id < MAX_CPUS, "CPU ID {} exceeds MAX_CPUS ({})", cpu_id, MAX_CPUS);
        
        let base = tss_ptr as u64;
        let limit = (core::mem::size_of::<Tss>() - 1) as u64;
        
        let access: u64 = 0x89;
        let flags: u64 = 0x00; // G=0 (limit in bytes), L=0

        let low = (limit & 0xFFFF) 
                | ((base & 0xFFFFFF) << 16) 
                | ((access & 0xFF) << 40) 
                | (((limit >> 16) & 0xF) << 48) 
                | ((flags & 0xF) << 52) 
                | ((base >> 24) << 56);
                
        let high = base >> 32;

        let tss_idx = 5 + (cpu_id * 2);
        self.entries[tss_idx] = low;
        self.entries[tss_idx + 1] = high;
    }
}

pub static mut GLOBAL_GDT: Gdt = Gdt::new();

static mut TSS_TABLE: [Tss; MAX_CPUS] = [Tss {
    reserved_1: 0, rsp0: 0, rsp1: 0, rsp2: 0, reserved_2: 0,
    ist1: 0, ist2: 0, ist3: 0, ist4: 0, ist5: 0, ist6: 0, ist7: 0,
    reserved_3: 0, reserved_4: 0, iomap_base: 0,
}; MAX_CPUS];

pub fn set_kernel_stack(cpu_id: usize, stack_top: u64) {
    assert!(cpu_id < MAX_CPUS, "CPU ID {} exceeds MAX_CPUS", cpu_id);
    unsafe {
        TSS_TABLE[cpu_id].rsp0 = stack_top;
    }
}

pub fn set_ist(cpu_id: usize, ist_index: usize, stack_top: u64) {
    assert!(cpu_id < MAX_CPUS, "CPU ID {} exceeds MAX_CPUS", cpu_id);
    assert!(ist_index >= 1 && ist_index <= 7, "IST index must be 1..=7");
    
    unsafe {
        match ist_index {
            1 => TSS_TABLE[cpu_id].ist1 = stack_top,
            2 => TSS_TABLE[cpu_id].ist2 = stack_top,
            3 => TSS_TABLE[cpu_id].ist3 = stack_top,
            4 => TSS_TABLE[cpu_id].ist4 = stack_top,
            5 => TSS_TABLE[cpu_id].ist5 = stack_top,
            6 => TSS_TABLE[cpu_id].ist6 = stack_top,
            7 => TSS_TABLE[cpu_id].ist7 = stack_top,
            _ => unreachable!(),
        }
    }
}

pub fn init_bsp() {
    info!("Initializing for BSP (CPU#0)");
    unsafe {
        #[allow(static_mut_refs)]
        GLOBAL_GDT.set_tss(0, &raw const TSS_TABLE[0]);
        #[allow(static_mut_refs)]
        GLOBAL_GDT.load();
        
        core::arch::asm!(
            "ltr {0:x}",
            in(reg) tss_selector(0),
            options(nostack, preserves_flags)
        );
    }
    info!("BSP initialized successfully.");
}

pub fn init_ap(cpu_id: usize) {
    info!("Initializing for AP (CPU#{})", cpu_id);
    
    unsafe {
        #[allow(static_mut_refs)]
        GLOBAL_GDT.set_tss(cpu_id, &raw const TSS_TABLE[cpu_id]);
        #[allow(static_mut_refs)]
        GLOBAL_GDT.load();

        core::arch::asm!(
            "ltr {0:x}",
            in(reg) tss_selector(cpu_id),
            options(nostack, preserves_flags)
        );
    }

    info!("AP initialized successfully (Selector: {:#X})", tss_selector(cpu_id));
}
