use core::sync::atomic::compiler_fence;

use crate::{kmsg::{Sink, SinkAttrs, SinkIdent}, sync::Nutex};

// cargo check: false positive
#[allow(unused)]
pub struct Dev;

impl Dev
{
    // cargo check: false positive
    #[allow(unused)]
    pub fn new() -> Self
    {
        unsafe
        {
            compiler_fence(core::sync::atomic::Ordering::SeqCst);
            x86::io::outb(0x3f8 + 1, 0  );
            compiler_fence(core::sync::atomic::Ordering::SeqCst);
            x86::io::outb(0x3f8 + 3, 128);
            compiler_fence(core::sync::atomic::Ordering::SeqCst);
            x86::io::outb(0x3f8    , 1  );
            compiler_fence(core::sync::atomic::Ordering::SeqCst);
            x86::io::outb(0x3f8 + 1, 0  );
            compiler_fence(core::sync::atomic::Ordering::SeqCst);
            x86::io::outb(0x3f8 + 3, 3  );
            compiler_fence(core::sync::atomic::Ordering::SeqCst);
            x86::io::outb(0x3f8 + 2, 7  );
            compiler_fence(core::sync::atomic::Ordering::SeqCst);
            x86::io::outb(0x3f8 + 4, 3  );
            compiler_fence(core::sync::atomic::Ordering::SeqCst);
            let _ = x86::io::inb(0x3f8 + 5);
            compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        Self
    }
}

unsafe impl Sync for Dev {}

// cargo check: false positive
#[allow(unused)]
static ID: u32 = super::str4_to_u32("DEV0");
 pub static LOCK: Nutex<()> = Nutex::new(());

impl Sink for Dev
{
    fn kind(&self) -> SinkIdent
    {
        SinkIdent
        {
            attrs: SinkAttrs::Port | SinkAttrs::Critical | SinkAttrs::Pretty,
            kind: ID
        }
    }

    fn write(&self, s: &str)
    {
        let _g = LOCK.lock();

        for byte in s.bytes()
        {
            unsafe 
            {
                while (x86::io::inb(0x3f8 + 5) & 0x20) == 0
                {
                    core::arch::asm!("pause");
                }
                x86::io::outb(0x3f8, byte);
            }
        }
        unsafe 
        {
            while (x86::io::inb(0x3f8 + 5) & 0x20) == 0
            {
                core::arch::asm!("pause");
            }
            x86::io::outb(0x3f8, b'\n');
        }

        drop(_g);
    }
}

lazy_static! {
    static ref _SINK: Dev = Dev::new();
    pub static ref SINK: &'static Dev = &_SINK;
}
