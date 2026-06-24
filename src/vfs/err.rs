//! # VFS Error Definitions
#[repr(usize)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    Unknown,
    NotAFile,
    OutOfBounds,
    NoEntry,
    NotADirectory,
    Found,
    AlreadyExists,
    InvalidPath,
    NotMounted,
    NotEmpty, // Added for directory unlinking
}
