//! # Process Management
//!
//! This module defines the `Process` structure, which represents a user‑space process
//! in the kernel. A process is a container for tasks (threads) that share a common
//! address space, virtual memory areas, filesystem root, and syscall handler.
//!
//! ## Overview
//!
//! In the kernel, a **process** is the primary unit of resource ownership. Each process
//! has:
//!
//! - A unique process ID (`pid`).
//! - An optional parent process (`parent`).
//! - An address space (page tables) shared by all its threads.
//! - A virtual memory manager (`Vmm`) that tracks the process's memory regions.
//! - A list of threads (`threads`) that belong to the process.
//! - A syscall handler function that interprets system calls for the process.
//! - A root filesystem view (`RootRef`) for the process's mount namespace.
//! - A security level (`level`) for privilege separation.
//!
//! A process is **not** a schedulable entity by itself; instead, each process has
//! one or more tasks (threads) that are scheduled independently. All tasks in a
//! process share the same address space, VMM, and root filesystem.
//!
//! ## Relationship to Tasks
//!
//! ```text
//! Process (pid: 42)
//!   ├── Task (tid: 100)  <-- main thread
//!   ├── Task (tid: 101)  <-- worker thread
//!   └── Task (tid: 102)  <-- worker thread
//! ```
//!
//! Each `Task` contains an `Arc<Process>` that links it to its owning process.
//! This allows multiple tasks to share the same address space and resources.
//!
//! ## Address Space
//!
//! The process's address space is represented by a `Polen` (page table manager)
//! wrapped in an `Arc<Nutex<Polen>>`. This allows:
//! - Safe concurrent access from multiple threads.
//! - Copy‑on‑write semantics for `fork()` (cloning the address space).
//! - Isolation between processes (different CR3 values).
//!
//! ## Virtual Memory Areas (VMAs)
//!
//! The process's `Vmm` (Virtual Memory Manager) tracks all mapped memory regions
//! (VMAs) in a Red‑Black tree. This is used for:
//! - Demand paging: handling page faults by mapping pages from VMAs.
//! - Memory mapping: `mmap` and `munmap` operations.
//! - Access control: checking that faults occur within valid VMAs with correct permissions.
//!
//! ## Cloning
//!
//! Processes can be cloned (for `fork()`). The `Clone` implementation for `Process`
//! creates a new process with:
//! - A new, unique PID.
//! - The parent PID set to the original process's PID.
//! - A cloned address space (copy‑on‑write).
//! - A cloned VMM.
//! - The same syscall handler.
//! - A cloned root filesystem reference.
//! - An empty thread list.
//!
//! The actual cloning of the address space is deferred to the page table manager
//! (`Polen::dup()`), which performs a shallow copy of the page tables with
//! copy‑on‑write semantics.
//!
//! ## PID Allocation
//!
//! Process IDs are allocated from a global counter (`NEXT`), protected by a
//! `Litex` (interrupt‑disabling spinlock). The counter starts at 0 and is
//! incremented for each new process.
//!
//! ## Syscall Handler
//!
//! Each process has a `syscall_handler` function pointer. This allows different
//! processes to have different system call ABIs (e.g., native Linux‑style syscalls,
//! or a custom microkernel IPC interface). The default handler is
//! `sched::native_syscall_handler`.
//!
//! ## Root Filesystem
//!
//! Each process has a `RootRef` (an `Arc<RootReg>`) that defines its mount
//! namespace. This allows processes to have isolated views of the filesystem
//! (e.g., chroot, containers). By default, each process gets a new `RootReg`
//! that is independent of other processes.
//!
//! ## Security Level
//!
//! The `level` field represents the process's security level (a simple numeric
//! privilege level). This is used by the VFS to enforce access control:
//! files with `LEVEL_READ`, `LEVEL_WRITE`, or `LEVEL_EXEC` flags can only be
//! accessed by processes with a matching or higher level.
//!
//! ## Safety
//!
//! - The `Process` struct uses `Arc` for shared ownership of the address space
//!   and VMM. This ensures that the resources are not freed while any task is
//!   still using them.
//! - The `address_space` is wrapped in a `Nutex` (interrupt‑disabling spinlock)
//!   to protect against concurrent modifications from multiple threads.
//! - The `NEXT` PID counter uses a `Litex` for safe, interrupt‑safe increments.
//! - The `syscall_handler` is a function pointer that must be safe to call
//!   from interrupt context (it is called from the syscall dispatcher, which
//!   runs in the context of the calling thread).

use alloc::sync::Arc;
use alloc::vec::Vec;
use crate::arch::trap::TrapFrame;
use crate::mem::ptm::Polen;
use crate::mem::vma::Vmm;
use crate::sync::{Litex, Nutex};
use crate::vfs::{RootRef, RootReg};

// ============================================================================
// TYPE ALIASES
// ============================================================================

/// A process ID, which is a 32‑bit unsigned integer.
pub type ProcId = u32;

// ============================================================================
// PROCESS STRUCTURE
// ============================================================================

/// A user‑space process, representing a running program with its own address space.
///
/// A process is a container for tasks (threads) that share the same memory
/// space, filesystem view, and system call handler.
///
/// # Fields
/// * `pid` – The unique process ID.
/// * `parent` – The optional PID of the parent process (for `wait()` and reaping).
/// * `address_space` – The page tables for this process, shared by all its threads.
/// * `vmm` – The virtual memory manager, tracking all mapped regions (VMAs).
/// * `threads` – The list of task IDs that belong to this process.
/// * `syscall_handler` – The function called to handle system calls from this process.
/// * `roots` – The filesystem mount namespace (root registry).
/// * `level` – The security level of the process.
///
/// # Examples
/// ```ignore
/// let proc = Process::default();
/// let task = Task::new_user(entry, stack_top, kernel_stack_top, priority, "my_task");
/// // The task's process will be set to `proc`.
/// ```
pub struct Process {
    /// The unique process ID.
    pub pid: ProcId,

    /// The PID of the parent process, if any.
    pub parent: Option<ProcId>,

    /// The address space (page tables) for this process.
    ///
    /// This is shared among all threads in the process. It is wrapped in an `Arc`
    /// and a `Nutex` for safe concurrent access from multiple CPUs.
    pub address_space: Arc<Nutex<Polen>>,

    /// The virtual memory manager for this process.
    ///
    /// Tracks all mapped regions (VMAs) and is used for demand paging and
    /// memory management operations.
    pub vmm: Arc<Nutex<Vmm>>,

    /// The list of task IDs that belong to this process.
    pub threads: Vec<super::task::TaskId>,

    /// The system call handler for this process.
    ///
    /// This function pointer determines how system calls are interpreted.
    /// The default is `sched::native_syscall_handler`.
    pub syscall_handler: fn(&mut TrapFrame),

    /// The mount namespace (root registry) for this process.
    ///
    /// This defines which filesystems are mounted and where. Processes can
    /// have isolated mount namespaces for container‑like behaviour.
    pub roots: RootRef,

    /// The security level of the process.
    ///
    /// Used by the VFS to enforce access control on files with level‑based
    /// permissions (`LEVEL_READ`, `LEVEL_WRITE`, `LEVEL_EXEC`).
    pub level: u16,
}

// ============================================================================
// PID ALLOCATOR
// ============================================================================

/// The global PID counter, protected by a `Litex` (interrupt‑disabling spinlock).
static NEXT: Litex<u32> = Litex::new(0);

/// Allocates the next available PID.
///
/// This function locks the global counter, reads the current value, increments
/// it (using unsafe access to the inner data), and returns the old value.
///
/// # Returns
/// The next PID (starting from 0).
///
/// # Safety
/// The `unsafe` block is used to write to the inner data of the `Litex`.
/// This is safe because the lock is held for the duration of the operation.
fn next() -> u32 {
    let next = NEXT.lock();
    let rv = *next;
    unsafe { *NEXT.inner() = rv + 1 }
    rv
}

// ============================================================================
// PROCESS IMPLEMENTATION
// ============================================================================

impl Process {
    /// Creates a new `Process` with default values.
    ///
    /// The default process has:
    /// - A new, unique PID.
    /// - No parent.
    /// - A new address space (via `Polen::reference()`).
    /// - A new VMM.
    /// - An empty thread list.
    /// - The native syscall handler (`sched::native_syscall_handler`).
    /// - A new, empty root registry.
    /// - Security level 0.
    ///
    /// # Examples
    /// ```ignore
    /// let proc = Process::default();
    /// assert_eq!(proc.pid, 0);
    /// ```
    pub fn new() -> Self {
        // FIXME: triple fault before entry point when opt-level > 0
        Self {
            pid: next(),
            parent: None,
            address_space: Arc::new(Nutex::new(Polen::reference())),
            vmm: Arc::new(Nutex::new(Vmm::new())),
            threads: Vec::new(),
            syscall_handler: crate::sched::native_syscall_handler,
            roots: RootRef::new(RootReg::new()),
            level: 0,
        }
    }
}

// ============================================================================
// CLONE IMPLEMENTATION
// ============================================================================

impl Clone for Process {
    /// Creates a new process by cloning an existing one.
    ///
    /// This is used for `fork()`‑like operations. The new process:
    /// - Gets a new, unique PID.
    /// - Sets its parent to the original process's PID.
    /// - Shares the same address space (copy‑on‑write).
    /// - Shares the same VMM.
    /// - Gets an empty thread list.
    /// - Uses the same syscall handler.
    /// - Shares the same root filesystem reference.
    /// - Uses the same security level.
    ///
    /// # Returns
    /// A new `Process` that is a clone of the current process.
    ///
    /// # Note
    /// The address space and VMM are shared via `Arc`. This means that
    /// modifications to the page tables or VMAs will be visible to both
    /// processes. For true copy‑on‑write semantics, the address space
    /// should be duplicated lazily when the first write occurs.
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
        }
    }
}

// ============================================================================
// FORMATTING TRAITS
// ============================================================================

impl core::fmt::Display for Process {
    /// Formats the process for debugging and logging.
    ///
    /// The display includes the PID, parent PID, CR3 (address space), and
    /// the number of threads.
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
