//! The kernel’s entry point (`_start`).
//!
//! This function is called by the bootloader (Limine). It initialises the
//! logging subsystem and then calls the kernel’s `main()`.

/// The first function called after the bootloader transfers control.
///
/// # Safety
/// This function is marked `extern "C"` and `no_mangle` to match the
/// bootloader’s expectation. It never returns.
#[unsafe(no_mangle)]
extern "C" fn _start() -> !
{
    crate::kmsg::init();
    let _ = crate::main();
    panic!("Kernel reached end of execution.");
}
