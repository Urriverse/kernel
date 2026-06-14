//! Kernel messaging subsystem.
//!
//! This module provides a global logging facility that dispatches log messages
//! to registered [`Sink`]s. Log levels range from `Panic` (most severe) down to
//! `Trace` (verbose debug). The logging macros (`info!`, `warn!`, etc.) are
//! defined in [`crate::macros`].
//!
//! # Example
//! ```
//! info!("Hello, world!");
//! error!("Something went wrong: {}", error_code);
//! ```

use core::fmt::Write;

use crate::sync::Nutex;
use heapless::{Vec, String};

mod dev;

/// Maximum length of a formatted log message (in bytes).
const MAX_MSG_LEN: usize = 1024;

/// A statically allocated log message buffer.
type Msg = String<MAX_MSG_LEN>;

/// Convert a 4‑byte string literal into a `u32` identifier.
///
/// This is used to create unique `kind` values for sinks.
///
/// # Panics
/// Panics if the input string is not exactly 4 bytes long.
pub const fn str4_to_u32(s: &str) -> u32
{
    let b = s.as_bytes();
    if b.len() != 4 { panic!("expected 4-byte literal"); }
    ((b[0] as u32) << 24) |
    ((b[1] as u32) << 16) |
    ((b[2] as u32) << 8)  |
     (b[3] as u32)
}

// Format string used for log messages. The `lowlog` feature produces a shorter
// format (no file/line).
#[cfg(    feature = "lowlog" )] macro_rules! FMT {() => {"~ {:7.3} : {}: {}"}}
#[cfg(not(feature = "lowlog"))] macro_rules! FMT {() => {"~ {:7.3} : {:>20}:{:<3}: {}: {}"}}

bitflags! {
    /// Attributes describing a log sink’s capabilities and importance.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SinkAttrs: u32
    {
        /// The sink is a pure virtual interface (no actual output).
        const Virtual  = 1 << 0;
        /// The sink is a buffer (e.g., ring buffer in memory).
        const Buffer   = 1 << 1;
        /// The sink is a physical port (e.g., serial, VGA).
        const Port     = 1 << 2;
        /// The sink is critical for system monitoring (always log here).
        const Critical = 1 << 3;
        /// The sink is optional; logging failures can be ignored.
        const Weak     = 1 << 4;
        /// The sink supports ANSI colour codes (e.g., a TTY).
        const Pretty   = 1 << 5;
        // More attributes can be added.
    }
}

/// Identification data for a log sink.
pub struct SinkIdent
{
    /// Attributes of the sink.
    pub attrs: SinkAttrs,
    /// A `u32` identifier (often created with [`str4_to_u32`]).
    pub kind: u32,
}

/// A trait for log output sinks.
///
/// Sinks are `&'static` and must be `Sync` so they can be used concurrently.
pub trait Sink: Sync
{
    /// Write a complete log line to the sink.
    fn write(&self, s: &str);

    /// Return the sink’s identification and attributes.
    fn kind(&self) -> SinkIdent;
}

/// Log severity level.
#[derive(Clone, Copy)]
pub enum AttLvl
{
    /// Unrecoverable error – system will panic or halt.
    Panic,
    /// Recoverable error.
    Error,
    /// Warning – unexpected but non‑fatal.
    Warn,
    /// Informational message.
    Info,
    /// Debugging information (may be disabled by feature `lowlog`).
    Debug,
    /// Trace level (very verbose, disabled by `lowlog`).
    Trace,
}

impl AttLvl
{
    /// Return the plain‑text name of the level (uppercase, fixed width).
    fn as_str(self) -> &'static str
    {
        match self
        {
            Self::Panic => "PANIC",
            Self::Error => "ERROR",
            Self::Warn  => " WARN",
            Self::Info  => " INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        }
    }

    /// Return the ANSI‑coloured name of the level.
    fn pretty(self) -> &'static str
    {
        match self
        {
            Self::Panic => "\x1b[35;1mPANIC\x1b[0m",
            Self::Error => "\x1b[31;1mERROR\x1b[0m",
            Self::Warn  => "\x1b[33;1m WARN\x1b[0m",
            Self::Info  => "\x1b[32;1m INFO\x1b[0m",
            Self::Debug => "\x1b[36;1mDEBUG\x1b[0m",
            Self::Trace => "\x1b[90;1mTRACE\x1b[0m",
        }
    }
}

/// Global list of registered log sinks, protected by a [`Nutex`].
pub static SINKS: Nutex<Vec<&'static dyn Sink, 256>> = Nutex::new(Vec::new());

/// Register a new log sink.
///
/// The sink must be `&'static` and implement the [`Sink`] trait.
pub fn add(sink: &'static dyn Sink)
{
    let _ = SINKS.lock().push(sink);
}

/// Initialise the logging subsystem.
///
/// Currently registers the development sink (`dev::SINK`) when the `devlog`
/// feature is enabled.
pub fn init()
{
    #[cfg(feature = "devlog")] add(*dev::SINK);
}

/// Emit a formatted log message to all registered sinks.
///
/// This function is called by the logging macros. It formats the message,
/// adds file/line information (unless `lowlog` is enabled), and writes to
/// every sink.
pub fn log(al: AttLvl, file: &'static str, line: u32, fa: core::fmt::Arguments<'_>)
{
    const INLEN: usize = MAX_MSG_LEN >> 1;
    let mut c = String::<INLEN>::new();
    let _ = c.write_fmt(fa);

    let g = SINKS.lock();

    for sink in &*g
    {
        let mut m = Msg::new();

        if (sink.kind().attrs & SinkAttrs::Pretty) != SinkAttrs::empty()
        {
            #[cfg(    feature = "lowlog" )] m.write_fmt(format_args!(FMT!(), 0.0f32, al.pretty(), c.as_str()));
            #[cfg(not(feature = "lowlog"))] m.write_fmt(format_args!(FMT!(), 0.0f32, file, line, al.pretty(), c.as_str()));
        }
        else
        {
            #[cfg(    feature = "lowlog" )] m.write_fmt(format_args!(FMT!(), 0.0f32, al.as_str(), c.as_str()));
            #[cfg(not(feature = "lowlog"))] m.write_fmt(format_args!(FMT!(), 0.0f32, file, line, al.as_str(), c.as_str()));
        }

        sink.write(m.as_str());
    }
}

/// Emit a static string log message to all registered sinks.
///
/// Equivalent to `log` but without formatting.
pub fn str_log(al: AttLvl, file: &'static str, line: u32, s: &str)
{
    let g = SINKS.lock();

    for sink in &*g
    {
        let mut m = Msg::new();

        if (sink.kind().attrs & SinkAttrs::Pretty) != SinkAttrs::empty()
        {
            #[cfg(    feature = "lowlog" )] m.write_fmt(format_args!(FMT!(), 0.0f32, al.pretty(), s));
            #[cfg(not(feature = "lowlog"))] m.write_fmt(format_args!(FMT!(), 0.0f32, file, line, al.pretty(), s));
        }
        else
        {
            #[cfg(    feature = "lowlog" )] m.write_fmt(format_args!(FMT!(), 0.0f32, al.as_str(), s));
            #[cfg(not(feature = "lowlog"))] m.write_fmt(format_args!(FMT!(), 0.0f32, file, line, al.as_str(), s));
        }

        sink.write(m.as_str());
    }
}
