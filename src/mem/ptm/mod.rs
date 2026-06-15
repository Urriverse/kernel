//! Page Table Manager (PTM).
//!
//! This module provides a complete page table management system for x86_64
//! with 4‑level paging. It is split into three parts:
//!
//! - `entry` – page table entry definitions and flags.
//! - `exco` – low‑level executor that directly manipulates page tables.
//! - `polen` – high‑level policy engine that automates mapping, unmapping,
//!   and merging of pages.

mod entry;
mod exco;
mod polen;

pub use entry::*;
pub use exco::{Area, Exco, Tab};
pub use polen::Polen;

use crate::mem::kdm::{Paddr, Vaddr, HHDM};
use crate::mem::pmr::{self, Kind};
