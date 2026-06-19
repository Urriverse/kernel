pub mod lapic;
pub mod handler;

pub use ::acpi::platform::Processor;

use crate::{arch::timer, mem::kdm::Vaddr};

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum DeliveryMode {
    Fixed        = 0b000 << 8,
    LowestPri    = 0b001 << 8,
    Smi          = 0b010 << 8,
    Nmi          = 0b100 << 8,
    Init         = 0b101 << 8,
    StartUp      = 0b110 << 8,
}

pub const ICR_LEVEL_ASSERT:  u32 = 1 << 14;
pub const ICR_DEST_MODE_PHYS: u32 = 0 << 11;
pub const ICR_DEST_MODE_LOG:  u32 = 1 << 11;

pub static mut TOTAL_CPUS: usize = 0;
pub static mut LAPIC_PHYS_ADDR: usize = 0;
lazy_static! {
    pub static ref TABLES: acpi::AcpiTables<handler::Hdl> = unsafe {
        acpi::AcpiTables::from_rsdp(
            handler::Hdl,
            Vaddr::from_raw(
                RSDP
                    .response()
                    .expect("Can't obtain RSDP")
                    .address as usize
                ).to_phys().to_raw()
        ).expect("Failed to parse ACPI tables")
    };
}

pub const SPURIOUS_VECTOR: u8 = 0xFF;

limine! { RSDP <= RsdpRequest }

pub fn init_bsp() {
    lapic::init();
    timer::init_bsp();
}

pub fn init() {
    lapic::enable();
    timer::init();
}

#[inline(always)]
pub fn eoi() {
    *lapic::LocalApic::new().eoi() = 0;
}

#[inline]
pub fn send_ipi(target_apic_id: u32, vector: u8, mode: DeliveryMode) {
    let lapic = lapic::LocalApic::new();

    while (*lapic.iclo() & (1 << 12)) != 0 {
        core::hint::spin_loop();
    }
    
    *lapic.ichi() = target_apic_id << 24;
    
    let icr_low = (vector as u32)
    |   (mode as u32)
    |   ICR_DEST_MODE_PHYS
    |   ICR_LEVEL_ASSERT;

    *lapic.iclo() = icr_low;
}

#[inline]
pub fn send_fixed_ipi(target_apic_id: u32, vector: u8) {
    send_ipi(target_apic_id, vector, DeliveryMode::Fixed);
}
