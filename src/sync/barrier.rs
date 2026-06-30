//! # Barrier – A Simple Spin Barrier (Flag)
//!
//! A minimal synchronization primitive that acts as a one‑time barrier or gate.
//! It allows one or more tasks to wait until the barrier is "opened" by another task.
//!
//! ## Overview
//!
//! `Barrier` is a simple flag‑based synchronization primitive. It has two states:
//! - **Closed** (`false`): Tasks calling `wait()` will spin‑loop until the flag
//!   becomes `true`.
//! - **Open** (`true`): Tasks calling `wait()` will return immediately; the
//!   barrier has been passed.
//!
//! This primitive is useful for coordinating the boot sequence of multiple
//! CPU cores, where the BSP (Bootstrap Processor) initializes subsystems and
//! then signals APs (Application Processors) to proceed. In the kernel, it is
//! used by the `Barrier!` macro to create barriers like `ARCH_INIT`, `MEM_INIT`,
//! `LATE_INIT`, and `DEV_INIT`.
//!
//! ## Characteristics
//!
//! - **Spin‑wait**: `wait()` spins in a tight loop (`spin_loop`) until the flag
//!   is open. This is suitable for short‑duration waits in early boot, where
//!   other synchronization primitives (like wait queues) are not yet available.
//! - **One‑time use**: Once opened, the barrier cannot be closed again (though
//!   `close()` is provided, it is typically not used).
//! - **No locking**: The flag is an `AtomicBool` with `Acquire`/`Release` ordering
//!   to ensure visibility across CPUs.
//! - **No sleeping**: `wait()` does not block or yield; it busy‑waits.
//!
//! ## Usage
//!
//! ```ignore
//! use crate::sync::Barrier;
//!
//! static BARRIER: Barrier = Barrier::new();
//!
//! // On BSP:
//! BARRIER.open();  // Signal that initialization is complete.
//!
//! // On AP:
//! BARRIER.wait();  // Spin until BSP opens the barrier.
//! ```
//!
//! The `Barrier!` macro creates multiple static `Barrier` instances:
//! ```ignore
//! Barrier! { ARCH_INIT MEM_INIT LATE_INIT DEV_INIT }
//! ```
//! This expands to:
//! ```ignore
//! static ARCH_INIT: Barrier = Barrier::new();
//! static MEM_INIT: Barrier = Barrier::new();
//! // etc.
//! ```
//!
//! ## Safety
//!
//! - The `AtomicBool` operations use `Acquire`/`Release` ordering to ensure
//!   that all writes performed before `open()` are visible to any CPU that
//!   observes the flag as `true` (via `wait()`).
//! - The primitive is `Sync` and can be accessed from multiple CPUs safely.
//! - `wait()` does not disable interrupts; it is safe to use in interrupt
//!   handlers, though spinning in an interrupt handler is generally discouraged.

pub use ketypes::KeBarrier as Barrier;
