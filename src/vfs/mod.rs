//! Virtual File System – public API and path resolution

mod fs;
mod inode;
mod root;
mod pvfs;
mod err;

pub use fs::*;
pub use inode::*;
pub use root::*;
#[allow(unused_imports)]
pub use pvfs::*;
pub use err::*;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::sync::Arc;

/// High‑level operations that take a MetaBlock reference.

#[allow(dead_code)]
pub fn lookup(mb: &MetaBlock, dir: InodeId, name: &str) -> Option<InodeId> {
    mb.fs.lookup(dir, name)
}

#[allow(dead_code)]
pub fn readdir(mb: &MetaBlock, dir: InodeId, offset: usize) -> Option<(String, InodeId)> {
    mb.fs.readdir(dir, offset)
}

#[allow(dead_code)]
pub fn read(mb: &MetaBlock, file: InodeId, offset: usize, buf: &mut [u8]) -> Result<usize, Error> {
    mb.fs.read(file, offset, buf)
}

#[allow(dead_code)]
pub fn write(mb: &MetaBlock, file: InodeId, offset: usize, buf: &[u8]) -> Result<usize, Error> {
    mb.fs.write(file, offset, buf)
}

#[allow(dead_code)]
pub fn truncate(mb: &MetaBlock, file: InodeId, new_size: usize) -> Result<(), Error> {
    mb.fs.truncate(file, new_size)
}

#[allow(dead_code)]
pub fn unlink(mb: &MetaBlock, inode: InodeId) -> Result<(), Error> {
    mb.fs.unlink(inode)
}

#[allow(dead_code)]
pub fn link(mb: &MetaBlock, parent: InodeId, name: &str, child: InodeId) -> Result<(), Error> {
    mb.fs.link(parent, name, child)
}

#[allow(dead_code)]
pub fn new(mb: &MetaBlock, inode: Inode, kind: Kind) -> Result<InodeId, Error> {
    mb.fs.new(mb.id, inode, kind)
}

#[allow(dead_code)]
pub fn stat(mb: &MetaBlock, inode: InodeId) -> Option<Inode> {
    mb.fs.stat(inode)
}

/// Resolve an absolute path from the root mount point named "root".
/// Returns the final InodeId and the MetaBlock that owns it.
#[allow(dead_code)]
pub fn resolve_absolute(roots: &RootReg, path: &str) -> Result<(InodeId, Arc<MetaBlock>), Error> {
    // Normalize path: remove leading/trailing slashes, split by '/'
    let trimmed = path.trim_matches('/');
    if trimmed.is_empty() {
        // Empty path means root
        let root_id = roots.lookup("root").ok_or(Error::NotMounted)?;
        let mb = get_mblock(root_id.1).ok_or(Error::NoEntry)?;
        return Ok((root_id, mb));
    }

    let components: Vec<&str> = trimmed.split('/').collect();

    // Start at the root mount
    let mut current_id = roots.lookup("root").ok_or(Error::NotMounted)?;
    let current_mb = get_mblock(current_id.1).ok_or(Error::NoEntry)?;

    for comp in components {
        if comp == "." {
            continue;
        } else if comp == ".." {
            // Get parent
            let parent_id = stat(&current_mb, current_id)
                .ok_or(Error::NoEntry)?
                .parent;
            current_id = parent_id;
        } else {
            // Lookup in current directory
            match lookup(&current_mb, current_id, comp) {
                Some(child_id) => {
                    current_id = child_id;
                }
                None => return Err(Error::NoEntry),
            }
        }
    }

    Ok((current_id, current_mb))
}

/// Helper: Check if an InodeId is a mount point in the given RootReg.
#[allow(dead_code)]
pub fn is_mount_point(roots: &RootReg, id: InodeId) -> bool {
    roots.snapshot().values().any(|&v| v == id)
}

// Enhanced resolution that switches MetaBlock when encountering a mount point.
#[allow(dead_code)]
pub fn resolve_absolute_with_mounts(roots: &RootReg, path: &str) -> Result<(InodeId, Arc<MetaBlock>), Error> {
    let trimmed = path.trim_matches('/');
    if trimmed.is_empty() {
        let root_id = roots.lookup("root").ok_or(Error::NotMounted)?;
        let mb = get_mblock(root_id.1).ok_or(Error::NoEntry)?;
        return Ok((root_id, mb));
    }

    let components: Vec<&str> = trimmed.split('/').collect();

    // Start at root
    let mut current_id = roots.lookup("root").ok_or(Error::NotMounted)?;
    let mut current_mb = get_mblock(current_id.1).ok_or(Error::NoEntry)?;

    for comp in components {
        if comp == "." {
            continue;
        } else if comp == ".." {
            let parent_id = stat(&current_mb, current_id)
                .ok_or(Error::NoEntry)?
                .parent;
            // If parent is a mount point, we need to switch to the mount's filesystem.
            if is_mount_point(roots, parent_id) {
                // The parent inode is the root of some mount.
                // Switch to that mount's MetaBlock.
                let parent_inode = stat(&current_mb, parent_id).ok_or(Error::NoEntry)?;
                // The parent's id.1 gives the MetaBlock id.
                let mb_id = parent_id.1;
                current_mb = get_mblock(mb_id).ok_or(Error::NoEntry)?;
                current_id = parent_id;
            } else {
                current_id = parent_id;
            }
        } else {
            // Normal lookup
            match lookup(&current_mb, current_id, comp) {
                Some(child_id) => {
                    // If the child is a mount point, we switch to its filesystem.
                    if is_mount_point(roots, child_id) {
                        let mb_id = child_id.1;
                        current_mb = get_mblock(mb_id).ok_or(Error::NoEntry)?;
                    }
                    current_id = child_id;
                }
                None => return Err(Error::NoEntry),
            }
        }
    }

    Ok((current_id, current_mb))
}

#[allow(dead_code)]
pub fn listdir(mb: &MetaBlock, dir: InodeId) -> alloc::collections::btree_map::BTreeMap<String, InodeId> {
    let mut map = alloc::collections::btree_map::BTreeMap::new();
    let mut offset = 0;
    while let Some((name, id)) = readdir(mb, dir, offset) {
        map.insert(name, id);
        offset += 1;
    }
    map
}

#[allow(dead_code)]
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
