// src/sched/task.rs
use crate::{arch::{current_cpu, trap::TrapFrame}, sched::{self, alloc_kstack, current_process, exit, proc::Process}};
use core::sync::atomic::{AtomicU64, Ordering};
use alloc::{borrow::ToOwned, boxed::Box, string::{String, ToString}, sync::Arc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Runnable,
    Running,
    Sleeping,
    Blocked,
    Zombie,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Priority(pub i32);

impl Priority {
    pub const fn nice_to_weight(nice: i32) -> u64 {
        const WEIGHTS: [u64; 40] = [
            88761, 71755, 58481, 46236, 37617, 30483, 24513, 19862,
            16124, 13031, 10550, 8546, 6912, 5594, 4519, 3659,
            2958, 2389, 1934, 1563, 1274, 1024, 833, 672,
            546, 441, 356, 287, 232, 187, 151, 122,
            98, 79, 64, 51, 41, 33, 27, 22,
        ];
        let nice = nice.clamp(-20, 19);
        WEIGHTS[(nice + 20) as usize]
    }
}

#[derive(Clone, Debug)]
pub struct Task {
    pub id: TaskId,
    pub state: TaskState,
    pub vruntime: u64,
    pub deadline: u64,
    pub weight: u64,
    pub slice: u64,
    pub ctx: Context,
    pub kernel_stack: usize,
    pub user_stack: usize,
    pub cpu_affinity: Option<usize>,
    pub name: alloc::string::String,
    pub parent: Option<TaskId>,
    pub exit_code: i32,
    pub process: Arc<Process>,
    pub kernel_stack_top: usize,
    pub rt_deadline: u64,
    pub cpu_locality: u64,
    pub current_root: Box<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct Context {
    pub frame: TrapFrame,
    pub fpu_state: FpuState,
}

#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct FpuState {
    pub area: [u8; 512],
    pub initialized: bool,
}

impl Default for FpuState {
    fn default() -> Self {
        Self {
            area: [0; 512],
            initialized: false,
        }
    }
}

static NEXT_TASK_ID: AtomicU64 = AtomicU64::new(1);

fn spur() {
    // Safely grab the task name without causing lock contention/panics
    let task_name = {
        let rq = super::rq::RUNQUEUES[current_cpu()].lock();
        rq.current_task().map(|t| t.name.clone()).unwrap_or("unknown".to_owned())
    };
    __panic_msg!("Task \"{}\" didn't call `exit`", task_name);
    sched::exit(-3);
}

fn spur_entry() { exit(-5) }

impl Default for Task {
    fn default() -> Self {
        let id = TaskId(NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed));
        let mut frame = unsafe { core::mem::zeroed::<TrapFrame>() };
        let kernel_stack = alloc_kstack(1<<15);
        let initial_rsp = kernel_stack;
        unsafe {
            *(initial_rsp as *mut u64) = spur as *const () as u64;
        }
        frame.rip = spur_entry as *const () as u64;
        frame.rsp = initial_rsp as u64;
        frame.cs = 0x08;
        frame.ss = 0x10;
        // let gs: u16; unsafe { core::arch::asm! { "mov {:x}, gs", out(reg) gs } } frame.gs = gs as u64;
        frame.rflags = 0x202;
        
        Self {
            id,
            state: TaskState::Runnable,
            vruntime: 0,
            deadline: 0,
            weight: Priority::nice_to_weight(Priority(0).0),
            slice: 10_000,
            ctx: Context { frame, fpu_state: FpuState::default() },
            kernel_stack,
            user_stack: 0,
            cpu_affinity: None,
            name: "".to_string(),
            parent: None,
            exit_code: -1,
            process: match current_process() { Some(p) => p, None => Arc::new(Process::default()) },
            kernel_stack_top: initial_rsp,
            rt_deadline: 0,
            cpu_locality: 0,
            current_root: Box::new("initramfs".to_string()),
        }
    }
}

impl Task {
    pub fn new() -> Box<Self> { Box::new(Self::default()) }

    pub fn new_nanostack() -> Box<Self> {
        let id = TaskId(NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed));
        let mut frame = unsafe { core::mem::zeroed::<TrapFrame>() };
        let kernel_stack = alloc_kstack(1<<15);
        let initial_rsp = kernel_stack;
        unsafe {
            *(initial_rsp as *mut u64) = spur as *const () as u64;
        }
        frame.rip = spur_entry as *const () as u64;
        frame.rsp = initial_rsp as u64;
        frame.cs = 0x08;
        frame.ss = 0x10;
        // let gs: u16; unsafe { core::arch::asm! { "mov {:x}, gs", out(reg) gs } } frame.gs = gs as u64;
        frame.rflags = 0x202;
        
        Box::new(Self {
            id,
            state: TaskState::Runnable,
            vruntime: 0,
            deadline: 0,
            weight: Priority::nice_to_weight(Priority(0).0),
            slice: 10_000,
            ctx: Context { frame, fpu_state: FpuState::default() },
            kernel_stack,
            user_stack: 0,
            cpu_affinity: None,
            name: "".to_string(),
            parent: None,
            exit_code: -1,
            process: match current_process() { Some(p) => p, None => Arc::new(Process::default()) },
            kernel_stack_top: initial_rsp,
            rt_deadline: 0,
            cpu_locality: 0,
            current_root: Box::new("initramfs".to_string()),
        })
    }

    pub fn new_arg(
        entry: fn(usize),
        arg: usize,
        kernel_stack_top: usize,
        priority: Priority,
        name: alloc::string::String,
    ) -> Box<Self> {
        let id = TaskId(NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed));
        let mut frame = unsafe { core::mem::zeroed::<TrapFrame>() };
        let initial_rsp = kernel_stack_top - 8;
        unsafe {
            *(initial_rsp as *mut u64) = spur as *const () as u64;
        }
        frame.rip = entry as *const () as u64;
        frame.rsp = initial_rsp as u64;
        frame.rdi = arg as u64;
        frame.cs = 0x08;
        frame.ss = 0x10;
        // let gs: u16; unsafe { core::arch::asm! { "mov {:x}, gs", out(reg) gs } } frame.gs = gs as u64;
        frame.rflags = 0x202;
        
        Box::new(Self {
            id,
            state: TaskState::Runnable,
            vruntime: 0,
            deadline: 0,
            weight: Priority::nice_to_weight(priority.0),
            slice: 10_000,
            ctx: Context { frame, fpu_state: FpuState::default() },
            kernel_stack: kernel_stack_top,
            user_stack: 0,
            cpu_affinity: None,
            name,
            parent: None,
            exit_code: -1,
            process: match current_process() { Some(p) => p, None => Arc::new(Process::default()) },
            kernel_stack_top,
            rt_deadline: 0,
            cpu_locality: 0,
            current_root: Box::new("initramfs".to_string()),
        })
    }
}
