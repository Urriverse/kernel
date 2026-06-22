//! # Synchronization Primitives
//!
//! This module provides a comprehensive set of synchronization primitives for the kernel,
//! ranging from simple spinlocks to reader‑writer locks and barrier primitives.
//! Each primitive is designed for specific use cases and offers different trade‑offs
//! between performance, safety, and functionality.
//!
//! ## Overview
//!
//! The sync module exports several mutually exclusive locking primitives, a read‑write lock,
//! and a simple barrier. They are all designed to be used in a `no_std` environment
//! and are suitable for kernel‑level synchronization across multiple CPU cores.
//!
//! ## Primitive Comparison
//!
//! | Primitive | Interrupts Disabled | Spins | Multi‑CPU Safe | Unsafe Inner | Use Case |
//! |-----------|---------------------|-------|----------------|--------------|----------|
//! | [`Mutex`] | No                  | Yes   | Yes            | No           | Simple spinlock for data shared between tasks, not interrupt context. |
//! | [`Nutex`] | Yes                 | Yes   | Yes            | No           | Spinlock with interrupt disabling; safe for interrupt handlers. |
//! | [`Litex`] | Yes                 | Yes   | Yes            | Yes          | Like `Nutex` but with unsafe inner access for early boot. |
//! | [`Nitex`] | Yes                 | No    | No (per‑CPU)   | Yes          | Interrupt‑only lock for per‑CPU data; no spinning. |
//! | [`RwLock`]| No                  | Yes   | Yes            | No           | Reader‑writer lock; multiple readers or single writer. |
//! | [`Barrier`] | No                  | Yes   | Yes            | No           | One‑time barrier flag; spins until opened. |
//!
//! ## Module Structure
//!
//! - **`mutex`**: Basic spinlock (`Mutex`). No interrupt disabling.
//! - **`nutex`**: Interrupt‑disabling spinlock (`Nutex`). Safe for interrupt handlers.
//! - **`litex`**: Interrupt‑disabling spinlock with unsafe inner access (`Litex`).
//! - **`nitex`**: Interrupt‑only lock (`Nitex`). Does not spin; per‑CPU only.
//! - **`rwlock`**: Reader‑writer lock (`RwLock`). Allows multiple readers or single writer.
//! - **`barrier`**: One‑time barrier (`Barrier`). Spins until opened.
//!
//! ## Usage Recommendations
//!
//! - **For most shared data**: Use [`Nutex`] if the data can be accessed from interrupt
//!   context, or [`Mutex`] if it is only accessed from task context.
//! - **For per‑CPU data**: Use [`Nitex`] (no spinning, interrupts disabled).
//! - **For read‑heavy data**: Use [`RwLock`] to allow concurrent readers.
//! - **For boot‑time barriers**: Use [`Barrier`] to synchronise BSP and APs.
//!
//! ## Safety
//!
//! All primitives are designed to be safe when used correctly. However, some provide
//! `unsafe` methods for raw access (e.g., `Litex::inner()`). The caller must ensure
//! that the lock is not bypassed when using these methods.

// ============================================================================
// MODULE DECLARATIONS
// ============================================================================

mod nutex;   // Interrupt‑disabling spinlock (safe)
mod mutex;   // Simple spinlock (no interrupt disabling)
mod nitex;   // Interrupt‑only lock (no spinning)
mod barrier;   // One‑time barrier
mod rwlock;  // Reader‑writer lock
mod litex;   // Interrupt‑disabling spinlock with unsafe inner

// ============================================================================
// RE‑EXPORTS
// ============================================================================

pub use nutex::*;   // Nutex, NutexGuard
pub use mutex::*;   // Mutex, MutexGuard
pub use nitex::*;   // Nitex, NitexGuard
pub use barrier::*;   // Barrier
pub use rwlock::*;  // RwLock, RwLockReadGuard, RwLockWriteGuard
pub use litex::*;   // Litex, LitexGuard
