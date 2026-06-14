//! Runtime initialisation for the kernel.
//!
//! This module contains the low‑level entry point (`_start`), the panic handler,
//! and the global allocator management (`gall`).

mod entry;
mod panic;
mod gall;
