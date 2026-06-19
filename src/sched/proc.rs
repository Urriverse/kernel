use alloc::sync::Arc;
use alloc::vec::Vec;
use crate::arch::trap::TrapFrame;
use crate::mem::ptm::Polen;
use crate::mem::vma::Vmm;
use crate::sync::Nutex;

pub type ProcId = u32;

pub struct Process {
    pub pid: ProcId,
    pub parent: Option<ProcId>,
    pub address_space: Arc<Nutex<Polen>>, 
    pub vmm: Arc<Nutex<Vmm>>,
    pub threads: Vec<super::task::TaskId>,
    pub syscall_handler: fn(&mut TrapFrame),
}

impl core::fmt::Debug for Process {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "Process {{ pid: {:?}, parent: {:?}, address_space: {:?}, threads (len): {:?} }}",
            self.pid,
            self.parent,
            self.address_space.lock().exco.cr3,
            self.threads.len(),
        ))
    }
}

impl core::fmt::Display for Process {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "Process {{ pid: {}, parent: {:?}, address_space: {}, threads (len): {} }}",
            self.pid,
            self.parent,
            self.address_space.lock().exco.cr3,
            self.threads.len(),
        ))
    }
}
