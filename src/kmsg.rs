pub use ketypes::mon::{lvl::AttLvl, sink::{Sink, Format}};
use core::fmt::Write;

use crate::sync::Litex;
use heapless::{Vec, String};

pub mod dev;

const MAX_MSG_LEN: usize = 1024;

type Msg = String<MAX_MSG_LEN>;

#[cfg(    feature = "lowlog" )] macro_rules! FMT {() => {"~ {:16.2} CPU #{} : {:>32} : {}: {}"}}
#[cfg(not(feature = "lowlog"))] macro_rules! FMT {() => {"~ {:16.2} CPU #{} : {:>20}:{:<3} : {}: {}"}}

// ============================================================================
// GLOBAL SINK REGISTRY
// ============================================================================

/// Global registry of log sinks.
///
/// This is a `Litex<Vec<&'static dyn Sink, 256>>` – a spinlock that disables
/// interrupts during the critical section. Up to 256 sinks can be registered.
pub static SINKS: Litex<Vec<&'static mut dyn Sink, 256>> = Litex::new(Vec::new());

/// Adds a sink to the global registry.
///
/// The sink must be `'static` (i.e., either a `static` variable or a leaked
/// reference). This function acquires the lock and pushes the sink.
pub fn add(sink: &'static mut dyn Sink) {
    let _ = SINKS.lock().push(sink);
}

/// Initializes the logging system.
///
/// If the `devlog` feature is enabled, this registers the serial sink
/// (`kmsg::dev::SINK`). This function is called very early in the boot process,
/// before any other subsystems.
pub fn init() {
    #[cfg(feature = "devlog")] add(dev::SINK);
}

// ============================================================================
// LOGGING FUNCTIONS
// ============================================================================

/// Internal logging function with formatting support.
///
/// This function constructs a formatted message, then iterates over all
/// registered sinks and writes the message to each one.
///
/// # Parameters
/// - `al`: log level
/// - `modpath`: module path (from `module_path!()`)
/// - `file`: source file name (from `file!()`)
/// - `line`: line number (from `line!()`)
/// - `fa`: format arguments (from `format_args!()`)
///
/// # Notes
/// - When `lowlog` is enabled, `file` and `line` are ignored to reduce the
///   message size.
/// - The sink's `Pretty` attribute determines whether ANSI colours are used.
/// - This function acquires the `SINKS` lock, so it must not be called from
///   interrupt handlers or panic contexts (use `str_log_noblock` for that).
#[allow(unused)]
pub fn log(al: AttLvl, modpath: &'static str, file: &'static str, line: u32, fa: core::fmt::Arguments<'_>) {
    #[cfg(feature = "lowlog")] let _ = file;
    #[cfg(feature = "lowlog")] let _ = line;

    // Build the message content (without prefix)
    const INLEN: usize = MAX_MSG_LEN >> 1;
    let mut c = String::<INLEN>::new();
    let _ = c.write_fmt(fa);

    // Lock the sink registry
    let mut g = SINKS.lock();

    for mut sink in &mut *g {
        let mut m = Msg::new();

        // Choose level representation based on sink's Pretty flag
        let lvl = match sink.format() {
            Format::Pretty => al.pretty(),
            Format::Regular => al.as_str(),
        };

        // Format the full message
        #[cfg(    feature = "lowlog" )]
        let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), modpath, lvl, c.as_str()));
        #[cfg(not(feature = "lowlog"))]
        let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), file, line, lvl, c.as_str()));

        sink.write_str(m.as_str());
    }
}

/// Internal logging function for literal strings (no formatting).
///
/// This is a faster path for `...!(str "...")` macro invocations.
/// It avoids the `format_args!` machinery and builds the message directly.
#[allow(unused)]
pub fn str_log(al: AttLvl, modpath: &'static str, file: &'static str, line: u32, s: &str) {
    #[cfg(feature = "lowlog")] let _ = file;
    #[cfg(feature = "lowlog")] let _ = line;

    let mut g = SINKS.lock();

    for mut sink in &mut *g {
        let mut m = Msg::new();

        let lvl = match sink.format() {
            Format::Pretty => al.pretty(),
            Format::Regular => al.as_str(),
        };

        #[cfg(    feature = "lowlog" )]
        let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), modpath, lvl, s));
        #[cfg(not(feature = "lowlog"))]
        let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), file, line, lvl, s));

        sink.write_str(m.as_str());
    }
}

/// Unsafe, lock‑free version of `str_log` for use in panic handlers.
///
/// This function accesses the `SINKS` registry without acquiring the lock.
/// It is only safe to call when the system is single‑threaded (e.g., during
/// panic, where interrupts are disabled and no other CPUs are running
/// the logging code).
///
/// # Safety
/// - Must not be called concurrently with any other logging function.
/// - Must not be called after the system has started multi‑core scheduling
///   (unless interrupts are disabled and no other CPU can access the registry).
#[allow(unused)]
pub unsafe fn str_log_noblock(al: AttLvl, modpath: &'static str, file: &'static str, line: u32, s: &str) {
    #[cfg(feature = "lowlog")] let _ = file;
    #[cfg(feature = "lowlog")] let _ = line;

    let mut g = unsafe { SINKS.inner() };

    for mut sink in &mut *g {
        let mut m = Msg::new();

        let lvl = match sink.format() {
            Format::Pretty => al.pretty(),
            Format::Regular => al.as_str(),
        };

        #[cfg(    feature = "lowlog" )]
        let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), modpath, lvl, s));
        #[cfg(not(feature = "lowlog"))]
        let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), file, line, lvl, s));

        sink.write_str(m.as_str());
    }
}
