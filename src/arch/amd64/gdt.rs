//! # Global Descriptor Table (GDT) and Task State Segment (TSS)
//!
//! This module manages the x86_64 Global Descriptor Table (GDT), which defines
//! segmentation and protection rings, along with the Task State Segment (TSS)
//! used for kernel stack switching and interrupt handling.
//!
//! ## Overview
//!
//! On x86_64, the GDT is used primarily for:
//! - Defining code and data segments for both kernel (Ring 0) and user (Ring 3).
//! - Providing the Task State Segment (TSS), which contains the kernel stack
//!   pointer (`rsp0`) for each CPU core.
//! - Supporting Interrupt Stack Tables (IST) for handling exceptions and
//!   interrupts with separate stacks.
//!
//! ## Structure
//!
//! The GDT is a table of 64‑bit segment descriptors. This implementation defines:
//!
//! - **Null Descriptor** (index 0): Required by the x86 architecture.
//! - **Kernel Code** (index 1): 64‑bit, Ring 0, readable/executable.
//! - **Kernel Data** (index 2): Ring 0, writable data segment.
//! - **User Code** (index 3): 64‑bit, Ring 3, readable/executable.
//! - **User Data** (index 4): Ring 3, writable data segment.
//! - **TSS Descriptors** (starting at index 5): One per CPU core, each pointing
//!   to a per‑CPU TSS structure.
//!
//! ## Per‑CPU TSS
//!
//! Each CPU core has its own TSS, which stores:
//! - `rsp0`: The kernel stack pointer for when the CPU switches from user to
//!   kernel mode (e.g., on interrupts or system calls).
//! - `ist1..ist7`: Interrupt Stack Table entries, used by the IDT for
//!   exceptions that need a dedicated stack (e.g., double fault).
//!
//! The TSS for each core is stored in a static array `TSS_TABLE`, indexed by
//! CPU ID.
//!
//! ## Initialization
//!
//! - **BSP**: `gdt::init_bsp()` is called during BSP initialization. It sets up
//!   the TSS for CPU 0, loads the GDT, and loads the TSS selector into `tr`.
//! - **APs**: `gdt::init_ap(cpu_id)` is called for each AP. It sets up the TSS
//!   for that CPU, loads the GDT (the same GDT is shared), and loads the TSS
//!   selector.
//!
//! ## Stack Switching
//!
//! The `set_kernel_stack(cpu_id, stack_top)` function updates the `rsp0` field
//! of the TSS for a given CPU. This is used by the scheduler to set the kernel
//! stack for the current task.
//!
//! ## Selectors
//!
//! Segment selectors are defined as constants:
//! - `KERNEL_CODE_SELECTOR`: 0x08
//! - `KERNEL_DATA_SELECTOR`: 0x10
//! - `USER_CODE_SELECTOR`: 0x18
//! - `USER_DATA_SELECTOR`: 0x20
//! - `tss_selector(cpu_id)`: 0x28 + (cpu_id * 16)
//!
//! The TSS selectors are spaced 16 bytes apart to accommodate the 16‑byte
//! TSS descriptor (two 64‑bit entries).
//!
//! ## Safety
//!
//! This module uses `static mut` for the GDT, TSS table, and other structures.
//! These are accessed during early boot (single‑threaded) and later via per‑CPU
//! operations that are safe because each CPU writes to its own TSS entry.
//! The `load()` and `set_tss()` functions use unsafe inline assembly to perform
//! privileged operations (`lgdt`, `ltr`, `mov ds`, etc.).

use super::MAX_CPUS;

// ============================================================================
// SEGMENT SELECTORS
// ============================================================================

/// Kernel code segment selector (Ring 0, executable).
pub const KERNEL_CODE_SELECTOR: u16 = 0x08;

/// Kernel data segment selector (Ring 0, writable).
pub const KERNEL_DATA_SELECTOR: u16 = 0x10;

/// User code segment selector (Ring 3, executable).
pub const USER_CODE_SELECTOR: u16 = 0x18;

/// User data segment selector (Ring 3, writable).
pub const USER_DATA_SELECTOR: u16 = 0x20;

/// Returns the TSS segment selector for a given CPU.
///
/// TSS descriptors start at index 5, with each descriptor taking 2 entries
/// (16 bytes). The selector is `(index << 3) | RPL (0)`.
#[inline]
pub const fn tss_selector(cpu_id: usize) -> u16 {
    (0x28 + (cpu_id * 16)) as u16
}

// ============================================================================
// TASK STATE SEGMENT (TSS)
// ============================================================================

/// Task State Segment (TSS) structure.
///
/// This structure holds the stack pointers for the CPU, used for privilege
/// level transitions and interrupt handling.
///
/// # Fields
/// - `reserved_1`: Must be zero.
/// - `rsp0`: Stack pointer for Ring 0 (kernel mode).
/// - `rsp1`: Stack pointer for Ring 1 (not used).
/// - `rsp2`: Stack pointer for Ring 2 (not used).
/// - `reserved_2`: Must be zero.
/// - `ist1..ist7`: Interrupt Stack Table entries (one per IST index).
/// - `reserved_3`, `reserved_4`, `iomap_base`: Must be zero for 64‑bit mode.
#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Tss {
    pub reserved_1: u32,
    pub rsp0: u64,
    pub rsp1: u64,
    pub rsp2: u64,
    pub reserved_2: u64,
    pub ist1: u64,
    pub ist2: u64,
    pub ist3: u64,
    pub ist4: u64,
    pub ist5: u64,
    pub ist6: u64,
    pub ist7: u64,
    pub reserved_3: u64,
    pub reserved_4: u16,
    pub iomap_base: u16,
}

// ============================================================================
// GLOBAL DESCRIPTOR TABLE (GDT)
// ============================================================================

/// The GDT structure, containing all segment descriptors.
///
/// The GDT is an array of 64‑bit descriptors. The total size is:
/// - 5 static descriptors (null, kernel code/data, user code/data)
/// - 2 descriptors per CPU for TSS (each TSS descriptor is 16 bytes)
#[repr(C, align(8))]
pub struct Gdt {
    entries: [u64; 5 + (MAX_CPUS * 2)],
}

/// Descriptor Table Register (GDTR / IDTR) structure.
///
/// This is loaded with `lgdt` or `lidt`.
#[repr(C, packed)]
pub struct Dtr {
    pub limit: u16,
    pub base: u64,
}

impl Gdt {
    /// Creates a new GDT with default segment descriptors.
    ///
    /// The descriptors are set up as follows:
    /// - **Null**: All zero.
    /// - **Kernel Code**: Present, Ring 0, 64‑bit, readable, executable.
    /// - **Kernel Data**: Present, Ring 0, writable.
    /// - **User Code**: Present, Ring 3, 64‑bit, readable, executable.
    /// - **User Data**: Present, Ring 3, writable.
    ///
    /// TSS descriptors are zero‑initialized and must be set with `set_tss()`.
    pub const fn new() -> Self {
        let mut entries = [0u64; 5 + (MAX_CPUS * 2)];

        // 0. Null descriptor (required by x86)
        entries[0] = 0x0000000000000000;

        // 1. Kernel Code (64-bit, present, ring 0, readable)
        // Base: 0, Limit: 0, Access: 0x9A (1001 1010), Flags: 0x20 (L=1)
        entries[1] = 0x00209A0000000000;

        // 2. Kernel Data (64-bit, present, ring 0, writable)
        // Base: 0, Limit: 0, Access: 0x92 (1001 0010), Flags: 0x00
        entries[2] = 0x0000920000000000;

        // 3. User Code (64-bit, present, ring 3, readable)
        // Base: 0, Limit: 0, Access: 0xFA (1111 1010), Flags: 0x20 (L=1)
        entries[3] = 0x0020FA0000000000;

        // 4. User Data (64-bit, present, ring 3, writable)
        // Base: 0, Limit: 0, Access: 0xF2 (1111 0010), Flags: 0x00
        entries[4] = 0x0000F20000000000;

        Self { entries }
    }

    /// Loads the GDT and reloads segment registers.
    ///
    /// This function:
    /// 1. Executes `lgdt` to load the GDT base and limit.
    /// 2. Reloads `ds`, `es`, `ss`, `fs`, `gs` with the kernel data selector.
    /// 3. Performs a far jump to reload `cs` with the kernel code selector.
    ///
    /// # Safety
    /// This function uses inline assembly and requires that the GDT is valid.
    pub unsafe fn load(&'static self) {
        let dtr = Dtr {
            limit: (size_of_val(&self.entries) - 1) as u16,
            base: self as *const _ as u64,
        };

        unsafe {
            core::arch::asm!(
                "lgdt [{0}]",
                in(reg) &dtr,
                options(readonly, nostack, preserves_flags)
            );

            Self::reload_segments();
        }
    }

    /// Reloads segment registers after a GDT load.
    ///
    /// This function:
    /// 1. Loads `ds`, `es`, `ss`, `fs`, `gs` with the kernel data selector.
    /// 2. Pushes the kernel code selector and a return address, then executes
    ///    `retfq` to perform a far return, reloading `cs`.
    #[inline]
    unsafe fn reload_segments() {
        unsafe {
            core::arch::asm!(
                "mov ds, {0:x}",
                "mov es, {0:x}",
                "mov ss, {0:x}",
                "mov fs, {0:x}",
                "mov gs, {0:x}",

                "push {1:r}",
                "lea {2}, [rip + 2f]",
                "push {2}",
                "retfq",
                "2:",
                in(reg) KERNEL_DATA_SELECTOR,
                in(reg) KERNEL_CODE_SELECTOR,
                out(reg) _,
                options(nostack, preserves_flags)
            );
        }
    }

    /// Sets the TSS descriptor for a given CPU.
    ///
    /// This function writes a 16‑byte TSS descriptor into the GDT at the
    /// appropriate slot. The descriptor includes:
    /// - Base address of the TSS.
    /// - Limit (size of the TSS).
    /// - Access flags (type 9, present).
    /// - Flags (G=0, L=0 for 64‑bit TSS).
    ///
    /// # Arguments
    /// * `cpu_id` – The CPU core index (0..MAX_CPUS-1).
    /// * `tss_ptr` – Pointer to the TSS structure.
    ///
    /// # Panics
    /// Panics if `cpu_id >= MAX_CPUS`.
    pub fn set_tss(&mut self, cpu_id: usize, tss_ptr: *const Tss) {
        assert!(cpu_id < MAX_CPUS, "CPU ID {} exceeds MAX_CPUS ({})", cpu_id, MAX_CPUS);

        let base = tss_ptr as u64;
        let limit = (size_of::<Tss>() - 1) as u64;

        // Access: present (0x80) | type (0x9) = 0x89
        let access: u64 = 0x89;
        // Flags: G=0 (limit in bytes), L=0 (for 64-bit TSS)
        let flags: u64 = 0x00;

        // First 64‑bit word of the descriptor.
        // Bits:
        //  0‑15:   Limit (low)
        //  16‑39:  Base (low)
        //  40‑47:  Access byte
        //  48‑51:  Limit (high)
        //  52‑55:  Flags
        //  56‑63:  Base (mid)
        let low = (limit & 0xFFFF)
                | ((base & 0xFFFFFF) << 16)
                | ((access & 0xFF) << 40)
                | (((limit >> 16) & 0xF) << 48)
                | ((flags & 0xF) << 52)
                | ((base >> 24) << 56);

        // Second 64‑bit word: Base (high 32 bits).
        let high = base >> 32;

        let tss_idx = 5 + (cpu_id * 2);
        self.entries[tss_idx] = low;
        self.entries[tss_idx + 1] = high;
    }
}

// ============================================================================
// GLOBAL GDT AND TSS TABLES
// ============================================================================

/// The global GDT, shared by all CPU cores.
///
/// This is `static mut` because it is modified during early boot and then
/// remains immutable thereafter.
pub static mut GLOBAL_GDT: Gdt = Gdt::new();

/// The per‑CPU TSS table.
///
/// Each CPU has its own TSS, stored in this static array.
static mut TSS_TABLE: [Tss; MAX_CPUS] = [Tss {
    reserved_1: 0, rsp0: 0, rsp1: 0, rsp2: 0, reserved_2: 0,
    ist1: 0, ist2: 0, ist3: 0, ist4: 0, ist5: 0, ist6: 0, ist7: 0,
    reserved_3: 0, reserved_4: 0, iomap_base: 0,
}; MAX_CPUS];

// ============================================================================
// PUBLIC API
// ============================================================================

/// Sets the kernel stack pointer (`rsp0`) for a given CPU.
///
/// This updates the TSS `rsp0` field, which is used when transitioning from
/// user mode to kernel mode (e.g., on interrupts or syscalls).
///
/// # Arguments
/// * `cpu_id` – The CPU core index.
/// * `stack_top` – The top address of the kernel stack (the stack grows down).
///
/// # Panics
/// Panics if `cpu_id >= MAX_CPUS`.
pub fn set_kernel_stack(cpu_id: usize, stack_top: u64) {
    assert!(cpu_id < MAX_CPUS, "CPU ID {} exceeds MAX_CPUS", cpu_id);
    unsafe {
        TSS_TABLE[cpu_id].rsp0 = stack_top;
    }
}

/// Sets an Interrupt Stack Table (IST) entry for a given CPU.
///
/// The IST provides dedicated stacks for specific interrupts (e.g., double fault).
/// This function sets one of the 7 IST entries.
///
/// # Arguments
/// * `cpu_id` – The CPU core index.
/// * `ist_index` – The IST index (1..7).
/// * `stack_top` – The top address of the stack for this IST.
///
/// # Panics
/// Panics if `cpu_id >= MAX_CPUS` or `ist_index` is not in 1..7.
pub fn set_ist(cpu_id: usize, ist_index: usize, stack_top: u64) {
    assert!(cpu_id < MAX_CPUS, "CPU ID {} exceeds MAX_CPUS", cpu_id);
    assert!((1..=7).contains(&ist_index), "IST index must be 1..=7");

    unsafe {
        match ist_index {
            1 => TSS_TABLE[cpu_id].ist1 = stack_top,
            2 => TSS_TABLE[cpu_id].ist2 = stack_top,
            3 => TSS_TABLE[cpu_id].ist3 = stack_top,
            4 => TSS_TABLE[cpu_id].ist4 = stack_top,
            5 => TSS_TABLE[cpu_id].ist5 = stack_top,
            6 => TSS_TABLE[cpu_id].ist6 = stack_top,
            7 => TSS_TABLE[cpu_id].ist7 = stack_top,
            _ => unreachable!(),
        }
    }
}

/// Initializes the GDT and TSS for the Bootstrap Processor (BSP, CPU 0).
///
/// This function:
/// 1. Sets up the TSS for CPU 0.
/// 2. Loads the GDT (`lgdt`).
/// 3. Loads the TSS selector (`ltr`) with `tss_selector(0)`.
///
/// # Safety
/// This is called during early boot with interrupts disabled.
pub fn init_bsp() {
    info!("Initializing for BSP (CPU#0)");
    unsafe {
        #[allow(static_mut_refs)]
        GLOBAL_GDT.set_tss(0, &raw const TSS_TABLE[0]);
        #[allow(static_mut_refs)]
        GLOBAL_GDT.load();

        core::arch::asm!(
            "ltr {0:x}",
            in(reg) tss_selector(0),
            options(nostack, preserves_flags)
        );
    }
    info!("BSP initialized successfully.");
}

/// Initializes the GDT and TSS for an Application Processor (AP).
///
/// This function:
/// 1. Sets up the TSS for the specified CPU.
/// 2. Loads the GDT (`lgdt`).
/// 3. Loads the TSS selector (`ltr`) with `tss_selector(cpu_id)`.
///
/// # Arguments
/// * `cpu_id` – The CPU core index (1..MAX_CPUS-1).
///
/// # Panics
/// Panics if `cpu_id >= MAX_CPUS`.
///
/// # Safety
/// This is called during AP boot with interrupts disabled.
pub fn init_ap(cpu_id: usize) {
    info!("Initializing for AP (CPU#{})", cpu_id);

    unsafe {
        #[allow(static_mut_refs)]
        GLOBAL_GDT.set_tss(cpu_id, &raw const TSS_TABLE[cpu_id]);
        #[allow(static_mut_refs)]
        GLOBAL_GDT.load();

        core::arch::asm!(
            "ltr {0:x}",
            in(reg) tss_selector(cpu_id),
            options(nostack, preserves_flags)
        );
    }

    info!("AP initialized successfully (Selector: {:#X})", tss_selector(cpu_id));
}
