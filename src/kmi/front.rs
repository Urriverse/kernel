#![allow(non_snake_case)]

use ketypes::*;

pub fn KeVtDeviceNew(name: KeStr) -> KeDevice {
    trace!("KeVtDeviceNew name={:?}", name);
    KeDevice::new(crate::dev::Device::new(name))
}
