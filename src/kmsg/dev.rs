//! Development‑only log sink (COM1 serial port).
//!
//! This sink implements the [`Sink`] trait for the legacy COM1 serial port
//! (I/O port `0x3f8`). It is used when the `devlog` feature is enabled.
//! The sink supports ANSI colour codes and is marked as `Critical` and `Pretty`.

use crate::kmsg::{AttLvl, Msg, Sink, SinkAttrs, SinkIdent};
use core::{cell::UnsafeCell, fmt::Write};
use heapless::{Deque, String};

/// The development log sink (COM1).
pub struct Dev;

impl Dev
{
    /// Create a new `Dev` sink and initialise the COM1 port.
    ///
    /// The initialisation sets the baud rate to 115200, 8N1, and enables FIFOs.
    pub fn new() -> Self
    {
        unsafe
        {
            // Disable interrupts
            x86::io::outb(0x3f8 + 1, 0);
            // Enable DLAB
            x86::io::outb(0x3f8 + 3, 128);
            // Set divisor to 1 (115200 baud)
            x86::io::outb(0x3f8 + 0, 1);
            x86::io::outb(0x3f8 + 1, 0);
            // 8N1, DLAB off
            x86::io::outb(0x3f8 + 3, 3);
            // Enable FIFO, clear, 14-byte threshold
            x86::io::outb(0x3f8 + 2, 7);
            // Enable IRQs (none)
            x86::io::outb(0x3f8 + 4, 3);
            // Clear any pending interrupts
            let _ = x86::io::inb(0x3f8 + 5);
        }
        Self
    }
}

// Safety: The Dev sink only touches memory‑mapped I/O (which is globally
// visible) and is safe to share between CPUs.
unsafe impl Sync for Dev {}

static ID: u32 = super::str4_to_u32("DEV0");

impl Sink for Dev
{
    /// Returns the sink identification: `DEV0` with `Port | Critical | Pretty`.
    fn kind(&self) -> SinkIdent
    {
        SinkIdent
        {
            attrs: SinkAttrs::Port | SinkAttrs::Critical | SinkAttrs::Pretty,
            kind: ID
        }
    }

    /// Write a string to COM1.
    ///
    /// Each byte is sent after waiting for the transmit FIFO to have space.
    /// A newline character is appended automatically.
    fn write(&self, s: &str)
    {
        for byte in s.bytes()
        {
            unsafe 
            {
                while (x86::io::inb(0x3f8 + 5) & 0x20) == 0
                {
                    core::arch::asm!("pause");
                }
                x86::io::outb(0x3f8, byte);
            }
        }
        unsafe 
        {
            while (x86::io::inb(0x3f8 + 5) & 0x20) == 0
            {
                core::arch::asm!("pause");
            }
            x86::io::outb(0x3f8, b'\n');
        }
    }
}

lazy_static! {
    static ref _SINK: Dev = Dev::new();
    /// The static reference to the development sink.
    pub static ref SINK: &'static Dev = &_SINK;
}
