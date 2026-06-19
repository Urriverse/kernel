use alloc::{borrow::ToOwned, string::String};

#[repr(C)]
pub enum Kind {
    Unknown,
    File,
    Directory,
    Socket, // AKA UNIX socket
    Stream, // AKA device file
    AbsLink, // AKA symlink
    RelLink, // AKA hardlink
}

#[repr(C)]
pub struct InodeVtable {
    //
}

#[repr(C)]
pub struct Inode {
    pub kind: Kind,
    pub mode: usize,
    pub size: usize,
    pub uid: u32,
    pub gid: u32,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
    pub vtable: &'static InodeVtable,
    pub name: String,
}

pub static EMPTY_VTABLE: InodeVtable = InodeVtable {};

impl Default for Inode {
    fn default() -> Self {
        Self {
            kind: Kind::Unknown,
            mode: 0,
            size: 0,
            uid: 0,
            gid: 0,
            atime: 0,
            mtime: 0,
            ctime: 0,
            vtable: &EMPTY_VTABLE,
            name: "".to_owned(),
        }
    }
}
