use crate::mem::kdm::Paddr;

#[derive(Clone, Copy, Debug)]
pub struct Hdl;

impl acpi::Handler for Hdl {
    fn acquire(&self, _mutex: acpi::Handle, _timeout: u16) -> Result<(), acpi::aml::AmlError> { unimplemented!() }
    fn breakpoint(&self) { unimplemented!() }
    fn create_mutex(&self) -> acpi::Handle { unimplemented!() }
    fn handle_debug(&self, _object: &acpi::aml::object::Object) { unimplemented!() }
    fn handle_fatal_error(&self, _fatal_type: u8, _fatal_code: u32, _fatal_arg: u64) { unimplemented!() }
    unsafe fn map_physical_region<T>(&self, physical_address: usize, size: usize) -> acpi::PhysicalMapping<Self, T> {
        use core::ptr::NonNull;
        acpi::PhysicalMapping {
            physical_start: physical_address,
            virtual_start: unsafe { NonNull::<T>::new_unchecked(Paddr::from_raw(physical_address).to_virt().to_ptr_mut()) },
            region_length: size,
            mapped_length: size,
            handler: *self,
        }
    }
    fn nanos_since_boot(&self) -> u64 { unimplemented!() }
    fn read_io_u16(&self, _port: u16) -> u16 { unimplemented!() }
    fn read_io_u32(&self, _port: u16) -> u32 { unimplemented!() }
    fn read_io_u8(&self, _port: u16) -> u8 { unimplemented!() }
    fn read_pci_u16(&self, _address: acpi::PciAddress, _offset: u16) -> u16 { unimplemented!() }
    fn read_pci_u32(&self, _address: acpi::PciAddress, _offset: u16) -> u32 { unimplemented!() }
    fn read_pci_u8(&self, _address: acpi::PciAddress, _offset: u16) -> u8 { unimplemented!() }
    fn read_u16(&self, _address: usize) -> u16 { unimplemented!() }
    fn read_u32(&self, _address: usize) -> u32 { unimplemented!() }
    fn read_u64(&self, _address: usize) -> u64 { unimplemented!() }
    fn read_u8(&self, _address: usize) -> u8 { unimplemented!() }
    fn release(&self, _mutex: acpi::Handle) { unimplemented!() }
    fn sleep(&self, _milliseconds: u64) { unimplemented!() }
    fn stall(&self, _microseconds: u64) { unimplemented!() }
    fn unmap_physical_region<T>(_region: &acpi::PhysicalMapping<Self, T>) { /* no-op */ }
    fn write_io_u16(&self, _port: u16, _value: u16) { unimplemented!() }
    fn write_io_u32(&self, _port: u16, _value: u32) { unimplemented!() }
    fn write_io_u8(&self, _port: u16, _value: u8) { unimplemented!() }
    fn write_pci_u16(&self, _address: acpi::PciAddress, _offset: u16, _value: u16) { unimplemented!() }
    fn write_pci_u32(&self, _address: acpi::PciAddress, _offset: u16, _value: u32) { unimplemented!() }
    fn write_pci_u8(&self, _address: acpi::PciAddress, _offset: u16, _value: u8) { unimplemented!() }
    fn write_u16(&self, _address: usize, _value: u16) { unimplemented!() }
    fn write_u32(&self, _address: usize, _value: u32) { unimplemented!() }
    fn write_u64(&self, _address: usize, _value: u64) { unimplemented!() }
    fn write_u8(&self, _address: usize, _value: u8) { unimplemented!() }
}
