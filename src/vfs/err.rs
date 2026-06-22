//! # VFS Error Definitions
//!
//! This module defines the error types used throughout the Virtual File System
//! subsystem. All VFS operations return `Result<T, Error>`, where `Error` is
//! an enum describing the various failure conditions that can occur during
//! filesystem operations.
//!
//! ## Error Categories
//!
//! The errors are designed to cover the most common filesystem operation failures:
//!
//! - **Lookup failures**: `NoEntry` – the requested file or directory does not exist.
//! - **Type mismatches**: `NotAFile`, `NotADirectory` – operation attempted on
//!   the wrong kind of inode.
//! - **Bounds errors**: `OutOfBounds` – read/write offset is beyond the file size.
//! - **Name conflicts**: `Found` – attempting to create an entry that already exists.
//! - **Generic failures**: `Unknown` – catch‑all for unexpected errors.
//!
//! ## Usage
//!
//! All VFS functions in the `vfs` module return `Result` with this `Error` type.
//! Filesystem implementations (`FsVtable` functions) also use this error type
//! to report failures to the VFS layer.
//!
//! ## Safety
//!
//! The `Error` enum is `#[repr(usize)]` and can be safely cast to/from `usize`
//! for FFI purposes (e.g., returning error codes to userspace). The discriminant
//! values are stable and should not be changed without careful consideration.

// ============================================================================
// ERROR ENUM
// ============================================================================

/// VFS operation error codes.
///
/// These are returned by all VFS functions when an operation cannot be completed.
/// The `#[repr(usize)]` attribute ensures that the variants have stable integer
/// values, suitable for FFI and system call interfaces.
#[repr(usize)]
#[derive(Debug)]
pub enum Error {
    /// An unknown or unspecified error occurred.
    ///
    /// This is a catch‑all for unexpected conditions that do not fit into
    /// other categories. It should be used sparingly; prefer more specific
    /// error types when possible.
    Unknown,

    /// The operation was attempted on a file, but the inode is not a regular file.
    ///
    /// For example, attempting to `read` or `write` on a directory will return
    /// this error.
    NotAFile,

    /// A read or write operation attempted to access an offset beyond the
    /// end of the file.
    ///
    /// This can also occur when truncating a file to a size larger than the
    /// current allocation (some filesystems may extend the file instead).
    OutOfBounds,

    /// The requested entry was not found in the directory or filesystem.
    ///
    /// This is typically returned by `lookup` when a name does not exist,
    /// or by `unlink` when the inode to remove is not present.
    NoEntry,

    /// The operation was attempted on a directory, but the inode is not a directory.
    ///
    /// For example, attempting to `link` or `lookup` on a regular file will
    /// return this error.
    NotADirectory,

    /// An entry with the same name already exists in the directory.
    ///
    /// This is returned by `link` or `add_root` when attempting to create
    /// an entry with a name that is already taken.
    Found,
}
