#![allow(non_snake_case)]

use ketypes::*;

pub fn KeVtDeviceNew(name: KeStr) -> Hdl![Device] {
    trace!("KeVtDeviceNew name={:?}", name);
    <hdl![Device]>::new(crate::dev::Device::new(name))
}
