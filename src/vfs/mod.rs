//! # Virtual File System (VFS)
//!
//! The VFS subsystem provides a unified interface for file operations across different
//! filesystem types. It abstracts the underlying storage and allows multiple filesystems
//! to be mounted, accessed, and managed consistently.
//!
//! ## Architecture Overview
//!
//! The VFS is designed around several key abstractions:
//!
//! - **Inode**: Represents a file, directory, or other filesystem object. Each inode
//!   stores metadata (size, permissions, timestamps, etc.) and a reference to its
//!   containing filesystem (`MetaBlock`). Inodes are identified by `InodeId`.
//!
//! - **MetaBlock**: A mount point or filesystem instance. It holds a pointer to a
//!   filesystem‑specific `FsVtable` (function table) and an opaque `data` pointer
//!   that points to the filesystem's internal state. It is wrapped in a `RwLock`
//!   for concurrent access.
//!
//! - **FsVtable**: A static table of function pointers that implement the filesystem's
//!   operations: `lookup`, `readdir`, `read`, `write`, `truncate`, `unlink`, `link`,
//!   `new`, `get`, `get_mut`. Each filesystem provides its own implementation.
//!
//! - **RootReg / RootRef**: Manages the mount points (root entries) for a process.
//!   Each process has a `RootRef` (an `Arc<RootReg>`) that maps mount point names
//!   (e.g., `"/"`, `"/proc"`) to `InodeId`s. This allows processes to have different
//!   namespace views.
//!
//! - **InodeId**: A tuple `(u32, u32)` where the first component is an inode number
//!   (unique within the filesystem) and the second is a `MetaBlock` identifier. It
//!   provides safe methods to retrieve the inode from the global registry.
//!
//! ## Filesystem Registration
//!
//! Each filesystem type (e.g., `PvfsMb` for the purely virtual filesystem) defines:
//! - A data structure that holds the filesystem's internal state (e.g., a `BTreeMap`
//!   of inodes).
//! - An instance of `FsVtable` with function pointers to the implementation.
//! - A mechanism to create a new `MetaBlock` by calling `new_mblock(id, &VTABLE, &state)`.
//!
//! The `MetaBlock` is then registered in the global `MBLK_REG` via `reg_mblk()`,
//! which returns a unique ID. This ID is used in `InodeId` to locate the filesystem.
//!
//! ## Inode Operations
//!
//! The module provides high‑level functions that operate on `InodeId`:
//! - `lookup(id, name)` – find a child inode by name (for directories).
//! - `listdir(id)` – list all entries in a directory.
//! - `read(id, offset, buf)` – read data from a file.
//! - `write(id, offset, buf)` – write data to a file.
//! - `link(id, name, new_id)` – create a new directory entry.
//! - `new(mb, inode, kind)` – create a new inode in a filesystem.
//!
//! These functions obtain the `Inode` from the `InodeId`, retrieve the `MetaBlock`,
//! and call the corresponding method from the `FsVtable`.
//!
//! ## Global Inode Registry
//!
//! The VFS maintains a global registry of `MetaBlock` instances in `MBLK_REG`,
//! a `RwLock<BTreeMap<u32, MetaBlock>>`. This registry is used to look up
//! the `MetaBlock` for a given ID. Inodes themselves are stored inside the
//! filesystem's internal data structures (e.g., in `PvfsMb.reg`), not in a
//! global table.
//!
//! ## Purely Virtual Filesystem (PVFS)
//!
//! `PvfsMb` is a simple, in‑memory filesystem that does not require a block device.
//! It stores files as `Vec<u8>` and directories as `Vec<(String, InodeId)>`.
//! It implements all `FsVtable` operations and is used early in boot for testing
//! and as a basis for other virtual filesystems (e.g., `procfs`, `sysfs`).
//!
//! ## Mount Points and Root Namespace
//!
//! Each process has a `RootRef` (shared reference-counted `RootReg`) that maps
//! mount point names (e.g., `"root"`, `"proc"`) to `InodeId`s. This allows
//! processes to have isolated views of the filesystem namespace. The `RootReg`
//! is protected by a `Litex` (interrupt‑disabling spinlock) for safe concurrent
//! access.
//!
//! ## Error Handling
//!
//! Most VFS operations return a `Result` with an `Error` enum (`Unknown`,
//! `NotAFile`, `OutOfBounds`, `NoEntry`, `NotADirectory`, `Found`). Errors are
//! propagated to the caller, which typically handles them by logging or returning
//! an appropriate userspace error code.

// ============================================================================
// SUBMODULES
// ============================================================================

mod inode;   // Inode, InodeId, MetaBlock registration
mod root;    // RootReg, RootRef – mount point management
mod pvfs;    // Purely virtual filesystem (PVFS)
mod err;     // VFS error types
mod mb;      // MetaBlock, FsVtable definitions

// ============================================================================
// RE-EXPORTS
// ============================================================================

pub use inode::*;
pub use root::*;
pub use pvfs::*;
pub use err::*;
pub use mb::*;

// ============================================================================
// VFS HIGH‑LEVEL OPERATIONS
// ============================================================================

use alloc::{collections::btree_map::BTreeMap, string::String};

/// Looks up a child inode by name in a directory.
///
/// # Arguments
/// * `this` – The directory inode ID.
/// * `name` – The name to look up.
///
/// # Returns
/// `Some(InodeId)` if the entry exists, `None` otherwise.
pub fn lookup(this: &InodeId, name: String) -> Option<InodeId> {
    if let Some(i) = this.get() {
        return (unsafe { i.mblock.inner().vtable().lookup })(i, name)
    }
    None
}

/// Lists all entries in a directory.
///
/// # Arguments
/// * `this` – The directory inode ID.
///
/// # Returns
/// A `BTreeMap<String, InodeId>` mapping entry names to inode IDs.
pub fn listdir(this: &InodeId) -> BTreeMap<String, InodeId> {
    let mut rv = BTreeMap::<String, InodeId>::new();

    if let Some(i) = this.get() {
        for ofs in 0.. {
            if let Some((name, id)) = (unsafe { i.mblock.inner().vtable().readdr })(i, ofs) {
                rv.insert(name, id);
            } else {
                break
            }
        }
    }
    rv
}

/// Reads data from a file at a given offset.
///
/// # Arguments
/// * `this` – The file inode ID.
/// * `offset` – The byte offset to start reading from.
/// * `buf` – The buffer to fill with data.
///
/// # Returns
/// The number of bytes read, or an `Error`.
pub fn read(this: &InodeId, offset: usize, buf: &mut [u8]) -> Result<usize, Error> {
    if let Some(i) = this.get() {
        return (unsafe { i.mblock.inner().vtable().read })(i, offset, buf)
    }
    Err(Error::Unknown)
}

/// Reads the entire contents of a file into a `String`.
///
/// # Arguments
/// * `this` – The file inode ID.
///
/// # Returns
/// `Ok(String)` if successful, `Err(())` on failure.
pub fn read_to_string(this: &InodeId) -> Result<String, ()> {
    if let Some(i) = this.get() {
        let mut buf = [0u8].repeat(i.size as usize);
        if read(this, 0, &mut buf).is_err() {
            return Err(())
        }
        if let Ok(s) = String::from_utf8(buf) {
            return Ok(s)
        }
    }
    Err(())
}

/// Writes data to a file at a given offset.
///
/// # Arguments
/// * `this` – The file inode ID.
/// * `offset` – The byte offset to start writing at.
/// * `buf` – The data to write.
///
/// # Returns
/// The number of bytes written, or an `Error`.
pub fn write(this: &InodeId, offset: usize, buf: &[u8]) -> Result<usize, Error> {
    if let Some(i) = this.get() {
        return (unsafe { i.mblock.inner().vtable().write })(i, offset, buf)
    }
    Err(Error::Unknown)
}

/// Creates a new directory entry (link) in a directory.
///
/// # Arguments
/// * `this` – The directory inode ID.
/// * `name` – The name of the new entry.
/// * `new` – The inode ID to link to.
///
/// # Returns
/// `Ok(())` on success, or an `Error`.
pub fn link(this: &InodeId, name: String, new: InodeId) -> Result<(), Error> {
    if let Some(i) = this.get_mut() {
        return (unsafe { i.mblock.inner().vtable().link })(i, name, new)
    }
    Err(Error::Unknown)
}

/// Creates a new inode in a filesystem.
///
/// # Arguments
/// * `mb` – The `MetaBlock` representing the filesystem.
/// * `inode` – The inode to create (must have `kind`, `flags`, etc. set).
/// * `kind` – The kind of inode (e.g., `File`, `Directory`).
///
/// # Returns
/// The new `InodeId`, or an `Error`.
pub fn new(mb: &MetaBlock, inode: Inode, kind: Kind) -> Result<InodeId, Error> {
    (unsafe { mb.inner().vtable().new })(mb, inode, kind)
}
