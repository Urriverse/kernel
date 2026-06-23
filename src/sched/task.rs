// src/sched/task.rs
use crate::{arch::trap::TrapFrame, sched::proc::Process};
use core::sync::atomic::{AtomicU64, Ordering};
use alloc::{boxed::Box, sync::Arc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
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
        // weight = 1024 * 1.25^(-nice)
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

#[derive(Clone)]
#[allow(dead_code)]
pub struct Task {
    pub id: TaskId,
    pub state: TaskState,
    pub vruntime: u64,
    pub deadline: u64,
    pub weight: u64,
    pub slice: u64,           // Requested time slice in virtual ticks
    pub ctx: Context,
    pub kernel_stack: usize,  // Top of kernel stack for this task
    pub user_stack: usize,    // Top of user stack (if user task)
    pub cpu_affinity: Option<usize>,  // None = any CPU
    pub name: &'static str,
    pub parent: Option<TaskId>,
    pub exit_code: i32,
    pub process: Arc<Process>,
    pub kernel_stack_top: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Context {
    pub frame: TrapFrame,
    pub fpu_state: FpuState,
}

#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct FpuState {
    pub area: [u8; 512],  // FXSAVE area (SSE/SSE2)
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

impl Task {
    pub fn new_kernel(
        entry: fn(),
        kernel_stack_top: usize,
        priority: Priority,
        name: &'static str,
    ) -> Box<Self> {
        let id = TaskId(NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed));
        let mut frame = unsafe { core::mem::zeroed::<TrapFrame>() };
        
        let initial_rsp = kernel_stack_top - 8;
        unsafe {
            *(initial_rsp as *mut u64) = 0; 
        }
        
        frame.rip = entry as *const () as u64;
        frame.rsp = initial_rsp as u64;
        frame.cs = 0x08;  // KERNEL_CODE_SELECTOR
        frame.ss = 0x10;  // KERNEL_DATA_SELECTOR
        frame.rflags = 0x202;  // IF=1
        
        Box::new(Self {
            id,
            state: TaskState::Runnable,
            vruntime: 0,
            deadline: 0,
            weight: Priority::nice_to_weight(priority.0),
            slice: 10_000,
            ctx: Context {
                frame,
                fpu_state: FpuState::default(),
            },
            kernel_stack: kernel_stack_top,
            user_stack: 0,
            cpu_affinity: None,
            name,
            parent: None,
            exit_code: -1,
            process: Arc::new(Process::new()),
            kernel_stack_top: 0,
        })
    }
}
