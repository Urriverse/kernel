//! # Scheduler Subsystem (EEVDF + NVDL)
//!
//! This module implements the kernel scheduler based on the **Earliest Eligible Virtual Deadline First (EEVDF)** algorithm,
//! extended with the **NVDL (Non-Virtual Deadline)** mechanism for strict real-time guarantees.

// ============================================================================
// SUBMODULES
// ============================================================================
pub mod task;   
pub mod rq;     
pub mod wq;     
pub mod proc;   

// ============================================================================
// IMPORTS
// ============================================================================
use crate::arch::trap::TrapFrame;
use crate::mem::vma::VmaFlags;
use crate::sched::proc::Process;
use crate::sync::Nutex;
use crate::vfs::RootRef;
use alloc::sync::Arc;
use alloc::{boxed::Box, collections::btree_map::BTreeMap};
use task::{Task, TaskId, TaskState, Priority};
use rq::RUNQUEUES;
use wq::WaitQueue;
use core::arch::naked_asm;
use core::sync::atomic::{AtomicU64, Ordering};
use crate::arch::paging::EntryFlags;

// ============================================================================
// GLOBAL STATE
// ============================================================================
pub static TASK_REGISTRY: Nutex<BTreeMap<TaskId, Box<Task>>> = Nutex::new(BTreeMap::new());
pub static EXIT_WQ: Nutex<WaitQueue> = Nutex::new(WaitQueue::new());
static TICKS_PER_MS: AtomicU64 = AtomicU64::new(0);

// ============================================================================
// INITIALIZATION
// ============================================================================
pub fn init(ticks_per_10ms: u64) {
    TICKS_PER_MS.store(ticks_per_10ms / 10, Ordering::Release);
    for (cpu, _) in RUNQUEUES.iter().enumerate().take(crate::arch::num_cpus()) {
        let stack = allocate_kernel_stack(16 * 1024);
        let idle = Task::new_kernel(idle_task, stack, Priority(19), "idle");
        let mut rq = RUNQUEUES[cpu].lock();
        rq.set_current(idle.id);
        rq.spawn_task(idle); // Use spawn_task for proper vruntime init
        if cpu == crate::arch::current_cpu() {
            crate::arch::percpu::set_kernel_stack(stack);
        }
    }
    crate::info!("Scheduler initialized with EEVDF + NVDL");
}

fn allocate_kernel_stack(size: usize) -> usize {
    let pages = size.div_ceil(4096);
    let paddr = crate::mem::upa::alloc(pages);
    if paddr.to_raw() == 0 {
        panic!("Failed to allocate kernel stack");
    }
    paddr.to_virt().to_raw() + size
}

fn idle_task() {
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

// ============================================================================
// TASK SPAWNING
// ============================================================================
pub fn spawn_kernel_task(
    entry: fn(),
    priority: Priority,
    name: &'static str,
    root: Option<RootRef>,
    cpu_affinity: Option<usize>,
) -> TaskId {
    let stack = allocate_kernel_stack(32 * 1024);
    let mut task = Task::new_kernel(entry, stack, priority, name);
    task.cpu_affinity = cpu_affinity;
    
    if let Some(x) = root {
        let mut proc;
        if let Some(p) = current_process() {
            proc = (*p).clone();
        } else {
            proc = Process::new();
        }
        proc.roots = x;
        task.process = Arc::new(proc);
    }
    
    let cpu = crate::arch::current_cpu();
    let rq = RUNQUEUES[cpu].lock();
    task.parent = rq.current_task_id();
    drop(rq);
    
    let id = task.id;
    RUNQUEUES[cpu].lock().spawn_task(task); // Use spawn_task
    id
}

pub fn spawn_kernel_task_with_arg(
    entry: fn(usize),
    arg: usize,
    priority: Priority,
    name: &'static str,
    root: Option<RootRef>,
    cpu_affinity: Option<usize>,
) -> TaskId {
    let stack = allocate_kernel_stack(32 * 1024);
    let mut task = Task::new_kernel_with_arg(entry, arg, stack, priority, name);
    task.cpu_affinity = cpu_affinity;
    
    if let Some(x) = root {
        let mut proc;
        if let Some(p) = current_process() {
            proc = (*p).clone();
        } else {
            proc = Process::new();
        }
        proc.roots = x;
        task.process = Arc::new(proc);
    }
    
    let cpu = crate::arch::current_cpu();
    let rq = RUNQUEUES[cpu].lock();
    task.parent = rq.current_task_id();
    drop(rq);
    
    let id = task.id;
    RUNQUEUES[cpu].lock().spawn_task(task); // Use spawn_task
    id
}

pub fn spawn_closure_task<F>(
    closure: F,
    priority: Priority,
    name: &'static str,
    root: Option<RootRef>,
    cpu_affinity: Option<usize>,
) -> TaskId
where
    F: FnOnce() + Send + 'static,
{
    let boxed = Box::new(closure);
    let arg = Box::into_raw(boxed) as usize;

    fn trampoline<F: FnOnce() + Send + 'static>(arg: usize) {
        let closure = unsafe { Box::from_raw(arg as *mut F) };
        closure();
        exit(0);
    }

    spawn_kernel_task_with_arg(trampoline::<F>, arg, priority, name, root, cpu_affinity)
}

// ============================================================================
// TASK EXIT & WAITING
// ============================================================================
pub fn exit(code: i32) -> ! {
    let cpu = crate::arch::current_cpu();
    let mut rq = RUNQUEUES[cpu].lock();
    let current_id = rq.current_task_id().unwrap();
    
    debug!(
        "Exiting task {} (PID {}) with code {}",
        current_id.0,
        current_process().unwrap_or(Arc::new(Process::new())).pid,
        code,
    );
    
    let mut task = rq.remove(current_id).unwrap();
    rq.clear_current();
    drop(rq);
    
    let init_id = TaskId(1);
    {
        let mut registry = TASK_REGISTRY.lock();
        for t in registry.values_mut() {
            if t.parent == Some(current_id) {
                t.parent = Some(init_id);
            }
        }
    }
    
    task.state = TaskState::Zombie;
    task.exit_code = code;
    TASK_REGISTRY.lock().insert(current_id, task);
    wakeup(&EXIT_WQ);
    yield_now();
    
    loop {
        unsafe {
            core::arch::asm! {
                "hlt"
            }
        }
    }
}

#[inline(always)]
pub fn yield_now() {
    unsafe {
        core::arch::asm!("int 33");
    }
}

pub fn sleep(wq: &Nutex<WaitQueue>) {
    let cpu = crate::arch::current_cpu();
    let mut rq = RUNQUEUES[cpu].lock();
    let current_id = rq.current_task_id().unwrap();
    rq.clear_current();
    
    // Properly remove from scheduling trees but keep in tasks map
    rq.sleep_task(current_id);
    wq.lock().sleep(current_id);
    
    drop(rq);
    yield_now();
}

pub fn wakeup(wq: &Nutex<WaitQueue>) {
    let cpu = crate::arch::current_cpu();
    let mut rq = RUNQUEUES[cpu].lock();
    if let Some(task_id) = wq.lock().wakeup_one() {
        // Apply Lag Decay and re-insert into scheduling trees
        rq.wakeup_task(task_id);
    }
}

#[allow(dead_code)]
pub fn wait_child(child_id: TaskId) -> i32 {
    loop {
        let mut registry = TASK_REGISTRY.lock();
        if let Some(task) = registry.get(&child_id)
            && task.state == TaskState::Zombie {
            let code = task.exit_code;
            registry.remove(&child_id);
            return code;
        }
        drop(registry);
        sleep(&EXIT_WQ);
    }
}

pub fn wait_any() -> Option<(TaskId, Box<Task>)> {
    loop {
        let mut registry = TASK_REGISTRY.lock();
        let zombie_id = registry.iter()
            .find(|(_, t)| t.state == TaskState::Zombie)
            .map(|(id, _)| *id);
            
        if let Some(id) = zombie_id {
            let task = registry.remove(&id).unwrap();
            return Some((id, task));
        }
        drop(registry);
        sleep(&EXIT_WQ);
    }
}

// ============================================================================
// SCHEDULER CORE
// ============================================================================
pub fn timer_tick(frame: &mut TrapFrame) {
    if crate::arch::current_cpu() == 0 {
        crate::arch::TIME_FROM_BOOT.fetch_add(10, Ordering::Relaxed);
    }
    reschedule(frame);
}

pub fn reschedule(frame: &mut TrapFrame) {
    crate::arch::acpi::eoi();
    let cpu = crate::arch::current_cpu();
    let mut rq = RUNQUEUES[cpu].lock();
    let ticks_per_10ms = crate::arch::timer::get_ticks_per_10ms();
    
    rq.update_vruntime(ticks_per_10ms);
    
    let current_id = rq.current_task_id();
    let next_id = rq.pick_next();
    
    if let Some(next_id) = next_id {
        if Some(next_id) != current_id {
            if let Some(curr_id) = current_id
                && let Some(old_task) = rq.tasks_mut().get_mut(&curr_id) {
                old_task.ctx.frame = *frame;
                if old_task.state == TaskState::Running {
                    old_task.state = TaskState::Runnable;
                }
                unsafe { core::arch::x86_64::_fxsave64(old_task.ctx.fpu_state.area.as_mut_ptr()); }
            }
            
            if let Some(new_task) = rq.tasks_mut().get_mut(&next_id) {
                new_task.state = TaskState::Running;
                *frame = new_task.ctx.frame;
                unsafe { core::arch::x86_64::_fxrstor64(new_task.ctx.fpu_state.area.as_ptr()); }
                
                let cpu = crate::arch::current_cpu();
                crate::arch::gdt::set_kernel_stack(cpu, new_task.kernel_stack_top as u64);
                crate::arch::percpu::set_kernel_stack(new_task.kernel_stack_top);
                
                if let Some(curr_id) = current_id {
                    let old_pid = unsafe { RUNQUEUES[cpu].inner() }.tasks().get(&curr_id).unwrap().process.pid;
                    let new_pid = new_task.process.pid;
                    if old_pid != new_pid {
                        let new_cr3 = new_task.process.address_space.lock().exco.cr3;
                        unsafe {
                            core::arch::asm!(
                                "mov cr3, {}",
                                in(reg) new_cr3,
                                options(nostack, preserves_flags)
                            );
                        }
                    }
                } else {
                    let new_cr3 = new_task.process.address_space.lock().exco.cr3;
                    unsafe {
                        core::arch::asm!("mov cr3, {}", in(reg) new_cr3, options(nostack, preserves_flags));
                    }
                }
                rq.set_current(next_id);
            }
        }
    } else {
        if let Some(curr_id) = current_id
            && let Some(curr_task) = rq.tasks_mut().get_mut(&curr_id) {
            curr_task.ctx.frame = *frame;
        }
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn yield_wrapper() -> ! {
    naked_asm!(
        "mov rax, [rsp + 8]",
        "and rax, 3",
        "cmp rax, 3",
        "jne 1f",
        "swapgs",
        "1:",
        "push r15", "push r14", "push r13", "push r12",
        "push r11", "push r10", "push r9", "push r8",
        "push rbp", "push rdi", "push rsi", "push rdx",
        "push rcx", "push rbx", "push rax",
        "mov rdi, rsp",
        "call {scheduler_tick}",
        "pop rax", "pop rbx", "pop rcx", "pop rdx",
        "pop rsi", "pop rdi", "pop rbp", "pop r8",
        "pop r9", "pop r10", "pop r11", "pop r12",
        "pop r13", "pop r14", "pop r15",
        "mov rax, [rsp + 8]",
        "and rax, 3",
        "cmp rax, 3",
        "jne 2f",
        "swapgs",
        "2:",
        "iretq",
        scheduler_tick = sym reschedule,
    );
}

// ============================================================================
// PROCESS & SYSCALL SUPPORT
// ============================================================================
pub fn current_process() -> Option<Arc<Process>> {
    let cpu = crate::arch::current_cpu();
    let rq = RUNQUEUES[cpu].lock();
    if let Some(id) = rq.current_task_id()
        && let Some(task) = rq.tasks().get(&id) {
        return Some(task.process.clone());
    }
    None
}

pub fn native_syscall_handler(frame: &mut TrapFrame) {
    match frame.rax {
        0 => {
            yield_now();
            frame.rax = 0;
        }
        1 => {
            let code = frame.rdi as i32;
            crate::info!("[Native SFD] Process {} exiting with code {}", current_process().unwrap().pid, code);
            exit(code);
        }
        _ => {
            crate::warn!("[Native SFD] Unknown syscall: {}", frame.rax);
            frame.rax = u64::MAX; 
        }
    }
}

pub fn syscall_dispatcher(frame: &mut TrapFrame) {
    let proc = match current_process() {
        Some(p) => p,
        None => {
            crate::error!("Syscall from unknown context!");
            frame.rax = u64::MAX; 
            return;
        }
    };
    (proc.syscall_handler)(frame);
}

// ============================================================================
// PAGE FAULT HANDLING
// ============================================================================
pub fn handle_page_fault(addr: usize, error_code: u64, rip: u64, _is_user: bool) {
    let is_present = (error_code & 0x1) != 0;
    let is_write   = (error_code & 0x2) != 0;

    let proc = match current_process() {
        Some(p) => p,
        None => panic!("Page fault in unknown context (no current process)"),
    };

    if is_present && is_write {
        let ptm = proc.address_space.lock();
        if let Some((paddr, flags)) = ptm.query(addr & !0xFFF)
            && flags.contains(EntryFlags::COPY_ON_WRITE) {
            drop(ptm);
            let new_paddr = crate::mem::upa::alloc(1);
            if new_paddr.to_raw() == 0 { panic!("OOM during CoW"); }
            let src = paddr.to_virt().to_ptr::<u8>();
            let dst = new_paddr.to_virt().to_ptr_mut::<u8>();
            unsafe { core::ptr::copy_nonoverlapping(src, dst, 4096); }
            
            let mut ptm = proc.address_space.lock();
            let mut new_flags = flags;
            new_flags.remove(EntryFlags::COPY_ON_WRITE);
            new_flags.insert(EntryFlags::WRITABLE);
            let _ = ptm.try_unmap(addr & !0xFFF, 4096);
            ptm.map_4k_block(addr & !0xFFF, new_paddr, new_flags).unwrap();
            return;
        }
    }

    if !is_present {
        let vmm = proc.vmm.lock();
        if let Some(vma) = vmm.find_overlap(addr) {
            let is_write_vma = vma.flags.contains(VmaFlags::WRITE);
            if is_write && !is_write_vma {
                drop(vmm);
                crate::info!("Process {} SEGFAULT: Write to Read-Only VMA at {:#X}", proc.pid, addr);
                exit(139);
            }
            drop(vmm); 
            
            let paddr = crate::mem::upa::alloc(1);
            if paddr.to_raw() == 0 { panic!("OOM during Demand Paging"); }
            
            let vaddr_ptr = paddr.to_virt().to_ptr_mut::<u8>();
            unsafe { core::ptr::write_bytes(vaddr_ptr, 0, 4096); }
            
            let mut ptm = proc.address_space.lock();
            let mut flags = EntryFlags::PRESENT | EntryFlags::USER_ACCESSIBLE;
            if is_write_vma {
                flags |= EntryFlags::WRITABLE;
            }
            ptm.map_4k_block(addr & !0xFFF, paddr, flags).unwrap();
            return;
        }
    }

    crate::info!("Process {} SEGFAULT at {:#X} (RIP: {:#X}, code: {:#X})", proc.pid, addr, rip, error_code);
    exit(139);
}

// ============================================================================
// NVDL PUBLIC API
// ============================================================================
pub fn set_rt_deadline(task_id: TaskId, deadline_ms: u64) {
    for cpu in 0..crate::arch::num_cpus() {
        let mut rq = RUNQUEUES[cpu].lock();
        if rq.tasks().contains_key(&task_id) {
            rq.set_rt_deadline(task_id, deadline_ms);
            return;
        }
    }
    crate::warn!("set_rt_deadline: task {} not found in any runqueue", task_id.0);
}
