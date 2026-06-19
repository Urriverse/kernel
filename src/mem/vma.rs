use alloc::alloc::{alloc, dealloc, Layout};
use core::ptr::NonNull;
use crate::sync::Nutex;

pub const MAX_CANONICAL_ADDR: usize = 0x0000_7FFF_FFFF_FFFF;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VmaFlags: u64 {
        const READ    = 1 << 0;
        const WRITE   = 1 << 1;
        const EXEC    = 1 << 2;
        const ANON    = 1 << 3;
        const FIXED   = 1 << 4;
        const GROWSUP = 1 << 5;
        const GROWSDN = 1 << 6;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

#[repr(C)]
#[derive(PartialEq)]
pub struct VmaNode {
    pub start: usize,
    pub end: usize,
    pub flags: VmaFlags,
    
    parent: Option<NonNull<VmaNode>>,
    left: Option<NonNull<VmaNode>>,
    right: Option<NonNull<VmaNode>>,
    color: Color,
    
    subtree_max_end: usize,
    subtree_min_start: usize,
    subtree_max_gap: usize,
}

impl VmaNode {
    fn new(start: usize, end: usize, flags: VmaFlags) -> Self {
        Self {
            start,
            end,
            flags,
            parent: None,
            left: None,
            right: None,
            color: Color::Red,
            subtree_max_end: end,
            subtree_min_start: start,
            subtree_max_gap: start,
        }
    }
}

#[inline]
fn update_augmentation(node: &mut NonNull<VmaNode>) {
    unsafe {
        let n = node.as_mut();
        let mut max_end = n.end;
        let mut min_start = n.start;
        let mut max_gap: usize;

        let left_max_end = n.left.map_or(0, |l| l.as_ref().subtree_max_end);
        let left_min_start = n.left.map_or(n.start, |l| l.as_ref().subtree_min_start);
        let left_max_gap = n.left.map_or(0, |l| l.as_ref().subtree_max_gap);

        let right_max_end = n.right.map_or(n.end, |r| r.as_ref().subtree_max_end);
        let right_min_start = n.right.map_or(n.start, |r| r.as_ref().subtree_min_start);
        let right_max_gap = n.right.map_or(0, |r| r.as_ref().subtree_max_gap);

        if left_max_end > max_end { max_end = left_max_end; }
        if right_max_end > max_end { max_end = right_max_end; }

        if left_min_start < min_start { min_start = left_min_start; }
        if right_min_start < min_start { min_start = right_min_start; }

        let gap_before_left = n.left.map_or(n.start, |_| left_min_start);
        let gap_left_to_node = n.start.saturating_sub(left_max_end);
        let gap_node_to_right = right_min_start.saturating_sub(n.end);

        max_gap = gap_before_left;
        if gap_left_to_node > max_gap { max_gap = gap_left_to_node; }
        if gap_node_to_right > max_gap { max_gap = gap_node_to_right; }
        if left_max_gap > max_gap { max_gap = left_max_gap; }
        if right_max_gap > max_gap { max_gap = right_max_gap; }

        n.subtree_max_end = max_end;
        n.subtree_min_start = min_start;
        n.subtree_max_gap = max_gap;
    }
}

struct VmaTree {
    root: Option<NonNull<VmaNode>>,
    cached_hint: usize,
}

unsafe impl Sync for VmaNode {}
unsafe impl Sync for VmaTree {}
unsafe impl Send for VmaNode {}
unsafe impl Send for VmaTree {}

impl VmaTree {
    const fn new() -> Self {
        Self {
            root: None,
            cached_hint: 0x1000_0000_0000,
        }
    }

    fn rotate_left(&mut self, mut x: NonNull<VmaNode>) {
        let mut y = unsafe { x.as_ref().right }.expect("rotate_left on node without right child");
        unsafe {
            x.as_mut().right = y.as_ref().left;
            if let Some(mut y_left) = y.as_ref().left {
                y_left.as_mut().parent = Some(x);
            }
            y.as_mut().parent = x.as_ref().parent;
            if let Some(mut p) = x.as_ref().parent {
                if p.as_ref().left == Some(x) { p.as_mut().left = Some(y); }
                else { p.as_mut().right = Some(y); }
            } else {
                self.root = Some(y);
            }
            y.as_mut().left = Some(x);
            x.as_mut().parent = Some(y);
            update_augmentation(&mut x);
            update_augmentation(&mut y);
        }
    }

    fn rotate_right(&mut self, mut y: NonNull<VmaNode>) {
        let mut x = unsafe { y.as_ref().left }.expect("rotate_right on node without left child");
        unsafe {
            y.as_mut().left = x.as_ref().right;
            if let Some(mut x_right) = x.as_ref().right {
                x_right.as_mut().parent = Some(y);
            }
            x.as_mut().parent = y.as_ref().parent;
            if let Some(mut p) = y.as_ref().parent {
                if p.as_ref().left == Some(y) { p.as_mut().left = Some(x); }
                else { p.as_mut().right = Some(x); }
            } else {
                self.root = Some(x);
            }
            x.as_mut().right = Some(y);
            y.as_mut().parent = Some(x);
            update_augmentation(&mut y);
            update_augmentation(&mut x);
        }
    }

    fn insert(&mut self, mut z: NonNull<VmaNode>) {
        let mut y: Option<NonNull<VmaNode>> = None;
        let mut x = self.root;

        while let Some(curr) = x {
            y = Some(curr);
            unsafe {
                if z.as_ref().start < curr.as_ref().start {
                    x = curr.as_ref().left;
                } else {
                    x = curr.as_ref().right;
                }
            }
        }

        unsafe {
            z.as_mut().parent = y;
            if y.is_none() {
                self.root = Some(z);
            } else if let Some(mut y_node) = y {
                if z.as_ref().start < y_node.as_ref().start {
                    y_node.as_mut().left = Some(z);
                } else {
                    y_node.as_mut().right = Some(z);
                }
            }
        }

        let mut curr = unsafe { z.as_ref().parent };
        while let Some(mut node) = curr {
            update_augmentation(&mut node);
            curr = unsafe { node.as_ref().parent };
        }

        self.insert_fixup(z);
    }

    fn insert_fixup(&mut self, mut z: NonNull<VmaNode>) {
        unsafe {
            while let Some(mut p) = z.as_ref().parent {
                if p.as_ref().color == Color::Black { break; }
                
                let mut gp = p.as_ref().parent.unwrap();
                if p.as_ref() == gp.as_ref().left.unwrap().as_ref() {
                    let y = gp.as_ref().right;
                    if let Some(mut y_node) = y {
                        if y_node.as_ref().color == Color::Red {
                            p.as_mut().color = Color::Black;
                            y_node.as_mut().color = Color::Black;
                            gp.as_mut().color = Color::Red;
                            z = gp;
                            continue;
                        }
                    }
                    if z == p.as_ref().right.expect("UB") {
                        z = p;
                        self.rotate_left(z);
                        p = z.as_ref().parent.unwrap();
                        gp = p.as_ref().parent.unwrap();
                    }
                    p.as_mut().color = Color::Black;
                    gp.as_mut().color = Color::Red;
                    self.rotate_right(gp);
                } else {
                    let y = gp.as_ref().left;
                    if let Some(mut y_node) = y {
                        if y_node.as_ref().color == Color::Red {
                            p.as_mut().color = Color::Black;
                            y_node.as_mut().color = Color::Black;
                            gp.as_mut().color = Color::Red;
                            z = gp;
                            continue;
                        }
                    }
                    if z == p.as_ref().left.expect("UB") {
                        z = p;
                        self.rotate_right(z);
                        p = z.as_ref().parent.unwrap();
                        gp = p.as_ref().parent.unwrap();
                    }
                    p.as_mut().color = Color::Black;
                    gp.as_mut().color = Color::Red;
                    self.rotate_left(gp);
                }
            }
            self.root.as_mut().unwrap().as_mut().color = Color::Black;
        }
    }

    fn remove(&mut self, z: NonNull<VmaNode>) {
        let mut y = z;
        let mut y_original_color = unsafe { y.as_ref().color };
        let x: Option<NonNull<VmaNode>>;

        unsafe {
            if z.as_ref().left.is_none() {
                x = z.as_ref().right;
                self.transplant(z, z.as_ref().right);
            } else if z.as_ref().right.is_none() {
                x = z.as_ref().left;
                self.transplant(z, z.as_ref().left);
            } else {
                y = self.tree_minimum(z.as_ref().right.unwrap());
                y_original_color = y.as_ref().color;
                x = y.as_ref().right;

                if y.as_ref().parent == Some(z) {
                    if let Some(mut x_node) = x {
                        x_node.as_mut().parent = Some(y);
                    }
                } else {
                    self.transplant(y, y.as_ref().right);
                    y.as_mut().right = z.as_ref().right;
                    if let Some(mut right) = y.as_ref().right {
                        right.as_mut().parent = Some(y);
                    }
                }
                self.transplant(z, Some(y));
                y.as_mut().left = z.as_ref().left;
                if let Some(mut left) = y.as_ref().left {
                    left.as_mut().parent = Some(y);
                }
                y.as_mut().color = z.as_ref().color;
            }
        }

        let mut curr = x.or_else(|| unsafe { y.as_ref().parent });
        while let Some(mut node) = curr {
            update_augmentation(&mut node);
            curr = unsafe { node.as_ref().parent };
        }

        if y_original_color == Color::Black {
            if let Some(x_node) = x {
                self.remove_fixup(x_node);
            }
        }
    }

    fn remove_fixup(&mut self, mut x: NonNull<VmaNode>) {
        while Some(x) != self.root && unsafe { x.as_ref().color } == Color::Black {
            let mut parent = unsafe { x.as_ref().parent }.unwrap();
            let is_left = unsafe { parent.as_ref().left == Some(x) };

            let mut w = if is_left {
                unsafe { parent.as_ref().right }.unwrap()
            } else {
                unsafe { parent.as_ref().left }.unwrap()
            };

            if unsafe { w.as_ref().color } == Color::Red {
                unsafe {
                    w.as_mut().color = Color::Black;
                    parent.as_mut().color = Color::Red;
                }
                if is_left { self.rotate_left(parent); } else { self.rotate_right(parent); }
                w = if is_left {
                    unsafe { parent.as_ref().right }.unwrap()
                } else {
                    unsafe { parent.as_ref().left }.unwrap()
                };
            }

            let left_black = unsafe { w.as_ref().left.map_or(true, |n| n.as_ref().color == Color::Black) };
            let right_black = unsafe { w.as_ref().right.map_or(true, |n| n.as_ref().color == Color::Black) };

            if left_black && right_black {
                unsafe { w.as_mut().color = Color::Red };
                x = parent;
            } else {
                if is_left {
                    if right_black {
                        if let Some(mut left) = unsafe { w.as_ref().left } {
                            unsafe { left.as_mut().color = Color::Black };
                        }
                        unsafe { w.as_mut().color = Color::Red };
                        self.rotate_right(w);
                        w = unsafe { parent.as_ref().right }.unwrap();
                    }
                    unsafe {
                        w.as_mut().color = parent.as_ref().color;
                        parent.as_mut().color = Color::Black;
                        if let Some(mut right) = w.as_ref().right {
                            right.as_mut().color = Color::Black;
                        }
                    }
                    self.rotate_left(parent);
                    x = self.root.unwrap();
                } else {
                    if left_black {
                        if let Some(mut right) = unsafe { w.as_ref().right } {
                            unsafe { right.as_mut().color = Color::Black };
                        }
                        unsafe { w.as_mut().color = Color::Red };
                        self.rotate_left(w);
                        w = unsafe { parent.as_ref().left }.unwrap();
                    }
                    unsafe {
                        w.as_mut().color = parent.as_ref().color;
                        parent.as_mut().color = Color::Black;
                        if let Some(mut left) = w.as_ref().left {
                            left.as_mut().color = Color::Black;
                        }
                    }
                    self.rotate_right(parent);
                    x = self.root.unwrap();
                }
            }
        }
        unsafe { x.as_mut().color = Color::Black };
    }

    fn transplant(&mut self, u: NonNull<VmaNode>, v: Option<NonNull<VmaNode>>) {
        unsafe {
            if u.as_ref().parent.is_none() {
                self.root = v;
            } else if let Some(mut p) = u.as_ref().parent {
                if p.as_ref().left == Some(u) {
                    p.as_mut().left = v;
                } else {
                    p.as_mut().right = v;
                }
            }
            if let Some(mut v_node) = v {
                v_node.as_mut().parent = u.as_ref().parent;
            }
        }
    }

    fn tree_minimum(&self, mut node: NonNull<VmaNode>) -> NonNull<VmaNode> {
        while let Some(left) = unsafe { node.as_ref().left } {
            node = left;
        }
        node
    }

    fn find_overlap(&self, addr: usize) -> Option<NonNull<VmaNode>> {
        let mut curr = self.root;
        while let Some(node) = curr {
            let n = unsafe { node.as_ref() };
            if addr >= n.start && addr < n.end {
                return Some(node);
            }
            if let Some(left) = n.left {
                if unsafe { left.as_ref().subtree_max_end } > addr {
                    curr = n.left;
                    continue;
                }
            }
            curr = n.right;
        }
        None
    }

    fn get_unmapped_area(&mut self, size: usize, align: usize, hint: usize) -> Option<usize> {
        if size == 0 || size > MAX_CANONICAL_ADDR {
            return None;
        }

        if let Some(root) = self.root {
            if unsafe { root.as_ref().subtree_max_gap } < size {
                let last_end = unsafe { root.as_ref().subtree_max_end };
                if MAX_CANONICAL_ADDR.saturating_sub(last_end) < size {
                    return None;
                }
            }
        }

        let mut curr = self.root;
        let mut last_end = 0usize;
        let mut best_addr: Option<usize> = None;

        while let Some(node) = curr {
            let n = unsafe { node.as_ref() };
            
            if n.start > last_end {
                let gap = n.start - last_end;
                if gap >= size {
                    let aligned = (last_end + align - 1) & !(align - 1);
                    if aligned >= last_end && aligned + size <= n.start {
                        let candidate = if hint >= last_end && hint < n.start {
                            let hint_aligned = (hint + align - 1) & !(align - 1);
                            if hint_aligned >= last_end && hint_aligned + size <= n.start {
                                hint_aligned
                            } else {
                                aligned
                            }
                        } else {
                            aligned
                        };
                        
                        if best_addr.is_none() || candidate < best_addr.unwrap() {
                            best_addr = Some(candidate);
                        }
                    }
                }
            }
            
            let go_left = if let Some(left) = n.left {
                (unsafe { left.as_ref().subtree_max_gap }) >= size
            } else {
                false
            };

            if go_left {
                curr = n.left;
            } else {
                let left_max_end = n.left.map_or(0, |l| unsafe { l.as_ref().subtree_max_end });
                last_end = if left_max_end > n.end { left_max_end } else { n.end };
                curr = n.right;
            }
        }
        
        if let Some(root) = self.root {
            let final_end = unsafe { root.as_ref().subtree_max_end };
            if MAX_CANONICAL_ADDR >= final_end {
                let remaining = MAX_CANONICAL_ADDR - final_end;
                if remaining >= size {
                    let aligned = (final_end + align - 1) & !(align - 1);
                    if aligned >= final_end && aligned + size <= MAX_CANONICAL_ADDR {
                        let candidate = if hint >= final_end {
                            let hint_aligned = (hint + align - 1) & !(align - 1);
                            if hint_aligned >= final_end && hint_aligned + size <= MAX_CANONICAL_ADDR {
                                hint_aligned
                            } else {
                                aligned
                            }
                        } else {
                            aligned
                        };
                        
                        if best_addr.is_none() || candidate < best_addr.unwrap() {
                            best_addr = Some(candidate);
                        }
                    }
                }
            }
        }

        if let Some(addr) = best_addr {
            self.cached_hint = addr + size;
            Some(addr)
        } else {
            None
        }
    }
}

pub struct Vmm {
    tree: Nutex<VmaTree>,
}

impl Vmm {
    pub const fn new() -> Self {
        Self {
            tree: Nutex::new(VmaTree::new()),
        }
    }

    pub fn alloc(&self, size: usize, align: usize, flags: VmaFlags, hint: usize) -> Option<usize> {
        let mut guard = self.tree.lock();
        let tree = &mut *guard;

        let target_hint = if flags.contains(VmaFlags::FIXED) {
            hint
        } else if hint != 0 {
            hint
        } else {
            tree.cached_hint
        };

        let start = tree.get_unmapped_area(size, align, target_hint)?;
        let end = start + size;

        if tree.find_overlap(start).is_some() || tree.find_overlap(end - 1).is_some() {
            return None;
        }

        let layout = Layout::new::<VmaNode>();
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return None;
        }

        let node = NonNull::new(ptr as *mut VmaNode).unwrap();
        unsafe {
            node.as_ptr().write(VmaNode::new(start, end, flags));
        }

        tree.insert(node);
        Some(start)
    }

    pub fn free(&self, start: usize) {
        let mut guard = self.tree.lock();
        let tree = &mut *guard;

        let mut curr = tree.root;
        let mut target_node: Option<NonNull<VmaNode>> = None;
        
        while let Some(node) = curr {
            let n = unsafe { node.as_ref() };
            if n.start == start {
                target_node = Some(node);
                break;
            } else if start < n.start {
                curr = n.left;
            } else {
                curr = n.right;
            }
        }

        if let Some(node) = target_node {
            tree.remove(node);
            let layout = Layout::new::<VmaNode>();
            unsafe {
                dealloc(node.as_ptr() as *mut u8, layout);
            }
        }
    }

    pub fn find_overlap(&self, addr: usize) -> Option<&VmaNode> {
        let guard = self.tree.lock();
        guard.find_overlap(addr).map(|ptr| unsafe { ptr.as_ref() })
    }
}
