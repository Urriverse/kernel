//! # Scheduler Subsystem (EEVDF + NVDL + SMP Load Balancing)
pub mod task;
pub mod rq;
pub mod wq;
pub mod proc;

use crate::arch::current_cpu;
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

pub static TASK_REGISTRY: Nutex<BTreeMap<TaskId, Box<Task>>> = Nutex::new(BTreeMap::new());
pub static EXIT_WQ: Nutex<WaitQueue> = Nutex::new(WaitQueue::new());
static TICKS_PER_MS: AtomicU64 = AtomicU64::new(0);
static BALANCE_TICKS: AtomicU64 = AtomicU64::new(0);

pub fn init(ticks_per_10ms: u64) {
    TICKS_PER_MS.store(ticks_per_10ms / 10, Ordering::Release);
    for (cpu, _) in RUNQUEUES.iter().enumerate().take(crate::arch::num_cpus()) {
        let stack = allocate_kernel_stack(16 * 1024);
        let mut idle = Task::new_kernel(idle_task, stack, Priority(19), "idle");
        
        // CRITICAL FIX: Pin the idle task to this CPU so the balancer never steals it!
        idle.cpu_affinity = Some(cpu); 
        
        let mut rq = RUNQUEUES[cpu].lock();
        rq.set_current(idle.id);
        rq.spawn_task(idle);
        
        if cpu == current_cpu() {
            crate::arch::percpu::set_kernel_stack(stack);
        }
    }
    crate::info!("Initialized with SMP Balancing");
}

fn allocate_kernel_stack(size: usize) -> usize {
    let pages = size.div_ceil(4096);
    let paddr = crate::mem::upa::alloc(pages);
    if paddr.to_raw() == 0 { panic!("Failed to allocate kernel stack"); }
    paddr.to_virt().to_raw() + size
}

#[unsafe(naked)]
fn idle_task() {
    core::arch::naked_asm! {
        "2:",
        "hlt",
        "jmp 2b"
    }
}

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
        let mut proc = current_process().map(|p| (*p).clone()).unwrap_or_else(Process::new);
        proc.roots = x;
        task.process = Arc::new(proc);
    }
    
    let target_cpu = match cpu_affinity {
        Some(cpu) if cpu < crate::arch::MAX_CPUS => cpu,
        _ => current_cpu(),
    };
    
    let rq = RUNQUEUES[target_cpu].lock();
    task.parent = rq.current_task_id();
    drop(rq);
    
    let id = task.id;
    RUNQUEUES[target_cpu].lock().spawn_task(task);
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
        let mut proc = current_process().map(|p| (*p).clone()).unwrap_or_else(Process::new);
        proc.roots = x;
        task.process = Arc::new(proc);
    }
    
    let target_cpu = match cpu_affinity {
        Some(cpu) if cpu < crate::arch::MAX_CPUS => cpu,
        _ => current_cpu(),
    };
    
    let rq = RUNQUEUES[target_cpu].lock();
    task.parent = rq.current_task_id();
    drop(rq);
    
    let id = task.id;
    RUNQUEUES[target_cpu].lock().spawn_task(task);
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

pub fn exit(code: i32) -> ! {
    let cpu = current_cpu();
    let mut rq = RUNQUEUES[cpu].lock();
    let current_id = rq.current_task_id().unwrap();
    let mut task = rq.remove(current_id).unwrap();
    
    let pid = task.process.pid;
    
    debug!("Exiting task \"{}\" (TID {} PID {}) with code {}", task.name, current_id.0, pid, code);
    
    rq.clear_current();
    drop(rq);
    
    let init_id = TaskId(1);
    {
        let mut registry = TASK_REGISTRY.lock();
        for t in registry.values_mut() {
            if t.parent == Some(current_id) { t.parent = Some(init_id); }
        }
    }
    
    task.state = TaskState::Zombie;
    task.exit_code = code;
    TASK_REGISTRY.lock().insert(current_id, task);
    
    crate::debug!("Task {} is now a zombie, waking up reaper...", current_id.0);
    wakeup(&EXIT_WQ);
    yield_now();
    loop { unsafe { core::arch::asm! { "hlt" } } }
}

#[inline(always)]
pub fn yield_now() { unsafe { core::arch::asm!("int 33"); } }

pub fn sleep(wq: &Nutex<WaitQueue>) {
    let cpu = current_cpu();
    
    // 1. Safely grab the task ID
    let current_id = {
        let rq = RUNQUEUES[cpu].lock();
        rq.current_task_id().unwrap()
    };

    // 2. LOCK ORDERING: Lock WQ first, then RQ
    let mut wq_guard = wq.lock();
    let mut rq = RUNQUEUES[cpu].lock();

    // 3. Add to WaitQueue
    wq_guard.sleep(current_id);
    
    // CRITICAL FIX: Properly remove the task from the EEVDF scheduling trees!
    // Do NOT manually set task.state = Sleeping here. `sleep_task` handles it.
    rq.sleep_task(current_id); 

    // 4. Drop locks
    drop(rq);
    drop(wq_guard);
    
    // 5. Yield the CPU
    yield_now();
}

pub fn wakeup(wq: &Nutex<WaitQueue>) {
    // 1. Lock WQ, extract the task, and DROP WQ immediately to prevent inversion
    let task_id = {
        let mut wq_guard = wq.lock();
        wq_guard.wakeup_one()
    };
    
    if let Some(task_id) = task_id {
        // 2. Search RQs safely
        let mut found = false;
        for cpu in 0..crate::arch::num_cpus() {
            let mut rq = RUNQUEUES[cpu].lock();
            if rq.tasks().contains_key(&task_id) {
                rq.wakeup_task(task_id);
                found = true;
                break;
            }
        }
        
        if !found {
            // Task might have already exited and been reaped. This is normal.
            crate::debug!("wakeup: task {} not found in any runqueue (likely already exited)", task_id.0);
        }
    }
}

#[allow(dead_code)]
pub fn wait_child(child_id: TaskId) -> i32 {
    loop {
        let mut registry = TASK_REGISTRY.lock();
        if let Some(task) = registry.get(&child_id) && task.state == TaskState::Zombie {
            let code = task.exit_code;
            registry.remove(&child_id);
            return code;
        }
        drop(registry);
        sleep(&EXIT_WQ);
    }
}

pub fn wait_any() -> Option<(TaskId, Box<Task>)> {
    'x: loop {
        let registry_opt = TASK_REGISTRY.try_lock();
        let mut registry = match registry_opt {
            Some(x) => x,
            None => {
                warn!("Can't obtain lock");
                break 'x None
            }
        };
        let zombie_id = registry.iter().find(|(_, t)| t.state == TaskState::Zombie).map(|(id, _)| *id);
        if let Some(id) = zombie_id {
            let task = registry.remove(&id).unwrap();
            return Some((id, task));
        }
        drop(registry);
        sleep(&EXIT_WQ);
    }
}

pub fn timer_tick(frame: &mut TrapFrame) {
    if current_cpu() == 0 {
        crate::arch::TIME_FROM_BOOT.fetch_add(10, Ordering::Relaxed);
        let ticks = BALANCE_TICKS.fetch_add(10, Ordering::Relaxed);
        if ticks % 100 == 0 { 
            balance_cpus();
        }
    }
    reschedule(frame);
}

fn balance_cpus() {
    let num_cpus = crate::arch::num_cpus();
    if num_cpus <= 1 { return; }
    
    let mut busiest_cpu = 0;
    let mut max_load = 0;
    let mut idlest_cpu = 0;
    let mut min_load = u64::MAX;
    
    for i in 0..num_cpus {
        let rq = RUNQUEUES[i].lock();
        let load = rq.load();
        if load > max_load { max_load = load; busiest_cpu = i; }
        if load < min_load { min_load = load; idlest_cpu = i; }
    }
    
    if max_load > min_load + 1024 && busiest_cpu != idlest_cpu {
        let (mut busy_rq, mut idle_rq) = if busiest_cpu < idlest_cpu {
            (RUNQUEUES[busiest_cpu].lock(), RUNQUEUES[idlest_cpu].lock())
        } else {
            (RUNQUEUES[idlest_cpu].lock(), RUNQUEUES[busiest_cpu].lock())
        };
        
        let mut stolen_id = None;
        for (&id, task) in busy_rq.tasks().iter() {
            if task.state == TaskState::Runnable 
               && task.rt_deadline == 0 
               && (task.cpu_affinity.is_none() || task.cpu_affinity == Some(idlest_cpu)) 
               && Some(id) != busy_rq.current_task_id() 
            {
                stolen_id = Some(id);
                break;
            }
        }
        
        if let Some(id) = stolen_id {
            if let Some(mut task) = busy_rq.remove(id) {
                let busy_min = busy_rq.min_vruntime();
                let idle_min = idle_rq.min_vruntime();
                
                let relative_vruntime = task.vruntime.saturating_sub(busy_min);
                task.vruntime = idle_min.saturating_add(relative_vruntime);
                task.deadline = task.vruntime + task.slice;
                
                idle_rq.insert(task);
                drop(busy_rq);
                drop(idle_rq);
                
                if idlest_cpu as u32 != current_cpu() as u32 {
                    crate::arch::acpi::send_fixed_ipi(idlest_cpu as u32, crate::arch::idt::IPI_VECTOR);
                }
            }
        }
    }
}

pub fn reschedule(frame: &mut TrapFrame) {
    crate::arch::acpi::eoi();
    let cpu = current_cpu();
    let mut rq = RUNQUEUES[cpu].lock();
    let ticks_per_10ms = crate::arch::timer::get_ticks_per_10ms();
    
    rq.update_vruntime(ticks_per_10ms);
    
    let current_id = rq.current_task_id();
    let next_id = rq.pick_next();
    
    if let Some(next_id) = next_id {
        if Some(next_id) != current_id {
            // FIX: Extract old_pid IMMUTABLY before we do any mutable borrows of `rq`.
            // If current_id is None (e.g., coming from idle), old_pid will be None.
            let old_pid = current_id.and_then(|id| rq.tasks().get(&id)).map(|t| t.process.pid);
            
            // 1. Save old task context (Mutable borrow #1, scoped and dropped)
            if let Some(curr_id) = current_id {
                if let Some(old_task) = rq.tasks_mut().get_mut(&curr_id) {
                    old_task.ctx.frame = *frame;
                    if old_task.state == TaskState::Running {
                        old_task.state = TaskState::Runnable;
                    }
                    unsafe { core::arch::x86_64::_fxsave64(old_task.ctx.fpu_state.area.as_mut_ptr()); }
                }
            }
            
            // 2. Load new task context (Mutable borrow #2)
            if let Some(new_task) = rq.tasks_mut().get_mut(&next_id) {
                new_task.state = TaskState::Running;
                *frame = new_task.ctx.frame;
                unsafe { core::arch::x86_64::_fxrstor64(new_task.ctx.fpu_state.area.as_ptr()); }
                
                let cpu = current_cpu();
                crate::arch::gdt::set_kernel_stack(cpu, new_task.kernel_stack_top as u64);
                crate::arch::percpu::set_kernel_stack(new_task.kernel_stack_top);
                
                let new_pid = new_task.process.pid;
                
                // 3. Switch CR3 if process changed (or if coming from idle where old_pid is None)
                if old_pid != Some(new_pid) {
                    let new_cr3 = new_task.process.address_space.lock().exco.cr3;
                    unsafe {
                        core::arch::asm!(
                            "mov cr3, {}",
                            in(reg) new_cr3,
                            options(nostack, preserves_flags)
                        );
                    }
                }
                rq.set_current(next_id);
            }
        }
    } else {
        // No next task, just update the current task's frame (e.g. idle task)
        if let Some(curr_id) = current_id {
            if let Some(curr_task) = rq.tasks_mut().get_mut(&curr_id) {
                curr_task.ctx.frame = *frame;
            }
        }
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn yield_wrapper() -> ! {
    naked_asm!(
        "mov rax, [rsp + 8]", "and rax, 3", "cmp rax, 3", "jne 1f", "swapgs", "1:",
        "push r15", "push r14", "push r13", "push r12", "push r11", "push r10", "push r9", "push r8",
        "push rbp", "push rdi", "push rsi", "push rdx", "push rcx", "push rbx", "push rax",
        "mov rdi, rsp", "call {scheduler_tick}",
        "pop rax", "pop rbx", "pop rcx", "pop rdx", "pop rsi", "pop rdi", "pop rbp", "pop r8",
        "pop r9", "pop r10", "pop r11", "pop r12", "pop r13", "pop r14", "pop r15",
        "mov rax, [rsp + 8]", "and rax, 3", "cmp rax, 3", "jne 2f", "swapgs", "2:", "iretq",
        scheduler_tick = sym reschedule,
    );
}

pub fn current_process() -> Option<Arc<Process>> {
    let cpu = current_cpu();
    let rq = RUNQUEUES[cpu].lock();
    if let Some(id) = rq.current_task_id() && let Some(task) = rq.tasks().get(&id) {
        return Some(task.process.clone());
    }
    None
}

pub fn native_syscall_handler(frame: &mut TrapFrame) {
    match frame.rax {
        0 => { yield_now(); frame.rax = 0; }
        1 => {
            let code = frame.rdi as i32;
            crate::info!("[Native SFD] Process {} exiting with code {}", current_process().unwrap().pid, code);
            exit(code);
        }
        _ => { crate::warn!("[Native SFD] Unknown syscall: {}", frame.rax); frame.rax = u64::MAX; }
    }
}

pub fn syscall_dispatcher(frame: &mut TrapFrame) {
    let proc = match current_process() {
        Some(p) => p,
        None => { crate::error!("Syscall from unknown context!"); frame.rax = u64::MAX; return; }
    };
    (proc.syscall_handler)(frame);
}

pub fn handle_page_fault(addr: usize, error_code: u64, rip: u64, is_user: bool) {
    let task_name = match RUNQUEUES[current_cpu()].lock().current_task() {
        Some(t) => t.name,
        None => "unknown",
    };

    let is_present = (error_code & 0x1) != 0;
    let is_write   = (error_code & 0x2) != 0;

    let proc = match current_process() {
        Some(p) => p,
        None => {
            __panic_msg!(
                "Page fault in unknown context (no current process), task {}, RIP {:018x}, addr {:018x}, code {:x} from {}",
                task_name, rip, addr, error_code, if is_user { "userspace" } else { "kernelspace" }
            );
            exit(-2);
        }
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
                crate::info!("Process {} SEGFAULT: Write to Read-Only VMA at {:#X}, task {}", proc.pid, addr, task_name);
                exit(139);
            }
            drop(vmm); 
            
            let paddr = crate::mem::upa::alloc(1);
            if paddr.to_raw() == 0 { panic!("OOM during Demand Paging"); }
            
            let vaddr_ptr = paddr.to_virt().to_ptr_mut::<u8>();
            unsafe { core::ptr::write_bytes(vaddr_ptr, 0, 4096); }
            
            let mut ptm = proc.address_space.lock();
            let mut flags = EntryFlags::PRESENT | EntryFlags::USER_ACCESSIBLE;
            if is_write_vma { flags |= EntryFlags::WRITABLE; }
            ptm.map_4k_block(addr & !0xFFF, paddr, flags).unwrap();
            return;
        }
    }

    crate::info!("Process {} SEGFAULT at {:#X} (RIP: {:#X}, code: {:#X}, task: {})", proc.pid, addr, rip, error_code, task_name);
    exit(139);
}

pub fn set_rt_deadline(task_id: TaskId, deadline_ms: u64) {
    for cpu in 0..crate::arch::num_cpus() {
        let mut rq = RUNQUEUES[cpu].lock();
        if rq.tasks().contains_key(&task_id) {
            rq.set_rt_deadline(task_id, deadline_ms);
            return;
        }
    }
    crate::warn!("set_rt_deadline: task {} not found", task_id.0);
}
