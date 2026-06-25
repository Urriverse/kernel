//! Virtual File System – public API and path resolution
mod fs;
mod inode;
mod root;
mod pvfs;
mod err;
mod rotar;

pub use fs::*;
pub use inode::*;
pub use root::*;
pub use err::*;
pub use rotar::*;
#[allow(unused_imports)]
pub use pvfs::*;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::sync::Arc;

// ============================================================================
// HIGH-LEVEL VFS OPERATIONS
// ============================================================================

pub fn lookup(mb: &MetaBlock, dir: InodeId, name: &str) -> Option<InodeId> {
    mb.fs.lookup(dir, name)
}

pub fn readdir(mb: &MetaBlock, dir: InodeId, offset: usize) -> Option<(String, InodeId)> {
    mb.fs.readdir(dir, offset)
}

pub fn read(mb: &MetaBlock, file: InodeId, offset: usize, buf: &mut [u8]) -> Result<usize, Error> {
    mb.fs.read(file, offset, buf)
}

pub fn write(mb: &MetaBlock, file: InodeId, offset: usize, buf: &[u8]) -> Result<usize, Error> {
    mb.fs.write(file, offset, buf)
}

pub fn truncate(mb: &MetaBlock, file: InodeId, new_size: usize) -> Result<(), Error> {
    mb.fs.truncate(file, new_size)
}

pub fn unlink(mb: &MetaBlock, dir: InodeId, name: &str) -> Result<(), Error> {
    mb.fs.unlink(dir, name)
}

pub fn link(mb: &MetaBlock, parent: InodeId, name: &str, child: InodeId) -> Result<(), Error> {
    mb.fs.link(parent, name, child)
}

pub fn new(mb: &MetaBlock, inode: Inode, kind: Kind) -> Result<InodeId, Error> {
    mb.fs.new(mb.id, inode, kind)
}

pub fn stat(mb: &MetaBlock, inode: InodeId) -> Option<Inode> {
    mb.fs.stat(inode)
}

// ============================================================================
// PATH RESOLUTION
// ============================================================================

pub fn is_mount_point(roots: &RootReg, id: InodeId) -> bool {
    roots.snapshot().values().any(|&v| v == id)
}

/// Resolve an absolute path from the root mount point named "root".
/// Does not cross filesystem boundaries (mount points).
pub fn resolve_absolute(roots: &RootReg, path: &str) -> Result<(InodeId, Arc<MetaBlock>), Error> {
    resolve_path(roots, path, false)
}

/// Resolve an absolute path, crossing filesystem boundaries when encountering mount points.
pub fn resolve_absolute_with_mounts(roots: &RootReg, path: &str) -> Result<(InodeId, Arc<MetaBlock>), Error> {
    resolve_path(roots, path, true)
}

/// Internal path resolution engine.
/// 
/// Uses a path stack to correctly resolve ".." without relying on `inode.parent`,
/// which is strictly necessary to support hardlinks and correctly traverse back 
/// across mount boundaries.
fn resolve_path(roots: &RootReg, path: &str, cross_mounts: bool) -> Result<(InodeId, Arc<MetaBlock>), Error> {
    let trimmed = path.trim_matches('/');
    
    let root_id = roots.lookup("root").ok_or(Error::NotMounted)?;
    let root_mb = get_mblock(root_id.1).ok_or(Error::NoEntry)?;
    
    if trimmed.is_empty() {
        return Ok((root_id, root_mb));
    }
    
    let components: Vec<&str> = trimmed.split('/').filter(|s| !s.is_empty()).collect();
    
    // The path stack tracks our traversal context.
    // This allows us to safely resolve ".." by simply popping the stack,
    // completely avoiding the need for a `parent` pointer in the Inode.
    let mut path_stack: Vec<(InodeId, Arc<MetaBlock>)> = vec![(root_id, root_mb)];
    
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
            let curr_stat = stat(curr_mb, *curr_id).ok_or(Error::NoEntry)?;
            if curr_stat.kind != Kind::Directory {
                return Err(Error::NotADirectory);
            }
            
            match lookup(curr_mb, *curr_id, comp) {
                Some(child_id) => {
                    let mut next_mb = curr_mb.clone();
                    let mut next_id = child_id;
                    
                    // If we hit a mount point and are allowed to cross it, switch MetaBlock
                    if cross_mounts && is_mount_point(roots, child_id) {
                        if let Some(mb) = get_mblock(child_id.1) {
                            next_mb = mb;
                            next_id = child_id; // The ID acts as the root of the new FS
                        }
                    }
                    
                    path_stack.push((next_id, next_mb));
                }
                None => return Err(Error::NoEntry),
            }
        }
    }
    
    let (final_id, final_mb) = path_stack.last().unwrap();
    Ok((*final_id, final_mb.clone()))
}

// ============================================================================
// HELPER UTILITIES
// ============================================================================

pub fn listdir(mb: &MetaBlock, dir: InodeId) -> alloc::collections::btree_map::BTreeMap<String, InodeId> {
    let mut map = alloc::collections::btree_map::BTreeMap::new();
    let mut offset = 0;
    while let Some((name, id)) = readdir(mb, dir, offset) {
        map.insert(name, id);
        offset += 1;
    }
    map
}

pub fn read_to_string(mb: &MetaBlock, file: InodeId) -> Result<String, Error> {
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
            Err(Error::OutOfBounds) => break,
            Err(e) => return Err(e),
        }
    }
    String::from_utf8(buf).map_err(|_| Error::Unknown)
}
