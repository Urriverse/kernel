//! Virtual File System – public API and path resolution
mod root;
mod pvfs;
mod rotar;

use alloc::collections::btree_map::BTreeMap;
use ketypes::sync::KeRwLock as RwLock;
pub use root::*;
pub use rotar::*;
#[allow(unused_imports)]
pub use pvfs::*;
pub use ketypes::vfs::*;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::sync::Arc;

// ============================================================================
// HIGH-LEVEL VFS OPERATIONS
// ============================================================================

pub fn lookup(mb: &KeMetaBlock, dir: KeInodeId, name: &str) -> Option<KeInodeId> {
    mb.fs.lookup(dir, name)
}

pub fn readdir(mb: &KeMetaBlock, dir: KeInodeId, offset: usize) -> Option<(String, KeInodeId)> {
    mb.fs.readdir(dir, offset)
}

pub fn read(mb: &KeMetaBlock, file: KeInodeId, offset: usize, buf: &mut [u8]) -> Result<usize, KeFsError> {
    mb.fs.read(file, offset, buf)
}

pub fn write(mb: &KeMetaBlock, file: KeInodeId, offset: usize, buf: &[u8]) -> Result<usize, KeFsError> {
    mb.fs.write(file, offset, buf)
}

pub fn truncate(mb: &KeMetaBlock, file: KeInodeId, new_size: usize) -> Result<(), KeFsError> {
    mb.fs.truncate(file, new_size)
}

pub fn unlink(mb: &KeMetaBlock, dir: KeInodeId, name: &str) -> Result<(), KeFsError> {
    mb.fs.unlink(dir, name)
}

pub fn link(mb: &KeMetaBlock, parent: KeInodeId, name: &str, child: KeInodeId) -> Result<(), KeFsError> {
    mb.fs.link(parent, name, child)
}

pub fn new(mb: &KeMetaBlock, inode: KeInode, kind: Kind) -> Result<KeInodeId, KeFsError> {
    mb.fs.new(mb.id, inode, kind)
}

pub fn stat(mb: &KeMetaBlock, inode: KeInodeId) -> Option<KeInode> {
    mb.fs.stat(inode)
}

// ============================================================================
// PATH RESOLUTION
// ============================================================================

pub fn is_mount_point(id: KeInodeId) -> bool {
    let roots = &*
    crate
    ::  sched
    ::  current_process()
    .   expect("No current process")
    .   roots;
    roots.snapshot().values().any(|&v| v == id)
}

/// Resolve an absolute path, crossing filesystem boundaries when encountering mount points.
pub fn resolve(path: &str) -> Result<(KeInodeId, Arc<KeMetaBlock>), KeFsError> {
    resolve_path(path, true)
}

/// Internal path resolution engine.
///
/// Supports the `mount_name:/path/to/file` syntax. If no `mount_name:` prefix 
/// is provided, it defaults to looking up the `"root"` mount point.
fn resolve_path(path: &str, cross_mounts: bool) -> Result<(KeInodeId, Arc<KeMetaBlock>), KeFsError> {
    let roots = &*
    crate
    ::  sched
    ::  current_process()
    .   expect("No current process")
    .   roots;

    // 1. Parse the "mount_name:/path" syntax
    let (mount_name, actual_path) = if let Some(pos) = path.find(':') {
        (&path[..pos], &path[pos + 1..])
    } else {
        // Fallback to "root" if no mount name is specified
        ("root", path)
    };

    let trimmed = actual_path.trim_matches('/');
    
    // 2. Lookup the root inode using the parsed mount name
    let root_id = roots.lookup(mount_name).ok_or(KeFsError::NotMounted)?;
    let root_mb = get_mblock(root_id.1).ok_or(KeFsError::NoEntry)?;

    if trimmed.is_empty() {
        return Ok((root_id, root_mb));
    }

    let components: Vec<&str> = trimmed.split('/').filter(|s| !s.is_empty()).collect();
    
    // The path stack tracks our traversal context.
    let mut path_stack: Vec<(KeInodeId, Arc<KeMetaBlock>)> = vec![(root_id, root_mb)];

    for comp in components {
        if comp == "." {
            continue;
        } else if comp == ".." {
            // Go up one directory. We never pop the root.
            if path_stack.len() > 1 {
                path_stack.pop();
            }
        } else {
            let (curr_id, curr_mb) = path_stack.last().unwrap();
            
            // Verify current node is a directory
            let curr_stat = stat(curr_mb, *curr_id).ok_or(KeFsError::NoEntry)?;
            if curr_stat.kind != Kind::Directory {
                return Err(KeFsError::NotADirectory);
            }
            
            match lookup(curr_mb, *curr_id, comp) {
                Some(child_id) => {
                    let mut next_mb = curr_mb.clone();
                    let mut next_id = child_id;
                    
                    // If we hit a mount point and are allowed to cross it, switch KeMetaBlock
                    if cross_mounts && is_mount_point(child_id) {
                        if let Some(mb) = get_mblock(child_id.1) {
                            next_mb = mb;
                            next_id = child_id; // The ID acts as the root of the new FS
                        }
                    }
                    path_stack.push((next_id, next_mb));
                }
                None => return Err(KeFsError::NoEntry),
            }
        }
    }

    let (final_id, final_mb) = path_stack.last().unwrap();
    Ok((*final_id, final_mb.clone()))
}

lazy_static! {
    static ref MBLK_REG: RwLock<(BTreeMap<u32, Arc<KeMetaBlock>>, u32)> =
        RwLock::new((BTreeMap::new(), 0));
}

pub fn register_mblock(fs: Arc<dyn KeFileSystem>) -> u32 {
    let mut reg = MBLK_REG.write();
    let id = reg.1;
    reg.1 += 1;
    let mb = Arc::new(KeMetaBlock::new(id, fs.clone()));
    reg.0.insert(id, mb);
    fs.set_mb_id(id);
    id
}

pub fn get_mblock(id: u32) -> Option<Arc<KeMetaBlock>> {
    MBLK_REG.read().0.get(&id).cloned()
}

// ============================================================================
// HELPER UTILITIES
// ============================================================================

pub fn listdir(mb: &KeMetaBlock, dir: KeInodeId) -> alloc::collections::btree_map::BTreeMap<String, KeInodeId> {
    let mut map = alloc::collections::btree_map::BTreeMap::new();
    let mut offset = 0;
    while let Some((name, id)) = readdir(mb, dir, offset) {
        map.insert(name, id);
        offset += 1;
    }
    map
}

pub fn read_to_string(mb: &KeMetaBlock, file: KeInodeId) -> Result<String, KeFsError> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 512];
    let mut offset = 0;
    loop {
        match read(mb, file, offset, &mut chunk) {
            Ok(0) => break,
            Ok(n) => {
                buf.extend_from_slice(&chunk[..n]);
                offset += n;
            }
            Err(KeFsError::OutOfBounds) => break,
            Err(e) => return Err(e),
        }
    }
    String::from_utf8(buf).map_err(|_| KeFsError::Unknown)
}

pub fn mount(name: String, mb: u32) -> Option<KeInodeId> {
    crate
    ::  sched
    ::  current_process()
    .   expect("No current process")
    .   roots
    .   mount(name, KeInodeId(0, mb))
}
