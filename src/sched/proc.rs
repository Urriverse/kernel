use core::sync::atomic::AtomicUsize;

// src/sched/proc.rs
use alloc::sync::Arc;
use alloc::vec::Vec;
use crate::arch::trap::TrapFrame;
use crate::mem::ptm::Polen;
use crate::mem::vma::Vmm;
use crate::sync::{Litex, Nutex};
use crate::vfs::{RootRef, RootReg};

pub type ProcId = u32;

#[derive(Debug)]
pub struct Process {
    pub pid: ProcId,
    pub parent: Option<ProcId>,
    pub address_space: Arc<Nutex<Polen>>,
    pub vmm: Arc<Nutex<Vmm>>,
    pub threads: Vec<super::task::TaskId>,
    pub syscall_handler: fn(&mut TrapFrame),
    pub roots: RootRef,
    pub level: u16,
    pub rc: AtomicUsize,
}

static NEXT: Litex<u32> = Litex::new(0);

fn next() -> u32 {
    let next = NEXT.lock();
    let rv = *next;
    unsafe { *NEXT.inner() = rv + 1 }
    rv
}

impl Process {
    pub fn new() -> Self {
        Self {
            pid: next(),
            parent: None,
            address_space: Arc::new(Nutex::new(Polen::reference())),
            vmm: Arc::new(Nutex::new(Vmm::new())),
            threads: Vec::new(),
            syscall_handler: crate::sched::native_syscall_handler,
            roots: RootRef::new(RootReg::new()),
            level: 0,
            rc: AtomicUsize::new(0),
        }
    }
}

impl Clone for Process {
    fn clone(&self) -> Self {
        Self {
            pid: next(),
            parent: Some(self.pid),
            address_space: self.address_space.clone(),
            vmm: self.vmm.clone(),
            threads: vec![],
            syscall_handler: self.syscall_handler,
            roots: self.roots.clone(),
            level: self.level,
            rc: AtomicUsize::new(0),
        }
    }
}

impl core::fmt::Display for Process {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "Process {{ pid: {}, parent: {:?}, address_space: {}, threads (len): {}, rc: {} }}",
            self.pid, self.parent, self.address_space.lock().exco.cr3, self.threads.len(), self.rc.load(core::sync::atomic::Ordering::SeqCst)
        ))
    }
}
