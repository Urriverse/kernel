//! # Scheduler Subsystem (EEVDF)
//!
//! This module implements the kernel scheduler based on the **Earliest Eligible Virtual Deadline First (EEVDF)** algorithm.
//! It manages task execution, preemption, and CPU time distribution across all cores.
//!
//! ## Overview
//!
//! The scheduler is responsible for:
//!
//! - **Task Management**: Creating, running, blocking, and terminating tasks.
//! - **CPU Scheduling**: Selecting the next task to run on each CPU using the EEVDF policy.
//! - **Synchronization**: Providing primitives for waiting and waking tasks.
//! - **Process Support**: Managing process IDs, address spaces, and system call handling.
//!
//! ## Key Concepts
//!
//! - **Task**: A schedulable unit of execution, either kernel or user mode. Each task has a
//!   `TaskId`, priority (`Priority`), weight, virtual runtime (`vruntime`), and deadline.
//! - **EEVDF**: Tasks are scheduled based on a virtual deadline. Each task gets a time slice
//!   (`slice`) and is assigned a deadline = `vruntime + slice`. The scheduler picks the task
//!   with the earliest deadline that is eligible (`vruntime <= min_vruntime`).
//! - **Runqueue**: Per-CPU queue of runnable tasks, organized by deadline in a `BTreeSet`.
//!   Each runqueue also tracks the current task and the minimum vruntime.
//! - **Process**: A container for tasks, address space, VMAs, and syscall handler.
//!   Each task belongs to a process (`Arc<Process>`).
//! - **WaitQueue**: A list of tasks waiting for an event; used for `sleep`/`wakeup`.
//! - **Zombie Reaping**: Tasks that exit become zombies and are reaped by the `reaper` task.
//!
//! ## Scheduling Algorithm (EEVDF)
//!
//! 1. Each task has a `vruntime` (accumulated virtual runtime) and a `slice` (time quota).
//! 2. When scheduled, the task runs until its `vruntime` reaches its `deadline` (`deadline = vruntime + slice`).
//! 3. The task with the smallest `deadline` among runnable tasks is selected (Earliest Deadline First).
//! 4. To prevent starvation, the runqueue maintains a `min_vruntime`; tasks with `vruntime` below it are
//!    considered eligible and get priority.
//! 5. On each timer tick (10 ms), the current task's `vruntime` is updated by:
//!    `delta_vruntime = delta_real_time * (NICE_0_WEIGHT / weight)`
//!    (where weight depends on priority, and `NICE_0_WEIGHT = 1024`).
//! 6. If the task's `vruntime >= deadline`, a new deadline is computed: `deadline = vruntime + slice`.
//!
//! ## CPU Affinity
//!
//! Each task can specify a preferred CPU (`cpu_affinity`). If `None`, it can run on any core.
//! The scheduler respects affinity when selecting a runqueue.
//!
//! ## Synchronization Primitives
//!
//! - **`sleep(wq)`**: Moves the current task to a wait queue and yields the CPU.
//! - **`wakeup(wq)`**: Wakes up one task from the wait queue, making it runnable.
//! - **`wait_child(id)`**: Waits for a specific child task to exit.
//! - **`wait_any()`**: Waits for any child task to exit.
//!
//! ## System Calls (Native ABI)
//!
//! - **sys_yield** (0): Yield the CPU voluntarily.
//! - **sys_exit** (1): Exit the current process with a code.
//!
//! ## Page Fault Handling
//!
//! The scheduler handles page faults via `handle_page_fault`:
//! - **Copy-on-Write (CoW)**: If a write fault occurs on a page with `COPY_ON_WRITE` flag,
//!   a private copy is created.
//! - **Demand Paging**: If the fault address falls within a VMA, a new physical page is allocated
//!   and mapped with appropriate permissions.
//! - **Segmentation Fault**: If the fault cannot be resolved, the process is terminated with SIGSEGV.
//!
//! ## Initialization
//!
//! `sched::init(ticks_per_10ms)` is called by the BSP after all subsystems are ready.
//! It initializes the per-CPU runqueues, creates an idle task for each CPU, and sets up
//! the current task. The scheduler is then ready to run.
//!
//! ## Safety
//!
//! - The runqueues are protected by `Nitex` (interrupt‑disabling spinlocks) to ensure
//!   safe concurrent access from multiple CPUs.
//! - Task registry (`TASK_REGISTRY`) is protected by a `Nutex` (also interrupt‑disabling).
//! - The scheduler uses inline assembly for context switching and interrupt handling.
//! - The `yield_wrapper` and `timer_wrapper` are naked functions that manipulate the stack
//!   and trap frames directly.

// ============================================================================
// SUBMODULES
// ============================================================================

pub mod task;   // Task structure, TaskId, Priority, Context
pub mod rq;     // Runqueue (per-CPU), EEVDF logic
pub mod wq;     // WaitQueue for task sleeping
pub mod proc;   // Process structure, address space, VMM, root FS

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

/// Global registry of all tasks (including zombies).
///
/// This map is used to look up tasks by ID, re-parent orphans, and reap zombies.
pub static TASK_REGISTRY: Nutex<BTreeMap<TaskId, Box<Task>>> = Nutex::new(BTreeMap::new());

/// Wait queue for tasks waiting for any child to exit.
pub static EXIT_WQ: Nutex<WaitQueue> = Nutex::new(WaitQueue::new());

/// Ticks per millisecond (derived from HPET calibration).
static TICKS_PER_MS: AtomicU64 = AtomicU64::new(0);

// ============================================================================
// INITIALIZATION
// ============================================================================

/// Initializes the scheduler.
///
/// # Arguments
/// * `ticks_per_10ms` – Number of APIC timer ticks per 10 ms (calibrated in `timer::init`).
///
/// # Operations
/// 1. Stores `ticks_per_10ms / 10` as ticks per ms.
/// 2. For each CPU, allocates a kernel stack and creates an idle task.
/// 3. Inserts the idle task into the CPU's runqueue and sets it as current.
/// 4. Sets the kernel stack for the BSP's per‑CPU data.
///
/// # Panics
/// If stack allocation fails (should not happen).
pub fn init(ticks_per_10ms: u64) {
    TICKS_PER_MS.store(ticks_per_10ms / 10, Ordering::Release);

    for (cpu, _) in RUNQUEUES.iter().enumerate().take(crate::arch::num_cpus()) {
        let stack = allocate_kernel_stack(16 * 1024);
        let idle = Task::new_kernel(idle_task, stack, Priority(19), "idle");
        let mut rq = RUNQUEUES[cpu].lock();
        rq.set_current(idle.id);
        rq.insert(idle);

        // Set kernel stack for the BSP (CPU 0) – other CPUs will set it when they start.
        if cpu == crate::arch::current_cpu() {
            crate::arch::percpu::set_kernel_stack(stack as u64);
        }
    }

    crate::info!("Scheduler initialized with EEVDF");
}

/// Allocates a kernel stack of the given size.
///
/// Returns the top address of the stack (the stack grows downward).
fn allocate_kernel_stack(size: usize) -> usize {
    let pages = size.div_ceil(4096);
    let paddr = crate::mem::upa::alloc(pages);
    if paddr.to_raw() == 0 {
        panic!("Failed to allocate kernel stack");
    }
    paddr.to_virt().to_raw() + size
}

/// Idle task – runs when no other task is runnable.
///
/// It simply halts the CPU, waiting for interrupts.
fn idle_task() {
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

// ============================================================================
// TASK SPAWNING
// ============================================================================

/// Spawns a new kernel task.
///
/// # Arguments
/// * `entry` – The function to run (must never return, or call `exit`).
/// * `priority` – The priority (niceness) of the task.
/// * `name` – Static name for debugging.
/// * `root` – Optional `RootRef` (VFS root) for the process.
///
/// # Returns
/// The `TaskId` of the newly spawned task.
///
/// # Notes
/// - The task's process is cloned from the current process (if any) or a default
///   process. The `root` is set if provided.
/// - The task is inserted into the runqueue of the current CPU.
/// - The parent is set to the current task.
pub fn spawn_kernel_task(entry: fn(), priority: Priority, name: &'static str, root: Option<RootRef>) -> TaskId {
    let stack = allocate_kernel_stack(32 * 1024);
    let mut task = Task::new_kernel(entry, stack, priority, name);

    if let Some(x) = root {
        let mut proc;
        if let Some(p) = current_process() {
            proc = (*p).clone()
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
    RUNQUEUES[cpu].lock().insert(task);
    id
}

// ============================================================================
// TASK EXIT & WAITING
// ============================================================================

/// Terminates the current task with the given exit code.
///
/// This function:
/// 1. Removes the task from its CPU's runqueue.
/// 2. Re‑parents any children to the init task (TaskId(1)).
/// 3. Marks the task as `Zombie` and stores it in the global task registry.
/// 4. Wakes up any waiters (via `EXIT_WQ`).
/// 5. Yields the CPU (never returns).
///
/// # Note
/// The function never returns; it yields and eventually the task is reaped.
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

/// Yields the CPU voluntarily (calls `int 33`).
#[inline(always)]
pub fn yield_now() {
    unsafe {
        core::arch::asm!("int 33");
    }
}

/// Puts the current task to sleep on a wait queue.
///
/// The task is removed from the runqueue and added to the wait queue.
/// It will be woken by `wakeup` on the same wait queue.
pub fn sleep(wq: &Nutex<WaitQueue>) {
    let cpu = crate::arch::current_cpu();
    let mut rq = RUNQUEUES[cpu].lock();
    let current_id = rq.current_task_id().unwrap();
    if let Some(mut task) = rq.remove(current_id) {
        task.state = TaskState::Sleeping;
        wq.lock().sleep(task.id);
        rq.insert(task);
    }
    drop(rq);
    yield_now();
}

/// Wakes up one task from a wait queue.
///
/// The task is removed from the wait queue and made runnable.
pub fn wakeup(wq: &Nutex<WaitQueue>) {
    let cpu = crate::arch::current_cpu();
    let mut rq = RUNQUEUES[cpu].lock();

    if let Some(task_id) = wq.lock().wakeup_one()
    && let Some(mut task) = rq.remove(task_id) {
        task.state = TaskState::Runnable;
        rq.insert(task);
    }
}

/// Waits for a specific child task to exit.
///
/// # Returns
/// The exit code of the child.
///
/// # Notes
/// This function blocks the current task until the child becomes a zombie,
/// then removes it from the registry and returns its exit code.
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

/// Waits for any child task to exit.
///
/// # Returns
/// `Some((TaskId, exit_code))` if a zombie child exists, or waits indefinitely.
///
/// # Notes
/// The zombie is removed from the registry before returning.
pub fn wait_any() -> Option<(TaskId, i32)> {
    loop {
        let mut registry = TASK_REGISTRY.lock();

        let zombie_id = registry.iter()
            .find(|(_, t)| t.state == TaskState::Zombie)
            .map(|(id, _)| *id);

        if let Some(id) = zombie_id {
            let task = registry.remove(&id).unwrap();
            return Some((id, task.exit_code));
        }
        drop(registry);
        sleep(&EXIT_WQ);
    }
}

// ============================================================================
// SCHEDULER CORE
// ============================================================================

/// Timer tick handler – called by the APIC timer interrupt (vector 32).
///
/// This function:
/// 1. Updates the system time (on BSP only).
/// 2. Calls `reschedule` to perform scheduling decisions.
pub fn timer_tick(frame: &mut TrapFrame) {
    if crate::arch::current_cpu() == 0 {
        crate::arch::TIME_FROM_BOOT.fetch_add(10, Ordering::Relaxed);
    }

    reschedule(frame);
}

/// Reschedules the current CPU.
///
/// This is the heart of the scheduler:
/// 1. Sends EOI to the APIC.
/// 2. Updates the current task's vruntime (using `rq.update_vruntime`).
/// 3. Picks the next task (using `rq.pick_next`).
/// 4. If the picked task is different from the current:
///    - Saves the current task's FPU state and trap frame.
///    - Loads the new task's FPU state and trap frame.
///    - If the process ID changed, switches CR3 to the new address space.
///    - Sets the current task in the runqueue.
/// 5. If the same task continues, updates its trap frame.
///
/// # Arguments
/// * `frame` – The trap frame from the interrupt (used to save/restore context).
///
/// # Safety
/// This function uses inline assembly to switch CR3 and modify registers.
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
                crate::arch::percpu::set_kernel_stack(new_task.kernel_stack_top as u64);

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
        } else {
            if let Some(curr_id) = current_id
            && let Some(curr_task) = rq.tasks_mut().get_mut(&curr_id) {
                curr_task.ctx.frame = *frame;
            }
        }
    }
}

/// Naked interrupt wrapper for `yield_now` (vector 33).
///
/// This function is called via `int 33` and performs the same context
/// save/restore as the timer interrupt, then calls `reschedule`.
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

/// Returns the current process (if any).
pub fn current_process() -> Option<Arc<Process>> {
    let cpu = crate::arch::current_cpu();
    let rq = RUNQUEUES[cpu].lock();
    if let Some(id) = rq.current_task_id()
    && let Some(task) = rq.tasks().get(&id) {
        return Some(task.process.clone());
    }
    None
}

/// Native system call handler (calls `syscall_dispatcher`).
///
/// This is the default syscall handler for processes.
/// It interprets `rax` as the syscall number and `rdi`, `rsi`, `rdx` as arguments.
pub fn native_syscall_handler(frame: &mut TrapFrame) {
    match frame.rax {
        0 => {
            // sys_yield
            yield_now();
            frame.rax = 0;
        }
        1 => {
            // sys_exit
            let code = frame.rdi as i32;
            crate::info!("[Native SFD] Process {} exiting with code {}", current_process().unwrap().pid, code);
            exit(code);
        }
        _ => {
            crate::warn!("[Native SFD] Unknown syscall: {}", frame.rax);
            frame.rax = u64::MAX; // -ENOSYS
        }
    }
}

/// Syscall dispatcher – called from the syscall entry point.
///
/// It retrieves the current process and delegates to its `syscall_handler`.
pub fn syscall_dispatcher(frame: &mut TrapFrame) {
    let proc = match current_process() {
        Some(p) => p,
        None => {
            crate::error!("Syscall from unknown context!");
            frame.rax = u64::MAX; // -ENOSYS
            return;
        }
    };

    (proc.syscall_handler)(frame);
}

// ============================================================================
// PAGE FAULT HANDLING
// ============================================================================

/// Handles page faults (from the page fault handler in IDT).
///
/// # Arguments
/// * `addr` – The faulting virtual address.
/// * `error_code` – Page fault error code (bits: present, write, user, etc.)
/// * `rip` – Instruction pointer that caused the fault.
/// * `_is_user` – Whether the fault occurred in user mode.
///
/// # Handling
/// - **Copy‑on‑Write**: If the fault is a write to a `COPY_ON_WRITE` page,
///   a private copy is made and the mapping is updated.
/// - **Demand Paging**: If the address falls within a VMA that allows access,
///   a new physical page is allocated and mapped.
/// - **Segmentation Fault**: Otherwise, the process is terminated with exit code 139 (SIGSEGV).
///
/// # Panics
/// - If the fault occurs in kernel mode (not user) and cannot be resolved.
/// - If the current process is unknown.
pub fn handle_page_fault(addr: usize, error_code: u64, rip: u64, _is_user: bool) {
    let is_present = (error_code & 0x1) != 0;
    let is_write   = (error_code & 0x2) != 0;

    // Intentional no-panic on kernel segfaults; if from module, just kill it.
    // if !is_user {
    //     panic!("KERNEL PAGE FAULT at {:#X} (code: {:#X}) RIP: {:#X}", addr, error_code, rip);
    // }

    let proc = match current_process() {
        Some(p) => p,
        None => panic!("Page fault in unknown context (no current process)"),
    };

    // Copy‑on‑Write
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

    // Demand paging
    if !is_present {
        let vmm = proc.vmm.lock();
        if let Some(vma) = vmm.find_overlap(addr) {
            // VMA access rights check
            let is_write_vma = vma.flags.contains(VmaFlags::WRITE);
            if is_write && !is_write_vma {
                drop(vmm);
                crate::info!("Process {} SEGFAULT: Write to Read-Only VMA at {:#X}", proc.pid, addr);
                exit(139);
            }
            drop(vmm); // free the lock before allocation

            // allocate phys page
            let paddr = crate::mem::upa::alloc(1);
            if paddr.to_raw() == 0 { panic!("OOM during Demand Paging"); }

            // zero it out
            let vaddr_ptr = paddr.to_virt().to_ptr_mut::<u8>();
            unsafe { core::ptr::write_bytes(vaddr_ptr, 0, 4096); }

            // map to address space
            let mut ptm = proc.address_space.lock();
            let mut flags = EntryFlags::PRESENT | EntryFlags::USER_ACCESSIBLE;
            if is_write_vma {
                flags |= EntryFlags::WRITABLE;
            }
            ptm.map_4k_block(addr & !0xFFF, paddr, flags).unwrap();
            return;
        }
    }

    // Segfault actually
    crate::info!("Process {} SEGFAULT at {:#X} (RIP: {:#X}, code: {:#X})", proc.pid, addr, rip, error_code);
    exit(139);
}
