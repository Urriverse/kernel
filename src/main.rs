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
#![feature(const_cmp)]
#![feature(naked_functions_rustic_abi)]

#![allow(clippy::missing_transmute_annotations)]

#![cfg_attr(not(debug_assertions), allow(unused_assignments))]

use alloc::{string::ToString as _, sync::Arc};

use crate::sched::current_process;

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

/// Kernel Module Interface - kernel functions export mechanism
pub mod kmi;

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

        // now all processors are in `boot` task

        unsafe {
            sched::REAPER = sched::spawn_kernel_task(
                reap,
                sched::task::Priority(5),
                "reaper",
                None,
                None
            );
        }

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

        sched::exit(0); // terminate `boot` task
    }

    for AP {
        // --------------------------------------------------------------------
        // AP INITIALIZATION
        // --------------------------------------------------------------------
        arch::early_init();
        ARCH_INIT.wait();
        arch::init_ap();
        MEM_INIT.wait();
        mem::init_ap();
        LATE_INIT.wait();
        arch::late_init();
        DEV_INIT.wait();
        SCHED_INIT.wait();

        unsafe { core::arch::asm! { "sti" } }

        sched::exit(0); // terminate `boot` task
    }
}

fn init() {
    ebus::init();

    // 1. Retrieve the initramfs module data from Limine
    let modules = MODULES.response().expect("Failed to get Limine modules").modules();
    if modules.is_empty() {
        error!("No initramfs module found!");
        sched::exit(1);
    }
    let initramfs_data = modules[0].data();
    
    // 2. Create the Rotar (Read-Only Tar) filesystem instance
    let initramfs = Arc::new(vfs::Rotar::new(initramfs_data));
    
    // 3. Register it in the global VFS registry
    let mb_id = vfs::register_mblock(initramfs.clone() as Arc<dyn vfs::FileSystem>);
    initramfs.set_mb_id(mb_id);
    
    // 4. Create a RootReg and mount it under the custom name "irfs"
    let roots = &current_process().expect("No current process").roots;
    roots.mount("initramfs".to_string(), vfs::InodeId(0, mb_id));

    // 5. Resolve the file using the "irfs:/hello.txt" syntax!
    let (init, mb) = vfs::resolve_absolute(&roots, "initramfs:/modules/km-init").expect("Can't resolve init");

    let size = vfs::stat(&mb, init).expect("Can't stat init inode").size;

    let mut buffer = [0u8].repeat(size as usize);

    vfs::read(&mb, init, 0, &mut buffer).expect("Failed to read init");

    kmi::init(&buffer);
    
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
fn reap() {
    loop {
        if let Some((id, task)) = sched::wait_any() {
            trace!("reaped zombie task {:?} {:?}, exit code: {}", id, task.name, task.exit_code);
        }
    }
}
