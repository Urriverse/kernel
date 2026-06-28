//! Rotar – Read-Only Tar Archive Filesystem (for initramfs)
use alloc::collections::btree_map::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::sync::Litex;
use ketypes::vfs::*;

#[repr(C)]
struct TarHeader {
    name: [u8; 100],
    mode: [u8; 8],
    uid: [u8; 8],
    gid: [u8; 8],
    size: [u8; 12],
    mtime: [u8; 12],
    chksum: [u8; 8],
    typeflag: u8,
    linkname: [u8; 100],
    magic: [u8; 6],
    version: [u8; 2],
    uname: [u8; 32],
    gname: [u8; 32],
    devmajor: [u8; 8],
    devminor: [u8; 8],
    prefix: [u8; 155],
    padding: [u8; 12],
}

struct RotarNode {
    #[allow(unused)]
    ino: u32,
    kind: Kind,
    size: u64,
    mtime: u64,
    data_offset: usize,
    data_len: usize,
}

pub struct Rotar {
    nodes: Litex<BTreeMap<u32, RotarNode>>,
    dirs: Litex<BTreeMap<u32, Vec<(String, u32)>>>,
    #[allow(unused)]
    paths: Litex<BTreeMap<String, u32>>,
    data: &'static [u8],
    mb_id: Litex<u32>,
}

fn parse_octal(bytes: &[u8]) -> usize {
    let mut res = 0;
    for &b in bytes {
        if b == 0 || b == b' ' || b == b'\n' { break; }
        if b >= b'0' && b <= b'7' {
            res = res * 8 + (b - b'0') as usize;
        } else {
            break;
        }
    }
    res
}

fn parse_string(bytes: &[u8]) -> &str {
    let len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    core::str::from_utf8(&bytes[..len]).unwrap_or("")
}

fn align_up(val: usize, align: usize) -> usize {
    (val + align - 1) & !(align - 1)
}

impl Rotar {
    pub fn new(data: &'static [u8]) -> Self {
        let mut nodes = BTreeMap::new();
        let mut dirs = BTreeMap::new();
        let mut paths = BTreeMap::new();
        
        let mut next_ino: u32 = 1;
        let root_ino = 0;
        
        nodes.insert(root_ino, RotarNode {
            ino: root_ino, kind: Kind::Directory, size: 0, mtime: 0, data_offset: 0, data_len: 0,
        });
        dirs.insert(root_ino, Vec::new());
        paths.insert(String::new(), root_ino);
        
        let mut offset = 0;
        while offset + 512 <= data.len() {
            let header_bytes = &data[offset..offset+512];
            if header_bytes.iter().all(|&b| b == 0) { break; }
            
            let header = unsafe { &*(header_bytes.as_ptr() as *const TarHeader) };
            
            let name_str = parse_string(&header.name);
            let prefix_str = parse_string(&header.prefix);
            let size = parse_octal(&header.size);
            let mtime = parse_octal(&header.mtime);
            let typeflag = header.typeflag;
            
            // Skip PAX extended headers and other special tar entries
            if typeflag == b'g' || typeflag == b'x' || typeflag == b'L' || typeflag == b'K' {
                offset += 512 + align_up(size, 512);
                continue;
            }
            
            let mut full_path = String::new();
            if !prefix_str.is_empty() {
                full_path.push_str(prefix_str);
                if !prefix_str.ends_with('/') { full_path.push('/'); }
            }
            full_path.push_str(name_str);
            
            let mut path = full_path.trim_matches('/').to_string();
            let is_dir = typeflag == b'5' || path.ends_with('/');
            if is_dir && path.ends_with('/') { path.pop(); }
            
            if path.is_empty() {
                offset += 512 + align_up(size, 512);
                continue;
            }
            
            let components: Vec<&str> = path.split('/').collect();
            let mut current_parent_ino = root_ino;
            let mut current_path = String::new();
            
            for (i, comp) in components.iter().enumerate() {
                if !current_path.is_empty() { current_path.push('/'); }
                current_path.push_str(comp);
                
                if let Some(&existing_ino) = paths.get(&current_path) {
                    current_parent_ino = existing_ino;
                } else {
                    let is_last = i == components.len() - 1;
                    let kind = if is_last {
                        if is_dir { Kind::Directory } 
                        else if typeflag == b'2' || typeflag == b'1' { Kind::SymLink }
                        else { Kind::File }
                    } else {
                        Kind::Directory
                    };
                    
                    let ino = next_ino;
                    next_ino += 1;
                    
                    let node = RotarNode { ino, kind, size: 0, mtime: 0, data_offset: 0, data_len: 0 };
                    nodes.insert(ino, node);
                    if kind == Kind::Directory { dirs.insert(ino, Vec::new()); }
                    paths.insert(current_path.clone(), ino);
                    
                    if let Some(parent_dir) = dirs.get_mut(&current_parent_ino) {
                        parent_dir.push((comp.to_string(), ino));
                    }
                    current_parent_ino = ino;
                }
            }
            
            if !is_dir {
                let final_ino = *paths.get(&path).unwrap();
                if let Some(node) = nodes.get_mut(&final_ino) {
                    node.size = size as u64;
                    node.mtime = mtime as u64;
                    node.data_offset = offset + 512;
                    node.data_len = size;
                }
            } else {
                let final_ino = *paths.get(&path).unwrap();
                if let Some(node) = nodes.get_mut(&final_ino) {
                    node.mtime = mtime as u64;
                }
            }
            offset += 512 + align_up(size, 512);
        }
        
        Self {
            nodes: Litex::new(nodes),
            dirs: Litex::new(dirs),
            paths: Litex::new(paths),
            data,
            mb_id: Litex::new(0),
        }
    }
}

impl FileSystem for Rotar {
    fn lookup(&self, dir: InodeId, name: &str) -> Option<InodeId> {
        let mb_id = *self.mb_id.lock();
        if dir.1 != mb_id { return None; }
        let dirs = self.dirs.lock();
        if let Some(children) = dirs.get(&dir.0) {
            for (child_name, child_ino) in children {
                if child_name == name { return Some(InodeId(*child_ino, mb_id)); }
            }
        }
        None
    }

    fn set_mb_id(&self, mb_id: u32) {
        *self.mb_id.lock() = mb_id;
    }

    fn readdir(&self, dir: InodeId, offset: usize) -> Option<(String, InodeId)> {
        let mb_id = *self.mb_id.lock();
        if dir.1 != mb_id { return None; }
        let dirs = self.dirs.lock();
        if let Some(children) = dirs.get(&dir.0) {
            children.get(offset).map(|(name, ino)| (name.clone(), InodeId(*ino, mb_id)))
        } else { None }
    }

    fn read(&self, file: InodeId, offset: usize, buf: &mut [u8]) -> Result<usize, Error> {
        let mb_id = *self.mb_id.lock();
        if file.1 != mb_id { return Err(Error::NoEntry); }
        let nodes = self.nodes.lock();
        if let Some(node) = nodes.get(&file.0) {
            if node.kind != Kind::File { return Err(Error::NotAFile); }
            if offset >= node.data_len { return Ok(0); }
            let available = node.data_len - offset;
            let to_read = core::cmp::min(available, buf.len());
            let start = node.data_offset + offset;
            buf[..to_read].copy_from_slice(&self.data[start..start+to_read]);
            Ok(to_read)
        } else { Err(Error::NoEntry) }
    }

    fn stat(&self, inode: InodeId) -> Option<Inode> {
        let mb_id = *self.mb_id.lock();
        if inode.1 != mb_id { return None; }
        let nodes = self.nodes.lock();
        nodes.get(&inode.0).map(|n| Inode {
            id: inode, kind: n.kind, flags: Flags::from_raw(0), size: n.size,
            uid: 0, gid: 0, atime: n.mtime, mtime: n.mtime, ctime: n.mtime, nlink: 1, private: [0; 34],
        })
    }

    fn write(&self, _file: InodeId, _offset: usize, _buf: &[u8]) -> Result<usize, Error> { Err(Error::Unknown) }
    fn truncate(&self, _file: InodeId, _new_size: usize) -> Result<(), Error> { Err(Error::Unknown) }
    fn unlink(&self, _dir: InodeId, _name: &str) -> Result<(), Error> { Err(Error::Unknown) }
    fn link(&self, _parent: InodeId, _name: &str, _child: InodeId) -> Result<(), Error> { Err(Error::Unknown) }
    fn new(&self, _mb_id: u32, _inode: Inode, _kind: Kind) -> Result<InodeId, Error> { Err(Error::Unknown) }
}
