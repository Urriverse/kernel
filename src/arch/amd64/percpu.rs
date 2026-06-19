use crate::arch::current_cpu;
use crate::arch::MAX_CPUS;

#[repr(C, align(64))]
#[derive(Debug)]
pub struct PerCpu {
    pub user_rsp: u64,
    pub kernel_stack_top: u64,
    pub cpu_id: usize,
}

impl PerCpu {
    pub const fn new() -> Self {
        Self {
            cpu_id: 0,
            kernel_stack_top: 0,
            user_rsp: 0,
        }
    }
}

static mut PERCPU_AREA: [PerCpu; MAX_CPUS] = [const { PerCpu::new() }; MAX_CPUS];

pub fn init() {
    current().cpu_id = current_cpu();
}

#[inline(always)]
pub fn current() -> &'static mut PerCpu {
    let cpu_id = current_cpu();
    debug_assert!(
        cpu_id < MAX_CPUS, 
        "CPU ID {} exceeds MAX_CPUS ({})", 
        cpu_id, 
        MAX_CPUS
    );
    
    #[allow(static_mut_refs)]
    unsafe {
        &mut *PERCPU_AREA.as_mut_ptr().add(cpu_id)
    }
}

pub fn init_syscall_gs(cpu_id: usize, kernel_stack_top: u64) {
    unsafe {
        PERCPU_AREA[cpu_id].kernel_stack_top = kernel_stack_top;
        // IA32_KERNEL_GS_BASE (0xC0000102) указывает на наши per-cpu данные
        crate::arch::wrmsr(0xC0000102, &PERCPU_AREA[cpu_id] as *const _ as u64);
    }
}

pub fn set_kernel_stack(stack_top: u64) {
    current().kernel_stack_top = stack_top;
}
