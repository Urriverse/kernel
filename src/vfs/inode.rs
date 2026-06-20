use alloc::{borrow::ToOwned, collections::btree_map::BTreeMap, string::String};

use crate::{sync::Litex, vfs::Error};

extrum! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum Flags: u64 {
        // directory flag (MUST be 0 if Inode::kind != Directory)
        DIR         = 1 << 0    ,

        // user owner rights
        USER_READ   = 1 << 1    ,
        USER_WRITE  = 1 << 2    ,
        USER_EXEC   = 1 << 3    ,

        // group owner rights
        GROUP_READ  = 1 << 4    ,
        GROUP_WRITE = 1 << 5    ,
        GROUP_EXEC  = 1 << 6    ,

        // others rights
        OTHER_READ  = 1 << 7    ,
        OTHER_WRITE = 1 << 8    ,
        OTHER_EXEC  = 1 << 9    ,

        // level-defined rights
        LEVEL_READ  = 1 << 10   ,
        LEVEL_WRITE = 1 << 11   ,
        LEVEL_EXEC  = 1 << 12   ,
    }
}

impl Flags {
    pub fn level(self) -> u16 { (self.0 >> 48) as u16 }
    pub fn set_level(&mut self, level: u16) {
        self.0 &= !0 << 16 >> 16;
        self.0 |= (level as u64) << 48;
    }
}

implement_display![Flags];

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct InodeId(u32);

impl InodeId {
    pub fn get<'a>(self) -> Option<&'a Inode> { get_inode(self) }
    pub fn get_mut<'a>(self) -> Option<&'a mut Inode> { get_inode_mut(self) }
}

#[repr(C)]
pub enum Kind {
    Unknown     ,
    File        ,
    Directory   ,
    Socket      , // AKA UNIX socket
    Virtual     , // AKA device file
    SymLink     ,
}

#[repr(C)]
pub struct FsVtable {
    pub lookup  :fn (inode: &    Inode, name    : String                ) -> Option<InodeId,         >,
    pub readdr  :fn (inode: &    Inode, offset  : usize                 ) -> Option<(String, InodeId)>,
    pub read    :fn (inode: &    Inode, offset  : usize , buf: &mut [u8]) -> Result<usize  , Error   >,
    pub write   :fn (inode: &    Inode, offset  : usize , buf: &    [u8]) -> Result<usize  , Error   >,
    pub trunc   :fn (inode: &    Inode, new_size: usize                 ) -> Result<()     , Error   >,
    pub unlink  :fn (inode: &mut Inode,                                 ) -> Result<()     , Error   >,
    pub link    :fn (inode: &mut Inode, name    : String, new: InodeId  ) -> Result<()     , Error   >,
    pub new     :fn (inode: &    Inode, kind    : Kind                  ) -> Result<()     , Error   >,
}

#[repr(C, align(128))]
pub struct Inode {
    pub id      :           InodeId     ,
    pub kind    :           Kind        ,
    pub flags   :           Flags       ,
    pub size    :           u64         ,
    pub uid     :           u16         ,
    pub gid     :           u16         ,
    pub atime   :           u64         ,
    pub mtime   :           u64         ,
    pub ctime   :           u64         ,
    pub parent  :           InodeId     ,
    pub vtable  : &'static  FsVtable    ,
    pub name    :           String      ,
    pub private :           [u8; 32]    ,
}

pub static EMPTY_VTABLE: FsVtable = FsVtable {
    lookup  :|_,_  |panic!("empty FS vtable"),
    readdr  :|_,_  |panic!("empty FS vtable"),
    read    :|_,_,_|panic!("empty FS vtable"),
    write   :|_,_,_|panic!("empty FS vtable"),
    trunc   :|_,_  |panic!("empty FS vtable"),
    unlink  :|_,   |panic!("empty FS vtable"),
    link    :|_,_,_|panic!("empty FS vtable"),
    new     :|_,_  |panic!("empty FS vtable"),
};

impl Inode {
    fn default<'a>() -> &'a mut Self {
        let rv = Self {
            id      : InodeId(0)        ,
            kind    : Kind::Unknown     ,
            flags   : Flags::from_raw(0),
            size    : 0                 ,
            uid     : 0                 ,
            gid     : 0                 ,
            atime   : 0                 ,
            mtime   : 0                 ,
            ctime   : 0                 ,
            parent  : InodeId(0)        ,
            vtable  : &EMPTY_VTABLE     ,
            name    : "".to_owned()     ,
            private : [0u8; 32]         ,
        };
        reg_inode(rv).get_mut().unwrap()
    }
}

impl Inode {
    pub fn new<'a>() -> &'a mut Self { Self::default() }
}

static INODE_REG: Litex<(BTreeMap<InodeId, Inode>, u32)> = Litex::new((BTreeMap::new(), 0));

pub fn reg_inode(inode: Inode) -> InodeId {
    let mut rn = INODE_REG.lock();
    let rv = rn.1;
    rn.0.insert(InodeId(rv), inode);
    rn.1 += 1;
    InodeId(rv)
}

pub fn unreg_inode(id: InodeId) -> Result<Inode, ()> {
    INODE_REG.lock().0.remove(&id).ok_or(())
}

pub fn get_inode<'a>(id: InodeId) -> Option<&'static Inode> {
    if INODE_REG.lock().0.contains_key(&id) {
        return Some(&unsafe { INODE_REG.inner() }.0[&id])
    }
    None
}

pub fn get_inode_mut<'a>(id: InodeId) -> Option<&'static mut Inode> {
    if INODE_REG.lock().0.contains_key(&id) {
        return unsafe { INODE_REG.inner() }.0.get_mut(&id)
    }
    None
}
