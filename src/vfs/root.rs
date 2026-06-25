//! Per‑process mount namespace (named roots)

use alloc::collections::btree_map::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;
use crate::sync::Litex;
use crate::vfs::InodeId;

/// Mount point registry for a process.
#[derive(Debug)]
pub struct RootReg(Litex<BTreeMap<String, InodeId>>);

impl RootReg {
    pub fn new() -> Self {
        Self(Litex::new(BTreeMap::new()))
    }

    /// Insert or replace a mount point.
    pub fn mount(&self, name: String, root: InodeId) -> Option<InodeId> {
        self.0.lock().insert(name, root)
    }

    /// Insert only if the name is free.
    pub fn mount_new(&self, name: String, root: InodeId) -> Result<(), InodeId> {
        let mut map = self.0.lock();
        if map.contains_key(&name) {
            Err(root)
        } else {
            map.insert(name, root);
            Ok(())
        }
    }

    /// Unmount (remove).
    pub fn unmount(&self, name: &str) -> Option<InodeId> {
        self.0.lock().remove(name)
    }

    /// Look up a mount point by name.
    pub fn lookup(&self, name: &str) -> Option<InodeId> {
        self.0.lock().get(name).copied()
    }

    /// Get a copy of all mount points (for debugging).
    pub fn snapshot(&self) -> BTreeMap<String, InodeId> {
        self.0.lock().clone()
    }
}

impl Default for RootReg {
    fn default() -> Self { Self::new() }
}

impl Clone for RootReg {
    fn clone(&self) -> Self {
        let map = self.0.lock().clone();
        Self(Litex::new(map))
    }
}

pub type RootRef = Arc<RootReg>;
