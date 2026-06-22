//! # Memory Management Subsystem
//!
//! This module provides the kernel's comprehensive memory management infrastructure,
//! from physical memory detection to virtual memory allocation and paging.
//!
//! ## Architecture Overview
//!
//! The memory subsystem is organized into several layers, each with a distinct
//! responsibility. They are initialized in a specific order during boot:
//!
//! 1. **Physical Memory Regions (PMR)** – detects and enumerates all physical
//!    memory regions provided by the bootloader (Limine memory map).
//!
//! 2. **Early Memory Allocator (EMA)** – a simple bump allocator used during
//!    early boot before the full memory management is available.
//!
//! 3. **Kernel Direct Mapping (KDM)** – manages the HHDM (High Half Direct Map),
//!    providing a direct linear mapping of all physical memory into the kernel's
//!    virtual address space.
//!
//! 4. **Page Frame Manager (PFM)** – manages metadata for physical page frames
//!    (SPARSEMEM model) and tracks their allocation state.
//!
//! 5. **Buddy System Allocator (BSA)** – a zone-based buddy allocator for
//!    physical memory, with per-CPU caches for performance.
//!
//! 6. **Unified Page Allocator (UPA)** – a facade that switches from EMA to BSA
//!    after migration, providing a uniform allocation interface.
//!
//! 7. **Sized Object Allocator (SOA)** – a slab allocator for small objects
//!    (≤ 2048 bytes), built on top of UPA.
//!
//! 8. **Page Table Manager (PTM)** – a per-address-space page table manager
//!    (`Polen`) that handles mapping, unmapping, and page splitting/merging.
//!
//! 9. **Virtual Memory Area (VMA)** – a Red-Black tree of virtual memory regions
//!    for userspace processes, used for demand paging and memory mapping.
//!
//! ## Initialization Flow
//!
//! The memory subsystem is initialized in two phases: BSP (boot processor) and AP
//! (application processors), with synchronization via barriers.
//!
//! ### BSP Initialization (`init_bsp`)
//! Called on the bootstrap processor.
//!
//! ```text
//! ema::init()     -> initialize early memory allocator
//! pfm::init()     -> initialize page frame metadata (SPARSEMEM)
//! kdm::init()     -> initialize kernel direct mapping (HHDM)
//! bsa::init()     -> initialize buddy system allocator
//! upa::migrate()  -> switch UPA from EMA to BSA
//! soa::init()     -> initialize sized allocator
//! gall::set_soa() -> switch global allocator to SOA backend
//! PTM.lock()      -> set current page table to the reference one
//! activate()      -> load CR3
//! ```
//!
//! ### AP Initialization (`init_ap`)
//! Called on each AP after waiting for `MEM_INIT` to open.
//!
//! ```text
//! PTM.lock().activate() → activate the shared page table
//! ```
//!
//! ## Key Data Structures
//!
//! - `PTM`: A `Nutex<Polen>` holding the current page table manager. Used
//!   for all kernel and per-process address space operations.
//! - `SPURIOUS`: A static, empty PML4 used as a temporary placeholder before
//!   the real page table is created.
//!
//! ## Safety Notes
//!
//! - `SPURIOUS` is `static mut` and accessed only during early boot before
//!   concurrency.
//! - `PTM` is wrapped in a `Nutex` (interrupt-disabling spinlock) to ensure
//!   safe concurrent access across CPUs.
//! - The global allocator is switched from a dummy to the SOA backend via
//!   `rt::gall::set_soa()`, which is an atomic operation.

// ============================================================================
// SUBMODULES
// ============================================================================

pub mod pmr;   // Physical Memory Regions
pub mod kdm;   // Kernel Direct Mapping (HHDM)
pub mod ema;   // Early Memory Allocator
pub mod ptm;   // Page Table Manager (+ <b>Pol</b>icy <b>En</b>gine)
pub mod upa;   // Unified Page Allocator
pub mod pfm;   // Page Frame Manager
pub mod bsa;   // Buddy System Allocator
pub mod soa;   // Slab Object Allocator
pub mod vma;   // Virtual Memory Areas

// ============================================================================
// GLOBAL STATE
// ============================================================================

/// A static, empty PML4 table used as a temporary placeholder.
///
/// # Safety
/// This is `static mut` and is only used during early boot before
/// any other cores are active or paging is fully set up. After
/// `mem::init_bsp()`, it is replaced with a proper page table.
///
/// This table is initially all zeroes (no entries), which allows
/// `Polen::from_exco` to create a clean address space.
#[allow(static_mut_refs)]
static mut SPURIOUS: crate::arch::paging::Tab = crate::arch::paging::Tab::new();

/// The global Page Table Manager, shared across all CPU cores.
///
/// This is a `Nutex` (interrupt-disabling spinlock) that holds the
/// current `Polen`, which manages the kernel's page tables.
///
/// During BSP init, we create a `Polen` from the `SPURIOUS` table
/// and then replace it with the reference `Polen` after other
/// subsystems (like SOA) are ready. APs simply activate the
/// reference `Polen` to share the kernel's address space.
///
/// # Invariants
/// - Must be initialized before any memory allocation occurs.
/// - After `init_bsp()`, the inner `Polen` is the reference page
///   table that maps all kernel memory and the HHDM.
#[allow(static_mut_refs)]
pub static PTM: crate::sync::Nutex<ptm::Polen> = crate::sync::Nutex::new(
    ptm::Polen::from_exco(
        crate::arch::paging::Exco::from_root(
            unsafe { &mut SPURIOUS },
            0u64,
            false
        )
    )
);

// ============================================================================
// INITIALIZATION FUNCTIONS
// ============================================================================

/// Initializes the memory subsystem on the Bootstrap Processor (BSP).
///
/// This function is called once from `main()` on CPU #0. It sequentially
/// initializes all memory management layers and then switches the global
/// allocator from the dummy to the SOA backend.
///
/// # Order of Operations
/// 1. `ema::init()` – set up the early memory allocator.
/// 2. `pfm::init()` – build the SPARSEMEM page frame metadata.
/// 3. `kdm::init()` – establish the HHDM direct mapping.
/// 4. `bsa::init()` – initialize the buddy system allocator.
/// 5. `upa::migrate()` – switch UPA from EMA to BSA.
/// 6. `soa::init()` – initialize the slab allocator.
/// 7. `crate::rt::gall::set_soa()` – set the global allocator to SOA.
/// 8. Replace `PTM` with the reference `Polen` (from `ptm::Polen::reference()`).
/// 9. Activate the new page table (load CR3).
///
/// # Panics
/// - If any of the allocator initializations fail (e.g., no usable memory).
/// - If the global allocator switch fails (should not happen).
///
/// # Safety
/// - Called before interrupts are enabled; single-threaded.
/// - Uses `unsafe` to write to `SPURIOUS` and `PTM`.
pub fn init_bsp() {
    // Initialize each component in order
    ema::init();   // early bump allocator
    pfm::init();   // page frame metadata
    kdm::init();   // HHDM mapping
    bsa::init();   // buddy system allocator
    upa::migrate(); // switch UPA backend to BSA
    soa::init();   // slab allocator

    // Switch the global allocator from dummy to SOA
    crate::rt::gall::set_soa();

    // Acquire the PTM lock and replace the inner Polen with the
    // reference Polen (which has the kernel mappings and HHDM).
    *PTM.lock() = ptm::Polen::reference();

    // Activate the page table (load CR3) on the current CPU.
    unsafe { PTM.lock().activate() };
}

/// Initializes the memory subsystem on an Application Processor (AP).
///
/// This function is called on each AP after waiting for the `MEM_INIT`
/// barrier to be opened by the BSP. It simply activates the shared
/// page table (the reference `Polen`) so that the AP uses the same
/// address space as the kernel.
///
/// # Safety
/// - Called with interrupts disabled; APs are still in early boot.
/// - Assumes `PTM` has been initialized by the BSP.
pub fn init_ap() {
    unsafe { PTM.lock().activate() };
}
