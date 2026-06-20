use core::ops::Index;

use alloc::{collections::BTreeMap, string::String, sync::Arc};

use crate::{sync::Litex, vfs::InodeId};

pub struct RootReg(Litex<BTreeMap<String, InodeId>>);

impl Clone for RootReg {
    fn clone(&self) -> Self {
        let rv = Self::new();

        let _g1 = self.0.lock();

        {
            let _g2 = rv.0.lock();

            unsafe { rv.0.inner().iter().clone_from(&self.0.inner().iter()) };
        }

        rv
    }

    fn clone_from(&mut self, source: &Self)
    where
        Self: core::marker::Destruct,
    {
        self.0.lock().iter().clone_from(&source.0.lock().iter());
    }
}

impl Default for RootReg {
    fn default() -> Self {
        Self::new()
    }
}

impl RootReg {
    pub fn new() -> Self {
        Self ( Litex::new(BTreeMap::new()) )
    }

    pub fn add_root(&self, name: String, inode: InodeId) -> Option<InodeId> {
        self.0.lock().insert(name, inode)
    }

    pub fn add_new_root(&self, name: String, inode: InodeId) -> Result<(), InodeId> {
        let mut reg = self.0.lock();

        // safe as we locked it before
        if !unsafe { self.0.inner() }.contains_key(&name) {
            reg.insert(name, inode);
            return Ok(())
        }

        Err(inode)
    }

    pub fn pop_root(&self, name: String) -> Option<InodeId> {
        self.0.lock().remove(&name)
    }
}

impl Index<String> for RootReg {
    type Output = InodeId;

    fn index(&self, index: String) -> &Self::Output {
        let _ = self.0.lock();
        unsafe { self.0.inner() }.get(&index).unwrap()
    }
}

pub type RootRef = Arc<RootReg>;
