//! # Kernel Macros
//!
//! This module defines the kernel's core macros, including logging macros (`trace!`,
//! `debug!`, `info!`, `warn!`, `error!`, `__panic_msg!`), the entry point macro
//! (`entry!`), and various helper macros for synchronization, Limine requests,
//! and CPU halting.
//!
//! ## Logging Macros
//!
//! The logging macros provide a convenient interface to the KMSG subsystem. They
//! automatically include the module path, file name, and line number in the log
//! message. The `trace!` and `debug!` macros are conditionally compiled; they are
//! disabled when the `lowlog` feature is enabled (used in release builds).
//!
//! Each macro has two forms:
//! - `info!(str "literal")` – for logging a string literal (faster).
//! - `info!("formatted {}", arg)` – for formatted output.
//!
//! ## Entry Macro (`entry!`)
//!
//! The `entry!` macro defines the kernel's entry points for both the BSP and APs.
//! It generates:
//! - An `ap_main` function for Application Processors.
//! - A `main` function for the Bootstrap Processor.
//!
//! The macro synchronises the APs with the BSP using a `Fueue` barrier (`__LAST`),
//! ensuring that APs wait until the BSP has completed its initialization before
//! entering the scheduler.
//!
//! ## Other Macros
//!
//! - **`start_aps!`**: Bootstraps all APs using the Limine SMP request.
//! - **`limine!`**: Declares a static Limine request.
//! - **`fueue!`**: Declares multiple static `Fueue` barriers.
//! - **`hang!`**: Enters an infinite loop (used for unrecoverable errors).

// ============================================================================
// LOGGING MACROS
// ============================================================================

/// Logs a message at the **Trace** level.
///
/// This macro is disabled when the `lowlog` feature is enabled (release builds).
/// It automatically includes the module path, file, and line number.
///
/// # Examples
/// ```ignore
/// trace!(str "Entering function");
/// trace!("Value of x: {}", x);
/// ```
#[macro_export]
macro_rules! trace {
    (str $s:expr) => {{
        #[cfg(not(feature = "lowlog"))]
        $crate::kmsg::str_log($crate::kmsg::AttLvl::Trace, concat!("kernel::", concat!("kernel::", module_path!())), file!(), line!(), $s);
    }};
    ($($arg:tt)+) => {{
        #[cfg(not(feature = "lowlog"))]
        $crate::kmsg::log($crate::kmsg::AttLvl::Trace, concat!("kernel::", module_path!()), file!(), line!(), format_args!($($arg)+));
    }};
}

/// Logs a message at the **Debug** level.
///
/// This macro is disabled when the `lowlog` feature is enabled (release builds).
/// It automatically includes the module path, file, and line number.
///
/// # Examples
/// ```ignore
/// debug!(str "Initializing device");
/// debug!("Allocated {} bytes", size);
/// ```
#[macro_export]
macro_rules! debug {
    (str $s:expr) => {{
        #[cfg(not(feature = "lowlog"))]
        $crate::kmsg::str_log($crate::kmsg::AttLvl::Debug, concat!("kernel::", module_path!()), file!(), line!(), $s);
    }};
    ($($arg:tt)+) => {{
        #[cfg(not(feature = "lowlog"))]
        $crate::kmsg::log($crate::kmsg::AttLvl::Debug, concat!("kernel::", module_path!()), file!(), line!(), format_args!($($arg)+));
    }};
}

/// Logs a message at the **Info** level.
///
/// This macro is always enabled (even in release builds). It is used for
/// general informational messages about the system's progress.
///
/// # Examples
/// ```ignore
/// info!(str "Kernel started");
/// info!("CPU #{} online", cpu_id);
/// ```
#[macro_export]
macro_rules! info {
    (str $s:expr) => {{
        $crate::kmsg::str_log($crate::kmsg::AttLvl::Info, concat!("kernel::", module_path!()), file!(), line!(), $s);
    }};
    ($($arg:tt)+) => {{
        $crate::kmsg::log($crate::kmsg::AttLvl::Info, concat!("kernel::", module_path!()), file!(), line!(), format_args!($($arg)+));
    }};
}

/// Logs a message at the **Warning** level.
///
/// This macro is always enabled. It is used for non‑fatal conditions that
/// may indicate a problem.
///
/// # Examples
/// ```ignore
/// warn!(str "Memory allocation failed, retrying");
/// warn!("Unsupported operation on device {}", dev_id);
/// ```
#[macro_export]
macro_rules! warn {
    (str $s:expr) => {{
        $crate::kmsg::str_log($crate::kmsg::AttLvl::Warn, concat!("kernel::", module_path!()), file!(), line!(), $s);
    }};
    ($($arg:tt)+) => {{
        $crate::kmsg::log($crate::kmsg::AttLvl::Warn, concat!("kernel::", module_path!()), file!(), line!(), format_args!($($arg)+));
    }};
}

/// Logs a message at the **Error** level.
///
/// This macro is always enabled. It is used for recoverable errors that
/// should be reported but do not cause a panic.
///
/// # Examples
/// ```ignore
/// error!(str "Failed to read from disk");
/// error!("Could not allocate memory for task {}", task_id);
/// ```
#[macro_export]
macro_rules! error {
    (str $s:expr) => {{
        $crate::kmsg::str_log($crate::kmsg::AttLvl::Error, concat!("kernel::", module_path!()), file!(), line!(), $s);
    }};
    ($($arg:tt)+) => {{
        $crate::kmsg::log($crate::kmsg::AttLvl::Error, concat!("kernel::", module_path!()), file!(), line!(), format_args!($($arg)+));
    }};
}

/// Logs a message at the **Panic** level.
///
/// This is an internal macro used by the panic handler. It is always enabled
/// and logs the message before the system halts.
#[macro_export]
macro_rules! __panic_msg {
    (str $s:expr) => {{
        $crate::kmsg::str_log($crate::kmsg::AttLvl::Panic, concat!("kernel::", module_path!()), file!(), line!(), $s);
    }};
    ($($arg:tt)+) => {{
        $crate::kmsg::log($crate::kmsg::AttLvl::Panic, concat!("kernel::", module_path!()), file!(), line!(), format_args!($($arg)+));
    }};
}

// ============================================================================
// ENTRY MACRO
// ============================================================================

/// Defines the kernel's BSP and AP entry points.
///
/// The macro generates two functions:
/// - `ap_main`: Entry point for each Application Processor (AP).
/// - `main`: Entry point for the Bootstrap Processor (BSP).
///
/// The BSP and AP code blocks are provided as arguments. The macro also
/// creates a `Fueue` barrier (`__LAST`) to synchronise the APs, ensuring
/// they wait for the BSP to complete its initialization before entering
/// the scheduler.
///
/// # Syntax
/// ```ignore
/// entry! {
///     for BSP {
///         // BSP initialization code
///     }
///     for AP {
///         // AP initialization code
///     }
/// }
/// ```
#[macro_export]
macro_rules! entry
{
    (for BSP { $($b:tt)* } $(;)? for AP { $($a:tt)* } $(;)?) =>
    {
        // ============================================================================
        // AP BOOTSTRAP MACRO
        // ============================================================================

        /// Bootstraps all Application Processors (APs) using the Limine SMP request.
        ///
        /// This macro iterates over the CPUs in the Limine SMP response and calls
        /// `bootstrap` on each AP, passing the `ap_main` function as the entry point.
        ///
        /// It is intended to be used inside the BSP block of the `entry!` macro.
        #[macro_export]
        macro_rules! start_aps {
            () => {
                for ap in SMP.cpus() { ap.bootstrap(ap_main, 0) }
            };
        }

        limine! { pub SMPR <= MpRequest: 0 }

        lazy_static! {
            static ref SMP: &'static limine::request::Response<limine::request::MpRespData> = SMPR.response().expect("Can't obtain SMP info");
        }
        unsafe extern "C" fn ap_main(_: &limine::mp::MpInfo) -> !
        {
            unsafe { core::arch::asm! { "cli" } }
            $($a)*
        }
        pub fn main() -> !
        {
            unsafe { core::arch::asm! { "cli" } }
            info!("Kernel v{} started.", env!("CARGO_PKG_VERSION"));
            $($b)*
        }
    }
}

// ============================================================================
// LIMINE REQUEST MACRO
// ============================================================================

/// Declares a static Limine request.
///
/// This macro creates a static variable of the given Limine request type
/// and places it in the `.limine_requests` section using a `link_section`
/// attribute. The bootloader will fill the request with the appropriate
/// response data.
///
/// # Syntax
/// ```ignore
/// limine! { pub NAME <= RequestType: args }
/// ```
///
/// # Examples
/// ```ignore
/// limine! { pub MEMMAP <= MemmapRequest }
/// limine! { pub SMPR <= MpRequest: 0 }
/// ```
#[macro_export]
macro_rules! limine {
    ( $(#[$attr:meta])* $vis:vis $name:ident <= $type:ident $(:)? $($arg:expr),*) => {
        $(#[$attr])*
        #[unsafe(link_section = ".requests")]
        $vis static $name: limine::request::$type = limine::request::$type::new($($arg),*);
    };
}

// ============================================================================
// FUEUE BARRIER MACRO
// ============================================================================

/// Declares one or more static `Barrier` barriers.
///
/// This macro creates static `Barrier` instances with the given names, which
/// can be used to synchronise multiple CPUs during boot.
///
/// # Syntax
/// ```ignore
/// barrier! { BARRIER1 BARRIER2 BARRIER3 }
/// ```
///
/// # Examples
/// ```ignore
/// barrier! { ARCH_INIT MEM_INIT LATE_INIT DEV_INIT }
/// ```
#[macro_export]
macro_rules! barrier {
    ( $($vis:vis $name:ident)+ ) => {
        $( #[allow(unused)] $vis static $name: $crate::sync::Barrier = $crate::sync::Barrier::new(); )+
    }
}

#[macro_export]
macro_rules! no_interrupts {
    { $($b:tt)* } => {
        {
            let ___ig = $crate::sync::Nitex::<()>::new(());
            let _ = ___ig.lock();
            $($b)*
        }
    };
}

#[macro_export]
macro_rules! hash {
    ($s:expr) => {{
        const fn fnv1a64(data: &[u8]) -> u64 {
            const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
            const FNV_PRIME: u64 = 0x100000001b3;
            let mut hash = FNV_OFFSET_BASIS;
            let mut i = 0;
            while i < data.len() {
                hash ^= data[i] as u64;
                hash = hash.wrapping_mul(FNV_PRIME);
                i += 1;
            }
            hash
        }
        fnv1a64($s)
    }};
}
