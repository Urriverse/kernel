// purely virtual FS (without block device storage)

use core::cmp::min;

use alloc::{collections::btree_map::BTreeMap, string::String, vec::Vec};

use crate::{sync::Litex, vfs::{Error, FsVtable, Inode, InodeId, Kind}};

enum Data {
    File(Vec<u8>),
    Dir(Vec<(String, InodeId)>),
}

static DATA: Litex<BTreeMap<InodeId, Data>> = Litex::new(BTreeMap::new());

fn is_file(id: InodeId) -> bool {
    match &DATA.lock()[&id] {
        Data::File(_) => true,
        _ => false,
    }
}

fn lookup(inode: &Inode, name: String) -> Option<InodeId> {
    match &DATA.lock()[&inode.id] {
        Data::Dir(d) => { for e in d { if e.0 == name { return Some(e.1.clone()) } } None },
        _ => { None }
    }
}

fn readdr(inode: &Inode, offset: usize) -> Option<(String, InodeId)> {
    match &DATA.lock()[&inode.id] {
        Data::Dir(d) => { for (i, e) in d.iter().enumerate() { if i == offset { return Some(e.clone()) } } None },
        _ => { None }
    }
}

fn read(inode: &Inode, offset: usize, buf: &mut [u8]) -> Result<usize, Error> {
    let _ = DATA.lock();

    if is_file(inode.id) {
        let dlen;
        {
            if let Data::File(d) = &unsafe{DATA.inner()}[&inode.id] {
                dlen = d.len();
            } else { dlen = 0 } // never happens
        }
        if let Data::File(data) = &unsafe{DATA.inner()}[&inode.id] {
            let ulen = min(dlen, buf.len());
            if ulen <= offset { return Err(Error::OutOfBounds) }
            let len = ulen - offset;
            for i in 0..len { buf[i] = data[i + offset]; }
            return Ok(len)
        }
    }
    Err(Error::NotAFile)
}

fn write(inode: &Inode, offset: usize, buf: &[u8]) -> Result<usize, Error> {
    let _ = DATA.lock();

    if is_file(inode.id) {
        let mut nbuf = vec![0].repeat(offset);

        if offset != 0 {
            if let Err(e) = read(&inode, 0, nbuf.as_mut_slice()) { return Err(e) }
        }

        nbuf.append(&mut buf.to_vec());

        *unsafe{DATA.inner()}.get_mut(&inode.id).unwrap() = Data::File(nbuf);

        return Ok(buf.len())
    }
    Err(Error::NotAFile)
}

fn trunc(inode: &Inode, new_size: usize) -> Result<(), Error> {
    let _ = DATA.lock();

    if is_file(inode.id) {
        if unsafe{DATA.inner()}.len() < new_size { return Err(Error::OutOfBounds) }

        let mut nbuf = vec![0].repeat(new_size);

        if let Err(e) = read(&inode, 0, nbuf.as_mut_slice()) { return Err(e) }

        *unsafe{DATA.inner()}.get_mut(&inode.id).unwrap() = Data::File(nbuf);
    }
    Err(Error::NotAFile)
}

fn unlink(inode: &mut Inode) -> Result<(), Error> {
    match DATA.lock().remove(&inode.id) {
        Some(_) => Ok(()),
        _ => Err(Error::NoEntry),
    }
}

fn link(inode: &mut Inode, name: String, new: InodeId) -> Result<(), Error> {
    let _ = DATA.lock();

    if !is_file(inode.id) {
        if let Data::Dir(d) = unsafe{DATA.inner()}.get_mut(&inode.id).unwrap() {
            for (n, _) in d {
                if *n == name { return Err(Error::Found) }
            }
        }
        if let Data::Dir(d) = unsafe{DATA.inner()}.get_mut(&inode.id).unwrap() {
            d.push((name, new))
        }
        return Ok(())
    }
    Err(Error::NotADirectory)
}

fn new(inode: &Inode, kind: Kind) -> Result<(), Error> {
    match kind {
        Kind::Directory => { DATA.lock().insert(inode.id, Data::Dir(vec![])); Ok(()) },
        Kind::File => { DATA.lock().insert(inode.id, Data::File(vec![])); Ok(()) },
        _ => { Err(Error::Unknown) }
    }
}

pub static PVFS_VTABLE: FsVtable = FsVtable {
    lookup  ,
    readdr  ,
    read    ,
    write   ,
    trunc   ,
    unlink  ,
    link    ,
    new     ,
};
