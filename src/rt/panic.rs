use core::fmt::Write as _;

static mut PANIC_BUF: heapless::String<64> = heapless::String::<64>::new();

#[panic_handler]
#[allow(static_mut_refs)]
fn panic(info: &core::panic::PanicInfo) -> !
{
    let loc = info.location().unwrap().clone();
    let line = loc.line();
    let _ = unsafe { PANIC_BUF.write_str(loc.file()) };
    let file = unsafe { PANIC_BUF.as_str() };
    crate::kmsg::log
    (
        crate::kmsg::AttLvl::Panic,
        "",
        file,
        line,
        format_args!("{}", info.message()),
    );

    crate::arch::exit();
}
