use core::fmt::Write;

use crate::sync::Litex;
use heapless::{Vec, String};

pub mod dev;

const MAX_MSG_LEN: usize = 1024;

type Msg = String<MAX_MSG_LEN>;

pub const fn str4_to_u32(s: &str) -> u32
{
    let b = s.as_bytes();
    if b.len() != 4 { panic!("expected 4-byte literal"); }
    ((b[0] as u32) << 24) |
    ((b[1] as u32) << 16) |
    ((b[2] as u32) << 8)  |
     (b[3] as u32)
}

#[cfg(    feature = "lowlog" )] macro_rules! FMT {() => {"~ {:16.2} CPU #{} : {:>32} : {}: {}"}}
#[cfg(not(feature = "lowlog"))] macro_rules! FMT {() => {"~ {:16.2} CPU #{} : {:>20}:{:<3} : {}: {}"}}

bitflags! {
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
        const _ = !0;
    }
}

pub struct SinkIdent
{
    /// Attributes of the sink.
    pub attrs: SinkAttrs,
    /// A `u32` sink kind identifier (often created with [`str4_to_u32`]).
    pub kind: u32,
}

pub trait Sink: Sync
{
    fn write(&self, s: &str);

    fn kind(&self) -> SinkIdent;
}

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

pub static SINKS: Litex<Vec<&'static dyn Sink, 256>> = Litex::new(Vec::new());

pub fn add(sink: &'static dyn Sink)
{
    let _ = SINKS.lock().push(sink);
}

pub fn init()
{
    #[cfg(feature = "devlog")] add(*dev::SINK);
}

#[allow(unused)]
pub fn log(al: AttLvl, modpath: &'static str, file: &'static str, line: u32, fa: core::fmt::Arguments<'_>)
{
    #[cfg(feature = "lowlog")] let _ = file;
    #[cfg(feature = "lowlog")] let _ = line;

    const INLEN: usize = MAX_MSG_LEN >> 1;
    let mut c = String::<INLEN>::new();
    let _ = c.write_fmt(fa);

    let g = SINKS.lock();

    for sink in &*g
    {
        let mut m = Msg::new();

        if (sink.kind().attrs & SinkAttrs::Pretty) != SinkAttrs::empty()
        {
            #[cfg(    feature = "lowlog" )]
            let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), modpath, al.pretty(), c.as_str()));
            #[cfg(not(feature = "lowlog"))]
            let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), file, line, al.pretty(), c.as_str()));
        }
        else
        {
            #[cfg(    feature = "lowlog" )]
            let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), modpath, al.as_str(), c.as_str()));
            #[cfg(not(feature = "lowlog"))]
            let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), file, line, al.as_str(), c.as_str()));
        }

        sink.write(m.as_str());
    }
}

#[allow(unused)]
pub fn str_log(al: AttLvl, modpath: &'static str, file: &'static str, line: u32, s: &str)
{
    #[cfg(feature = "lowlog")] let _ = file;
    #[cfg(feature = "lowlog")] let _ = line;

    let g = SINKS.lock();

    for sink in &*g
    {
        let mut m = Msg::new();

        let l;

        if (sink.kind().attrs & SinkAttrs::Pretty) != SinkAttrs::empty() {
            l = al.pretty();
        } else {
            l = al.as_str();
        }

        #[cfg(    feature = "lowlog" )]
        let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), modpath, l, s));

        #[cfg(not(feature = "lowlog"))]
        let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), file, line, l, s));

        sink.write(m.as_str());
    }
}

#[allow(unused)]
pub unsafe fn str_log_noblock(al: AttLvl, modpath: &'static str, file: &'static str, line: u32, s: &str)
{
    #[cfg(feature = "lowlog")] let _ = file;
    #[cfg(feature = "lowlog")] let _ = line;

    let g = unsafe { SINKS.inner() };

    for sink in &*g
    {
        let mut m = Msg::new();

        let l;

        if (sink.kind().attrs & SinkAttrs::Pretty) != SinkAttrs::empty() {
            l = al.pretty();
        } else {
            l = al.as_str();
        }

        #[cfg(    feature = "lowlog" )]
        let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), modpath, l, s));

        #[cfg(not(feature = "lowlog"))]
        let _ = m.write_fmt(format_args!(FMT!(), crate::arch::get_time_from_boot_s(), crate::arch::current_cpu(), file, line, l, s));

        sink.write(m.as_str());
    }
}
