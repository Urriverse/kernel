use alloc::{collections::BTreeMap, string::String};

use crate::{sync::Litex, vfs::Inode};

static ROOT_REG: Litex<BTreeMap<String, Inode>> = Litex::new(BTreeMap::new());

pub fn add_root(name: String, inode: Inode) -> Option<Inode> {
    ROOT_REG.lock().insert(name, inode)
}

pub fn add_new_root(name: String, inode: Inode) -> Result<(), Inode> {
    let mut reg = ROOT_REG.lock();

    // safe as we locked it before
    if !unsafe { ROOT_REG.inner() }.contains_key(&name) {
        reg.insert(name, inode);
        return Ok(())
    }

    Err(inode)
}

pub fn pop_root(name: String) -> Option<Inode> {
    ROOT_REG.lock().remove(&name)
}
