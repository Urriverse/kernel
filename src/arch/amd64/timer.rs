use core::hint::unlikely;

use crate::{arch::paging::EntryFlags, mem::kdm::{Paddr, Vaddr}, sync::Nutex};
use core::arch::naked_asm;
use core::sync::atomic::{AtomicU64, Ordering};

static TICKS_PER_10MS: AtomicU64 = AtomicU64::new(0);

#[unsafe(naked)]
pub unsafe extern "C" fn timer_wrapper() -> ! {
    naked_asm!(
        "mov rax, [rsp + 8]",
        "and rax, 3",
        "cmp rax, 3",
        "jne 1f",
        "swapgs",
        "1:",

        "push r15", "push r14", "push r13", "push r12",
        "push r11", "push r10", "push r9", "push r8",
        "push rbp", "push rdi", "push rsi", "push rdx",
        "push rcx", "push rbx", "push rax",

        "mov rdi, rsp",
        "call {scheduler_tick}",

        "pop rax", "pop rbx", "pop rcx", "pop rdx",
        "pop rsi", "pop rdi", "pop rbp", "pop r8",
        "pop r9", "pop r10", "pop r11", "pop r12",
        "pop r13", "pop r14", "pop r15",

        "mov rax, [rsp + 8]",
        "and rax, 3",
        "cmp rax, 3",
        "jne 2f",
        "swapgs",
        "2:",

        "iretq",

        scheduler_tick = sym crate::sched::timer_tick,
    );
}

const HPET_VMA: usize = 0xFFFFFFFFFFFFE000;

#[derive(Debug, Clone, Copy)]
pub struct Hpet;

impl Hpet {
    const HPET_CAP: usize = 0x000;
    const HPET_CFG: usize = 0x010;
    const HPET_COUNTER: usize = 0x0F0;

    pub const ENABLE: u32 = 1;

    #[inline(always)]
    pub fn disable(&self) {
        *self.cfg() &= !Hpet::ENABLE;
    }

    #[inline(always)]
    pub fn enable(&self) {
        *self.cfg() |= Hpet::ENABLE;
    }

    #[inline(always)]
    pub fn reset(&self) {
        *self.counter() = 0;
    }

    // capability
    #[inline(always)]
    pub fn cap(&self) -> u64 { *Vaddr::from_raw( HPET_VMA + Self::HPET_CAP ).to_ref::<u64>() }
    
    // configuration
    #[inline(always)]
    pub fn cfg(&self) -> &mut u32 { Vaddr::from_raw( HPET_VMA + Self::HPET_CFG ).to_ref_mut::<u32>() }
    
    // counter
    #[inline(always)]
    pub fn counter(&self) -> &mut u64 { Vaddr::from_raw( HPET_VMA + Self::HPET_COUNTER ).to_ref_mut::<u64>() }
}

pub static INSTANCE: Nutex<Hpet> = Nutex::new(Hpet);

pub fn init_bsp() {
    let hpet_info = acpi::HpetInfo::new(&super::acpi::TABLES).expect("Failed to parse HPET table");
    let hpet_base_paddr = hpet_info.base_address as usize;

    info!("HPET found at physical address: {:p}", hpet_base_paddr as *const());

    match crate::mem::PTM.lock().map_4k_block(
        HPET_VMA,
        Paddr::from_raw(hpet_base_paddr),
        EntryFlags::PRESENT
            | EntryFlags::WRITABLE
            | EntryFlags::CACHE_DISABLE
            | EntryFlags::WRITE_THROUGH
    ) {
        Ok(_) => {},
        Err(e) => panic!("Can't map HPET: {}", e)
    };
}

pub fn init() {
    // Sync guaranteed, so we can temporarily go direct (no sync primitives).
    let inst = Hpet;
    let lapic = super::acpi::lapic::LocalApic;

    let cap = inst.cap();
    let period_fs = (cap >> 32) as u64;
    if unlikely(period_fs == 0) { panic!("HPET period is 0, cannot calibrate!") }

    let target_fs = 1_000_000_000_000u64;
    let hpet_ticks_to_wait = target_fs / period_fs;

    inst.disable();
    inst.reset();

    *lapic.div() = 3; // x16 (code 3)
    *lapic.lvt_timer() = 0x00010000; // oneshot
    *lapic.icr() = !0; // maximum initial value

    inst.enable();

    let start_hpet = *inst.counter();

    // FIXME: sometimes this code shoots (quitely rarely)
    while (*inst.counter() - start_hpet) < hpet_ticks_to_wait {
        core::hint::spin_loop();
    }

    inst.disable();

    let cur_lapic = *lapic.ccr();
    let elapsed = !0 - cur_lapic;

    TICKS_PER_10MS.store(elapsed as u64, Ordering::Relaxed);

    info!("APIC timer calibrated: {} ticks per 10ms", elapsed);

    *lapic.lvt_timer() = (1 << 17) | (crate::arch::idt::TIMER_VECTOR as u32); // periodic mode
    *lapic.icr() = elapsed;
}

pub fn get_ticks_per_10ms() -> u64 { TICKS_PER_10MS.load(Ordering::Relaxed) }
