//! Purely Virtual Filesystem – in‑memory, no persistence

use alloc::collections::btree_map::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::min;
use crate::sync::Litex;
use crate::vfs::{FileSystem, Inode, InodeId, Kind, Error};

/// Content of an inode.
enum Data {
    File(Vec<u8>),
    Dir(Vec<(String, InodeId)>),
}

/// PVFS private state.
pub struct Pvfs {
    reg: Litex<BTreeMap<u32, Inode>>,   // inode number -> metadata
    data: Litex<BTreeMap<InodeId, Data>>, // full id -> file/dir content
    nxt: Litex<u32>,                     // next free inode number
}

#[allow(dead_code)]
impl Pvfs {
    pub fn new() -> Self {
        Self {
            reg: Litex::new(BTreeMap::new()),
            data: Litex::new(BTreeMap::new()),
            nxt: Litex::new(0),
        }
    }
}

impl FileSystem for Pvfs {
    fn lookup(&self, dir: InodeId, name: &str) -> Option<InodeId> {
        let data_guard = self.data.lock();
        if let Some(Data::Dir(entries)) = data_guard.get(&dir) {
            for (n, id) in entries {
                if n == name {
                    return Some(*id);
                }
            }
        }
        None
    }

    fn readdir(&self, dir: InodeId, offset: usize) -> Option<(String, InodeId)> {
        let data_guard = self.data.lock();
        if let Some(Data::Dir(entries)) = data_guard.get(&dir) {
            entries.get(offset).cloned()
        } else {
            None
        }
    }

    fn read(&self, file: InodeId, offset: usize, buf: &mut [u8]) -> Result<usize, Error> {
        let data_guard = self.data.lock();
        match data_guard.get(&file) {
            Some(Data::File(data)) => {
                if offset >= data.len() {
                    return Err(Error::OutOfBounds);
                }
                let len = min(data.len() - offset, buf.len());
                buf[..len].copy_from_slice(&data[offset..offset+len]);
                Ok(len)
            }
            _ => Err(Error::NotAFile),
        }
    }

    fn write(&self, file: InodeId, offset: usize, buf: &[u8]) -> Result<usize, Error> {
        let mut data_guard = self.data.lock();
        match data_guard.get_mut(&file) {
            Some(Data::File(data)) => {
                let new_len = core::cmp::max(data.len(), offset + buf.len());
                data.resize(new_len, 0);
                data[offset..offset+buf.len()].copy_from_slice(buf);
                // Update size in metadata
                drop(data_guard);
                let mut reg_guard = self.reg.lock();
                if let Some(inode) = reg_guard.get_mut(&file.0) {
                    inode.size = new_len as u64;
                }
                Ok(buf.len())
            }
            _ => Err(Error::NotAFile),
        }
    }

    fn truncate(&self, file: InodeId, new_size: usize) -> Result<(), Error> {
        let mut data_guard = self.data.lock();
        match data_guard.get_mut(&file) {
            Some(Data::File(data)) => {
                data.resize(new_size, 0);
                drop(data_guard);
                let mut reg_guard = self.reg.lock();
                if let Some(inode) = reg_guard.get_mut(&file.0) {
                    inode.size = new_size as u64;
                }
                Ok(())
            }
            _ => Err(Error::NotAFile),
        }
    }

    fn unlink(&self, inode: InodeId) -> Result<(), Error> {
        // Remove data
        let mut data_guard = self.data.lock();
        if data_guard.remove(&inode).is_none() {
            return Err(Error::NoEntry);
        }
        drop(data_guard);

        // Get parent ID from metadata
        let parent_id = {
            let reg_guard = self.reg.lock();
            if let Some(inode) = reg_guard.get(&inode.0) {
                inode.parent
            } else {
                return Err(Error::NoEntry);
            }
        };

        // Remove from parent's directory entries
        let mut data_guard = self.data.lock();
        if let Some(Data::Dir(entries)) = data_guard.get_mut(&parent_id) {
            entries.retain(|(_, id)| *id != inode);
        }
        drop(data_guard);

        // Remove from reg
        let mut reg_guard = self.reg.lock();
        reg_guard.remove(&inode.0);
        Ok(())
    }

    fn link(&self, parent: InodeId, name: &str, child: InodeId) -> Result<(), Error> {
        let mut data_guard = self.data.lock();
        match data_guard.get_mut(&parent) {
            Some(Data::Dir(entries)) => {
                // Check for duplicate
                for (n, _) in entries.iter() {
                    if n == name {
                        return Err(Error::Found);
                    }
                }
                entries.push((name.to_string(), child));
                Ok(())
            }
            _ => Err(Error::NotADirectory),
        }
    }

    fn new(&self, mb_id: u32, mut inode: Inode, kind: Kind) -> Result<InodeId, Error> {
        // Allocate new inode number
        let mut nxt_guard = self.nxt.lock();
        let ino = *nxt_guard;
        *nxt_guard += 1;
        drop(nxt_guard);

        let id = InodeId(ino, mb_id);
        inode.id = id;
        inode.kind = kind;

        // Insert metadata
        let mut reg_guard = self.reg.lock();
        reg_guard.insert(ino, inode);
        drop(reg_guard);

        // Insert data
        let mut data_guard = self.data.lock();
        match kind {
            Kind::File => data_guard.insert(id, Data::File(Vec::new())),
            Kind::Directory => data_guard.insert(id, Data::Dir(Vec::new())),
            _ => return Err(Error::Unknown),
        };

        Ok(id)
    }

    fn stat(&self, inode: InodeId) -> Option<Inode> {
        if let Some(x) = self.reg.lock().get(&inode.0) {
            return Some(unsafe { x.dublicate() })
        }
        None
    }
}
