//! Filesystem abstraction, MetaBlock, and global registry
use alloc::string::String;
use alloc::sync::Arc;
use alloc::collections::btree_map::BTreeMap;
use crate::sync::RwLock;
use crate::vfs::{InodeId, Inode, Kind, Error};

/// The core interface that every filesystem must implement.
pub trait FileSystem: Send + Sync {
    fn lookup(&self, dir: InodeId, name: &str) -> Option<InodeId>;
    fn readdir(&self, dir: InodeId, offset: usize) -> Option<(String, InodeId)>;
    fn read(&self, file: InodeId, offset: usize, buf: &mut [u8]) -> Result<usize, Error>;
    fn write(&self, file: InodeId, offset: usize, buf: &[u8]) -> Result<usize, Error>;
    fn truncate(&self, file: InodeId, new_size: usize) -> Result<(), Error>;
    
    // Updated signature for hardlink support
    fn unlink(&self, dir: InodeId, name: &str) -> Result<(), Error>;
    
    fn link(&self, parent: InodeId, name: &str, child: InodeId) -> Result<(), Error>;
    fn new(&self, mb_id: u32, inode: Inode, kind: Kind) -> Result<InodeId, Error>;
    fn stat(&self, inode: InodeId) -> Option<Inode>;
}

/// MetaBlock (superblock) holds a filesystem instance and its unique ID.
pub struct MetaBlock {
    pub id: u32,
    pub fs: Arc<dyn FileSystem>,
}

impl MetaBlock {
    pub fn new(id: u32, fs: Arc<dyn FileSystem>) -> Self {
        MetaBlock { id, fs }
    }
}

// Global registry: maps ID -> Arc<MetaBlock>
lazy_static! {
    static ref MBLK_REG: RwLock<(BTreeMap<u32, Arc<MetaBlock>>, u32)> =
        RwLock::new((BTreeMap::new(), 0));
}

pub fn register_mblock(fs: Arc<dyn FileSystem>) -> u32 {
    let mut reg = MBLK_REG.write();
    let id = reg.1;
    reg.1 += 1;
    let mb = Arc::new(MetaBlock::new(id, fs));
    reg.0.insert(id, mb);
    id
}

pub fn get_mblock(id: u32) -> Option<Arc<MetaBlock>> {
    MBLK_REG.read().0.get(&id).cloned()
}
