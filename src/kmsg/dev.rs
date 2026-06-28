use core::sync::atomic::compiler_fence;

use crate::{kmsg::Sink, sync::Nutex};

static LOCK: Nutex<()> = Nutex::new(());

// cargo check: false positive
#[allow(unused)]
pub struct Devel;

impl Devel
{
    // cargo check: false positive
    #[allow(unused)]
    pub fn init()
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
    }
}

impl core::fmt::Write for Devel {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
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

        Ok(())
    }
}

unsafe impl Sync for Devel {}
unsafe impl Send for Devel {}

impl Sink for Devel {
    fn format(&self) -> ketypes::mon::sink::Format {
        ketypes::mon::sink::Format::Pretty
    }
}

static mut _SINK: Devel = Devel;

lazy_static! {
    pub static ref SINK: &'static mut Devel = {
        Devel::init();
        #[allow(static_mut_refs)]
        unsafe { &mut _SINK }
    };
}
