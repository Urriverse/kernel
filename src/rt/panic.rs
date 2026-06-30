use core::fmt::Write as _;

use crate::{kmsg, sync::Nutex};

static mut PANIC_BUF: heapless::String<64> = heapless::String::<64>::new();

#[inline(never)]
pub fn print_stack_trace(bp: usize) {
    let mut frame_ptr = bp;
    let mut count = 0;

    unsafe { kmsg::str_log_noblock(
        kmsg::KeAttLvl::Error,
        "",
        file!(),
        line!(),
        "Stack trace:"
    ) };

    unsafe { kmsg::str_log_noblock(
        kmsg::KeAttLvl::Error,
        "",
        file!(),
        line!(),
        "$$ST:BEGIN$$"
    ) };

    while frame_ptr != 0 && count < 32 {
        let ret_addr = unsafe { *(frame_ptr as *const usize).add(1) };

        let mut msg = heapless::String::<32>::new();
        let _ = msg.write_fmt(format_args!("  #{:02} 0x{:016X}", count, ret_addr));
        unsafe { kmsg::str_log_noblock(
            kmsg::KeAttLvl::Error,
            "",
            file!(),
            line!(),
            msg.as_str()
        ) };

        frame_ptr = unsafe { *(frame_ptr as *const usize) };
        count += 1;
    }

    unsafe { kmsg::str_log_noblock(
        kmsg::KeAttLvl::Error,
        "",
        file!(),
        line!(),
        "$$ST:END$$"
    ) };
}

static PANIC_LOCK: Nutex<()> = Nutex::new(());

#[panic_handler]
#[allow(static_mut_refs)]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    let _g1 = PANIC_LOCK.lock();
    let _g2 = kmsg::SINKS.lock();
    let loc = *info.location().unwrap();
    let line = loc.line();
    let _ = unsafe { PANIC_BUF.write_str(loc.file()) };
    let file = unsafe { PANIC_BUF.as_str() };

    let mut s = heapless::String::<256>::new();

    let _ = s.write_fmt(format_args!("{}", info.message()));

    unsafe { kmsg::str_log_noblock(
        kmsg::KeAttLvl::Panic,
        "",
        file,
        line,
        &s,
    ) };

    let mut bp: usize;
    unsafe {
        core::arch::asm!("mov {}, rbp", out(reg) bp);
    }

    print_stack_trace(bp);

    drop(_g1);
    drop(_g2);

    crate::sched::exit(-1);
}
