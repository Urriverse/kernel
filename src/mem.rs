//! Memory management modules.
//!
//! This module exports several submodules for memory management:
//! - [`reg`] – physical memory map (from Limine).
//! - [`kdm`] – kernel direct mapper (HHDM offset).
//! - [`ema`] – early memory allocator (used before a real heap is available).
//! - [`ptm`] – page table manager (x86_64 4‑level paging).
//! - [`upa`] – unified page allocator (front‑end for physical page allocation).

pub mod pmr;
pub mod kdm;
pub mod ema;
pub mod ptm;
pub mod upa;
pub mod pfm;
pub mod bsu;
pub mod bsa;
pub mod bua;
pub mod soa;
