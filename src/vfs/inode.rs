//! # VFS Inode Management
//!
//! This module defines the core inode structure, inode identifiers, and the global
//! registry of filesystem instances (`MetaBlock`s). Inodes represent filesystem
//! objects (files, directories, sockets, etc.) and are the primary interface for
//! VFS operations.
//!
//! ## Overview
//!
//! An **inode** is a metadata structure that describes a filesystem object. It
//! contains:
//! - **Identity**: `id` (`InodeId`), which is a combination of an inode number
//!   (unique within the filesystem) and a `MetaBlock` ID.
//! - **Type**: `kind` (`Kind`), indicating whether it's a file, directory, socket,
//!   virtual device, or symlink.
//! - **Permissions**: `flags` (`Flags`), which encode POSIX‑style read/write/execute
//!   permissions for user, group, and others, plus a security level.
//! - **Metadata**: `size`, `uid`, `gid`, `atime`, `mtime`, `ctime`.
//! - **Parent**: `parent` (`InodeId`), linking to the containing directory.
//! - **Filesystem binding**: `mblock` (`&'static MetaBlock`), a reference to the
//!   filesystem instance that owns this inode.
//! - **Name**: `name` (`String`), the entry name (used for lookup).
//! - **Private data**: `private` (`[u8; 32]`), a small buffer for filesystem‑specific
//!   usage.
//!
//! ## Inode Identifiers (`InodeId`)
//!
//! An `InodeId` is a tuple `(u32, u32)`:
//! - **First component**: The inode number, unique within the filesystem.
//! - **Second component**: The `MetaBlock` ID, which identifies the filesystem
//!   instance in the global registry.
//!
//! The `InodeId` provides methods `get()` and `get_mut()` that look up the
//! `MetaBlock` from the global registry and then fetch the inode from the
//! filesystem's internal data structures. This provides a safe way to obtain
//! a reference to the inode.
//!
//! ## MetaBlock Registry
//!
//! The global registry (`MBLK_REG`) is a `RwLock<BTreeMap<u32, MetaBlock>>` that
//! maps `MetaBlock` IDs to their instances. This registry is used by `InodeId`
//! to locate the filesystem when performing operations.
//!
//! - `reg_mblk(mb)`: Registers a new `MetaBlock` and returns its ID.
//! - `unreg_mblk(id)`: Unregisters a `MetaBlock` by ID.
//! - `get_mblk(id)`: Returns a reference to the `MetaBlock` for a given ID.
//!
//! ## Inode Flags (`Flags`)
//!
//! The `Flags` struct uses bitflags to encode:
//! - Directory flag (`DIR`).
//! - Read/write/execute permissions for user, group, and others.
//! - Level‑based permissions (for security levels).
//! - A 16‑bit security level stored in the upper bits.
//!
//! ## Empty VTable
//!
//! The `EMPTY_VTABLE` is a static `FsVtable` that panics on any operation. It is
//! used as a placeholder when an inode is created before being properly associated
//! with a filesystem.

use alloc::{borrow::ToOwned, collections::btree_map::BTreeMap, string::String};

use crate::{sync::RwLock, vfs::{FsVtable, MetaBlock, new_mblock}};

// ============================================================================
// INODE FLAGS
// ============================================================================

extrum! {
    /// Permissions and attributes for an inode.
    ///
    /// These flags encode POSIX‑like permissions and a security level.
    #[derive(Clone, Copy, PartialEq)]
    pub enum Flags: u64 {
        // Directory flag
        // The inode is a directory.
        DIR         = 1 << 0    ,

        // User owner rights
        USER_READ   = 1 << 1    ,
        USER_WRITE  = 1 << 2    ,
        USER_EXEC   = 1 << 3    ,

        // Group owner rights
        GROUP_READ  = 1 << 4    ,
        GROUP_WRITE = 1 << 5    ,
        GROUP_EXEC  = 1 << 6    ,

        // Others rights
        OTHER_READ  = 1 << 7    ,
        OTHER_WRITE = 1 << 8    ,
        OTHER_EXEC  = 1 << 9    ,

        // Level-defined rights
        LEVEL_READ  = 1 << 10   ,
        LEVEL_WRITE = 1 << 11   ,
        LEVEL_EXEC  = 1 << 12   ,
    }
}

impl Flags {
    /// Returns the security level stored in the upper bits of the flags.
    pub fn level(self) -> u16 { (self.0 >> 48) as u16 }

    /// Sets the security level in the upper bits of the flags.
    pub fn set_level(&mut self, level: u16) {
        self.0 &= !0 << 16 >> 16;
        self.0 |= (level as u64) << 48;
    }
}

implement_display![Flags];

// ============================================================================
// INODE IDENTIFIER
// ============================================================================

/// A unique identifier for an inode.
///
/// The first component is an inode number (unique within the filesystem),
/// and the second is the `MetaBlock` ID (identifying the filesystem instance).
#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct InodeId(pub u32, pub u32);

impl InodeId {
    /// Returns an immutable reference to the inode, if it exists.
    ///
    /// This function looks up the `MetaBlock` from the registry using the
    /// second component of the ID, then calls the `get` method of the
    /// filesystem's vtable to retrieve the inode.
    ///
    /// # Returns
    /// `Some(&'static Inode)` if the inode exists, otherwise `None`.
    pub fn get(self) -> Option<&'static Inode> {
        if let Some(mb) = get_mblk(self.1)
        && let Some(rv) = (mb.read().vtable().get)(mb, self.0) {
            return Some(rv)
        }
        None
    }

    /// Returns a mutable reference to the inode, if it exists.
    ///
    /// Similar to `get()`, but returns a mutable reference for in‑place updates.
    pub fn get_mut(self) -> Option<&'static mut Inode> {
        if let Some(mb) = get_mblk(self.1)
        && let Some(rv) = (mb.read().vtable().get_mut)(mb, self.0) {
            return Some(rv)
        }
        None
    }
}

// ============================================================================
// INODE KIND
// ============================================================================

/// The type of a filesystem object.
#[repr(C)]
pub enum Kind {
    /// Unknown or uninitialized.
    Unknown     ,
    /// A regular file.
    File        ,
    /// A directory (can contain entries).
    Directory   ,
    /// A UNIX domain socket.
    Socket      ,
    /// A virtual/device file (e.g., `/dev/null`).
    Virtual     ,
    /// A symbolic link.
    SymLink     ,
}

// ============================================================================
// INODE STRUCTURE
// ============================================================================

/// Metadata for a filesystem object.
///
/// This structure is stored in the filesystem's internal data structures
/// (e.g., in `PvfsMb.reg`). It is aligned to 128 bytes for cache efficiency.
#[repr(C, align(128))]
pub struct Inode {
    /// Unique identifier for this inode.
    pub id      :                InodeId    ,
    /// The kind of object (file, directory, etc.).
    pub kind    :                Kind       ,
    /// Permissions and attributes.
    pub flags   :                Flags      ,
    /// Size of the file in bytes (for directories, number of entries).
    pub size    :                u64        ,
    /// Owner user ID.
    pub uid     :                u16        ,
    /// Owner group ID.
    pub gid     :                u16        ,
    /// Last access time (in seconds since epoch).
    pub atime   :                u64        ,
    /// Last modification time.
    pub mtime   :                u64        ,
    /// Last status change time.
    pub ctime   :                u64        ,
    /// Parent inode (the containing directory).
    pub parent  :                InodeId    ,
    /// Reference to the filesystem instance that owns this inode.
    pub mblock  :       &'static MetaBlock  ,
    /// Entry name (used for lookups).
    pub name    :                String     ,
    /// Filesystem‑specific private data (32 bytes).
    pub private :                [u8; 32]   ,
}

// ============================================================================
// EMPTY VTABLE AND META BLOCK
// ============================================================================

/// A dummy filesystem vtable that panics on any operation.
///
/// This is used as a placeholder for inodes that are not yet associated with
/// a real filesystem.
pub static EMPTY_VTABLE: FsVtable = FsVtable {
    lookup  :|_,_  |panic!("empty FS vtable"),
    readdr  :|_,_  |panic!("empty FS vtable"),
    read    :|_,_,_|panic!("empty FS vtable"),
    write   :|_,_,_|panic!("empty FS vtable"),
    trunc   :|_,_  |panic!("empty FS vtable"),
    unlink  :|_,   |panic!("empty FS vtable"),
    link    :|_,_,_|panic!("empty FS vtable"),
    new     :|_,_,_|panic!("empty FS vtable"),
    get     :|_,_  |panic!("empty FS vtable"),
    get_mut :|_,_  |panic!("empty FS vtable"),
};

lazy_static! {
    /// A dummy `MetaBlock` associated with `EMPTY_VTABLE`.
    pub static ref EMPTY_MBLOCK: MetaBlock = new_mblock(!0, &EMPTY_VTABLE, &mut ());
}

// ============================================================================
// INODE DEFAULT CONSTRUCTOR
// ============================================================================

impl Default for Inode {
    /// Creates a new, default‑initialized inode.
    ///
    /// The inode is assigned `id = InodeId(0, 0)`, `kind = Kind::Unknown`,
    /// and all other fields are zero or empty. The `mblock` points to the
    /// global `EMPTY_MBLOCK`.
    fn default() -> Self {
        Self {
            id      : InodeId(0, 0)     ,
            kind    : Kind::Unknown     ,
            flags   : Flags::from_raw(0),
            size    : 0                 ,
            uid     : 0                 ,
            gid     : 0                 ,
            atime   : 0                 ,
            mtime   : 0                 ,
            ctime   : 0                 ,
            parent  : InodeId(0, 0)     ,
            mblock  : &*EMPTY_MBLOCK    ,
            name    : "".to_owned()     ,
            private : [0u8; 32]         ,
        }
    }
}

impl Inode {
    /// Public constructor for a new, default inode.
    ///
    /// Use this to create a new inode before adding it to a filesystem.
    pub fn new() -> Self { Self::default() }
}

// ============================================================================
// GLOBAL META BLOCK REGISTRY
// ============================================================================

lazy_static! {
    /// Global registry of filesystem instances (`MetaBlock`).
    ///
    /// This is a `RwLock` that protects a `BTreeMap` mapping `MetaBlock` IDs to
    /// their instances, along with a counter for the next available ID.
    ///
    /// The registry is seeded with the `EMPTY_MBLOCK` at ID `!0`.
    static ref MBLK_REG: RwLock<(BTreeMap<u32, MetaBlock>, u32)> = {
        let v: RwLock<(BTreeMap<u32, MetaBlock>, u32)> = RwLock::new((BTreeMap::new(), 0u32));
        let _ = v.write().0.insert(!0, RwLock::new(EMPTY_MBLOCK.read().clone()));
        v
    };
}

/// Registers a new `MetaBlock` in the global registry.
///
/// # Arguments
/// * `mb` – The `MetaBlock` to register.
///
/// # Returns
/// A unique ID that can be used in `InodeId` to refer to this filesystem.
pub fn reg_mblk(mb: MetaBlock) -> u32 {
    let mut rn = MBLK_REG.write();
    let rv = rn.1;
    mb.write().id = rv;
    rn.0.insert(rv, mb);
    rn.1 += 1;
    rv
}

/// Unregisters a `MetaBlock` from the global registry.
///
/// # Arguments
/// * `id` – The ID of the `MetaBlock` to remove.
///
/// # Returns
/// `Ok(MetaBlock)` if the registry entry was found and removed, `Err(())` otherwise.
pub fn unreg_mblk(id: u32) -> Result<MetaBlock, ()> {
    MBLK_REG.write().0.remove(&id).ok_or(())
}

/// Retrieves a `MetaBlock` from the registry by ID.
///
/// # Arguments
/// * `id` – The ID of the `MetaBlock`.
///
/// # Returns
/// `Some(&'static MetaBlock)` if the ID exists, otherwise `None`.
///
/// # Safety
/// This function uses unsafe code to obtain a reference with static lifetime.
/// The caller must ensure that the `MetaBlock` is not freed while the reference
/// is held (the registry holds ownership).
pub fn get_mblk(id: u32) -> Option<&'static MetaBlock> {
    if MBLK_REG.read().0.contains_key(&id) {
        return Some(&unsafe { MBLK_REG.inner() }.0[&id])
    }
    None
}

/// Retrieves a mutable reference to a `MetaBlock` from the registry by ID.
///
/// # Safety
/// This function uses unsafe code; the caller must ensure proper synchronization.
pub fn get_inode_mut(id: u32) -> Option<&'static mut MetaBlock> {
    if MBLK_REG.read().0.contains_key(&id) {
        return unsafe { MBLK_REG.inner() }.0.get_mut(&id)
    }
    None
}
