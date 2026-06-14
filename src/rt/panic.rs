//! Kernel panic handler.
//!
//! This module provides the `#[panic_handler]` function that logs the panic
//! message and then enters an infinite loop.

use core::fmt::Write as _;

/// Buffer used to store the file name (because `PanicInfo::location().file()`
/// returns a `&str` that may not live long enough). We copy it into this static
/// buffer.
static mut PANIC_BUF: heapless::String<64> = heapless::String::<64>::new();

/// The panic handler.
///
/// It logs the panic message using the kernel logging system, then spins forever.
#[panic_handler]
#[allow(static_mut_refs)]
fn panic(info: &core::panic::PanicInfo) -> !
{
    let loc = info.location().unwrap().clone();
    let line = loc.line();
    unsafe { PANIC_BUF.write_str(loc.file()) };
    let file = unsafe { PANIC_BUF.as_str() };
    crate::kmsg::log
    (
        crate::kmsg::AttLvl::Panic,
        file,
        line,
        format_args!("{}", info.message()),
    );

    #[cfg(feature = "devlog")]
    unsafe
    {
        x86::io::outb(0x501, 1);
    }

    loop { core::hint::spin_loop() }
}
