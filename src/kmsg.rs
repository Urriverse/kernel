//! # Kernel Message Logging (KMSG)
//!
//! This module provides a flexible, multi‑sink logging system for the kernel.
//! It supports multiple log levels, configurable output sinks (e.g., serial port,
//! framebuffer, ring buffer), and conditional compilation to reduce overhead in
//! production builds.
//!
//! ## Overview
//!
//! The KMSG system is built around the following concepts:
//!
//! - **Log levels** (`AttLvl`): `Panic`, `Error`, `Warn`, `Info`, `Debug`, `Trace`.
//!   The `Debug` and `Trace` levels can be disabled at compile time via the
//!   `lowlog` feature to reduce code size and improve performance.
//!
//! - **Sinks** (`Sink`): Output destinations that implement the `Sink` trait.
//!   Each sink has a set of attributes (`SinkAttrs`) and a kind identifier.
//!   Sinks are registered globally and receive every log message.
//!
//! - **Global Sink Registry** (`SINKS`): A static `Litex<Vec<&'static dyn Sink>>`
//!   that holds all registered sinks. Logging functions iterate over this list
//!   and write to each sink in turn.
//!
//! - **Formatting**: Each log message includes a timestamp, CPU ID, source
//!   location (file/line or module), log level, and the message itself. The
//!   format can be customized based on the `lowlog` feature and sink attributes.
//!
//! ## Usage
//!
//! The KMSG system is used via the logging macros defined in `macros.rs`:
//!
//! - `trace!`, `debug!`, `info!`, `warn!`, `error!`, `__panic_msg!`
//!
//! These macros accept either a format string with arguments or a literal `str`.
//! They automatically include the module path, file, and line number.
//!
//! Example:
//! ```ignore
//! info!("Kernel started on CPU #{}", current_cpu());
//! debug!(str "Initializing device model...");
//! ```
//!
//! ## Sinks
//!
//! A sink is any type that implements the `Sink` trait. The trait requires:
//! - `write(&self, s: &str)`: output the formatted message.
//! - `kind(&self) -> SinkIdent`: return the sink's attributes and kind ID.
//!
//! The built‑in serial sink (`kmsg::dev::Dev`) writes to COM1 (0x3F8) and is
//! registered when the `devlog` feature is enabled (in development profiles).
//! It supports ANSI colour codes (`Pretty` attribute) for better readability.
//!
//! Additional sinks (e.g., framebuffer, network, file) can be added by
//! implementing the trait and calling `kmsg::add()` during early boot.
//!
//! ## Features
//!
//! - `lowlog`: Disables `Debug` and `Trace` messages entirely. Also uses a
//!   more compact log format (omits file and line numbers). Intended for
//!   production/release builds.
//! - `devlog`: Registers the serial sink at `kmsg::init()`. Used in
//!   development profiles.
//!
//! ## Safety
//!
//! - The global sink registry (`SINKS`) is protected by a `Litex` (an interrupt‑
//!   disabling spinlock) to ensure safe concurrent access from multiple CPUs.
//! - The `str_log_noblock` function is `unsafe` because it accesses the registry
//!   without locking; it is used only in panic handling where locking could
//!   cause deadlocks.
//!
//! ## Initialization
//!
//! `kmsg::init()` is called early in `_start()` before any other subsystem.
//! If the `devlog` feature is enabled, it registers the serial sink.
//! After that, all log messages are delivered to all registered sinks.

use core::fmt::Write;

use crate::sync::Litex;
use heapless::{Vec, String};

pub mod dev;

// ============================================================================
// CONSTANTS & TYPES
// ============================================================================

/// Maximum length of a log message (including formatting overhead).
const MAX_MSG_LEN: usize = 1024;

/// Internal type alias for a formatted log message.
type Msg = String<MAX_MSG_LEN>;

/// Converts a 4‑character literal into a `u32` identifier.
///
/// Used to create compact, human‑readable kind IDs for sinks (e.g., `"DEV0"`).
///
/// # Panics
/// Panics if the input string is not exactly 4 bytes long.
pub const fn str4_to_u32(s: &str) -> u32 {
    let b = s.as_bytes();
    if b.len() != 4 { panic!("expected 4-byte literal"); }
    ((b[0] as u32) << 24) |
    ((b[1] as u32) << 16) |
    ((b[2] as u32) << 8)  |
     (b[3] as u32)
}

// ============================================================================
// FORMAT STRINGS (conditional on `lowlog`)
// ============================================================================

/// Format string for log messages.
///
/// - With `lowlog`: `"~ {:16.2} CPU #{} : {:>32} : {}: {}"`
///   (time, CPU, module, level, message)
/// - Without `lowlog`: `"~ {:16.2} CPU #{} : {:>20}:{:<3} : {}: {}"`
///   (time, CPU, file, line, level, message)
#[cfg(    feature = "lowlog" )] macro_rules! FMT {() => {"~ {:16.2} CPU #{} : {:>32} : {}: {}"}}
#[cfg(not(feature = "lowlog"))] macro_rules! FMT {() => {"~ {:16.2} CPU #{} : {:>20}:{:<3} : {}: {}"}}

// ============================================================================
// SINK ATTRIBUTES
// ============================================================================

bitflags! {
    /// Attributes that describe the capabilities and behaviour of a log sink.
    ///
    /// These flags are used by the logging system to decide how to format
    /// messages for a particular sink.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SinkAttrs: u32 {
        /// The sink is a pure virtual interface (no actual output).
        const Virtual  = 1 << 0;
        /// The sink is a buffer (e.g., ring buffer in memory).
        const Buffer   = 1 << 1;
        /// The sink is a physical port (e.g., serial, VGA).
        const Port     = 1 << 2;
        /// The sink is critical for system monitoring; logging should always
        /// succeed (e.g., serial console).
        const Critical = 1 << 3;
        /// The sink is optional; failures can be ignored.
        const Weak     = 1 << 4;
        /// The sink supports ANSI colour codes (e.g., a TTY).
        const Pretty   = 1 << 5;
    }
}

/// Identifier for a log sink.
///
/// Combines attributes and a kind ID (from `str4_to_u32`).
#[allow(dead_code)]
pub struct SinkIdent {
    /// Attributes of the sink.
    pub attrs: SinkAttrs,
    /// A `u32` sink kind identifier (often created with [`str4_to_u32`]).
    pub kind: u32,
}

/// Trait for log sinks.
///
/// Any type that implements this trait can be registered as a log sink.
pub trait Sink: Sync {
    /// Write a string to the sink.
    ///
    /// The implementation should handle any necessary buffering, locking,
    /// or hardware interactions.
    fn write(&self, s: &str);

    /// Return the sink's identifier (attributes + kind).
    fn kind(&self) -> SinkIdent;
}

// ============================================================================
// LOG LEVELS
// ============================================================================

/// Log level severity.
///
/// Levels are ordered by severity, with `Panic` being the most critical and
/// `Trace` the least. The `Debug` and `Trace` levels are disabled when the
/// `lowlog` feature is enabled.
#[derive(Clone, Copy)]
pub enum AttLvl {
    /// Unrecoverable error – system will panic or halt.
    Panic,
    /// Recoverable error.
    Error,
    /// Warning – unexpected but non‑fatal.
    Warn,
    /// Informational message.
    Info,
    /// Debugging information (disabled by `lowlog`).
    Debug,
    /// Trace level (very verbose, disabled by `lowlog`).
    Trace,
}

impl AttLvl {
    /// Returns a 5‑character uppercase string representation.
    fn as_str(self) -> &'static str {
        match self {
            Self::Panic => "PANIC",
            Self::Error => "ERROR",
            Self::Warn  => " WARN",
            Self::Info  => " INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        }
    }

    /// Returns an ANSI‑coloured representation (used when the sink has `Pretty`).
    fn pretty(self) -> &'static str {
        match self {
            Self::Panic => "\x1b[35;1mPANIC\x1b[0m",
            Self::Error => "\x1b[31;1mERROR\x1b[0m",
            Self::Warn  => "\x1b[33;1m WARN\x1b[0m",
            Self::Info  => "\x1b[32;1m INFO\x1b[0m",
            Self::Debug => "\x1b[36;1mDEBUG\x1b[0m",
            Self::Trace => "\x1b[90;1mTRACE\x1b[0m",
        }
    }
}

// ============================================================================
// GLOBAL SINK REGISTRY
// ============================================================================

/// Global registry of log sinks.
///
/// This is a `Litex<Vec<&'static dyn Sink, 256>>` – a spinlock that disables
/// interrupts during the critical section. Up to 256 sinks can be registered.
pub static SINKS: Litex<Vec<&'static dyn Sink, 256>> = Litex::new(Vec::new());

/// Adds a sink to the global registry.
///
/// The sink must be `'static` (i.e., either a `static` variable or a leaked
/// reference). This function acquires the lock and pushes the sink.
#[allow(dead_code)]
pub fn add(sink: &'static dyn Sink) {
    let _ = SINKS.lock().push(sink);
}

/// Initializes the logging system.
///
/// If the `devlog` feature is enabled, this registers the serial sink
/// (`kmsg::dev::SINK`). This function is called very early in the boot process,
/// before any other subsystems.
pub fn init() {
    #[cfg(feature = "devlog")] add(*dev::SINK);
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
    let g = SINKS.lock();

    for sink in &*g {
        let mut m = Msg::new();

        // Choose level representation based on sink's Pretty flag
        let lvl = if (sink.kind().attrs & SinkAttrs::Pretty) != SinkAttrs::empty() {
            al.pretty()
        } else {
            al.as_str()
        };

        // Format the full message
        #[cfg(    feature = "lowlog" )]
        let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), modpath, lvl, c.as_str()));
        #[cfg(not(feature = "lowlog"))]
        let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), file, line, lvl, c.as_str()));

        sink.write(m.as_str());
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

    let g = SINKS.lock();

    for sink in &*g {
        let mut m = Msg::new();

        let lvl = if (sink.kind().attrs & SinkAttrs::Pretty) != SinkAttrs::empty() {
            al.pretty()
        } else {
            al.as_str()
        };

        #[cfg(    feature = "lowlog" )]
        let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), modpath, lvl, s));
        #[cfg(not(feature = "lowlog"))]
        let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), file, line, lvl, s));

        sink.write(m.as_str());
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

    let g = unsafe { SINKS.inner() };

    for sink in &*g {
        let mut m = Msg::new();

        let lvl = if (sink.kind().attrs & SinkAttrs::Pretty) != SinkAttrs::empty() {
            al.pretty()
        } else {
            al.as_str()
        };

        #[cfg(    feature = "lowlog" )]
        let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), modpath, lvl, s));
        #[cfg(not(feature = "lowlog"))]
        let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), file, line, lvl, s));

        sink.write(m.as_str());
    }
}
