//! # Purely Virtual Filesystem (PVFS)
//!
//! The PVFS is a simple, in‑memory filesystem that does not rely on any block
//! device or persistent storage. It is used primarily for:
//! - Early boot testing and validation of the VFS subsystem.
//! - Temporary file storage in ramdisks (e.g., `tmpfs`‑like functionality).
//!
//! ## Architecture
//!
//! PVFS stores all data in memory, using a global `BTreeMap` protected by a
//! `Litex` (interrupt‑disabling spinlock). The map associates `InodeId`s with
//! `Data` variants:
//!
//! - `Data::File(Vec<u8>)`: A regular file with its contents stored as a byte vector.
//! - `Data::Dir(Vec<(String, InodeId)>)`: A directory, storing a list of entries
//!   (name and inode ID).
//!
//! ## Filesystem Structure
//!
//! The PVFS is managed through a `PvfsMb` struct, which holds:
//! - `reg`: A `BTreeMap<u32, Inode>` mapping inode numbers to inode metadata.
//! - `nxt`: The next available inode number.
//!
//! Each inode stores its `id`, `kind`, `flags`, `size`, and other metadata. The
//! actual file data or directory entries are stored separately in the global `DATA`
//! map, keyed by the inode's full `InodeId`.
//!
//! ## Global Data Map
//!
//! ```text
//! DATA: Litex<BTreeMap<InodeId, Data>>
//! ```
//!
//! This map is the actual storage backend for PVFS. It is protected by a `Litex`
//! spinlock (which disables interrupts) to ensure safe concurrent access.
//!
//! ## VTable Operations
//!
//! PVFS implements the full `FsVtable` interface:
//!
//! - **`lookup`**: Searches a directory for an entry by name, returning the
//!   corresponding `InodeId`.
//! - **`readdr`**: Reads a directory entry by index, returning `(name, inode_id)`.
//! - **`read`**: Reads data from a file at a given offset into a buffer.
//! - **`write`**: Writes data to a file at a given offset, truncating or extending
//!   the file as needed.
//! - **`trunc`**: Truncates a file to a new size (currently returns `Err` for
//!   directories; files are truncated by shrinking the `Vec<u8>`).
//! - **`unlink`**: Removes a file or directory from its parent, freeing its data.
//! - **`link`**: Adds a new directory entry in a parent directory.
//! - **`new`**: Creates a new inode (file or directory) and returns its `InodeId`.
//! - **`get` / `get_mut`**: Retrieves an inode by its number from the filesystem's
//!   `reg` map.
//!
//! ## Limitations
//!
//! - No persistence: data is lost on reboot.
//! - No hard link count or reference tracking (but the VFS layer provides
//!   directory entries).
//! - No support for symlinks or sockets (though the `Kind` enum includes them,
//!   they are not implemented).
//! - Truncation of directories is not supported (returns `NotADirectory`).
//!
//! ## Usage Example
//!
//! ```ignore
//! let mbinst = PvfsMb::new();
//! let mblock = new_mblock(0, &PVFS_VTABLE, &mut mbinst);
//! let mb_id = reg_mblk(mblock);
//!
//! let inode = Inode::new();
//! let id = vfs::new(&mblock, inode, Kind::File).unwrap();
//! vfs::write(&id, 0, b"Hello, world!").unwrap();
//! let mut buf = [0; 13];
//! vfs::read(&id, 0, &mut buf).unwrap();
//! assert_eq!(&buf, b"Hello, world!");
//! ```
//!
//! ## Safety
//!
//! - The global `DATA` map is protected by a `Litex` (interrupt‑disabling spinlock).
//!   All operations on the map are performed while holding the lock.
//! - The `unsafe` blocks in `get` and `get_mut` are used to obtain references with
//!   static lifetime from the `BTreeMap`; this is safe because the map is never
//!   deallocated and the references are used only within the filesystem's lifetime.

use core::cmp::min;

use alloc::{collections::btree_map::BTreeMap, string::String, vec::Vec};

use crate::{sync::Litex, vfs::{Error, FsVtable, Inode, InodeId, Kind, MetaBlock}};

// ============================================================================
// DATA TYPES
// ============================================================================

/// The actual content stored for an inode.
///
/// - `File`: A byte vector representing the file's data.
/// - `Dir`: A vector of directory entries (name, inode ID).
enum Data {
    File(Vec<u8>),
    Dir(Vec<(String, InodeId)>),
}

// ============================================================================
// GLOBAL DATA STORAGE
// ============================================================================

/// Global storage for all PVFS files and directories.
///
/// This map is keyed by the full `InodeId` and contains the `Data` variant.
/// It is protected by a `Litex` spinlock.
static DATA: Litex<BTreeMap<InodeId, Data>> = Litex::new(BTreeMap::new());

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Returns `true` if the inode identified by `id` is a file.
#[inline]
fn is_file(id: InodeId) -> bool {
    matches!(&DATA.lock()[&id], Data::File(_))
}

// ============================================================================
// VTABLE FUNCTIONS
// ============================================================================

/// Look up an entry by name in a directory.
///
/// # Arguments
/// * `inode` – The directory inode.
/// * `name` – The name to look up.
///
/// # Returns
/// `Some(InodeId)` if found, `None` otherwise.
fn lookup(inode: &Inode, name: String) -> Option<InodeId> {
    match &DATA.lock()[&inode.id] {
        Data::Dir(d) => {
            for e in d {
                if e.0 == name {
                    return Some(e.1);
                }
            }
            None
        }
        _ => None,
    }
}

/// Read a directory entry by index.
///
/// # Arguments
/// * `inode` – The directory inode.
/// * `offset` – The index of the entry to read.
///
/// # Returns
/// `Some((name, inode_id))` if the index is valid, `None` otherwise.
fn readdr(inode: &Inode, offset: usize) -> Option<(String, InodeId)> {
    match &DATA.lock()[&inode.id] {
        Data::Dir(d) => {
            for (i, e) in d.iter().enumerate() {
                if i == offset {
                    return Some(e.clone());
                }
            }
            None
        }
        _ => None,
    }
}

/// Read data from a file.
///
/// # Arguments
/// * `inode` – The file inode.
/// * `offset` – The byte offset to start reading from.
/// * `buf` – The buffer to fill with data.
///
/// # Returns
/// The number of bytes read, or an `Error`.
fn read(inode: &Inode, offset: usize, buf: &mut [u8]) -> Result<usize, Error> {
    let _ = DATA.lock();

    if is_file(inode.id) {
        let data = unsafe { DATA.inner() }.get(&inode.id).ok_or(Error::NoEntry)?;
        let dlen = match data {
            Data::File(d) => d.len(),
            _ => return Err(Error::NotAFile),
        };
        if offset >= dlen {
            return Err(Error::OutOfBounds);
        }
        let ulen = min(dlen - offset, buf.len());
        let data_vec = match data {
            Data::File(d) => d,
            _ => unreachable!(),
        };
        buf[..ulen].copy_from_slice(&data_vec[offset..(ulen + offset)]);
        Ok(ulen)
    } else {
        Err(Error::NotAFile)
    }
}

/// Write data to a file.
///
/// The file is extended if the offset is beyond the current end, filling
/// with zero bytes. The buffer is then written at the offset.
///
/// # Arguments
/// * `inode` – The file inode.
/// * `offset` – The byte offset to start writing at.
/// * `buf` – The data to write.
///
/// # Returns
/// The number of bytes written, or an `Error`.
fn write(inode: &Inode, offset: usize, buf: &[u8]) -> Result<usize, Error> {
    let _ = DATA.lock();

    if !is_file(inode.id) {
        return Err(Error::NotAFile);
    }

    // Get current data (must exist)
    let current_data = unsafe { DATA.inner() }.get_mut(&inode.id).ok_or(Error::NoEntry)?;
    let old_len = match current_data {
        Data::File(d) => d.len(),
        _ => return Err(Error::NotAFile),
    };

    // Determine new length
    let new_len = if offset + buf.len() > old_len {
        offset + buf.len()
    } else {
        old_len
    };

    // Create a new vector and copy old data + new data
    let mut new_data = vec![0u8; new_len];
    // Copy old data if any
    if old_len > 0 {
        let old = match current_data {
            Data::File(d) => d,
            _ => unreachable!(),
        };
        new_data[0..old_len].copy_from_slice(old);
    }
    // Write new data at offset
    new_data[offset..offset + buf.len()].copy_from_slice(buf);

    // Replace the data
    *current_data = Data::File(new_data);

    Ok(buf.len())
}

/// Truncate a file to a new size.
///
/// # Arguments
/// * `inode` – The file inode.
/// * `new_size` – The new size in bytes.
///
/// # Returns
/// `Ok(())` on success, or an `Error`.
/// For directories, returns `NotADirectory`.
fn trunc(inode: &Inode, new_size: usize) -> Result<(), Error> {
    let _ = DATA.lock();

    if !is_file(inode.id) {
        return Err(Error::NotADirectory);
    }

    let data = unsafe { DATA.inner() }.get_mut(&inode.id).ok_or(Error::NoEntry)?;
    match data {
        Data::File(d) => {
            if new_size > d.len() {
                d.resize(new_size, 0);
            } else {
                d.truncate(new_size);
            }
            Ok(())
        }
        _ => Err(Error::NotAFile),
    }
}

/// Remove a child entry from a directory.
///
/// This helper is called from `unlink` to remove the entry from the parent.
fn remove_child(inode: &mut Inode, child: InodeId) {
    if let Data::Dir(d) = unsafe { DATA.inner() }.get_mut(&inode.id).unwrap() {
        d.retain(|e| e.1 != child);
    }
}

/// Unlink (remove) an inode from its parent.
///
/// This removes the inode's data from the global `DATA` map and also removes
/// the entry from its parent directory.
fn unlink(inode: &mut Inode) -> Result<(), Error> {
    match DATA.lock().remove(&inode.id) {
        Some(_) => {
            if let Some(i) = inode.parent.get_mut() {
                remove_child(i, inode.parent);
            }
            Ok(())
        }
        _ => Err(Error::NoEntry),
    }
}

/// Add a new directory entry (link) in a parent directory.
///
/// # Arguments
/// * `inode` – The parent directory inode (must be a directory).
/// * `name` – The name of the new entry.
/// * `new` – The `InodeId` to link to.
///
/// # Returns
/// `Ok(())` on success, or an `Error`.
fn link(inode: &mut Inode, name: String, new: InodeId) -> Result<(), Error> {
    let _ = DATA.lock();

    if !is_file(inode.id) {
        // Must be a directory to add entries
        let data = unsafe { DATA.inner() }.get_mut(&inode.id).ok_or(Error::NoEntry)?;
        if let Data::Dir(d) = data {
            // Check if name already exists
            for (n, _) in d.iter() {
                if *n == name {
                    return Err(Error::Found);
                }
            }
            d.push((name, new));
            return Ok(());
        }
    }
    Err(Error::NotADirectory)
}

/// Create a new inode in the filesystem.
///
/// # Arguments
/// * `mb` – The `MetaBlock` representing this PVFS instance.
/// * `inode` – The inode to create (must have `kind` and `flags` set).
/// * `kind` – The kind of inode (only `File` and `Directory` are supported).
///
/// # Returns
/// The new `InodeId`, or an `Error`.
fn new(mb: &MetaBlock, mut inode: Inode, kind: Kind) -> Result<InodeId, Error> {
    let _ = mb.write();
    match kind {
        Kind::Directory => {
            DATA.lock().insert(inode.id, Data::Dir(vec![]));
        }
        Kind::File => {
            DATA.lock().insert(inode.id, Data::File(vec![]));
        }
        _ => return Err(Error::Unknown),
    }
    unsafe {
        let mbr = (
            core::ptr::addr_of!(
                *mb
                    .inner()
                    .data_mut::<PvfsMb>()
            )
            as *mut ()
            as *mut PvfsMb
        )
            .as_mut_unchecked();
        inode.id = InodeId(mbr.nxt, mb.inner().id);
        mbr.nxt += 1;
        Ok(inode.id)
    }
}

/// Get an immutable reference to an inode by its inode number.
///
/// # Arguments
/// * `mb` – The `MetaBlock` of the filesystem.
/// * `id` – The inode number.
///
/// # Returns
/// `Some(&Inode)` if the inode exists, `None` otherwise.
fn get(mb: &MetaBlock, id: u32) -> Option<&'static Inode> {
    let _ = mb.read();

    unsafe {
        (
            core::ptr::addr_of!(
                *mb
                    .inner()
                    .data_ref::<PvfsMb>()
            )
        )
            .as_ref_unchecked()
    }
        .reg
        .get(&id)
}

/// Get a mutable reference to an inode by its inode number.
///
/// # Arguments
/// * `mb` – The `MetaBlock` of the filesystem.
/// * `id` – The inode number.
///
/// # Returns
/// `Some(&mut Inode)` if the inode exists, `None` otherwise.
fn get_mut(mb: &MetaBlock, id: u32) -> Option<&'static mut Inode> {
    let _ = mb.read();

    unsafe {
        (
            core::ptr::addr_of!(
                *mb
                    .inner()
                    .data_mut::<PvfsMb>()
            )
            as *mut ()
            as *mut PvfsMb
        )
            .as_mut_unchecked()
    }
        .reg
        .get_mut(&id)
}

// ============================================================================
// FILESYSTEM INSTANCE STRUCTURE
// ============================================================================

/// Per‑instance state for a PVFS mount.
///
/// This structure is stored in the `MetaBlock`'s `data` field.
pub struct PvfsMb {
    /// Map from inode number to inode metadata.
    reg: BTreeMap<u32, Inode>,
    /// Next available inode number.
    nxt: u32,
}

impl PvfsMb {
    /// Creates a new, empty PVFS instance.
    pub fn new() -> Self {
        Self {
            reg: BTreeMap::new(),
            nxt: 0,
        }
    }
}

// ============================================================================
// VTABLE EXPORT
// ============================================================================

/// The virtual function table for PVFS.
///
/// This static table provides all the operation pointers required by the VFS
/// to interact with a PVFS instance.
pub static PVFS_VTABLE: FsVtable = FsVtable {
    lookup,
    readdr,
    read,
    write,
    trunc,
    unlink,
    link,
    new,
    get,
    get_mut,
};
