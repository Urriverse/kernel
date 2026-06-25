//! Purely Virtual Filesystem – in‑memory, no persistence
use alloc::collections::btree_map::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::min;
use crate::sync::Litex;
use crate::vfs::{FileSystem, Inode, InodeId, Kind, Error};

/// Directory Entry mapping a name to an InodeId
pub struct Dentry {
    pub name: String,
    pub inode_id: InodeId,
}

/// Content of an inode.
enum Data {
    File(Vec<u8>),
    Dir(Vec<Dentry>),
}

/// PVFS private state.
pub struct Pvfs {
    reg: Litex<BTreeMap<u32, Inode>>,   // inode number -> metadata
    data: Litex<BTreeMap<InodeId, Data>>, // full id -> file/dir content
    nxt: Litex<u32>,                     // next free inode number
}

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
            for entry in entries {
                if entry.name == name {
                    return Some(entry.inode_id);
                }
            }
        }
        None
    }

    fn readdir(&self, dir: InodeId, offset: usize) -> Option<(String, InodeId)> {
        let data_guard = self.data.lock();
        if let Some(Data::Dir(entries)) = data_guard.get(&dir) {
            entries.get(offset).map(|e| (e.name.clone(), e.inode_id))
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

    fn unlink(&self, dir: InodeId, name: &str) -> Result<(), Error> {
        let mut data_guard = self.data.lock();
        
        // 1. Find target inode ID without removing yet
        let target_inode_id = match data_guard.get(&dir) {
            Some(Data::Dir(entries)) => {
                match entries.iter().find(|e| e.name == name) {
                    Some(e) => e.inode_id,
                    None => return Err(Error::NoEntry),
                }
            }
            _ => return Err(Error::NotADirectory),
        };

        // 2. Check if target is a non-empty directory
        if let Some(Data::Dir(entries)) = data_guard.get(&target_inode_id) {
            if !entries.is_empty() {
                return Err(Error::NotEmpty);
            }
        }

        // 3. Safe to remove the dentry
        if let Some(Data::Dir(entries)) = data_guard.get_mut(&dir) {
            entries.retain(|e| e.name != name);
        }

        // 4. Decrement nlink and delete if 0
        let mut delete_inode = false;
        {
            let mut reg_guard = self.reg.lock();
            if let Some(inode) = reg_guard.get_mut(&target_inode_id.0) {
                inode.nlink = inode.nlink.saturating_sub(1);
                if inode.nlink == 0 {
                    delete_inode = true;
                }
            }
        }

        if delete_inode {
            data_guard.remove(&target_inode_id);
            let mut reg_guard = self.reg.lock();
            reg_guard.remove(&target_inode_id.0);
        }

        Ok(())
    }

    fn link(&self, parent: InodeId, name: &str, child: InodeId) -> Result<(), Error> {
        let mut data_guard = self.data.lock();
        match data_guard.get_mut(&parent) {
            Some(Data::Dir(entries)) => {
                if entries.iter().any(|e| e.name == name) {
                    return Err(Error::Found);
                }
                entries.push(Dentry { name: name.to_string(), inode_id: child });
                drop(data_guard);
                
                let mut reg_guard = self.reg.lock();
                if let Some(inode) = reg_guard.get_mut(&child.0) {
                    inode.nlink += 1;
                }
                Ok(())
            }
            _ => Err(Error::NotADirectory),
        }
    }

    fn new(&self, mb_id: u32, mut inode: Inode, kind: Kind) -> Result<InodeId, Error> {
        let mut nxt_guard = self.nxt.lock();
        let ino = *nxt_guard;
        *nxt_guard += 1;
        drop(nxt_guard);
        
        let id = InodeId(ino, mb_id);
        inode.id = id;
        inode.kind = kind;
        inode.nlink = 0; // Unlinked initially
        
        let mut reg_guard = self.reg.lock();
        reg_guard.insert(ino, inode);
        drop(reg_guard);
        
        let mut data_guard = self.data.lock();
        match kind {
            Kind::File => data_guard.insert(id, Data::File(Vec::new())),
            Kind::Directory => data_guard.insert(id, Data::Dir(Vec::new())),
            _ => return Err(Error::Unknown),
        };
        Ok(id)
    }

    fn stat(&self, inode: InodeId) -> Option<Inode> {
        self.reg.lock().get(&inode.0).copied()
    }
}
