//! # Kernel Main Entry Point
//!
//! This module serves as the primary entry point for the kernel. It orchestrates
//! the entire bootstrapping process across all CPU cores (BSP and APs) and
//! initializes every major subsystem in the correct order.
//!
//! ## Boot Flow Overview
//!
//! 1. **BSP (Bootstrap Processor) Initialization**
//!    - `_start()` → `main()` (via `rt/entry.rs`)
//!    - Architecture early init (`arch::early_init_bs`)
//!    - Start APs (`start_aps!`)
//!    - Architecture init (`arch::init_bsp`)
//!    - Memory init (`mem::init_bsp`)
//!    - Architecture late init (`arch::late_init_bsp`, `arch::late_init`)
//!    - Device init (`dev::init`)
//!    - Scheduler init (`sched::init`)
//!    - Spawn initial tasks (reaper, test)
//!
//! 2. **AP (Application Processor) Initialization**
//!    - Each AP follows a synchronized boot sequence using `Fueue` barriers:
//!      - `ARCH_INIT.wait()` – wait for BSP arch init
//!      - `MEM_INIT.wait()` – wait for BSP memory init
//!      - `LATE_INIT.wait()` – wait for BSP late init
//!      - `DEV_INIT.wait()` – wait for device init
//!
//! 3. **Final State**
//!    - All cores enter HLT loop
//!    - Scheduler takes over for task management
//!
//! ## Sync Primitives Used
//!
//! - `fueue!` – creates `Fueue` barriers (`ARCH_INIT`, `MEM_INIT`, `LATE_INIT`, `DEV_INIT`)
//! - `entry!` – defines the BSP/AP entry points with proper synchronization
//!
//! ## Safety
//!
//! This module contains unsafe code for:
//! - Raw pointer manipulation for `MetaBlock` and `RootRef`
//! - AP bootstrap via Limine SMP
//! - Static mutable access to `SPURIOUS` PML4

#![no_std]
#![no_main]

#![feature(unsafe_cell_access)]
#![feature(abi_x86_interrupt)]
#![feature(const_trait_impl)]
#![feature(likely_unlikely)]
#![feature(const_destruct)]
#![feature(const_cmp)]
#![feature(mem_copy_fn)]
#![feature(naked_functions_rustic_abi)]

#![allow(unused)]
#![allow(clippy::missing_transmute_annotations)]
#![warn(unused_braces)]
#![warn(unused_comparisons)]
#![warn(unused_import_braces)]
#![warn(unused_imports)]
#![warn(unused_labels)]
#![warn(unused_mut)]
#![warn(unused_parens)]
#![warn(unused_qualifications)]
#![warn(unused_unsafe)]
#![warn(dead_code)]

#![cfg_attr(not(debug_assertions), allow(unused_assignments))]

use core::ptr::addr_of;

use alloc::{string::ToString, sync::Arc};
use sched::current_process;

// ============================================================================
// EXTERNAL CRATES
// ============================================================================

/// Extrum – "leaky" enumerations
#[macro_use]
pub extern crate extrum;

/// Bitflags – for flag-based bitmask types
#[macro_use]
pub extern crate bitflags;

/// Lazy_static – for lazy-initialized static data
#[macro_use]
pub extern crate lazy_static;

/// Alloc – provides `Vec`, `Box`, `String`, etc. (the global allocator is set in `rt/gall.rs`)
#[macro_use]
pub extern crate alloc;

// ============================================================================
// INTERNAL MODULES
// ============================================================================

/// Macros – logging, entry point, Limine requests, etc.
#[macro_use]
pub mod macros;

/// Runtime – entry point, panic handler, global allocator
pub mod rt;

/// Synchronization primitives (mutexes, rwlocks, barriers, etc.)
pub mod sync;

/// Kernel message logging system
pub mod kmsg;

/// Memory management (allocators, paging, physical memory regions, etc.)
pub mod mem;

/// Architecture-specific code (x86_64: GDT, IDT, paging, ACPI, syscalls, etc.)
pub mod arch;

/// Device model (driver framework, device registration, method calls)
pub mod dev;

/// Scheduler (EEVDF-based task scheduler, processes, runqueues)
pub mod sched;

/// Virtual File System (VFS) – inodes, mount points, file operations
pub mod vfs;

/// Event Bus (EBus) - asynchronous kernel communication subsystem
pub mod ebus;

// ============================================================================
// SYNCHRONIZATION BARRIERS
// ============================================================================

// Barriers for coordinating multi-core initialization.
//
// Each barrier is a simple flag that is initially closed. BSP opens them
// sequentially as each subsystem becomes ready, and APs block on each
// barrier before proceeding.
//
// - `ARCH_INIT` – architecture initialization complete
// - `MEM_INIT`  – memory management initialized
// - `LATE_INIT` – late architecture init
// - `DEV_INIT`  – device model initialized
barrier! { ARCH_INIT MEM_INIT LATE_INIT DEV_INIT SCHED_INIT }

limine! { MODULES <= ModulesRequest }

// ============================================================================
// KERNEL ENTRY POINT (BSP + AP)
// ============================================================================

// Main entry point – defines both BSP and AP entry functions.
//
// ## BSP Flow (core 0)
// 1. Early architecture init (paging, CPUID, percpu)
// 2. Start all APs via Limine (`start_aps!`)
// 3. Complete architecture init (GDT, IDT, syscall)
// 4. Initialize memory management (EMA, PFM, KDM, UPA, SOA, PTM)
// 5. Late architecture init (ACPI, HPET, APIC, timer, interrupts)
// 6. Device model init
// 7. Scheduler init
// 8. Spawn `reaper` (zombie reaper) and `test` (VFS test) tasks
//
// ## AP Flow (all other cores)
// 1. Early architecture init (CPUID, percpu)
// 2. Block on `ARCH_INIT` → wait for BSP arch init
// 3. Complete AP architecture init (GDT, IDT)
// 4. Block on `MEM_INIT` → wait for BSP memory init
// 5. Activate shared page table
// 6. Block on `LATE_INIT` → wait for BSP late init
// 7. Block on `DEV_INIT` → wait for device init
// 8. Join scheduler (HLT loop)
entry! {
    for BSP {
        // --------------------------------------------------------------------
        // PHASE 1: Architecture Early Initialization (BSP)
        // --------------------------------------------------------------------
        arch::early_init_bs();

        // Start all APs (each AP will execute `for AP` block)
        start_aps!();

        // --------------------------------------------------------------------
        // PHASE 2: Architecture Full Initialization (BSP)
        // --------------------------------------------------------------------
        arch::init_bsp();

        // Signal that architecture init is complete -> APs can proceed
        ARCH_INIT.open();

        // --------------------------------------------------------------------
        // PHASE 3: Memory Management Initialization (BSP)
        // --------------------------------------------------------------------
        mem::init_bsp();

        // Signal that memory init is complete -> APs can proceed
        MEM_INIT.open();

        // --------------------------------------------------------------------
        // PHASE 4: Late Architecture Initialization (BSP)
        // --------------------------------------------------------------------
        arch::late_init_bsp();
        arch::late_init();

        // Signal that late init is complete -> APs can proceed
        LATE_INIT.open();

        // --------------------------------------------------------------------
        // PHASE 5: Device Model Initialization (BSP)
        // --------------------------------------------------------------------
        dev::init();

        // Signal that device init is complete -> APs can proceed
        DEV_INIT.open();

        // --------------------------------------------------------------------
        // PHASE 6: Scheduler Initialization (BSP)
        // --------------------------------------------------------------------
        let ticks_per_10ms = arch::timer::get_ticks_per_10ms();
        sched::init(ticks_per_10ms);

        unsafe { core::arch::asm! { "sti" } }

        SCHED_INIT.open();

        // --------------------------------------------------------------------
        // PHASE 7: Spawn the reaper task
        // --------------------------------------------------------------------

        let _ = sched::spawn_kernel_task(
            init,
            sched::task::Priority(0),
            "init",
            Some(
                vfs::RootRef::new(
                    vfs::RootReg::new()
                )
            ),
            None
        );

        loop {
            crate::info!("waiting for any child to exit...");
            if let Some((id, task)) = sched::wait_any() {
                crate::debug!("X: {:p}", addr_of!(*task));
                crate::info!("reaped zombie task {:?} {:?}, exit code: {}", id, task.name, task.exit_code);
            }
        }
    }

    for AP {
        // --------------------------------------------------------------------
        // AP INITIALIZATION
        // --------------------------------------------------------------------

        // Early AP init (CPUID, percpu, etc.)
        arch::early_init();

        // Wait for BSP to complete architecture init
        ARCH_INIT.wait();

        // Full AP init (GDT, IDT, etc.)
        arch::init_ap();

        // Wait for BSP to complete memory init
        MEM_INIT.wait();

        // AP memory init (activate shared page table)
        mem::init_ap();

        // Wait for BSP to complete late init
        LATE_INIT.wait();

        // AP late init (ACPI, etc.)
        arch::late_init();

        // Wait for BSP to complete device init
        DEV_INIT.wait();

        SCHED_INIT.wait();

        unsafe {
            core::arch::asm! {
                "sti"
            }
        }
    }
}

fn vfs_test() {
    // Create PVFS instance
    let pvfs = Arc::new(vfs::Pvfs::new());
    let mb_id = vfs::register_mblock(pvfs.clone() as Arc<dyn vfs::FileSystem>);
    let mb = vfs::get_mblock(mb_id).unwrap();
    // Create root directory
    let root_inode = vfs::Inode::default();
    let root_id = vfs::new(&mb, root_inode, vfs::Kind::Directory).expect("Failed to create root dir");
    // Mount it as "root" in the process namespace
    let roots = current_process().expect("NOPID").roots.clone();
    roots.mount_new("root".to_string(), root_id).expect("Mount failed");
    // Create a file
    let file_inode = vfs::Inode::default();
    let file_id = vfs::new(&mb, file_inode, vfs::Kind::File).expect("Failed to create file");
    // Link it into root
    vfs::link(&mb, root_id, "testfile", file_id).expect("Link failed");
    // Write and read back
    vfs::write(&mb, file_id, 0, b"[NOT FAILED]").expect("Write failed");
    let mut buf = *b"[FAILED]    ";
    let n = vfs::read(&mb, file_id, 0, &mut buf).expect("Read failed");
    debug!("vfs test: {}", str::from_utf8(&buf[..n]).unwrap());
    sched::exit(0);
}

/// Zombie reaper task – reaps terminated child processes.
///
/// This task runs in an infinite loop, waiting for any child process to exit.
/// When a child exits (becomes a zombie), it:
/// 1. Logs the exit event
/// 2. Collects the exit code
/// 3. Removes the zombie from the task registry
///
/// # Note
/// This is a kernel task and never exits; it runs forever to ensure no
/// zombies accumulate.
fn init() {
    let _ = sched::spawn_kernel_task(
        vfs_test,
        sched::task::Priority(0),
        "VFS test",
        Some(
            vfs::RootRef::new(
                vfs::RootReg::new()
            )
        ),
        None
    );

    ebus::init();

    sched::exit(0);
}
