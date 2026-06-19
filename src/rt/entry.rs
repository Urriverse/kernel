#[unsafe(no_mangle)]
extern "C" fn _start() -> !
{
    crate::kmsg::init(); crate::main()
}
