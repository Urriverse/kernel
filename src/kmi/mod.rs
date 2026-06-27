pub mod kmdl;
pub mod mbs;
pub mod kst;

pub fn init(elf: &[u8]) {
    mbs::run_module(elf).expect("Can't run bootstrap module");
}
