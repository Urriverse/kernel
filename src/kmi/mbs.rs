//

use alloc::sync::Arc;

use crate::sched::proc::Process;

pub fn run_module(_elf: &[u8]) -> Result<Arc<Process>, usize> {
    debug!("");
    Err(!0)
}
