extrum! {
    #[derive(Clone, Copy, PartialEq, Default)]
    pub enum Kind: u64
    {
        USABLE      = 0,
        RESERVED    = 1,
        ACPI        = 2,
        ACPI_NVS    = 3,
        BAD         = 4,
        BOOTLOADER  = 5,
        KERNEL      = 6,
        FRAMEBUF    = 7,
        MAPRESERVED = 8,
    }
}

implement_display![Kind];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Region
{
    pub base: usize,
    pub len: usize,
    pub kind: Kind,
}

pub struct Iter
{
    next: usize,
}

limine! { pub MEMMAP <= MemmapRequest }

lazy_static!
{
    static ref MMAP: &'static [&'static limine::memmap::Entry] = MEMMAP.response().expect("Can't obtain memory regions info.").entries();
}

pub fn nth(n: usize) -> Option<Region> {
    let o = MMAP.get(n);
    match o {
        Some(e) => Some (
            Region {
                base: e.base as usize,
                len: e.length as usize,
                kind: Kind(e.type_),
            }
        ),
        None => None,
    }
}

pub fn nth_unchecked(n: usize) -> Region {
    let e = MMAP[n];
    Region {
        base: e.base as usize,
        len: e.length as usize,
        kind: Kind(e.type_),
    }
}

impl Iter
{
    pub(super) const fn new() -> Self { Self { next: 0 } }

    pub fn next(&mut self) -> Option<Region>
    {
        if self.next < MMAP.len()
        {
            let e = MMAP[self.next];
            self.next += 1;
            Some
            (
                Region
                {
                    base: e.base as usize,
                    len: e.length as usize,
                    kind: Kind(e.type_),
                }
            )
        }
        else
        {
            self.next = 0;
            None
        }
    }
}

impl Iterator for Iter
{
    type Item = Region;
    fn next(&mut self) -> Option<Self::Item>
    {
        Iter::next(self)
    }
}

pub fn iter() -> Iter
{
    Iter::new()
}

pub fn len() -> usize {
    MMAP.len()
}

pub fn dump()
{
    debug!("Memory regions:");
    for r in iter()
    {

        #[cfg(feature = "lowlog")] let _ = r;
        debug!("~ base {:-12X} of {:>12} KiB, {:<16}", r.base, (r.len + 1023) >> 10, r.kind);
    }
}
