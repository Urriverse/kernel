//! Logging and entry point macros.
//!
//! This module provides macros for kernel logging at various levels
//! (`trace!`, `debug!`, `info!`, `warn!`, `error!`, `__panic_msg!`) and a macro
//! to define the kernel entry point (`entry!`).
//!
//! The logging macros automatically include file name, line number, and a
//! formatted message. They are conditional on the `lowlog` feature (when
//! `lowlog` is enabled, `trace!` and `debug!` are disabled).

/// Emit a trace‑level log message.
///
/// Two forms are supported:
/// - `trace!(str "literal")` – for a static string without formatting.
/// - `trace!("format {}", arg)` – for a formatted message.
///
/// This macro is disabled when the `lowlog` feature is enabled.
#[macro_export]
macro_rules! trace
{
    (str $s:expr) =>
    {{
        #[cfg(not(feature = "lowlog"))]
        crate::kmsg::str_log(crate::kmsg::AttLvl::Trace, file!(), line!(), $s);
    }};
    ($($arg:tt)+) =>
    {{
        #[cfg(not(feature = "lowlog"))]
        crate::kmsg::log(crate::kmsg::AttLvl::Trace, file!(), line!(), format_args!($($arg)+));
    }};
}

/// Emit a debug‑level log message.
///
/// Two forms are supported:
/// - `debug!(str "literal")` – for a static string without formatting.
/// - `debug!("format {}", arg)` – for a formatted message.
///
/// This macro is disabled when the `lowlog` feature is enabled.
#[macro_export]
macro_rules! debug
{
    (str $s:expr) =>
    {{
        #[cfg(not(feature = "lowlog"))]
        crate::kmsg::str_log(crate::kmsg::AttLvl::Debug, file!(), line!(), $s);
    }};
    ($($arg:tt)+) =>
    {{
        #[cfg(not(feature = "lowlog"))]
        crate::kmsg::log(crate::kmsg::AttLvl::Debug, file!(), line!(), format_args!($($arg)+));
    }};
}

/// Emit an info‑level log message.
///
/// Two forms are supported:
/// - `info!(str "literal")` – for a static string without formatting.
/// - `info!("format {}", arg)` – for a formatted message.
///
/// This macro is always enabled (no feature gate).
#[macro_export]
macro_rules! info
{
    (str $s:expr) =>
    {{
        crate::kmsg::str_log(crate::kmsg::AttLvl::Info, file!(), line!(), $s);
    }};
    ($($arg:tt)+) =>
    {{
        crate::kmsg::log(crate::kmsg::AttLvl::Info, file!(), line!(), format_args!($($arg)+));
    }};
}

/// Emit a warning‑level log message.
///
/// Two forms are supported:
/// - `warn!(str "literal")` – for a static string without formatting.
/// - `warn!("format {}", arg)` – for a formatted message.
///
/// This macro is always enabled.
#[macro_export]
macro_rules! warn
{
    (str $s:expr) =>
    {{
        crate::kmsg::str_log(crate::kmsg::AttLvl::Warn, file!(), line!(), $s);
    }};
    ($($arg:tt)+) =>
    {{
        crate::kmsg::log(crate::kmsg::AttLvl::Warn, file!(), line!(), format_args!($($arg)+));
    }};
}

/// Emit an error‑level log message.
///
/// Two forms are supported:
/// - `error!(str "literal")` – for a static string without formatting.
/// - `error!("format {}", arg)` – for a formatted message.
///
/// This macro is always enabled.
#[macro_export]
macro_rules! error
{
    (str $s:expr) =>
    {{
        crate::kmsg::str_log(crate::kmsg::AttLvl::Error, file!(), line!(), $s);
    }};
    ($($arg:tt)+) =>
    {{
        crate::kmsg::log(crate::kmsg::AttLvl::Error, file!(), line!(), format_args!($($arg)+));
    }};
}

/// Internal macro used by the panic handler to log panic messages.
///
/// Not intended for direct use.
#[macro_export]
macro_rules! __panic_msg
{
    (str $s:expr) =>
    {{
        crate::kmsg::str_log(crate::kmsg::AttLvl::Panic, file!(), line!(), $s);
    }};
    ($($arg:tt)+) =>
    {{
        crate::kmsg::log(crate::kmsg::AttLvl::Panic, file!(), line!(), format_args!($($arg)+));
    }};
}

/// Define the kernel entry point.
///
/// This macro expands to a `pub fn main()` containing the provided body.
/// It is used in `main.rs` to wrap the kernel's main logic.
///
/// # Example
/// ```ignore
/// entry! {
///     info!("Kernel started");
///     // ...
/// }
/// ```
#[macro_export]
macro_rules! entry
{
    ($($body:tt)*) =>
    {
        pub fn main()
        {
            $($body)*
        }
    };
}
