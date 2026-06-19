#[macro_export]
macro_rules! trace
{
    (str $s:expr) =>
    {{
        #[cfg(not(feature = "lowlog"))]
        crate::kmsg::str_log(crate::kmsg::AttLvl::Trace, module_path!(), file!(), line!(), $s);
    }};
    ($($arg:tt)+) =>
    {{
        #[cfg(not(feature = "lowlog"))]
        crate::kmsg::log(crate::kmsg::AttLvl::Trace, module_path!(), file!(), line!(), format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! debug
{
    (str $s:expr) =>
    {{
        #[cfg(not(feature = "lowlog"))]
        crate::kmsg::str_log(crate::kmsg::AttLvl::Debug, module_path!(), file!(), line!(), $s);
    }};
    ($($arg:tt)+) =>
    {{
        #[cfg(not(feature = "lowlog"))]
        crate::kmsg::log(crate::kmsg::AttLvl::Debug, module_path!(), file!(), line!(), format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! info
{
    (str $s:expr) =>
    {{
        crate::kmsg::str_log(crate::kmsg::AttLvl::Info, module_path!(), file!(), line!(), $s);
    }};
    ($($arg:tt)+) =>
    {{
        crate::kmsg::log(crate::kmsg::AttLvl::Info, module_path!(), file!(), line!(), format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! warn
{
    (str $s:expr) =>
    {{
        crate::kmsg::str_log(crate::kmsg::AttLvl::Warn, module_path!(), file!(), line!(), $s);
    }};
    ($($arg:tt)+) =>
    {{
        crate::kmsg::log(crate::kmsg::AttLvl::Warn, module_path!(), file!(), line!(), format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! error
{
    (str $s:expr) =>
    {{
        crate::kmsg::str_log(crate::kmsg::AttLvl::Error, module_path!(), file!(), line!(), $s);
    }};
    ($($arg:tt)+) =>
    {{
        crate::kmsg::log(crate::kmsg::AttLvl::Error, module_path!(), file!(), line!(), format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! __panic_msg
{
    (str $s:expr) =>
    {{
        crate::kmsg::str_log(crate::kmsg::AttLvl::Panic, module_path!(), file!(), line!(), $s);
    }};
    ($($arg:tt)+) =>
    {{
        crate::kmsg::log(crate::kmsg::AttLvl::Panic, module_path!(), file!(), line!(), format_args!($($arg)+));
    }};
}

#[macro_export]
macro_rules! entry
{
    (for BSP { $($b:tt)* } $(;)? for AP { $($a:tt)* } $(;)?) =>
    {
        fueue!{__LAST}
        unsafe extern "C" fn ap_main(_: &limine::mp::MpInfo) -> !
        {
            $($a)*
            __LAST.wait();
            loop { core::hint::spin_loop(); }
        }
        pub fn main() -> !
        {
            info!("Kernel v{} started.", env!("CARGO_PKG_VERSION"));
            $($b)*
            __LAST.open();
            loop { core::hint::spin_loop(); }
        }
    }
}

#[macro_export]
macro_rules! start_aps {
    () => {
        for ap in SMP.cpus() { ap.bootstrap(ap_main, 0) }
    };
}

#[macro_export]
macro_rules! limine {
    ($vis:vis $name:ident <= $type:ident $(:)? $($arg:expr),*) => {
        #[unsafe(link_section = ".requests")]
        $vis static $name: limine::request::$type = limine::request::$type::new($($arg),*);
    };
}

#[macro_export]
macro_rules! fueue {
    ( $($vis:vis $name:ident)+ ) => {
        $( #[allow(unused)] $vis static $name: crate::sync::Fueue = crate::sync::Fueue::new(); )+
    }
}

#[macro_export]
macro_rules! hang { () => { loop { core::hint::spin_loop() } } }
