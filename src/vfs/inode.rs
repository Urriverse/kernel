//! Inode metadata and identifiers

use alloc::string::String;

extrum! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum Flags: u64 {
        DIR         = 1 << 0,
        USER_READ   = 1 << 1,
        USER_WRITE  = 1 << 2,
        USER_EXEC   = 1 << 3,
        GROUP_READ  = 1 << 4,
        GROUP_WRITE = 1 << 5,
        GROUP_EXEC  = 1 << 6,
        OTHER_READ  = 1 << 7,
        OTHER_WRITE = 1 << 8,
        OTHER_EXEC  = 1 << 9,
        LEVEL_READ  = 1 << 10,
        LEVEL_WRITE = 1 << 11,
        LEVEL_EXEC  = 1 << 12,
    }
}

impl Flags {
    pub fn level(self) -> u16 { (self.0 >> 48) as u16 }
    pub fn set_level(&mut self, level: u16) {
        self.0 &= !0 << 16 >> 16;
        self.0 |= (level as u64) << 48;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InodeId(pub u32, pub u32); // (inode number, metablock id)

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Unknown,
    File,
    Directory,
    Socket,
    Virtual,
    SymLink,
}

/// Inode metadata – stored inside the filesystem.
#[repr(C, align(128))]
pub struct Inode {
    pub id      : InodeId,
    pub kind    : Kind,
    pub flags   : Flags,
    pub size    : u64,
    pub uid     : u16,
    pub gid     : u16,
    pub atime   : u64,
    pub mtime   : u64,
    pub ctime   : u64,
    pub parent  : InodeId,
    pub name    : String,
    pub private : [u8; 32],
}

impl Default for Inode {
    fn default() -> Self {
        Self {
            id      : InodeId(0, 0),
            kind    : Kind::Unknown,
            flags   : Flags::from_raw(0),
            size    : 0,
            uid     : 0,
            gid     : 0,
            atime   : 0,
            mtime   : 0,
            ctime   : 0,
            parent  : InodeId(0, 0),
            name    : String::new(),
            private : [0u8; 32],
        }
    }
}

impl Inode {
    pub fn new() -> Self { Self::default() }
    pub unsafe fn dublicate(&self) -> Self {
        let mut rv = Self::new();
        rv.kind = self.kind;
        rv.flags = self.flags;
        rv.size = self.size;
        rv.uid = self.uid;
        rv.gid = self.gid;
        rv.atime = self.atime;
        rv.mtime = self.mtime;
        rv.ctime = self.ctime;
        rv.parent = self.parent;
        rv.private = self.private.clone();
        rv.id = InodeId(0, 0);
        rv
    }
}
