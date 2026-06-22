//! # VFS Root Registry (Mount Points)
//!
//! This module manages the mount point namespace for processes. Each process has a
//! `RootRef` (an `Arc<RootReg>`) that maps mount point names (e.g., `"/"`, `"/proc"`,
//! `"/dev"`) to `InodeId`s. This allows processes to have isolated filesystem views
//! and supports mount namespaces.
//!
//! ## Overview
//!
//! The root registry is the process‑level equivalent of a mount table. It associates
//! a name (e.g., `"root"`, `"proc"`, `"tmp"`) with an `InodeId` representing the
//! root inode of that filesystem. All path resolution starts from this registry.
//!
//! ## Structure
//!
//! - **`RootReg`**: The actual registry, which is a `Litex<BTreeMap<String, InodeId>>`.
//!   The `Litex` is an interrupt‑disabling spinlock that protects the map.
//! - **`RootRef`**: A type alias for `Arc<RootReg>`, allowing shared ownership of
//!   the registry across tasks and processes. This enables efficient cloning and
//!   sharing of mount namespaces.
//!
//! ## Operations
//!
//! - **`add_root(name, inode)`**: Inserts or overwrites a mount point. Returns the
//!   previous value, if any.
//! - **`add_new_root(name, inode)`**: Inserts a mount point only if it does not
//!   already exist. Returns `Ok(())` on success, `Err(inode)` if the name is taken.
//! - **`pop_root(name)`**: Removes a mount point and returns its `InodeId`, if it existed.
//! - **`Index<String>`**: Allows direct indexing, e.g., `roots["proc"]`.
//!
//! ## Cloning
//!
//! `RootReg` implements `Clone` by creating a new `RootReg` and copying the
//! contents of the map from the source. This creates a deep copy of the mount
//! namespace, allowing for isolated namespaces while still sharing the underlying
//! `Arc<RootReg>` when desired.
//!
//! ## Safety
//!
//! - The `Litex` lock disables interrupts during critical sections, ensuring that
//!   operations on the map are atomic with respect to interrupts and other CPUs.
//! - The `unsafe` code in `clone_from` and `Index` is used to access the inner
//!   map after locking, which is safe because the lock is held.
//! - The `RootRef` type alias (`Arc<RootReg>`) provides safe shared ownership.

// ============================================================================
// IMPORTS
// ============================================================================

use core::ops::Index;

use alloc::{collections::btree_map::BTreeMap, string::String, sync::Arc};

use crate::{sync::Litex, vfs::InodeId};

// ============================================================================
// ROOT REGISTRY
// ============================================================================

/// A process‑local mount point registry.
///
/// This struct holds a `BTreeMap` that maps mount point names (e.g., `"root"`,
/// `"proc"`) to `InodeId`s. The map is protected by a `Litex` (interrupt‑disabling
/// spinlock) for safe concurrent access from multiple CPUs.
///
/// # Examples
/// ```ignore
/// let roots = RootReg::new();
/// let root_inode = InodeId(0, 0);
/// roots.add_root("root".to_string(), root_inode);
///
/// let proc_inode = InodeId(1, 0);
/// roots.add_new_root("proc".to_string(), proc_inode).unwrap();
///
/// assert_eq!(roots["proc"], proc_inode);
/// ```
pub struct RootReg(Litex<BTreeMap<String, InodeId>>);

impl RootReg {
    /// Creates a new, empty root registry.
    pub fn new() -> Self {
        Self(Litex::new(BTreeMap::new()))
    }

    /// Inserts a mount point, overwriting any existing entry with the same name.
    ///
    /// # Arguments
    /// * `name` – The mount point name (e.g., `"root"`).
    /// * `inode` – The `InodeId` of the root inode of the filesystem.
    ///
    /// # Returns
    /// `Some(old_inode)` if an entry with the same name already existed,
    /// otherwise `None`.
    pub fn add_root(&self, name: String, inode: InodeId) -> Option<InodeId> {
        self.0.lock().insert(name, inode)
    }

    /// Inserts a mount point only if the name is not already taken.
    ///
    /// # Arguments
    /// * `name` – The mount point name.
    /// * `inode` – The `InodeId` of the root inode.
    ///
    /// # Returns
    /// `Ok(())` if the insertion succeeded, or `Err(inode)` if the name was
    /// already present.
    pub fn add_new_root(&self, name: String, inode: InodeId) -> Result<(), InodeId> {
        let mut reg = self.0.lock();

        // Safe: we hold the lock
        if !unsafe { self.0.inner() }.contains_key(&name) {
            reg.insert(name, inode);
            return Ok(())
        }

        Err(inode)
    }

    /// Removes a mount point by name.
    ///
    /// # Arguments
    /// * `name` – The mount point name to remove.
    ///
    /// # Returns
    /// `Some(inode)` if the entry existed and was removed, otherwise `None`.
    pub fn pop_root(&self, name: String) -> Option<InodeId> {
        self.0.lock().remove(&name)
    }
}

impl Default for RootReg {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// CLONE IMPLEMENTATION
// ============================================================================

impl Clone for RootReg {
    /// Creates a deep copy of the mount registry.
    ///
    /// This is useful for creating isolated mount namespaces for new processes.
    /// The clone acquires locks on both the source and destination maps and
    /// copies all entries.
    fn clone(&self) -> Self {
        let rv = Self::new();

        let _g1 = self.0.lock();

        {
            let _g2 = rv.0.lock();

            unsafe { rv.0.inner().iter().clone_from(&self.0.inner().iter()) };
        }

        rv
    }

    /// Clones another `RootReg` into this one, clearing existing entries.
    ///
    /// This is a more efficient way to replace the contents of a `RootReg`
    /// with another's contents.
    fn clone_from(&mut self, source: &Self)
    where
        Self: core::marker::Destruct,
    {
        self.0.lock().iter().clone_from(&source.0.lock().iter());
    }
}

// ============================================================================
// INDEX TRAIT IMPLEMENTATION
// ============================================================================

impl Index<String> for RootReg {
    type Output = InodeId;

    /// Indexes into the registry by mount point name.
    ///
    /// # Panics
    /// Panics if the name does not exist in the registry.
    fn index(&self, index: String) -> &Self::Output {
        let _ = self.0.lock();
        unsafe { self.0.inner() }.get(&index).unwrap()
    }
}

// ============================================================================
// TYPE ALIAS
// ============================================================================

/// An atomically reference‑counted root registry.
///
/// This type alias simplifies sharing of mount namespaces across tasks and processes.
/// Multiple tasks can share the same `RootRef` to have a consistent view of the
/// filesystem namespace, or each can have its own copy for isolation.
pub type RootRef = Arc<RootReg>;
