//! # Per‑CPU Data Management
//!
//! This module provides infrastructure for per‑CPU data structures on x86_64.
//! Each CPU core has its own dedicated data region, accessible via the `gs`
//! segment register. This allows efficient, lock‑free access to CPU‑local
//! data such as the current CPU ID, kernel stack top, and user stack pointer.
//!
//! ## Overview
//!
//! On multi‑core systems, many data structures need to be per‑CPU to avoid
//! contention and to maintain correctness. Examples include:
//! - Current CPU ID.
//! - Kernel stack top for the current task (for syscall entry).
//! - User stack pointer (for returning to user mode).
//! - Per‑CPU runqueues and caches.
//!
//! The x86_64 architecture provides the `gs` segment register, which can be
//! set to point to a per‑CPU data region via the `IA32_GS_BASE` and
//! `IA32_KERNEL_GS_BASE` MSRs. This module manages those MSRs and provides
//! a convenient interface to the per‑CPU data.
//!
//! ## Structure
//!
//! The per‑CPU data is defined in the `PerCpu` struct:
//!
//! ```text
//! struct PerCpu {
//!     user_rsp: u64,           // User stack pointer (for syscall return)
//!     kernel_stack_top: u64,   // Kernel stack top (for syscall entry)
//!     cpu_id: usize,           // CPU ID (0 .. MAX_CPUS-1)
//! }
//! ```
//!
//! This structure is stored in a static array `PERCPU_AREA`, one entry per CPU.
//! The size of the array is `MAX_CPUS` (currently 64). The structure is
//! cache‑line aligned (64 bytes) to prevent false sharing.
//!
//! ## Accessing Per‑CPU Data
//!
//! The per‑CPU data for the current CPU is accessed via `percpu::current()`.
//! This function:
//! 1. Calls `arch::current_cpu()` to get the current CPU ID (using `rdpid`
//!    or `rdmsr(IA32_TSC_AUX)`).
//! 2. Returns a mutable reference to `PERCPU_ARRAY[cpu_id]`.
//!
//! The `gs` segment is set up by the `percpu::init_syscall_gs()` function,
//! which writes `IA32_KERNEL_GS_BASE` to point to the per‑CPU data for that
//! core. The `swapgs` instruction is then used by the syscall entry/exit
//! handlers to switch between the user `gs` and the kernel `gs`.
//!
//! ## Initialization
//!
//! - **BSP**: `percpu::init()` is called during `arch::init_bsp()`. It sets
//!   `cpu_id` for CPU 0.
//! - **APs**: `percpu::init()` is called during `arch::init_ap()` for each AP.
//!   It sets `cpu_id` for that core.
//! - **Syscall GS**: `percpu::init_syscall_gs(cpu_id, kernel_stack_top)` is
//!   called during both BSP and AP init. It sets `IA32_KERNEL_GS_BASE` to
//!   point to the per‑CPU data for that core.
//!
//! ## Safety
//!
//! - The `PERCPU_ARRAY` is `static mut` and is accessed via raw pointers in
//!   `current()`. This is safe because each CPU accesses only its own entry,
//!   and the array is never deallocated.
//! - The MSR writes (`wrmsr`) are privileged operations and require that the
//!   kernel is running in Ring 0.
//! - The `gs` segment is used in interrupt and syscall handlers; the `swapgs`
//!   instruction must be used correctly to switch between user and kernel GS.

use crate::arch::current_cpu;
use crate::arch::MAX_CPUS;

// ============================================================================
// PER‑CPU STRUCTURE
// ============================================================================

/// Per‑CPU data structure.
///
/// This structure holds CPU‑local information. It is aligned to 64 bytes
/// to avoid false sharing between CPU cores.
///
/// # Fields
/// - `user_rsp`: The user‑mode stack pointer. Set when returning to user space.
/// - `kernel_stack_top`: The top of the kernel stack for the current task.
///   Used by the syscall entry handler to switch to the kernel stack.
/// - `cpu_id`: The ID of the current CPU (0 .. MAX_CPUS-1).
#[repr(C, align(64))]
#[derive(Debug)]
pub struct PerCpu {
    pub user_rsp: u64,
    pub kernel_stack_top: usize,
    pub cpu_id: usize,
}

impl PerCpu {
    /// Creates a new, zero‑initialized `PerCpu` structure.
    pub const fn new() -> Self {
        Self {
            cpu_id: 0,
            kernel_stack_top: 0,
            user_rsp: 0,
        }
    }
}

// ============================================================================
// GLOBAL PER‑CPU ARRAY
// ============================================================================

/// Static array of per‑CPU data, one entry per CPU.
///
/// This array is indexed by CPU ID. The entries are initialized to zero
/// and are filled in during CPU initialization.
///
/// # Safety
/// This is `static mut`; it is written during early boot (single‑threaded)
/// and read thereafter in a per‑CPU manner.
static mut PERCPU_AREA: [PerCpu; MAX_CPUS] = [const { PerCpu::new() }; MAX_CPUS];

// ============================================================================
// INITIALIZATION
// ============================================================================

/// Initializes the per‑CPU data for the current CPU.
///
/// This function sets the `cpu_id` field of the current CPU's `PerCpu` entry.
/// It is called during both BSP and AP initialization.
///
/// # Safety
/// This function uses `current_cpu()` to get the CPU ID and mutably borrows
/// the `PERCPU_ARRAY` entry. It is safe because it is called only once per CPU
/// during boot, with interrupts disabled.
pub fn init() {
    current().cpu_id = current_cpu();
}

// ============================================================================
// PUBLIC API
// ============================================================================

/// Returns a mutable reference to the per‑CPU data for the current CPU.
///
/// This function:
/// 1. Calls `arch::current_cpu()` to get the current CPU ID.
/// 2. Returns a reference to `PERCPU_ARRAY[cpu_id]`.
///
/// # Panics
/// Panics if the CPU ID is out of bounds (>= MAX_CPUS).
///
/// # Safety
/// The returned reference is `'static` and mutable. It is safe because:
/// - Each CPU accesses only its own entry.
/// - The array is never deallocated or moved.
/// - The reference is used only within the context of the current CPU.
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

/// Sets up the `IA32_KERNEL_GS_BASE` MSR for the current CPU.
///
/// This function:
/// 1. Writes the per‑CPU data address for the current CPU into `IA32_KERNEL_GS_BASE`.
/// 2. Also sets `kernel_stack_top` in the per‑CPU data.
///
/// The `IA32_KERNEL_GS_BASE` MSR holds the base address of the kernel's `gs`
/// segment. On syscall entry, the `swapgs` instruction switches from the
/// user `gs` to this kernel `gs`, making the per‑CPU data accessible.
///
/// # Arguments
/// * `cpu_id` – The CPU core index.
/// * `kernel_stack_top` – The top of the kernel stack for this CPU.
///
/// # Panics
/// Panics if `cpu_id >= MAX_CPUS`.
///
/// # Safety
/// This function uses `wrmsr` to write an MSR, which is a privileged operation.
pub fn init_syscall_gs(cpu_id: usize, kernel_stack_top: usize) {
    unsafe {
        PERCPU_AREA[cpu_id].kernel_stack_top = kernel_stack_top;
        // IA32_KERNEL_GS_BASE (0xC0000102) points to our per‑CPU data.
        crate::arch::wrmsr(0xC0000102, &PERCPU_AREA[cpu_id] as *const _ as u64);
    }
}

/// Sets the kernel stack top in the per‑CPU data for the current CPU.
///
/// This is used by the scheduler when switching to a new task to update
/// the kernel stack pointer that will be used on the next syscall or interrupt.
///
/// # Arguments
/// * `stack_top` – The top address of the kernel stack (the stack grows down).
pub fn set_kernel_stack(stack_top: usize) {
    current().kernel_stack_top = stack_top;
}
