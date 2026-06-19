use core::alloc::Layout;
use core::ptr::NonNull;
use core::debug_assert;
use crate::mem::{upa, kdm::Vaddr};
use crate::sync::Nutex;

const PAGE_SIZE: usize = 4096;
const CLASS_SIZES: [usize; 9] = [8, 16, 32, 64, 128, 256, 512, 1024, 2048];

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlabState {
    Partial = 0,
    Full = 1,
}

#[repr(C)]
struct SlabHeader {
    free_count: usize,
    free_head: Option<NonNull<u8>>,
    next: Option<NonNull<SlabHeader>>,
    prev: Option<NonNull<SlabHeader>>,
    state: SlabState,
}

struct SlabClassInner {
    partial_slabs: Option<NonNull<SlabHeader>>,
    full_slabs: Option<NonNull<SlabHeader>>,
}

struct SlabClass {
    size: usize,
    obj_per_slab: usize,
    first_obj_offset: usize,
    inner: Nutex<SlabClassInner>,
}

impl SlabClass {
    const fn new(size: usize) -> Self {
        let header_size = core::mem::size_of::<SlabHeader>();
        let first_obj_offset = (header_size + 7) & !7;
        
        let obj_per_slab = (PAGE_SIZE - first_obj_offset) / size;
        
        Self {
            size,
            obj_per_slab,
            first_obj_offset,
            inner: Nutex::new(SlabClassInner {
                partial_slabs: None,
                full_slabs: None,
            }),
        }
    }

    fn alloc(&self) -> Option<NonNull<u8>> {
        let mut inner = self.inner.lock();

        if let Some(header_ptr) = inner.partial_slabs {
            let header = unsafe { &mut *header_ptr.as_ptr() };
            debug_assert!(header.state == SlabState::Partial);
            debug_assert!(header.free_count > 0);
            
            let ptr = header.free_head.unwrap();
            header.free_head = unsafe { *ptr.cast::<Option<NonNull<u8>>>().as_ptr() };
            header.free_count -= 1;

            if header.free_count == 0 {
                self.remove_from_list(&mut inner.partial_slabs, header_ptr);
                header.state = SlabState::Full;
                self.add_to_list(&mut inner.full_slabs, header_ptr);
            }
            return Some(ptr);
        }

        let paddr = upa::alloc(1);
        if paddr.to_raw() == 0 {
            return None;
        }

        let vaddr = paddr.to_virt().to_raw() as *mut u8;
        let header_ptr = NonNull::new(vaddr as *mut SlabHeader).unwrap();

        unsafe {
            let header = &mut *header_ptr.as_ptr();
            header.next = None;
            header.prev = None;
            header.state = SlabState::Partial;

            let mut current = vaddr.add(self.first_obj_offset);
            let end = vaddr.add(PAGE_SIZE);
            let mut prev_next: Option<NonNull<u8>> = None;

            while current.add(self.size) <= end {
                let node = NonNull::new(current).unwrap();
                node.cast::<Option<NonNull<u8>>>().as_ptr().write(prev_next);
                prev_next = Some(node);
                current = current.add(self.size);
            }

            header.free_head = prev_next;
            header.free_count = self.obj_per_slab;
        }

        self.add_to_list(&mut inner.partial_slabs, header_ptr);

        let header = unsafe { &mut *header_ptr.as_ptr() };
        debug_assert!(header.free_count > 0);
        
        let ptr = header.free_head.unwrap();
        header.free_head = unsafe { *ptr.cast::<Option<NonNull<u8>>>().as_ptr() };
        header.free_count -= 1;

        if header.free_count == 0 {
            self.remove_from_list(&mut inner.partial_slabs, header_ptr);
            header.state = SlabState::Full;
            self.add_to_list(&mut inner.full_slabs, header_ptr);
        }

        Some(ptr)
    }

    fn free(&self, ptr: NonNull<u8>) {
        let mut inner = self.inner.lock();
        
        let slab_base = (ptr.as_ptr() as usize) & !(PAGE_SIZE - 1);
        let header_ptr = NonNull::new(slab_base as *mut SlabHeader).unwrap();

        unsafe {
            let header = &mut *header_ptr.as_ptr();
            
            ptr.cast::<Option<NonNull<u8>>>().as_ptr().write(header.free_head);
            header.free_head = Some(ptr);
            header.free_count += 1;

            if header.free_count == self.obj_per_slab {
                if header.state == SlabState::Partial {
                    self.remove_from_list(&mut inner.partial_slabs, header_ptr);
                } else {
                    self.remove_from_list(&mut inner.full_slabs, header_ptr);
                }
                
                let paddr = Vaddr::from_raw(slab_base).to_phys();
                upa::free(paddr);
            } else if header.free_count == 1 {
                debug_assert!(header.state == SlabState::Full);
                self.remove_from_list(&mut inner.full_slabs, header_ptr);
                header.state = SlabState::Partial;
                self.add_to_list(&mut inner.partial_slabs, header_ptr);
            }
        }
    }

    fn add_to_list(&self, list: &mut Option<NonNull<SlabHeader>>, node: NonNull<SlabHeader>) {
        unsafe {
            let n = &mut *node.as_ptr();
            n.prev = None;
            n.next = *list;
            if let Some(mut head) = *list {
                head.as_mut().prev = Some(node);
            }
            *list = Some(node);
        }
    }

    fn remove_from_list(&self, list: &mut Option<NonNull<SlabHeader>>, node: NonNull<SlabHeader>) {
        unsafe {
            let n = &mut *node.as_ptr();
            
            let is_in_list = if *list == Some(node) {
                true
            } else if let Some(prev) = n.prev {
                prev.as_ref().next == Some(node)
            } else {
                false
            };

            if !is_in_list {
                return;
            }

            if let Some(mut prev) = n.prev {
                prev.as_mut().next = n.next;
            } else {
                *list = n.next;
            }
            if let Some(mut next) = n.next {
                next.as_mut().prev = n.prev;
            }
            
            n.prev = None;
            n.next = None;
        }
    }
}

pub struct Soa {
    classes: [SlabClass; CLASS_SIZES.len()],
}

impl Soa {
    pub const fn new() -> Self {
        Self {
            classes: [
                SlabClass::new(CLASS_SIZES[0]),
                SlabClass::new(CLASS_SIZES[1]),
                SlabClass::new(CLASS_SIZES[2]),
                SlabClass::new(CLASS_SIZES[3]),
                SlabClass::new(CLASS_SIZES[4]),
                SlabClass::new(CLASS_SIZES[5]),
                SlabClass::new(CLASS_SIZES[6]),
                SlabClass::new(CLASS_SIZES[7]),
                SlabClass::new(CLASS_SIZES[8]),
            ],
        }
    }

    fn find_class(&self, layout: Layout) -> Option<usize> {
        if layout.align() > 8 {
            return None;
        }
        let mut size = layout.size();
        if size == 0 {
            size = 1;
        }
        if size > 2048 {
            return None;
        }
        CLASS_SIZES.iter().position(|&s| s >= size)
    }
}

static mut SOA_INSTANCE: Soa = Soa::new();

pub fn init() {
    info!("SOA: Initialized with immediate shrink ({} classes)", CLASS_SIZES.len());
}

#[allow(static_mut_refs)]
pub fn alloc(layout: Layout) -> *mut u8 {
    let soa = unsafe { &SOA_INSTANCE };

    if let Some(class_idx) = soa.find_class(layout) {
        if let Some(ptr) = soa.classes[class_idx].alloc() {
            return ptr.as_ptr();
        }
    }

    let pages = (layout.size() + 4095) / 4096;
    let paddr = upa::alloc(pages);
    if paddr.to_raw() == 0 {
        return core::ptr::null_mut();
    }
    paddr.to_virt().to_raw() as *mut u8
}

#[allow(static_mut_refs)]
pub fn free(ptr: *mut u8, layout: Layout) {
    if ptr.is_null() {
        return;
    }

    let soa = unsafe { &SOA_INSTANCE };

    if layout.align() > 8 || layout.size() > 2048 {
        let vaddr = Vaddr::from_raw(ptr as usize);
        upa::free(vaddr.to_phys());
    } else {
        if let Some(class_idx) = soa.find_class(layout) {
            let ptr_nn = NonNull::new(ptr).unwrap();
            soa.classes[class_idx].free(ptr_nn);
        } else {
            let vaddr = Vaddr::from_raw(ptr as usize);
            upa::free(vaddr.to_phys());
        }
    }
}

#[allow(static_mut_refs)]
pub fn dump_stats() {
    let soa = unsafe { &SOA_INSTANCE };
    info!("--- SOA Statistics ---");
    for (i, class) in soa.classes.iter().enumerate() {
        let inner = class.inner.lock();
        
        let mut partial_count = 0;
        let mut curr = inner.partial_slabs;
        while let Some(node) = curr {
            partial_count += 1;
            curr = unsafe { node.as_ref().next };
        }
        
        let mut full_count = 0;
        curr = inner.full_slabs;
        while let Some(node) = curr {
            full_count += 1;
            curr = unsafe { node.as_ref().next };
        }

        info!(
            "Class {} ({} bytes): {} partial, {} full (obj/slab: {})",
            i, class.size, partial_count, full_count, class.obj_per_slab
        );
    }
    info!("----------------------");
}
