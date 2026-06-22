//! # VFS MetaBlock and Function Table
//!
//! This module defines the core abstractions that bind a filesystem implementation
//! to the VFS layer: the **MetaBlock** and the **FsVtable**.
//!
//! ## Overview
//!
//! The VFS is designed to be filesystem‑agnostic. To achieve this, each filesystem
//! instance (mount point) is represented by a `MetaBlock`, which is essentially a
//! container that holds:
//!
//! - A unique identifier (`id`).
//! - A pointer to a static **vtable** (`FsVtable`) containing function pointers
//!   for all filesystem operations.
//! - An opaque `data` pointer that points to the filesystem‑specific state
//!   (e.g., `PvfsMb` for the purely virtual filesystem, or a block device cache
//!   for a disk‑based filesystem).
//!
//! ## MetaBlock
//!
//! `MetaBlock` is a type alias for `RwLock<MetaBlockInner>`. This provides:
//! - **Read‑write locking**: Allows concurrent reads and exclusive writes, safe
//!   for multi‑CPU access.
//! - **Interior mutability**: The vtable and data can be accessed via the lock.
//!
//! ### MetaBlockInner
//!
//! The inner structure contains:
//! - `id: u32` – A unique identifier assigned by the global registry (`MBLK_REG`).
//!   This ID is used in `InodeId` to locate the filesystem.
//! - `vtable: &'static FsVtable` – A reference to the filesystem's operation table.
//! - `data: usize` – An opaque pointer to the filesystem's private state. This
//!   is typically a `*mut` to a struct like `PvfsMb`.
//!
//! The `data` field can be accessed via `data_ref::<T>()` and `data_mut::<T>()`,
//! which are safe methods that cast the pointer to the appropriate type.
//!
//! ## FsVtable
//!
//! The `FsVtable` is a struct of ten function pointers that every filesystem must
//! implement:
//!
//! - **`lookup`**: Find a child inode by name in a directory.
//! - **`readdr`**: Read a directory entry by index (for listing).
//! - **`read`**: Read data from a file at a given offset.
//! - **`write`**: Write data to a file at a given offset.
//! - **`trunc`**: Truncate a file to a new size.
//! - **`unlink`**: Remove an inode from its parent directory and free its data.
//! - **`link`**: Add a new directory entry in a parent directory.
//! - **`new`**: Create a new inode (file or directory) in the filesystem.
//! - **`get`**: Retrieve an immutable reference to an inode by its number.
//! - **`get_mut`**: Retrieve a mutable reference to an inode by its number.
//!
//! Each function pointer has a specific signature that takes the `MetaBlock` (or
//! `Inode`) and appropriate arguments, returning `Option`, `Result`, or the
//! requested data.
//!
//! ## Creating a MetaBlock
//!
//! The `new_mblock(id, vtab, data)` function creates a new `MetaBlock` instance.
//! It is typically called by a filesystem's initialization routine after creating
//! the private state. The resulting `MetaBlock` is then registered with the global
//! registry via `vfs::reg_mblk()`.
//!
//! ## Safety
//!
//! The `data` pointer in `MetaBlockInner` is `usize` and is cast to a concrete
//! type in `data_ref` and `data_mut`. The caller must ensure:
//! - The pointer is valid and points to the correct type.
//! - The pointer is not used after the filesystem instance is destroyed.
//! - The `data` is properly synchronized (the `MetaBlock` lock protects access).
//!
//! The `FsVtable` is `static` and must be immutable. All function pointers
//! should be safe to call concurrently (they must handle their own locking).
//!
//! ## Locking Strategy
//!
//! `MetaBlock` is a `RwLock`. This means:
//! - Multiple readers can access the vtable and data simultaneously.
//! - A writer has exclusive access, typically used when modifying the filesystem's
//!   internal state (e.g., when creating or deleting inodes).
//! - The lock is held for the duration of the operation, ensuring consistency.

use core::ptr::addr_of_mut;

use alloc::string::String;

use crate::{sync::RwLock, vfs::{Error, Inode, InodeId, Kind}};

// ============================================================================
// FILESYSTEM VIRTUAL TABLE
// ============================================================================

/// The virtual function table for a filesystem.
///
/// This structure contains function pointers for all operations that a filesystem
/// must support. Each filesystem provides a static instance of this table with
/// its own implementations.
///
/// # Operation Signatures
///
/// - **lookup**: `fn(&Inode, String) -> Option<InodeId>`
///   Searches for an entry by name in a directory inode.
///
/// - **readdr**: `fn(&Inode, usize) -> Option<(String, InodeId)>`
///   Reads a directory entry by index (0‑based).
///
/// - **read**: `fn(&Inode, usize, &mut [u8]) -> Result<usize, Error>`
///   Reads data from a file at a given offset into a buffer.
///
/// - **write**: `fn(&Inode, usize, &[u8]) -> Result<usize, Error>`
///   Writes data to a file at a given offset from a buffer.
///
/// - **trunc**: `fn(&Inode, usize) -> Result<(), Error>`
///   Truncates a file to a new size (or extends with zeros).
///
/// - **unlink**: `fn(&mut Inode) -> Result<(), Error>`
///   Removes an inode from its parent directory and frees resources.
///
/// - **link**: `fn(&mut Inode, String, InodeId) -> Result<(), Error>`
///   Adds a new entry in a directory inode.
///
/// - **new**: `fn(&MetaBlock, Inode, Kind) -> Result<InodeId, Error>`
///   Creates a new inode in the filesystem.
///
/// - **get**: `fn(&MetaBlock, u32) -> Option<&'static Inode>`
///   Returns an immutable reference to an inode by its number.
///
/// - **get_mut**: `fn(&MetaBlock, u32) -> Option<&'static mut Inode>`
///   Returns a mutable reference to an inode by its number.
#[repr(C)]
pub struct FsVtable {
    pub lookup  : fn (inode: &    Inode, name    : String                ) -> Option<InodeId>,
    pub readdr  : fn (inode: &    Inode, offset  : usize                 ) -> Option<(String, InodeId)>,
    pub read    : fn (inode: &    Inode, offset  : usize , buf: &mut [u8]) -> Result<usize  , Error>,
    pub write   : fn (inode: &    Inode, offset  : usize , buf: &    [u8]) -> Result<usize  , Error>,
    pub trunc   : fn (inode: &    Inode, new_size: usize                 ) -> Result<()     , Error>,
    pub unlink  : fn (inode: &mut Inode,                                 ) -> Result<()     , Error>,
    pub link    : fn (inode: &mut Inode, name    : String, new: InodeId  ) -> Result<()     , Error>,
    pub new     : fn (mb: &MetaBlock, inode: Inode, kind    : Kind      ) -> Result<InodeId, Error>,
    pub get     : fn (mb: &MetaBlock, id   :      u32                    ) -> Option<&'static Inode>,
    pub get_mut : fn (mb: &MetaBlock, id   :      u32                    ) -> Option<&'static mut Inode>,
}

// ============================================================================
// META BLOCK INNER
// ============================================================================

/// The inner data of a `MetaBlock`.
///
/// This structure is stored inside the `RwLock` and holds the filesystem's
/// identifier, vtable, and opaque state pointer.
#[derive(Clone)]
pub struct MetaBlockInner {
    /// The unique identifier of this filesystem instance.
    /// This ID is used in `InodeId` to reference this filesystem.
    pub id: u32,

    /// The virtual function table that implements all filesystem operations.
    vtable: &'static FsVtable,

    /// Opaque pointer to the filesystem‑specific private state.
    /// This is typically a `*mut` to a struct like `PvfsMb`.
    data: usize,
}

impl MetaBlockInner {
    /// Returns an immutable reference to the filesystem's private data.
    ///
    /// # Type Parameters
    /// * `T` – The type of the private data structure.
    ///
    /// # Safety
    /// The caller must ensure that the `data` pointer is valid and points to
    /// an instance of `T`. This method performs a cast from `usize` to `*const T`
    /// and dereferences it.
    pub fn data_ref<T>(&self) -> &T {
        unsafe {
            (self.data as *const T).as_ref_unchecked()
        }
    }

    /// Returns a mutable reference to the filesystem's private data.
    ///
    /// # Type Parameters
    /// * `T` – The type of the private data structure.
    ///
    /// # Safety
    /// The caller must ensure that the `data` pointer is valid and points to
    /// an instance of `T`. This method performs a cast from `usize` to `*mut T`
    /// and dereferences it.
    pub fn data_mut<T>(&mut self) -> &mut T {
        unsafe {
            (self.data as *mut T).as_mut_unchecked()
        }
    }

    /// Returns a reference to the vtable.
    #[inline]
    pub fn vtable(&self) -> &'static FsVtable {
        self.vtable
    }
}

// ============================================================================
// META BLOCK TYPE ALIAS
// ============================================================================

/// A filesystem instance (mount point), protected by a read‑write lock.
///
/// `MetaBlock` is a type alias for `RwLock<MetaBlockInner>`. It provides safe
/// concurrent access to the filesystem's state and operations.
pub type MetaBlock = RwLock<MetaBlockInner>;

// ============================================================================
// META BLOCK CONSTRUCTOR
// ============================================================================

/// Creates a new `MetaBlock` instance.
///
/// # Arguments
/// * `id` – The initial ID (will be overwritten by the registry).
/// * `vtab` – A static reference to the filesystem's vtable.
/// * `data` – A mutable reference to the filesystem's private state.
///   This is typically a `&mut` to a struct like `PvfsMb`.
///
/// # Returns
/// A new `MetaBlock` (i.e., a `RwLock<MetaBlockInner>`).
///
/// # Example
/// ```ignore
/// let mut pvfs = PvfsMb::new();
/// let mblock = new_mblock(0, &PVFS_VTABLE, &mut pvfs);
/// let id = reg_mblk(mblock);
/// ```
///
/// # Safety
/// The `data` reference is converted to a raw pointer. The caller must ensure
/// that the data remains valid for the lifetime of the `MetaBlock`.
pub fn new_mblock(id: u32, vtab: &'static FsVtable, data: &mut ()) -> MetaBlock {
    MetaBlock::new(MetaBlockInner {
        id,
        vtable: vtab,
        data: addr_of_mut!(*data) as usize,
    })
}
