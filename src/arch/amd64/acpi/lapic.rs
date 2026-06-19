use crate::{arch::paging::EntryFlags, mem::kdm::{Paddr, Vaddr}};

const LOCAL_APIC_VMA: usize = 0xFFFFFFFFFFFFF000;

#[derive(Debug, Clone, Copy)]
pub struct LocalApic;

impl LocalApic {
    const LAPIC_ID:        usize = 0x020;
    const LAPIC_VERSION:   usize = 0x030;
    const LAPIC_TPR:       usize = 0x080;
    const LAPIC_EOI:       usize = 0x0B0;
    const LAPIC_SVR:       usize = 0x0F0;
    const LAPIC_ICR_LOW:   usize = 0x300;
    const LAPIC_ICR_HIGH:  usize = 0x310;
    const LAPIC_LVT_TIMER: usize = 0x320;
    const LAPIC_LVT_LINT0: usize = 0x350;
    const LAPIC_LVT_LINT1: usize = 0x360;
    const LAPIC_LVT_ERROR: usize = 0x370;
    const LAPIC_TIMER_DCR: usize = 0x3E0;
    const LAPIC_TIMER_ICR: usize = 0x380;
    const LAPIC_TIMER_CCR: usize = 0x390;

    #[inline(always)]
    pub const fn new() -> Self { INSTANCE }

    // LAPIC ID is RO
    #[inline(always)]
    pub fn id(&self) -> u32 { *Vaddr::from_raw( LOCAL_APIC_VMA + Self::LAPIC_ID ).to_ref::<u32>() }

    // LAPIC version is RO
    #[inline(always)]
    pub fn version(&self) -> u32 { *Vaddr::from_raw( LOCAL_APIC_VMA + Self::LAPIC_VERSION ).to_ref::<u32>() }

    // Local vector table - timer
    #[inline(always)]
    pub fn lvt_timer(&self) -> &mut u32 { Vaddr::from_raw( LOCAL_APIC_VMA + Self::LAPIC_LVT_TIMER ).to_ref_mut() }

    // Local vector table - lint0
    #[inline(always)]
    pub fn lvt_lint0(&self) -> &mut u32 { Vaddr::from_raw( LOCAL_APIC_VMA + Self::LAPIC_LVT_LINT0 ).to_ref_mut() }

    // Local vector table - lint1
    #[inline(always)]
    pub fn lvt_lint1(&self) -> &mut u32 { Vaddr::from_raw( LOCAL_APIC_VMA + Self::LAPIC_LVT_LINT1 ).to_ref_mut() }

    // Local vector table - error
    #[inline(always)]
    pub fn lvt_error(&self) -> &mut u32 { Vaddr::from_raw( LOCAL_APIC_VMA + Self::LAPIC_LVT_ERROR ).to_ref_mut() }

    // Spurious interrupt vector
    #[inline(always)]
    pub fn svr(&self) -> &mut u32 { Vaddr::from_raw( LOCAL_APIC_VMA + Self::LAPIC_SVR ).to_ref_mut() }

    // End of interrupt
    #[inline(always)]
    pub fn eoi(&self) -> &mut u32 { Vaddr::from_raw( LOCAL_APIC_VMA + Self::LAPIC_EOI ).to_ref_mut() }

    // Interrupt command - low
    #[inline(always)]
    pub fn iclo(&self) -> &mut u32 { Vaddr::from_raw( LOCAL_APIC_VMA + Self::LAPIC_ICR_LOW ).to_ref_mut() }

    // Interrupt command - high
    #[inline(always)]
    pub fn ichi(&self) -> &mut u32 { Vaddr::from_raw( LOCAL_APIC_VMA + Self::LAPIC_ICR_HIGH ).to_ref_mut() }

    // Timer - divisor
    #[inline(always)]
    pub fn div(&self) -> &mut u32 { Vaddr::from_raw( LOCAL_APIC_VMA + Self::LAPIC_TIMER_DCR ).to_ref_mut() }

    // Timer - initial counter
    #[inline(always)]
    pub fn icr(&self) -> &mut u32 { Vaddr::from_raw( LOCAL_APIC_VMA + Self::LAPIC_TIMER_ICR ).to_ref_mut() }

    // Timer - current counter
    #[inline(always)]
    pub fn ccr(&self) -> &mut u32 { Vaddr::from_raw( LOCAL_APIC_VMA + Self::LAPIC_TIMER_CCR ).to_ref_mut() }

    // Timer - task priority
    #[inline(always)]
    pub fn tpr(&self) -> &mut u32 { Vaddr::from_raw( LOCAL_APIC_VMA + Self::LAPIC_TPR ).to_ref_mut() }
}

static INSTANCE: LocalApic = LocalApic;

pub fn init() {
    let local_apic_address;

    let interrupt_model
    =   acpi::platform::InterruptModel::new(&super::TABLES).expect("Failed to parse interrupt model (MADT)");

    let aps;

    // 1. processors
    match interrupt_model.1 {
        Some(pi) => aps = pi.application_processors,
        None => {
            panic!("Can't obtain PU topology from ACPI");
        }
    }

    unsafe {
        super::TOTAL_CPUS = aps.len() + 1;
    }

    match interrupt_model.0 {
        acpi::platform::InterruptModel::Apic(x) => {
            local_apic_address = x.local_apic_address;
        },
        _ => { panic!("Unsupported host") }
    }

    match crate::mem::PTM.lock().map_4k_block(
        LOCAL_APIC_VMA,  // VMA - 4k
        Paddr::from_raw(local_apic_address as usize),
        EntryFlags::PRESENT
            |   EntryFlags::WRITABLE
            |   EntryFlags::WRITE_THROUGH
            |   EntryFlags::CACHE_DISABLE
    ) {
        Ok(_) => {},
        Err(e) => panic!("Can't map LAPIC: {}", e)
    };
}

pub fn enable() {
    *INSTANCE.lvt_timer()  = 1 << 16;
    *INSTANCE.lvt_lint0()  = 1 << 16;
    *INSTANCE.lvt_lint0()  = 1 << 16;
    *INSTANCE.lvt_error()  = 1 << 16;
    *INSTANCE.svr()        = (1u32 << 8) | (super::SPURIOUS_VECTOR as u32);
}
