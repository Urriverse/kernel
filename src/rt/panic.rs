use core::fmt::Write as _;

use crate::{kmsg, sync::Nutex};

static mut PANIC_BUF: heapless::String<64> = heapless::String::<64>::new();

#[inline(never)]
pub fn print_stack_trace(mut frame_ptr: usize) {
    let mut count = 1;

    unsafe { kmsg::str_log_noblock(
        kmsg::KeAttLvl::Error,
        "",
        file!(),
        line!(),
        "Stack trace:"
    ) };

    let mut msg = heapless::String::<32>::new();
    let _ = msg.write_fmt(format_args!("  RSP 0x{:016X}", frame_ptr));
    unsafe { kmsg::str_log_noblock(
        kmsg::KeAttLvl::Error,
        "",
        file!(),
        line!(),
        msg.as_str()
    ) };

    while frame_ptr > 0xffff800000000000 && frame_ptr < 0xffffffffffffffff && count < 32 {
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

    msg = heapless::String::<32>::new();
    let _ = msg.write_fmt(format_args!("  ..."));

    unsafe { kmsg::str_log_noblock(
        kmsg::KeAttLvl::Error,
        "",
        file!(),
        line!(),
        msg.as_str()
    ) };
}

static PANIC_LOCK: Nutex<()> = Nutex::new(());

Export! {
    #[panic_handler]
    #[allow(static_mut_refs)]
    pub fn panic as ExecPanic(info: &core::panic::PanicInfo) -> ! where kernel 0.1 {
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
}
