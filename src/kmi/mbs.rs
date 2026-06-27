//

use alloc::sync::Arc;

use crate::sched::proc::Process;

pub fn run_module(elf: &[u8]) -> Result<Arc<Process>, usize> {
    debug!("");

    let bytes = elf::ElfBytes::<elf::endian::NativeEndian>::minimal_parse(elf).expect("Can't parse bootstrap module");

    for s in bytes.segments().expect("X").iter() {
        debug!("S {:?}", s);
    }
    debug!("");

    for s in bytes.symbol_table().expect("Y").iter() {
        debug!("G {:?}", s);
    }

    Err(!0)
}
