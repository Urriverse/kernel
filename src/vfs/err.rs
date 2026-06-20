#[repr(usize)]
#[derive(Debug)]
pub enum Error {
    Unknown,
    NotAFile,
    OutOfBounds,
    NoEntry,
    NotADirectory,
    Found,
}
