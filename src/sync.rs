//! Synchronisation primitives.
//!
//! This module exports three types of mutexes:
//! - [`Nutex`] – a spinlock that disables interrupts while the lock is held.
//! - [`Mutex`] – a plain spinlock without interrupt manipulation.
//! - [`Nitex`] – a “no‑interrupt” mutex that only disables/enables interrupts,
//!   without any spinning (the lock is implicit – it always succeeds).
//!
//! All primitives are designed for kernel‑space usage and are `Send` + `Sync`
//! when their contents are `Send`.

mod nutex;
mod mutex;
mod nitex;

pub use nutex::*;
pub use mutex::*;
pub use nitex::*;
