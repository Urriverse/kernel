# Project Tree

```
└── kernel/
    ├── .cargo/
    │   └── config.toml
    ├── .gitignore
    ├── CLA.md
    ├── Cargo.toml
    ├── README.md
    ├── drafts/
    │   ├── driverkit.rs
    │   └── fb.rs
    ├── etc/
    │   ├── Cargo.in
    │   ├── font.psf
    │   ├── limine.conf
    │   ├── linker.ld
    │   └── profiles/
    │       ├── debug.toml
    │       ├── dev-release.toml
    │       ├── dev.toml
    │       └── release.toml
    ├── project_structure.md
    └── src/
        ├── arch/
        │   ├── amd64/
        │   │   ├── acpi/
        │   │   │   ├── handler.rs
        │   │   │   └── lapic.rs
        │   │   ├── acpi.rs
        │   │   ├── gdt.rs
        │   │   ├── idt.rs
        │   │   ├── mod.rs
        │   │   ├── paging.rs
        │   │   ├── percpu.rs
        │   │   ├── syscall.rs
        │   │   ├── timer.rs
        │   │   └── trap.rs
        │   └── mod.rs
        ├── dev/
        │   └── .gitkeep
        ├── dev.rs
        ├── kmsg/
        │   └── dev.rs
        ├── kmsg.rs
        ├── macros.rs
        ├── main.rs
        ├── mem/
        │   ├── bsa.rs
        │   ├── ema.rs
        │   ├── kdm.rs
        │   ├── pfm.rs
        │   ├── pmr.rs
        │   ├── ptm.rs
        │   ├── soa.rs
        │   ├── upa.rs
        │   └── vma.rs
        ├── mem.rs
        ├── rt/
        │   ├── entry.rs
        │   ├── gall.rs
        │   ├── mod.rs
        │   └── panic.rs
        ├── sched/
        │   ├── mod.rs
        │   ├── proc.rs
        │   ├── rq.rs
        │   ├── task.rs
        │   └── wq.rs
        ├── sync/
        │   ├── barrier.rs
        │   ├── litex.rs
        │   ├── mutex.rs
        │   ├── nitex.rs
        │   ├── nutex.rs
        │   └── rwlock.rs
        ├── sync.rs
        └── vfs/
            ├── err.rs
            ├── inode.rs
            ├── mb.rs
            ├── mod.rs
            ├── pvfs.rs
            └── root.rs
```

## File Contents

### `.gitignore`

```
target/
Cargo.lock
Cargo.toml

```

### `CLA.md`

```md
### CONTRIBUTOR LICENSE AGREEMENT

Thank you for your interest in contributing to **Kernel** ("the Project").  
This Contributor License Agreement ("Agreement") is entered into between **you**
(the "Contributor") and **Ivan Chetchasov** (an individual, acting on behalf of
the "Urriverse" project prior to its formal incorporation as a legal entity)
(the "Maintainer").

By submitting a Contribution to the Project (including but not limited to source
code, documentation, or other materials), you agree to be bound by the terms of
this Agreement.

#### 1. Definitions
- **"Contribution"** means any original work of authorship, including any
modifications, additions, or derivative works, that you intentionally submit to
the Project for inclusion therein.
- **"Project"** means the software project hosted at
https://github.com/Urriverse/kernel, including all its source code,
documentation, and associated materials.

#### 2. Grant of Rights

**2.1 Copyright License**  
Subject to the terms of this Agreement, you hereby grant to the Maintainer a
worldwide, royalty‑free, **exclusive**, perpetual, irrevocable, and fully
sublicensable copyright license, with the right to:
- reproduce, prepare derivative works of, publicly display, publicly perform,
and distribute your Contribution;
- sublicense your Contribution to third parties on any terms, including
proprietary or commercial terms, without any restriction;
- transfer such sublicenses to any number of third parties.

**2.2 Patent License**  
You also grant to the Maintainer a perpetual, irrevocable, non‑exclusive,
worldwide, royalty‑free patent license to make, have made, use, offer to sell,
sell, import, and otherwise transfer your Contribution, where such license
applies only to those patent claims licensable by you that are necessarily
infringed by your Contribution alone or by the combination of your Contribution
with the Project. This patent license does not extend to any other use of the
Project.

**2.3 Outbound License**  
The Maintainer agrees that the Project (including your Contribution) will always
remain available to the public under the terms of the **GNU Affero General Public**
**License, version 3 (AGPLv3)**, unless otherwise explicitly communicated in
writing. This obligation does not limit the Maintainer's right to offer the Project
under additional licenses (including, but not limited to, commercial licenses) as
permitted by the exclusive license granted in Section 2.1.

#### 3. Representations and Warranties
You represent and warrant that:
- you have the full legal right and authority to enter into this Agreement and to
grant the rights set forth herein;
- your Contribution is your original creation, or if it incorporates works of third
parties, you have obtained all necessary permissions and licenses to grant the rights
hereunder;
- your Contribution does not and will not infringe or violate any third‑party rights,
including intellectual property, privacy, or other proprietary rights;
- you have disclosed in writing to the Maintainer any known limitations or encumbrances
affecting your Contribution.

#### 4. No Obligation
The Maintainer is under no obligation to use, incorporate, or distribute your Contribution.
The Maintainer may, in its sole discretion, decide whether to accept your Contribution
and may remove it at any time.

#### 5. Miscellaneous

**5.1 Governing Law and Dispute Resolution**  
This Agreement shall be governed by and construed in accordance with the laws of the
State of **Delaware**, United States, without regard to its conflict of laws principles.
Any dispute, claim, or controversy arising out of or relating to this Agreement shall be
resolved exclusively in the **federal or state courts located in New Castle County, Delaware**,
and each party irrevocably consents to the personal jurisdiction and venue of such courts.

**5.2 Assignment to Future Legal Entity**  
The Contributor irrevocably consents to the assignment, transfer, or novation by the
Maintainer of all rights granted under this Agreement (including the exclusive copyright
license and patent license) to any legal entity that the Maintainer may form, provided that
such legal entity agrees in writing to assume the obligations set forth in Section 2.3
(Outbound License) with respect to the Project. Notice of such assignment will be posted in
the Project's repository.

**5.3 Protection for Russian Residents**  
For Contributors who are residents of the Russian Federation, the choice of Delaware law
under Section 5.1 does not deprive them of the protection afforded by mandatory provisions
of Russian law that cannot be derogated from by contract, provided that such mandatory
provisions do not conflict with the exclusive nature of the license granted in Section 2.1
or the assignment rights under Section 5.2.

**5.4 Entire Agreement and Severability**  
This Agreement constitutes the entire understanding between the parties with respect to
its subject matter. If any provision of this Agreement is held to be invalid, illegal, or
unenforceable by a court of competent jurisdiction, such provision shall be modified to the
minimum extent necessary to make it enforceable, and the remaining provisions shall continue
in full force and effect.

**5.5 No Partnership or Employment**  
Nothing in this Agreement shall be construed as creating a partnership, joint venture,
agency, or employment relationship between the Contributor and the Maintainer.

**5.6 Irrevocability**  
The rights and licenses granted herein are irrevocable. The Contributor acknowledges that
the Maintainer may rely on them and that they may be assigned or sublicensed as provided herein.

**By submitting a pull request, commit, or other contribution to the Project, you acknowledge**
**that you have read, understood, and agreed to the terms of this Agreement.**

```

### `README.md`

```md
# Kernel

[Documentation: The Kernel Book](https://github.com/Urriverse/kernel-book)

[Build system: Kenv](https://github.com/Urriverse/kenv)

## TODO

- Finish the VFS
- Implement initramfs
- Implement the KMI (nKMI and uKMI)
- Write nanokkit and microkkit (for nKMI and uKMI, respectively)
- Create an architecture diagram (graphical)

## License

AGPLv3 &copy; Urriverse

```

### `Cargo.toml`

```toml
[package]
name = "kernel"
version = "0.4.3-2.5"
edition = "2024"

[dependencies]
bitflags = "2"
extrum = { version = "0.1.3", features = ["no_std"] }
heapless = "0"
lazy_static = { version = "1", features = ["spin_no_std"] }
limine = "0"
x86 = "0"
x86_64 = "0"
acpi = { version = "6", features = ["alloc"] }

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
opt-level = 0 # TODO: fix issue and change back to 3
lto = true
debug = true
strip = false
overflow-checks = false

[profile.dbg]
inherits = "dev"

[profile.dev-release]
inherits = "release"

[features]
lowlog = []
devlog = []

```

### `etc/linker.ld`

```
PHDRS
{
    headers PT_PHDR PHDRS;
    text    PT_LOAD FILEHDR PHDRS;
    rodata  PT_LOAD;
    data    PT_LOAD;
    dynamic PT_DYNAMIC;
}

KERNEL_VMA = 0xFFFFFFFFFF000000;

SECTIONS
{
    . = SIZEOF_HEADERS + KERNEL_VMA;
    executable_start = . - SIZEOF_HEADERS;

    .text : {
        *(.text .text.*)
    } :text

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        *(.sdata2 .sdata2.*)
    } :rodata

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)

        KEEP(*(.limine_requests_start_marker))
        KEEP(*(.limine_requests))
        KEEP(*(.limine_requests_end_marker))
    } :data

    .dynamic : {
        *(.dynamic)
    } :data :dynamic

    .bss : {
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        *(COMMON)
    } :data

    executable_end = .;

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note .note.*)
        *(.interp)
    }
}
```

### `etc/limine.conf`

```
# wallpaper: boot():/boot/wallpaper.png
timeout: 0

/VAOS
    protocol: limine
    path: boot():/boot/kernel
    # module_path: /boot/initramfs.tar
    # module_cmdline: initramfs

```

### `etc/Cargo.in`

```
[package]
name = "{KERNEL_NAME}"
version = "0.4.3-2.5"
edition = "2024"

[dependencies]
bitflags = "2"
extrum = {{ version = "0.1.3", features = ["no_std"] }}
heapless = "0"
lazy_static = {{ version = "1", features = ["spin_no_std"] }}
limine = "0"
x86 = "0"
x86_64 = "0"
acpi = {{ version = "6", features = ["alloc"] }}

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
opt-level = 0 # TODO: fix issue and change back to 3
lto = false
debug = true
strip = false
overflow-checks = false

[profile.dbg]
inherits = "dev"

[profile.dev-release]
inherits = "release"

[features]
lowlog = []
devlog = []

```

### `etc/profiles/dev.toml`

```toml
[profile]
description = "Profile for debugging in emulator."

[env]
PROFILE = "dev"

[cargo]
profile = "dev"
features = [ "devlog" ]

```

### `etc/profiles/debug.toml`

```toml
[profile]
description = "Profile for debugging on real hardware."

[env]
PROFILE = "debug"

[cargo]
profile = "dbg"
features = [ "devlog" ]

```

### `etc/profiles/release.toml`

```toml
[profile]
description = "Profile for production on real hardware."

[env]
PROFILE = "release"

[cargo]
profile = "release"
features = [
    "lowlog",
#    "devlog"
]

```

### `etc/profiles/dev-release.toml`

```toml
[profile]
description = "Profile for production on real hardware."

[env]
PROFILE = "release"

[cargo]
profile = "dev-release"
features = [
    "lowlog",
    "devlog",
]

```

### `.cargo/config.toml`

```toml
[build]
target = "x86_64-unknown-none"

[target.x86_64-unknown-none]
rustflags = [
    "-C", "link-arg=-Tetc/linker.ld", 
    "-C", "relocation-model=static", 
    "-C", "code-model=kernel", 
    "-C", "no-redzone=y",
    "-C", "force-frame-pointers=yes",
]

```

### `src/macros.rs`

```rs
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
        $crate::kmsg::str_log($crate::kmsg::AttLvl::Trace, module_path!(), file!(), line!(), $s);
    }};
    ($($arg:tt)+) => {{
        #[cfg(not(feature = "lowlog"))]
        $crate::kmsg::log($crate::kmsg::AttLvl::Trace, module_path!(), file!(), line!(), format_args!($($arg)+));
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
        $crate::kmsg::str_log($crate::kmsg::AttLvl::Debug, module_path!(), file!(), line!(), $s);
    }};
    ($($arg:tt)+) => {{
        #[cfg(not(feature = "lowlog"))]
        $crate::kmsg::log($crate::kmsg::AttLvl::Debug, module_path!(), file!(), line!(), format_args!($($arg)+));
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
        $crate::kmsg::str_log($crate::kmsg::AttLvl::Info, module_path!(), file!(), line!(), $s);
    }};
    ($($arg:tt)+) => {{
        $crate::kmsg::log($crate::kmsg::AttLvl::Info, module_path!(), file!(), line!(), format_args!($($arg)+));
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
        $crate::kmsg::str_log($crate::kmsg::AttLvl::Warn, module_path!(), file!(), line!(), $s);
    }};
    ($($arg:tt)+) => {{
        $crate::kmsg::log($crate::kmsg::AttLvl::Warn, module_path!(), file!(), line!(), format_args!($($arg)+));
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
        $crate::kmsg::str_log($crate::kmsg::AttLvl::Error, module_path!(), file!(), line!(), $s);
    }};
    ($($arg:tt)+) => {{
        $crate::kmsg::log($crate::kmsg::AttLvl::Error, module_path!(), file!(), line!(), format_args!($($arg)+));
    }};
}

/// Logs a message at the **Panic** level.
///
/// This is an internal macro used by the panic handler. It is always enabled
/// and logs the message before the system halts.
#[macro_export]
macro_rules! __panic_msg {
    (str $s:expr) => {{
        $crate::kmsg::str_log($crate::kmsg::AttLvl::Panic, module_path!(), file!(), line!(), $s);
    }};
    ($($arg:tt)+) => {{
        $crate::kmsg::log($crate::kmsg::AttLvl::Panic, module_path!(), file!(), line!(), format_args!($($arg)+));
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
        
        barrier!{__LAST}
        unsafe extern "C" fn ap_main(_: &limine::mp::MpInfo) -> !
        {
            $($a)*
            __LAST.wait();
            $crate::sched::yield_now();
            loop {
                unsafe { core::arch::asm!("hlt"); }
            }
        }
        pub fn main() -> !
        {
            info!("Kernel v{} started.", env!("CARGO_PKG_VERSION"));
            $($b)*
            __LAST.open();
            $crate::sched::yield_now();
            loop {
                unsafe { core::arch::asm!("hlt"); }
            }
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
    ($vis:vis $name:ident <= $type:ident $(:)? $($arg:expr),*) => {
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

```

### `src/sync.rs`

```rs
//! # Synchronization Primitives
//!
//! This module provides a comprehensive set of synchronization primitives for the kernel,
//! ranging from simple spinlocks to reader‑writer locks and barrier primitives.
//! Each primitive is designed for specific use cases and offers different trade‑offs
//! between performance, safety, and functionality.
//!
//! ## Overview
//!
//! The sync module exports several mutually exclusive locking primitives, a read‑write lock,
//! and a simple barrier. They are all designed to be used in a `no_std` environment
//! and are suitable for kernel‑level synchronization across multiple CPU cores.
//!
//! ## Primitive Comparison
//!
//! | Primitive | Interrupts Disabled | Spins | Multi‑CPU Safe | Unsafe Inner | Use Case |
//! |-----------|---------------------|-------|----------------|--------------|----------|
//! | [`Mutex`] | No                  | Yes   | Yes            | No           | Simple spinlock for data shared between tasks, not interrupt context. |
//! | [`Nutex`] | Yes                 | Yes   | Yes            | No           | Spinlock with interrupt disabling; safe for interrupt handlers. |
//! | [`Litex`] | Yes                 | Yes   | Yes            | Yes          | Like `Nutex` but with unsafe inner access for early boot. |
//! | [`Nitex`] | Yes                 | No    | No (per‑CPU)   | Yes          | Interrupt‑only lock for per‑CPU data; no spinning. |
//! | [`RwLock`]| No                  | Yes   | Yes            | No           | Reader‑writer lock; multiple readers or single writer. |
//! | [`Barrier`] | No                  | Yes   | Yes            | No           | One‑time barrier flag; spins until opened. |
//!
//! ## Module Structure
//!
//! - **`mutex`**: Basic spinlock (`Mutex`). No interrupt disabling.
//! - **`nutex`**: Interrupt‑disabling spinlock (`Nutex`). Safe for interrupt handlers.
//! - **`litex`**: Interrupt‑disabling spinlock with unsafe inner access (`Litex`).
//! - **`nitex`**: Interrupt‑only lock (`Nitex`). Does not spin; per‑CPU only.
//! - **`rwlock`**: Reader‑writer lock (`RwLock`). Allows multiple readers or single writer.
//! - **`barrier`**: One‑time barrier (`Barrier`). Spins until opened.
//!
//! ## Usage Recommendations
//!
//! - **For most shared data**: Use [`Nutex`] if the data can be accessed from interrupt
//!   context, or [`Mutex`] if it is only accessed from task context.
//! - **For per‑CPU data**: Use [`Nitex`] (no spinning, interrupts disabled).
//! - **For read‑heavy data**: Use [`RwLock`] to allow concurrent readers.
//! - **For boot‑time barriers**: Use [`Barrier`] to synchronise BSP and APs.
//!
//! ## Safety
//!
//! All primitives are designed to be safe when used correctly. However, some provide
//! `unsafe` methods for raw access (e.g., `Litex::inner()`). The caller must ensure
//! that the lock is not bypassed when using these methods.

// ============================================================================
// MODULE DECLARATIONS
// ============================================================================

mod nutex;   // Interrupt‑disabling spinlock (safe)
mod mutex;   // Simple spinlock (no interrupt disabling)
mod nitex;   // Interrupt‑only lock (no spinning)
mod barrier;   // One‑time barrier
mod rwlock;  // Reader‑writer lock
mod litex;   // Interrupt‑disabling spinlock with unsafe inner

// ============================================================================
// RE‑EXPORTS
// ============================================================================

pub use nutex::*;   // Nutex, NutexGuard
pub use mutex::*;   // Mutex, MutexGuard
pub use nitex::*;   // Nitex, NitexGuard
pub use barrier::*;   // Barrier
pub use rwlock::*;  // RwLock, RwLockReadGuard, RwLockWriteGuard
pub use litex::*;   // Litex, LitexGuard

```

### `src/kmsg.rs`

```rs
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

```

### `src/main.rs`

```rs
//! # Kernel Main Entry Point
//!
//! This module serves as the primary entry point for the kernel. It orchestrates
//! the entire bootstrapping process across all CPU cores (BSP and APs) and
//! initializes every major subsystem in the correct order.
//!
//! ## Boot Flow Overview
//!
//! 1. **BSP (Bootstrap Processor) Initialization**
//!    - `_start()` → `main()` (via `rt/entry.rs`)
//!    - Architecture early init (`arch::early_init_bs`)
//!    - Start APs (`start_aps!`)
//!    - Architecture init (`arch::init_bsp`)
//!    - Memory init (`mem::init_bsp`)
//!    - Architecture late init (`arch::late_init_bsp`, `arch::late_init`)
//!    - Device init (`dev::init`)
//!    - Scheduler init (`sched::init`)
//!    - Spawn initial tasks (reaper, test)
//!
//! 2. **AP (Application Processor) Initialization**
//!    - Each AP follows a synchronized boot sequence using `Fueue` barriers:
//!      - `ARCH_INIT.wait()` – wait for BSP arch init
//!      - `MEM_INIT.wait()` – wait for BSP memory init
//!      - `LATE_INIT.wait()` – wait for BSP late init
//!      - `DEV_INIT.wait()` – wait for device init
//!
//! 3. **Final State**
//!    - All cores enter HLT loop
//!    - Scheduler takes over for task management
//!
//! ## Sync Primitives Used
//!
//! - `fueue!` – creates `Fueue` barriers (`ARCH_INIT`, `MEM_INIT`, `LATE_INIT`, `DEV_INIT`)
//! - `entry!` – defines the BSP/AP entry points with proper synchronization
//!
//! ## Safety
//!
//! This module contains unsafe code for:
//! - Raw pointer manipulation for `MetaBlock` and `RootRef`
//! - AP bootstrap via Limine SMP
//! - Static mutable access to `SPURIOUS` PML4

#![no_std]
#![no_main]

#![feature(unsafe_cell_access)]
#![feature(abi_x86_interrupt)]
#![feature(const_trait_impl)]
#![feature(likely_unlikely)]
#![feature(const_destruct)]
#![feature(const_cmp)]

#![allow(unused)]
#![allow(clippy::missing_transmute_annotations)]
#![warn(unused_braces)]
#![warn(unused_comparisons)]
#![warn(unused_import_braces)]
#![warn(unused_labels)]
#![warn(unused_mut)]
#![warn(unused_parens)]
#![warn(unused_qualifications)]
#![warn(unused_unsafe)]

#![cfg_attr(not(debug_assertions), allow(unused_assignments))]

use core::ptr::addr_of;
use alloc::string::ToString;
use crate::{sched::current_process, vfs::{MetaBlock, PvfsMb}};

// ============================================================================
// EXTERNAL CRATES
// ============================================================================

/// Extrum – "leaky" enumerations
#[macro_use]
pub extern crate extrum;

/// Bitflags – for flag-based bitmask types
#[macro_use]
pub extern crate bitflags;

/// Lazy_static – for lazy-initialized static data
#[macro_use]
pub extern crate lazy_static;

/// Alloc – provides `Vec`, `Box`, `String`, etc. (the global allocator is set in `rt/gall.rs`)
#[macro_use]
pub extern crate alloc;

// ============================================================================
// INTERNAL MODULES
// ============================================================================

/// Macros – logging, entry point, Limine requests, etc.
#[macro_use]
mod macros;

/// Runtime – entry point, panic handler, global allocator
mod rt;

/// Synchronization primitives (mutexes, rwlocks, barriers, etc.)
mod sync;

/// Kernel message logging system
mod kmsg;

/// Memory management (allocators, paging, physical memory regions, etc.)
mod mem;

/// Architecture-specific code (x86_64: GDT, IDT, paging, ACPI, syscalls, etc.)
mod arch;

/// Device model (driver framework, device registration, method calls)
#[allow(unused)] mod dev;

/// Scheduler (EEVDF-based task scheduler, processes, runqueues)
mod sched;

/// Virtual File System (VFS) – inodes, mount points, file operations
mod vfs;

// ============================================================================
// SYNCHRONIZATION BARRIERS
// ============================================================================

// Barriers for coordinating multi-core initialization.
//
// Each barrier is a simple flag that is initially closed. BSP opens them
// sequentially as each subsystem becomes ready, and APs block on each
// barrier before proceeding.
//
// - `ARCH_INIT` – architecture initialization complete
// - `MEM_INIT`  – memory management initialized
// - `LATE_INIT` – late architecture init
// - `DEV_INIT`  – device model initialized
barrier! { ARCH_INIT MEM_INIT LATE_INIT DEV_INIT }

// ============================================================================
// KERNEL ENTRY POINT (BSP + AP)
// ============================================================================

// Main entry point – defines both BSP and AP entry functions.
//
// ## BSP Flow (core 0)
// 1. Early architecture init (paging, CPUID, percpu)
// 2. Start all APs via Limine (`start_aps!`)
// 3. Complete architecture init (GDT, IDT, syscall)
// 4. Initialize memory management (EMA, PFM, KDM, UPA, SOA, PTM)
// 5. Late architecture init (ACPI, HPET, APIC, timer, interrupts)
// 6. Device model init
// 7. Scheduler init
// 8. Spawn `reaper` (zombie reaper) and `test` (VFS test) tasks
//
// ## AP Flow (all other cores)
// 1. Early architecture init (CPUID, percpu)
// 2. Block on `ARCH_INIT` → wait for BSP arch init
// 3. Complete AP architecture init (GDT, IDT)
// 4. Block on `MEM_INIT` → wait for BSP memory init
// 5. Activate shared page table
// 6. Block on `LATE_INIT` → wait for BSP late init
// 7. Block on `DEV_INIT` → wait for device init
// 8. Join scheduler (HLT loop)
entry! {
    for BSP {
        // --------------------------------------------------------------------
        // PHASE 1: Architecture Early Initialization (BSP)
        // --------------------------------------------------------------------
        arch::early_init_bs();

        // Start all APs (each AP will execute `for AP` block)
        start_aps!();

        // --------------------------------------------------------------------
        // PHASE 2: Architecture Full Initialization (BSP)
        // --------------------------------------------------------------------
        arch::init_bsp();

        // Signal that architecture init is complete -> APs can proceed
        ARCH_INIT.open();

        // --------------------------------------------------------------------
        // PHASE 3: Memory Management Initialization (BSP)
        // --------------------------------------------------------------------
        mem::init_bsp();

        // Signal that memory init is complete -> APs can proceed
        MEM_INIT.open();

        // --------------------------------------------------------------------
        // PHASE 4: Late Architecture Initialization (BSP)
        // --------------------------------------------------------------------
        arch::late_init_bsp();
        arch::late_init();

        // Signal that late init is complete -> APs can proceed
        LATE_INIT.open();

        // --------------------------------------------------------------------
        // PHASE 5: Device Model Initialization (BSP)
        // --------------------------------------------------------------------
        dev::init();

        // Signal that device init is complete -> APs can proceed
        DEV_INIT.open();

        // --------------------------------------------------------------------
        // PHASE 6: Scheduler Initialization (BSP)
        // --------------------------------------------------------------------
        let ticks_per_10ms = arch::timer::get_ticks_per_10ms();
        sched::init(ticks_per_10ms);

        // --------------------------------------------------------------------
        // PHASE 7: Spawn Initial Kernel Tasks (BSP)
        // --------------------------------------------------------------------

        // Spawn the reaper task – reaps zombie processes
        let _ = sched::spawn_kernel_task(
            reaper,
            sched::task::Priority(-1),
            "reaper",
            Some(
                vfs::RootRef::new(
                    vfs::RootReg::new()
                )
            )
        );

        // Spawn the VFS test task – validates the VFS implementation
        let _ = sched::spawn_kernel_task(
            vfs_test,
            sched::task::Priority(0),
            "test",
            Some(
                vfs::RootRef::new(
                    vfs::RootReg::new()
                )
            )
        );
    }

    for AP {
        // --------------------------------------------------------------------
        // AP INITIALIZATION
        // --------------------------------------------------------------------

        // Early AP init (CPUID, percpu, etc.)
        arch::early_init();

        // Wait for BSP to complete architecture init
        ARCH_INIT.wait();

        // Full AP init (GDT, IDT, etc.)
        arch::init_ap();

        // Wait for BSP to complete memory init
        MEM_INIT.wait();

        // AP memory init (activate shared page table)
        mem::init_ap();

        // Wait for BSP to complete late init
        LATE_INIT.wait();

        // AP late init (ACPI, etc.)
        arch::late_init();

        // Wait for BSP to complete device init
        DEV_INIT.wait();
    }
}

// ============================================================================
// LAZY-STATIC VFS TEST STRUCTURES
// ============================================================================

lazy_static! {
    /// Purely virtual filesystem instance for testing.
    ///
    /// `PvfsMb` is an in-memory filesystem that doesn't require a block device.
    /// It's used to test VFS operations early in the boot process.
    static ref MBINST: PvfsMb = PvfsMb::new();

    /// Static reference to a `MetaBlock` used for the test filesystem.
    ///
    /// This is a globally accessible mount block that provides VFS operations
    /// via the `PVFS_VTABLE` function table.
    static ref TESTFSMBLK0: &'static MetaBlock
    =   vfs::get_mblk(
        vfs::reg_mblk(
            vfs::new_mblock(
                0, &vfs::PVFS_VTABLE,
                unsafe {
                    (
                        addr_of!(*MBINST) as *mut ()
                    ).as_mut_unchecked()
                }
            )
        )
    ).unwrap();
}

// ============================================================================
// KERNEL TASKS
// ============================================================================

/// VFS test task – validates in-memory filesystem operations.
///
/// This task:
/// 1. Retrieves the current process's root registry
/// 2. Creates a new root mount point named "pv" for the test FS
/// 3. Creates a new file in the test FS
/// 4. Writes a marker string to the file
/// 5. Reads back the data and logs it
/// 6. Exits with code 0
///
/// # Panics
/// Panics if any VFS operation fails, which would indicate a VFS regression.
fn vfs_test() {
    // Get the current process's root registry
    let roots = current_process().expect("NOPID").roots.clone();
    debug!("Got roots");

    // Create a new inode and attach it to the test filesystem
    let mut inode = vfs::Inode::new();
    inode.mblock = &TESTFSMBLK0;
    debug!("Inode created");

    // Register a new root mount point named "pv"
    if let Err(e) = roots.add_new_root("pv".to_string(), inode.id) {
        panic!("{:?}", e);
    }
    debug!("Root created");

    // Create a file in the test filesystem
    let iid = match vfs::new(&TESTFSMBLK0, inode, vfs::Kind::File) {
        Ok(i) => i,
        Err(e) => panic!("{:?}", e),
    };
    debug!("File created");

    // Write test data to the file
    if let Err(e) = vfs::write(&iid, 0, b"[NOT FAILED]") {
        panic!("E: {:?}", e)
    }
    debug!("File written");

    // Read back the data
    let mut buf: [u8; 12] = *b"[FAILED]    ";
    if let Err(e) = vfs::read(&iid, 0, &mut buf) {
        panic!("E: {:?}", e)
    }
    debug!("vfs test: {}", str::from_utf8(&buf).unwrap());

    // Exit cleanly
    sched::exit(0);
}

/// Zombie reaper task – reaps terminated child processes.
///
/// This task runs in an infinite loop, waiting for any child process to exit.
/// When a child exits (becomes a zombie), it:
/// 1. Logs the exit event
/// 2. Collects the exit code
/// 3. Removes the zombie from the task registry
///
/// # Note
/// This is a kernel task and never exits; it runs forever to ensure no
/// zombies accumulate.
fn reaper() {
    loop {
        crate::info!("waiting for any child to exit...");
        if let Some((id, code)) = sched::wait_any() {
            crate::info!("reaped zombie task {:?}, exit code: {}", id, code);
        }
    }
}

```

### `src/mem.rs`

```rs
//! # Memory Management Subsystem
//!
//! This module provides the kernel's comprehensive memory management infrastructure,
//! from physical memory detection to virtual memory allocation and paging.
//!
//! ## Architecture Overview
//!
//! The memory subsystem is organized into several layers, each with a distinct
//! responsibility. They are initialized in a specific order during boot:
//!
//! 1. **Physical Memory Regions (PMR)** – detects and enumerates all physical
//!    memory regions provided by the bootloader (Limine memory map).
//!
//! 2. **Early Memory Allocator (EMA)** – a simple bump allocator used during
//!    early boot before the full memory management is available.
//!
//! 3. **Kernel Direct Mapping (KDM)** – manages the HHDM (High Half Direct Map),
//!    providing a direct linear mapping of all physical memory into the kernel's
//!    virtual address space.
//!
//! 4. **Page Frame Manager (PFM)** – manages metadata for physical page frames
//!    (SPARSEMEM model) and tracks their allocation state.
//!
//! 5. **Buddy System Allocator (BSA)** – a zone-based buddy allocator for
//!    physical memory, with per-CPU caches for performance.
//!
//! 6. **Unified Page Allocator (UPA)** – a facade that switches from EMA to BSA
//!    after migration, providing a uniform allocation interface.
//!
//! 7. **Sized Object Allocator (SOA)** – a slab allocator for small objects
//!    (≤ 2048 bytes), built on top of UPA.
//!
//! 8. **Page Table Manager (PTM)** – a per-address-space page table manager
//!    (`Polen`) that handles mapping, unmapping, and page splitting/merging.
//!
//! 9. **Virtual Memory Area (VMA)** – a Red-Black tree of virtual memory regions
//!    for userspace processes, used for demand paging and memory mapping.
//!
//! ## Initialization Flow
//!
//! The memory subsystem is initialized in two phases: BSP (boot processor) and AP
//! (application processors), with synchronization via barriers.
//!
//! ### BSP Initialization (`init_bsp`)
//! Called on the bootstrap processor.
//!
//! ```text
//! ema::init()     -> initialize early memory allocator
//! pfm::init()     -> initialize page frame metadata (SPARSEMEM)
//! kdm::init()     -> initialize kernel direct mapping (HHDM)
//! bsa::init()     -> initialize buddy system allocator
//! upa::migrate()  -> switch UPA from EMA to BSA
//! soa::init()     -> initialize sized allocator
//! gall::set_soa() -> switch global allocator to SOA backend
//! PTM.lock()      -> set current page table to the reference one
//! activate()      -> load CR3
//! ```
//!
//! ### AP Initialization (`init_ap`)
//! Called on each AP after waiting for `MEM_INIT` to open.
//!
//! ```text
//! PTM.lock().activate() → activate the shared page table
//! ```
//!
//! ## Key Data Structures
//!
//! - `PTM`: A `Nutex<Polen>` holding the current page table manager. Used
//!   for all kernel and per-process address space operations.
//! - `SPURIOUS`: A static, empty PML4 used as a temporary placeholder before
//!   the real page table is created.
//!
//! ## Safety Notes
//!
//! - `SPURIOUS` is `static mut` and accessed only during early boot before
//!   concurrency.
//! - `PTM` is wrapped in a `Nutex` (interrupt-disabling spinlock) to ensure
//!   safe concurrent access across CPUs.
//! - The global allocator is switched from a dummy to the SOA backend via
//!   `rt::gall::set_soa()`, which is an atomic operation.

// ============================================================================
// SUBMODULES
// ============================================================================

pub mod pmr;   // Physical Memory Regions
pub mod kdm;   // Kernel Direct Mapping (HHDM)
pub mod ema;   // Early Memory Allocator
pub mod ptm;   // Page Table Manager (+ <b>Pol</b>icy <b>En</b>gine)
pub mod upa;   // Unified Page Allocator
pub mod pfm;   // Page Frame Manager
pub mod bsa;   // Buddy System Allocator
pub mod soa;   // Slab Object Allocator
pub mod vma;   // Virtual Memory Areas

// ============================================================================
// GLOBAL STATE
// ============================================================================

/// A static, empty PML4 table used as a temporary placeholder.
///
/// # Safety
/// This is `static mut` and is only used during early boot before
/// any other cores are active or paging is fully set up. After
/// `mem::init_bsp()`, it is replaced with a proper page table.
///
/// This table is initially all zeroes (no entries), which allows
/// `Polen::from_exco` to create a clean address space.
#[allow(static_mut_refs)]
static mut SPURIOUS: crate::arch::paging::Tab = crate::arch::paging::Tab::new();

/// The global Page Table Manager, shared across all CPU cores.
///
/// This is a `Nutex` (interrupt-disabling spinlock) that holds the
/// current `Polen`, which manages the kernel's page tables.
///
/// During BSP init, we create a `Polen` from the `SPURIOUS` table
/// and then replace it with the reference `Polen` after other
/// subsystems (like SOA) are ready. APs simply activate the
/// reference `Polen` to share the kernel's address space.
///
/// # Invariants
/// - Must be initialized before any memory allocation occurs.
/// - After `init_bsp()`, the inner `Polen` is the reference page
///   table that maps all kernel memory and the HHDM.
#[allow(static_mut_refs)]
pub static PTM: crate::sync::Nutex<ptm::Polen> = crate::sync::Nutex::new(
    ptm::Polen::from_exco(
        crate::arch::paging::Exco::from_root(
            unsafe { &mut SPURIOUS },
            0u64,
            false
        )
    )
);

// ============================================================================
// INITIALIZATION FUNCTIONS
// ============================================================================

/// Initializes the memory subsystem on the Bootstrap Processor (BSP).
///
/// This function is called once from `main()` on CPU #0. It sequentially
/// initializes all memory management layers and then switches the global
/// allocator from the dummy to the SOA backend.
///
/// # Order of Operations
/// 1. `ema::init()` – set up the early memory allocator.
/// 2. `pfm::init()` – build the SPARSEMEM page frame metadata.
/// 3. `kdm::init()` – establish the HHDM direct mapping.
/// 4. `bsa::init()` – initialize the buddy system allocator.
/// 5. `upa::migrate()` – switch UPA from EMA to BSA.
/// 6. `soa::init()` – initialize the slab allocator.
/// 7. `crate::rt::gall::set_soa()` – set the global allocator to SOA.
/// 8. Replace `PTM` with the reference `Polen` (from `ptm::Polen::reference()`).
/// 9. Activate the new page table (load CR3).
///
/// # Panics
/// - If any of the allocator initializations fail (e.g., no usable memory).
/// - If the global allocator switch fails (should not happen).
///
/// # Safety
/// - Called before interrupts are enabled; single-threaded.
/// - Uses `unsafe` to write to `SPURIOUS` and `PTM`.
pub fn init_bsp() {
    // Initialize each component in order
    ema::init();   // early bump allocator
    pfm::init();   // page frame metadata
    kdm::init();   // HHDM mapping
    bsa::init();   // buddy system allocator
    upa::migrate(); // switch UPA backend to BSA
    soa::init();   // slab allocator

    // Switch the global allocator from dummy to SOA
    crate::rt::gall::set_soa();

    // Acquire the PTM lock and replace the inner Polen with the
    // reference Polen (which has the kernel mappings and HHDM).
    *PTM.lock() = ptm::Polen::reference();

    // Activate the page table (load CR3) on the current CPU.
    unsafe { PTM.lock().activate() };
}

/// Initializes the memory subsystem on an Application Processor (AP).
///
/// This function is called on each AP after waiting for the `MEM_INIT`
/// barrier to be opened by the BSP. It simply activates the shared
/// page table (the reference `Polen`) so that the AP uses the same
/// address space as the kernel.
///
/// # Safety
/// - Called with interrupts disabled; APs are still in early boot.
/// - Assumes `PTM` has been initialized by the BSP.
pub fn init_ap() {
    unsafe { PTM.lock().activate() };
}

```

### `src/dev.rs`

```rs
//! # Device Model
//!
//! This module provides a generic, extensible device driver framework for the kernel.
//! It allows drivers to register devices, expose methods (operations), and dispatch
//! calls to those methods from userspace or other kernel components.
//!
//! ## Overview
//!
//! The device model is built around the following concepts:
//!
//! - **DeviceId**: A unique, generation‑aware identifier for a device. It encodes
//!   both an index into the global device registry and a generation number to
//!   detect stale handles.
//!
//! - **Device**: A struct that holds a device's name, parent, driver‑specific data,
//!   and a map of method IDs to function pointers (`DeviceMethod`).
//!
//! - **Registry**: A global, lock‑protected array of optional `Device` boxes. It
//!   manages allocation, deallocation, and lookup of devices.
//!
//! - **DeviceMethod**: An `extern "C"` function that takes a `DeviceId` and an
//!   opaque `usize` argument, returning a `DeviceResult`. This ABI is stable
//!   and callable from any context.
//!
//! ## Method Dispatch
//!
//! Method calls are performed via `call_method(id, method_id, arg)`, which:
//! 1. Looks up the device in the registry.
//! 2. Finds the method in the device's method table.
//! 3. Calls the method with the provided argument.
//!
//! Methods are identified by a `u64` hash (typically a FNV‑1a hash of a string
//! like `"fb.get_info"`), generated via the `interface!` macro from `driverkit`.
//!
//! ## Driver Data
//!
//! Each device can store an opaque `usize` (typically a pointer to driver‑specific
//! state) in `driver_data`. This is set via `set_driver_data` and retrieved with
//! `get_driver_data`.
//!
//! ## Usage Example (from `etc/fb.rs`)
//!
//! ```ignore
//! let mut dev = Device::new("fb0");
//! dev.add_method(FB_GET_INFO, fb_get_info_method);
//! dev.add_method(FB_CLEAR, fb_clear_method);
//! dev.add_method(FB_PLOT, fb_plot_method);
//!
//! let state = Box::new(FbState { ... });
//! dev.driver_data = Box::into_raw(state) as usize;
//!
//! let dev_id = crate::dev::register_device(dev).unwrap();
//! ```
//!
//! ## Safety
//!
//! The device model is safe to use from multiple CPUs; the registry is protected
//! by a `Nutex` (interrupt‑disabling spinlock). However, driver authors must
//! ensure that their method implementations are thread‑safe and handle any
//! necessary synchronization internally.
//!
//! ## Limitations
//!
//! - Maximum of 4096 devices.
//! - Methods are `extern "C"` and cannot capture context; driver state must be
//!   accessed via `driver_data` or global structures.

use crate::sync::Nutex;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;

// ============================================================================
// TYPES
// ============================================================================

/// A 64‑bit identifier for a device method.
///
/// Typically generated by hashing a method name (e.g., `"fb.get_info"`) using
/// FNV‑1a via the `interface!` macro.
pub type MethodId = u64;

extrum! {
    /// Device operation status codes.
    ///
    /// These are returned by methods as part of `DeviceResult`. A `SUCCESS` status
    /// indicates the operation succeeded; any other value indicates an error.
    #[derive(Clone, Copy, PartialEq)]
    pub enum DeviceStatus: usize {
        SUCCESS = 0,
        NOT_FOUND = 1,
        INVALID_ARG = 2,
        BUSY = 3,
        IO_ERROR = 4,
        UNSUPPORTED = usize::MAX,
    }
}

/// The result of a device method call.
///
/// This C‑compatible struct is returned by all `DeviceMethod` functions.
/// It contains a `value` (e.g., number of bytes written) and a `status` code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct DeviceResult {
    /// The return value of the method (semantics depend on the method).
    pub value: usize,
    /// The status code indicating success or error.
    pub status: DeviceStatus,
}

impl DeviceResult {
    /// Creates a new result with the given value and status.
    #[inline]
    pub const fn new(value: usize, status: DeviceStatus) -> Self {
        Self { value, status }
    }

    /// Creates a successful result.
    #[inline]
    pub const fn ok(value: usize) -> Self {
        Self { value, status: DeviceStatus::SUCCESS }
    }

    /// Creates an error result with the given status.
    #[inline]
    pub const fn err(status: DeviceStatus) -> Self {
        Self { value: 0, status }
    }

    /// Converts this result into a Rust `Result<usize, DeviceStatus>`.
    #[inline]
    pub fn as_result(self) -> Result<usize, DeviceStatus> {
        if self.status == DeviceStatus::SUCCESS {
            Ok(self.value)
        } else {
            Err(self.status)
        }
    }

    /// Constructs a `DeviceResult` from a Rust `Result`.
    #[inline]
    pub fn from_result(res: Result<usize, DeviceStatus>) -> Self {
        match res {
            Ok(value) => Self::ok(value),
            Err(status) => Self::err(status),
        }
    }
}

/// A device method function pointer.
///
/// All device methods have this ABI: they take a `DeviceId` and an opaque
/// `usize` argument (typically a pointer to a method‑specific argument struct),
/// and return a `DeviceResult`.
pub type DeviceMethod = extern "C" fn(DeviceId, usize) -> DeviceResult;

/// A unique, generation‑aware device identifier.
///
/// The ID is a 32‑bit value that encodes:
/// - Lower 20 bits: device index (0..1023)
/// - Upper 12 bits: generation number (incremented on each registration at the
///   same index)
///
/// This prevents use‑after‑free bugs: a stale `DeviceId` from a previous
/// incarnation of a device at the same index will have a different generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct DeviceId(u32);

impl DeviceId {
    /// Returns the index part of the ID.
    #[inline]
    pub const fn index(self) -> usize {
        (self.0 & 0x000FFFFF) as usize
    }

    /// Returns the generation part of the ID.
    #[inline]
    pub const fn generation(self) -> u16 {
        (self.0 >> 20) as u16
    }

    /// Constructs a new `DeviceId` from an index and generation.
    #[inline]
    pub const fn new(index: usize, gen_: u16) -> Self {
        Self(((gen_ as u32) << 20) | (index as u32))
    }

    /// Returns `true` if this is the null device ID.
    #[inline]
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }

    /// The null device ID, representing an invalid device.
    pub const NULL: Self = Self(0);
}

// ============================================================================
// DEVICE STRUCTURE
// ============================================================================

/// A registered device.
///
/// This struct holds all information about a device, including its name,
/// parent relationship, driver‑specific data, and the table of methods it
/// supports.
pub struct Device {
    /// The assigned device ID (set by the registry upon registration).
    pub id: DeviceId,

    /// Human‑readable device name (e.g., `"fb0"`, `"sda"`).
    pub name: String,

    /// Optional parent device ID (for hierarchical device trees).
    pub parent: Option<DeviceId>,

    /// Opaque driver‑specific data (e.g., pointer to driver state).
    pub driver_data: usize,

    /// Method table: mapping from method ID to function pointer.
    pub methods: BTreeMap<MethodId, DeviceMethod>,
}

impl Device {
    /// Creates a new device with the given name.
    ///
    /// The device is initially unregistered (`id` is `NULL`) and has no methods.
    pub fn new(name: &str) -> Box<Self> {
        Box::new(Self {
            id: DeviceId::NULL,
            name: String::from(name),
            parent: None,
            driver_data: 0,
            methods: BTreeMap::new(),
        })
    }

    /// Adds a method to the device's method table.
    ///
    /// # Arguments
    /// * `method_id` – The unique identifier for the method.
    /// * `method`    – The function pointer to be called.
    pub fn add_method(&mut self, method_id: MethodId, method: DeviceMethod) {
        self.methods.insert(method_id, method);
    }

    /// Retrieves a method by its ID, if present.
    pub fn get_method(&self, method_id: MethodId) -> Option<DeviceMethod> {
        self.methods.get(&method_id).copied()
    }
}

// ============================================================================
// GLOBAL REGISTRY
// ============================================================================

/// Maximum number of devices that can be registered.
const MAX_DEVICES: usize = 4096;

/// The global device registry.
///
/// This struct maintains an array of optional `Device` boxes, along with
/// a generation counter for each slot to support generation‑aware IDs.
struct Registry {
    /// The actual device storage. `None` means the slot is free.
    devices: [Option<Box<Device>>; MAX_DEVICES],

    /// Generation counter for each slot, incremented on each registration.
    generations: [u16; MAX_DEVICES],
}

impl Registry {
    /// Creates a new, empty registry.
    const fn new() -> Self {
        Self {
            devices: [const { None }; MAX_DEVICES],
            generations: [0; MAX_DEVICES],
        }
    }

    /// Registers a device, assigning it a unique `DeviceId`.
    ///
    /// Returns `Some(id)` if a free slot was found, or `None` if the registry
    /// is full.
    fn register(&mut self, mut device: Box<Device>) -> Option<DeviceId> {
        for (i, slot) in self.devices.iter_mut().enumerate() {
            if slot.is_none() {
                // Increment the generation for this slot.
                self.generations[i] = self.generations[i].wrapping_add(1);
                let gen_ = self.generations[i];
                let id = DeviceId::new(i, gen_);

                device.id = id;
                *slot = Some(device);
                return Some(id);
            }
        }
        None
    }

    /// Unregisters a device by its ID.
    ///
    /// Returns `true` if the device was found and removed, `false` otherwise.
    fn unregister(&mut self, id: DeviceId) -> bool {
        let idx = id.index();
        if idx >= MAX_DEVICES {
            return false;
        }

        if let Some(device) = &self.devices[idx]
        && device.id.generation() == id.generation() {
            self.devices[idx] = None;
            return true;
        }
        false
    }

    /// Looks up a device by its ID.
    ///
    /// Returns `Some(&Device)` if the ID is valid and the generation matches,
    /// otherwise `None`.
    fn get_device(&self, id: DeviceId) -> Option<&Device> {
        let idx = id.index();
        if idx >= MAX_DEVICES {
            return None;
        }

        if let Some(device) = &self.devices[idx]
        && device.id.generation() == id.generation() {
            return Some(device);
        }
        None
    }
}

/// The global device registry, protected by a `Nutex` (interrupt‑disabling
/// spinlock) for safe concurrent access from multiple CPUs.
static REGISTRY: Nutex<Registry> = Nutex::new(Registry::new());

// ============================================================================
// PUBLIC API
// ============================================================================

/// Initializes the device model.
///
/// Currently a no‑op, but called from `main()` to mark the subsystem as ready.
pub fn init() {
    info!("Device model initialized");
}

/// Registers a new device with the system.
///
/// The device is inserted into the global registry and assigned a unique
/// `DeviceId`. The device's `id` field is updated to reflect the assigned ID.
///
/// # Returns
/// `Some(id)` on success, `None` if the registry is full.
pub fn register_device(device: Box<Device>) -> Option<DeviceId> {
    REGISTRY.lock().register(device)
}

/// Unregisters a device by its ID.
///
/// # Returns
/// `true` if the device was found and removed, `false` otherwise.
pub fn unregister_device(id: DeviceId) -> bool {
    REGISTRY.lock().unregister(id)
}

/// Sets the driver‑specific data for a device.
///
/// # Returns
/// `true` if the device was found and the data was updated, `false` otherwise.
pub fn set_driver_data(id: DeviceId, data: usize) -> bool {
    let mut guard = REGISTRY.lock();
    if let Some(device) = guard.devices[id.index()].as_mut()
    && device.id.generation() == id.generation() {
        device.driver_data = data;
        return true;
    }
    false
}

/// Retrieves the driver‑specific data for a device.
///
/// # Returns
/// `Some(data)` if the device exists, `None` otherwise.
pub fn get_driver_data(id: DeviceId) -> Option<usize> {
    let guard = REGISTRY.lock();
    guard.get_device(id).map(|dev| dev.driver_data)
}

/// Invokes a method on a device.
///
/// This function looks up the device and method, then calls the method with
/// the provided argument. The registry lock is released before the method call
/// to avoid deadlocks if the method itself tries to access the registry.
///
/// # Returns
/// A `DeviceResult` containing the method's return value and status.
pub fn call_method(id: DeviceId, method_id: MethodId, arg: usize) -> DeviceResult {
    let guard = REGISTRY.lock();

    // Look up the device
    let device = match guard.get_device(id) {
        Some(dev) => dev,
        None => return DeviceResult::err(DeviceStatus::NOT_FOUND),
    };

    // Look up the method
    let method = match device.get_method(method_id) {
        Some(m) => m,
        None => return DeviceResult::err(DeviceStatus::UNSUPPORTED),
    };

    // Release the lock before calling the method
    drop(guard);

    // Invoke the method
    method(id, arg)
}

```

### `src/kmsg/dev.rs`

```rs
use crate::{kmsg::{Sink, SinkAttrs, SinkIdent}, sync::Nutex};

// cargo check: false positive
#[allow(unused)]
pub struct Dev;

impl Dev
{
    // cargo check: false positive
    #[allow(unused)]
    pub fn new() -> Self
    {
        unsafe
        {
            x86::io::outb(0x3f8 + 1, 0  );
            x86::io::outb(0x3f8 + 3, 128);
            x86::io::outb(0x3f8    , 1  );
            x86::io::outb(0x3f8 + 1, 0  );
            x86::io::outb(0x3f8 + 3, 3  );
            x86::io::outb(0x3f8 + 2, 7  );
            x86::io::outb(0x3f8 + 4, 3  );
            let _ = x86::io::inb(0x3f8 + 5);
        }
        Self
    }
}

unsafe impl Sync for Dev {}

// cargo check: false positive
#[allow(unused)]
static ID: u32 = super::str4_to_u32("DEV0");

pub static LOCK: Nutex<()> = Nutex::new(());

impl Sink for Dev
{
    fn kind(&self) -> SinkIdent
    {
        SinkIdent
        {
            attrs: SinkAttrs::Port | SinkAttrs::Critical | SinkAttrs::Pretty,
            kind: ID
        }
    }

    fn write(&self, s: &str)
    {
        let _g = LOCK.lock();

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

        drop(_g);
    }
}

lazy_static! {
    static ref _SINK: Dev = Dev::new();
    pub static ref SINK: &'static Dev = &_SINK;
}

```

### `src/rt/mod.rs`

```rs
mod entry;
mod panic;
pub mod gall;

```

### `src/rt/entry.rs`

```rs
#[unsafe(no_mangle)]
extern "C" fn _start() -> !
{
    crate::kmsg::init(); crate::main()
}

```

### `src/rt/gall.rs`

```rs
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicPtr, Ordering};
use core::ptr::addr_of_mut;

pub trait Gall: Sync {
    fn alloc(&self, l: Layout) -> *mut u8;
    fn free(&self, l: Layout, ptr: *mut u8);
}

struct Fake;
unsafe impl Sync for Fake {}
impl Gall for Fake {
    fn alloc(&self, l: Layout) -> *mut u8 {
        error!(
            "Heap unavailable. Requested {} bytes, {}-aligned.",
            l.size(), l.align()
        );
        core::ptr::null_mut::<u8>()
    }
    fn free(&self, l: Layout, ptr: *mut u8) {
        error!(
            "Heap unavailable. Memory leak of size {} at {:#X}, {}-aligned.",
            l.size(), ptr as usize, l.align()
        );
    }
}

static mut FAKE_ITSELF: Fake = Fake;
#[allow(static_mut_refs)]
static mut FAKE_REF: &'static mut dyn Gall = unsafe { &mut FAKE_ITSELF };

struct SoaBackend;
unsafe impl Sync for SoaBackend {}
impl Gall for SoaBackend {
    fn alloc(&self, l: Layout) -> *mut u8 {
        crate::mem::soa::alloc(l)
    }
    fn free(&self, l: Layout, ptr: *mut u8) {
        crate::mem::soa::free(ptr, l)
    }
}

static mut SOA_ITSELF: SoaBackend = SoaBackend;
#[allow(static_mut_refs)]
static mut SOA_REF: &'static mut dyn Gall = unsafe { &mut SOA_ITSELF };

static GALL: AtomicPtr<&'static mut dyn Gall> = AtomicPtr::new(addr_of_mut!(FAKE_REF));

struct GallMuxer;
unsafe impl GlobalAlloc for GallMuxer {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let this = unsafe { GALL.load(Ordering::Acquire).as_ref_unchecked() };
        this.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let this = unsafe { GALL.load(Ordering::Acquire).as_ref_unchecked() };
        this.free(layout, ptr)
    }
}

#[global_allocator]
static MUXER: GallMuxer = GallMuxer;

pub fn set_soa() {
    let new_ptr: *mut &'static mut dyn Gall = addr_of_mut!(SOA_REF);
    match GALL.compare_exchange(
        addr_of_mut!(FAKE_REF),
        new_ptr,
        Ordering::SeqCst,
        Ordering::Relaxed,
    ) {
        Ok(_) => {}
        Err(_) => panic!("Global allocator already set"),
    }
    info!("gall: switched to SOA backend");
}

```

### `src/rt/panic.rs`

```rs
use core::fmt::Write as _;

use crate::{kmsg, sync::Nutex};

static mut PANIC_BUF: heapless::String<64> = heapless::String::<64>::new();

#[inline(never)]
fn print_stack_trace() {
    let mut bp: usize;
    unsafe {
        core::arch::asm!("mov {}, rbp", out(reg) bp);
    }

    let mut frame_ptr = bp;
    let mut count = 0;

    unsafe { kmsg::str_log_noblock(
        kmsg::AttLvl::Error,
        "",
        file!(),
        line!(),
        "Stack trace:"
    ) };

    unsafe { kmsg::str_log_noblock(
        kmsg::AttLvl::Error,
        "",
        file!(),
        line!(),
        "$$ST:BEGIN$$"
    ) };

    while frame_ptr != 0 && count < 32 {
        let ret_addr = unsafe { *(frame_ptr as *const usize).add(1) };

        let mut msg = heapless::String::<32>::new();
        let _ = msg.write_fmt(format_args!("  #{:02} 0x{:016X}", count, ret_addr));
        unsafe { kmsg::str_log_noblock(
            kmsg::AttLvl::Error,
            "",
            file!(),
            line!(),
            msg.as_str()
        ) };

        frame_ptr = unsafe { *(frame_ptr as *const usize) };
        count += 1;
    }

    unsafe { kmsg::str_log_noblock(
        kmsg::AttLvl::Error,
        "",
        file!(),
        line!(),
        "$$ST:END$$"
    ) };
}

static PANIC_LOCK: Nutex<()> = Nutex::new(());

#[panic_handler]
#[allow(static_mut_refs)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _g1 = PANIC_LOCK.lock();
    let _g2 = kmsg::SINKS.lock();
    let loc = *info.location().unwrap();
    let line = loc.line();
    let _ = unsafe { PANIC_BUF.write_str(loc.file()) };
    let file = unsafe { PANIC_BUF.as_str() };

    let mut s = heapless::String::<256>::new();

    let _ = s.write_fmt(format_args!("{}", info.message()));

    unsafe { kmsg::str_log_noblock(
        kmsg::AttLvl::Panic,
        "",
        file,
        line,
        &s,
    ) };

    print_stack_trace();

    drop(_g1);
    drop(_g2);

    crate::sched::exit(-1);
}

```

### `src/mem/ema.rs`

```rs
use crate::{mem::pmr::Kind, sync::Nutex};

use super::{pmr::{self, Region}, kdm::Paddr};

pub struct EarlyMemAlloc
{
    pub(super) top     : usize,
    pub(super) bottom  : usize,
    pub(super) limit   : usize,
}

impl EarlyMemAlloc
{
    pub(super) fn new() -> Self
    {
        let mut largest: Region = Region::default();

        for region in pmr::iter()
        {
            if region.kind == Kind::USABLE && region.len > largest.len
            {
                largest = region;
            }
        }

        if largest == Region::default()
        {
            panic!("Can't initialize EMA: no usable memory")
        }

        let bottom = (largest.base + largest.len) & !0xfff;
        let top = bottom;
        let limit = (largest.base + 4095) & !0xfff;

        info!("EMA initialized.");
        // debug!("~ top      = {:#X}", top   );
        // debug!("~ bottom   = {:#X}", bottom);
        // debug!("~ limit    = {:#X}", limit );

        Self { top, bottom, limit }
    }

    #[inline]
    pub(super) fn touch(&self) {}

    pub(super) fn alloc(&mut self, count: usize) -> usize {
        let count = (count + 4095) & !4095;
        if self.top < self.limit || self.top - count < self.limit {
            error!("EMA: out of memory (requested {} bytes)", count);
            return 0;
        }
        self.top -= count;
        self.top
    }
    
    pub fn allocated_range(&self) -> (usize, usize) {
        if self.top >= self.bottom {
            (0, 0)
        } else {
            (self.top, self.bottom)
        }
    }
}

lazy_static! {
    pub static ref EMA: Nutex<EarlyMemAlloc> = Nutex::new(EarlyMemAlloc::new());
}

pub fn init()
{
    EMA.lock().touch();
}

pub fn alloc(count: usize) -> Paddr
{
    Paddr::from_raw(EMA.lock().alloc(count << 12))
}

pub fn usage() -> usize
{
    let ema = EMA.lock();
    (ema.bottom - ema.top) >> 12
}

pub fn get_allocated_range() -> (usize, usize) {
    EMA.lock().allocated_range()
}

```

### `src/mem/kdm.rs`

```rs
use heapless::Vec;
use crate::mem::pmr::{self, Region, Kind};

limine! { pub HHDMR <= HhdmRequest }

lazy_static! {
    pub static ref HHDM: usize = HHDMR.response().expect("Can't obtain HHDM offset.").offset as usize;
    
    static ref REGIONS: Vec<Region, 128> = {
        let mut regions = Vec::new();
        
        for region in pmr::iter() {
            match region.kind {
                Kind::USABLE | 
                Kind::KERNEL | 
                Kind::BOOTLOADER |
                Kind::FRAMEBUF |
                Kind::ACPI |
                Kind::ACPI_NVS |
                Kind::RESERVED => {
                    let _ = regions.push(region);
                }
                _ => continue,
            }
        }
        
        info!("KDM: Initialized. HHDM offset: {:#X}", *HHDM);
        
        regions
    };
}

pub fn init() {
    let _ = &*HHDM;
    let _ = &*REGIONS;
}

#[inline]
pub fn regions() -> &'static [Region] {
    &REGIONS
}

#[inline]
pub fn region_count() -> usize {
    REGIONS.len()
}

#[inline]
pub fn is_mapped(paddr: usize) -> bool {
    for region in REGIONS.iter() {
        if paddr >= region.base && paddr < region.base + region.len {
            return true;
        }
    }
    false
}

#[inline]
pub fn is_range_mapped(paddr: usize, size: usize) -> bool {
    let end = paddr + size;
    for region in REGIONS.iter() {
        let region_end = region.base + region.len;
        if paddr >= region.base && end <= region_end {
            return true;
        }
    }
    false
}

pub fn find_region(paddr: usize) -> Option<Region> {
    for region in REGIONS.iter() {
        if paddr >= region.base && paddr < region.base + region.len {
            return Some(*region);
        }
    }
    None
}

#[derive(Clone, Copy, Debug)]
pub struct Paddr(usize);

#[derive(Clone, Copy, Debug)]
pub struct Vaddr(usize);

impl Paddr {
    #[inline(always)]
    pub const fn from_raw(r: usize) -> Self {
        Self(r)
    }

    #[inline(always)]
    pub const fn to_raw(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn to_virt(self) -> Vaddr {
        Vaddr(self.0 + *HHDM)
    }

    #[inline]
    pub fn try_to_virt(self) -> Option<Vaddr> {
        if is_mapped(self.0) {
            Some(Vaddr(self.0 + *HHDM))
        } else {
            None
        }
    }

    #[inline]
    pub fn is_mapped(self) -> bool {
        is_mapped(self.0)
    }
}

impl Vaddr {
    #[inline(always)]
    pub const fn from_raw(r: usize) -> Self {
        Self(r)
    }

    #[inline(always)]
    pub fn from_ptr<T>(ptr: *const T) -> Self {
        Self::from_raw(ptr as usize)
    }

    #[inline(always)]
    pub fn from_ptr_mut<T>(ptr: *mut T) -> Self {
        Self::from_raw(ptr as usize)
    }

    #[inline(always)]
    pub fn from_ref<T>(r: &'_ T) -> Self {
        Self::from_ptr(r)
    }

    #[inline(always)]
    pub fn from_ref_mut<T>(r: &'_ mut T) -> Self {
        Self::from_ptr_mut(r)
    }

    #[inline(always)]
    pub fn to_raw(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn to_phys(self) -> Paddr {
        Paddr(self.0 - *HHDM)
    }

    #[inline(always)]
    pub const fn to_ptr<T>(self) -> *const T {
        self.0 as *const T
    }

    #[inline(always)]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_ptr_mut<T>(self) -> *mut T {
        self.0 as *mut T
    }

    #[inline(always)]
    pub const fn to_ref<'a, T>(self) -> &'a T {
        unsafe {
            self.to_ptr::<T>().as_ref_unchecked()
        }
    }

    #[inline(always)]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_ref_mut<'a, T>(self) -> &'a mut T {
        unsafe {
            self.to_ptr_mut::<T>().as_mut_unchecked()
        }
    }

    #[inline]
    pub fn is_in_hhdm(self) -> bool {
        self.0 >= *HHDM
    }
}

```

### `src/mem/upa.rs`

```rs
use core::ptr::addr_of;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::mem::kdm::Paddr;
use crate::mem::ema;

struct Backend {
    alloc: fn(usize) -> Paddr,
    free: fn(Paddr),
}

fn free_stub(_p: Paddr) {
    warn!("upa::free called before migration – memory leak");
}

static EARLY_BACKEND: Backend = Backend {
    alloc: ema::alloc,
    free: free_stub,
};

static LATE_BACKEND: Backend = Backend {
    alloc: crate::mem::bsa::alloc,
    free: crate::mem::bsa::free,
};

static CURRENT_BACKEND: AtomicPtr<Backend> =
    AtomicPtr::new(addr_of!(EARLY_BACKEND) as *mut Backend);

pub fn alloc(count: usize) -> Paddr {
    if count == 0 {
        return Paddr::from_raw(0)
    }
    let backend = unsafe { &*CURRENT_BACKEND.load(Ordering::Acquire) };
    (backend.alloc)(count)
}

pub fn free(p: Paddr) {
    if p.to_raw() == 0 {
        return
    }
    let backend = unsafe { &*CURRENT_BACKEND.load(Ordering::Acquire) };
    (backend.free)(p)
}

pub fn migrate() {
    let expected = addr_of!(EARLY_BACKEND) as *mut Backend;
    let new = addr_of!(LATE_BACKEND) as *mut Backend;

    if CURRENT_BACKEND
        .compare_exchange(expected, new, Ordering::SeqCst, Ordering::Relaxed)
        .is_err()
    {
        panic!("UPA already migrated");
    }

    info!("UPA migrated to BSA backend");
}

```

### `src/mem/pmr.rs`

```rs
//! # Physical Memory Regions (PMR)
//!
//! This module provides an interface to the physical memory map provided by the
//! bootloader (Limine). It enumerates all memory regions, their types, base addresses,
//! and lengths, and allows iteration over them.
//!
//! ## Overview
//!
//! The PMR module is responsible for:
//!
//! - Parsing the Limine memory map response.
//! - Categorizing regions by their `Kind` (usable, reserved, ACPI, kernel, etc.).
//! - Providing an iterator over all regions (`pmr::iter()`).
//! - Allowing random access to regions by index (`pmr::nth`, `pmr::nth_unchecked`).
//!
//! ## Memory Region Kinds
//!
//! The `Kind` enum mirrors the Limine memory map entry types:
//!
//! - `USABLE` – Normal RAM that can be used by the kernel.
//! - `RESERVED` – Reserved for hardware or firmware; do not use.
//! - `ACPI` – ACPI reclaimable memory.
//! - `ACPI_NVS` – ACPI NVS memory (non‑volatile storage).
//! - `BAD` – Memory with errors; should be avoided.
//! - `BOOTLOADER` – Bootloader‑reserved memory (may be usable after boot).
//! - `KERNEL` – Memory occupied by the kernel image.
//! - `FRAMEBUF` – Framebuffer memory (mapped to the display).
//! - `MAPRESERVED` – Reserved for memory‑mapped I/O.
//!
//! ## Usage
//!
//! The PMR module is used early in the boot process by the memory management
//! subsystem to discover available physical memory for the early allocator (EMA),
//! the page frame manager (PFM), and the buddy allocator (BSA).
//!
//! Example:
//! ```ignore
//! for region in pmr::iter() {
//!     if region.kind == pmr::Kind::USABLE {
//!         // Use this region for memory allocation
//!     }
//! }
//! ```
//!
//! ## Lazy Initialization
//!
//! The memory map is stored in a `lazy_static` (`MMAP`) which fetches the Limine
//! response when first accessed. This ensures that the response is available
//! before any PMR functions are called.
//!
//! ## Safety
//!
//! - The Limine memory map is guaranteed to be valid by the bootloader.
//! - All functions are safe; they just read the static data.
//! - The iterator does not perform bounds checks on `MMAP`; it relies on the
//!   length provided by Limine.

// ============================================================================
// TYPES
// ============================================================================

extrum! {
    /// Physical memory region type.
    ///
    /// This enum corresponds to the `type` field of Limine memory map entries.
    /// The numeric values match the Limine specification.
    #[derive(Clone, Copy, PartialEq, Default)]
    pub enum Kind: u64 {
        USABLE      = 0,
        RESERVED    = 1,
        ACPI        = 2,
        ACPI_NVS    = 3,
        BAD         = 4,
        BOOTLOADER  = 5,
        KERNEL      = 6,
        FRAMEBUF    = 7,
        MAPRESERVED = 8,
    }
}

implement_display![Kind];

/// A physical memory region.
///
/// Represents a contiguous range of physical memory with a given type.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Region {
    /// Base physical address of the region (in bytes).
    pub base: usize,
    /// Length of the region (in bytes).
    pub len: usize,
    /// Type of the region (usable, reserved, etc.).
    pub kind: Kind,
}

/// Iterator over physical memory regions.
///
/// This struct is returned by `pmr::iter()` and yields `Region` items.
pub struct Iter {
    next: usize,
}

// ============================================================================
// LIMINE REQUEST & GLOBAL STATE
// ============================================================================

// Limine request for the memory map.
//
// This is a static request that the bootloader fills with the memory map.
// The response is accessed via `MEMMAP.response()`.
limine! { pub MEMMAP <= MemmapRequest }

// Lazy‑initialized reference to the Limine memory map entries.
//
// The map is a slice of Limine `Entry` structs, each containing base, length,
// and type information.
lazy_static! {
    static ref MMAP: &'static [&'static limine::memmap::Entry] =
        MEMMAP.response().expect("Can't obtain memory regions info.").entries();
}

// ============================================================================
// PUBLIC API
// ============================================================================

/// Returns the memory region at the given index, if it exists.
///
/// # Arguments
/// * `n` – The index of the region.
///
/// # Returns
/// `Some(Region)` if the index is valid, otherwise `None`.
pub fn nth(n: usize) -> Option<Region> {
    let o = MMAP.get(n);
    o.map(
        |e| Region {
            base: e.base as usize,
            len: e.length as usize,
            kind: Kind(e.type_),
        }
    )
}

/// Returns the memory region at the given index without bounds checking.
///
/// # Safety
/// The caller must ensure that `n < pmr::len()`.
///
/// # Arguments
/// * `n` – The index of the region.
///
/// # Returns
/// A `Region` struct.
pub fn nth_unchecked(n: usize) -> Region {
    let e = MMAP[n];
    Region {
        base: e.base as usize,
        len: e.length as usize,
        kind: Kind(e.type_),
    }
}

/// Returns an iterator over all physical memory regions.
///
/// The iterator yields `Region` structs in the order provided by Limine.
pub fn iter() -> Iter {
    Iter::new()
}

/// Returns the total number of memory regions.
pub fn len() -> usize {
    MMAP.len()
}

/// Dumps all memory regions to the log (for debugging).
///
/// This function logs each region's base, size, and type at the `debug` level.
pub fn dump() {
    debug!("Memory regions:");
    for r in iter() {
        #[cfg(feature = "lowlog")] let _ = r;
        debug!("~ base {:-12X} of {:>12} KiB, {:<16}", r.base, (r.len + 1023) >> 10, r.kind);
    }
}

// ============================================================================
// ITERATOR IMPLEMENTATION
// ============================================================================

impl Iter {
    /// Creates a new iterator starting at index 0.
    pub(super) const fn new() -> Self {
        Self { next: 0 }
    }

    /// Advances the iterator and returns the next region, if any.
    ///
    /// This method is used internally by the `Iterator` trait implementation.
    pub fn next(&mut self) -> Option<Region> {
        if self.next < MMAP.len() {
            let e = MMAP[self.next];
            self.next += 1;
            Some(Region {
                base: e.base as usize,
                len: e.length as usize,
                kind: Kind(e.type_),
            })
        } else {
            self.next = 0;
            None
        }
    }
}

impl Iterator for Iter {
    type Item = Region;

    /// Returns the next region.
    fn next(&mut self) -> Option<Self::Item> {
        Iter::next(self)
    }
}

```

### `src/mem/pfm.rs`

```rs
//! # Page Frame Manager (PFM) – SPARSEMEM Physical Memory Metadata
//!
//! This module implements the **SPARSEMEM** model for managing physical page frame metadata.
//! It provides a way to track the state of each physical 4 KiB page frame in the system,
//! including its allocation status, order (for buddy allocation), and additional
//! per‑page information.
//!
//! ## Overview
//!
//! The kernel manages physical memory in 4 KiB pages. For each page frame, we need
//! to store metadata such as:
//! - Whether the page is free, allocated, or reserved.
//! - The order of the page (for buddy allocator).
//! - A reference count (for shared pages).
//! - Private data (used by the allocator for free list linking).
//!
//! Instead of allocating a large contiguous array for all page frames (which could be
//! enormous on systems with 64+ GiB of RAM), we use the **SPARSEMEM** model:
//! - Memory is divided into **sections** (each 16 MiB, i.e., 4096 pages).
//! - We allocate a page frame metadata array for each section only when that section
//!   contains usable memory.
//! - A top‑level array of pointers (`SECTIONS`) maps section indices to the per‑section
//!   metadata arrays.
//!
//! This approach reduces memory overhead for sparse memory layouts (e.g., NUMA systems
//! with large holes) and allows the metadata to be allocated on demand.
//!
//! ## Structure
//!
//! - **`Page`**: The metadata structure for a single 4 KiB page frame. It contains
//!   atomic fields for flags, order, count, and private data. The fields are atomic
//!   to allow lock‑free updates from multiple CPUs.
//! - **`PageFlags`**: Bit flags indicating the page's state (`RESERVED`, `FREE`,
//!   `ALLOCATED`, `BUDDY_HEAD`).
//! - **`PageFrame`**: A convenient wrapper around a physical frame number (PFN) that
//!   provides methods to access the associated `Page` and manipulate its fields.
//!
//! ## Initialization (`pfm::init`)
//!
//! 1. **Determine the maximum PFN**: Iterates over all physical memory regions
//!    (from PMR) to find the highest PFN.
//! 2. **Allocate the section pointer array**: Allocates a contiguous array of
//!    pointers (using EMA) to hold the base address of each section's metadata.
//! 3. **Allocate per‑section metadata**: For each section that overlaps with a
//!    usable memory region, allocate enough pages (from EMA) to store `Page`
//!    structures for all 4096 frames in that section, and initialize them.
//! 4. **Mark page states**: Iterates over all memory regions and sets each page's
//!    flags: `FREE` for usable regions, `RESERVED` for all other types.
//!
//! After initialization, the PFM provides functions to retrieve the `Page` for any
//! physical address or PFN.
//!
//! ## Usage
//!
//! The PFM is used by the buddy allocator (BSA) and other memory management components
//! to query and update page states. For example:
//! - `get_page(pfn)` returns a reference to the page metadata.
//! - `PageFrame::try_alloc()` atomically sets the page's state from `FREE` to `ALLOCATED`.
//! - `PageFrame::order()` returns the buddy order of the page.
//!
//! ## Safety
//!
//! - The module uses `static mut` for `SECTIONS` and `MAX_SECTIONS`, which are
//!   only accessed after initialization and before any other CPU cores are active.
//! - The `Page` fields are atomic, allowing safe concurrent access from multiple
//!   CPUs without locks.
//! - The `get_page` and `get_page_ptr` functions perform bounds and null checks
//!   before returning a pointer, ensuring memory safety.
//! - The `PageFrame` methods use `unsafe` internally to dereference pointers, but
//!   they are safe wrappers that validate the existence of the page.

use core::{
    ptr,
    sync::atomic::{AtomicU8, AtomicU32, Ordering},
};

use crate::mem::pmr::{self, Kind};

// ============================================================================
// PAGE FLAGS
// ============================================================================

bitflags! {
    /// Flags describing the state of a physical page frame.
    ///
    /// These are stored in the `flags` field of `Page` and are updated atomically.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PageFlags: u32 {
        /// The page is reserved (e.g., for firmware, ACPI, or kernel image).
        const RESERVED   = 1 << 0;
        /// The page is free and available for allocation.
        const FREE       = 1 << 1;
        /// The page is currently allocated.
        const ALLOCATED  = 1 << 2;
        /// This page is the head of a buddy block (used by the buddy allocator).
        const BUDDY_HEAD = 1 << 3;
    }
}

// ============================================================================
// PAGE METADATA STRUCTURE
// ============================================================================

/// Metadata for a single physical page frame (4 KiB).
///
/// This structure is stored in the per‑section metadata arrays.
/// All fields are atomic to support lock‑free updates from multiple CPUs.
#[derive(Debug)]
#[repr(C)]
pub struct Page {
    /// Page state flags (`PageFlags`), updated atomically.
    pub flags: AtomicU32,
    /// Buddy order (0 for 4 KiB, 1 for 8 KiB, etc.), updated atomically.
    pub order: AtomicU8,
    /// Padding to align the following fields.
    pub _pad: [u8; 3],
    /// Reference count (used for shared pages, e.g., CoW).
    pub count: AtomicU32,
    /// Private data (used by the allocator, e.g., next pointer in free list).
    pub private: AtomicU32,
}

impl Default for Page {
    /// Creates a new `Page` with all flags cleared and fields set to zero.
    fn default() -> Self {
        Self {
            flags: AtomicU32::new(PageFlags::empty().bits()),
            order: AtomicU8::new(0),
            _pad: [0; 3],
            count: AtomicU32::new(0),
            private: AtomicU32::new(0),
        }
    }
}

// ============================================================================
// SPARSEMEM CONSTANTS
// ============================================================================

/// Shift for section size: 24 bits → 16 MiB.
const SECTION_SHIFT: u32 = 24;
/// Size of a section in bytes (16 MiB).
const SECTION_SIZE: usize = 1 << SECTION_SHIFT;
/// Number of 4 KiB pages in one section.
const PAGES_PER_SECTION: usize = SECTION_SIZE / 4096;

// ============================================================================
// GLOBAL STATIC DATA
// ============================================================================

/// Pointer to the array of section pointers.
///
/// Each entry points to the per‑section `Page` array, or is `null` if the section
/// has no usable memory.
///
/// # Safety
/// This is `static mut` and is initialized once during `pfm::init()`.
static mut SECTIONS: *mut *mut Page = ptr::null_mut();

/// The number of sections (i.e., length of the `SECTIONS` array).
///
/// # Safety
/// This is `static mut` and is set during `pfm::init()`.
static mut MAX_SECTIONS: usize = 0;

// ============================================================================
// INITIALIZATION
// ============================================================================

/// Initializes the Page Frame Manager (SPARSEMEM).
///
/// This function must be called once, early in the boot process (on the BSP),
/// before any other memory management components use the PFM.
///
/// # Operations
///
/// 1. **Determine maximum PFN**: Scans all physical memory regions (from PMR)
///    to find the highest page frame number.
/// 2. **Allocate section pointer array**: Uses the Early Memory Allocator (EMA)
///    to allocate a contiguous array of `*mut Page` pointers, one per section.
/// 3. **Allocate per‑section metadata**: For each section that overlaps with a
///    usable memory region, allocates pages (via EMA) to hold `Page` structures
///    for all 4096 frames in that section, and zero‑initializes them.
/// 4. **Initialize page flags**: Iterates over all memory regions and sets each
///    page's flags: `FREE` for usable regions, `RESERVED` for all other types.
///
/// # Panics
/// - If the section pointer array or any per‑section metadata allocation fails
///   (i.e., EMA returns 0).
/// - If there is no usable memory.
///
/// # Notes
/// - This function uses the EMA, which is still active at this point. After
///   `upa::migrate()` is called, the EMA is no longer available.
/// - The PFM must be initialized before the Buddy System Allocator (BSA) or
///   any other allocator that relies on page metadata.
pub fn init() {
    // Calculate the maximum physical frame number (PFN) from all memory regions.
    let mut max_pfn = 0;
    for region in pmr::iter() {
        let end_pfn = (region.base + region.len).div_ceil(4096);
        if end_pfn > max_pfn {
            max_pfn = end_pfn;
        }
    }

    if max_pfn == 0 {
        warn!("PFM: No memory regions found");
        return;
    }

    // Determine the number of sections required.
    let max_sec = max_pfn.div_ceil(PAGES_PER_SECTION);

    // Allocate the section pointer array using EMA.
    let sec_array_bytes = max_sec * size_of::<*mut Page>();
    let sec_array_pages = sec_array_bytes.div_ceil(4096);
    let sec_array_paddr = crate::mem::ema::alloc(sec_array_pages);
    if sec_array_paddr.to_raw() == 0 {
        panic!("PFM: Failed to allocate sections array");
    }

    let sec_array_ptr: *mut *mut Page = sec_array_paddr.to_virt().to_ptr_mut();
    // Initialize all section pointers to null.
    for i in 0..max_sec {
        unsafe {
            ptr::write(sec_array_ptr.add(i), ptr::null_mut());
        }
    }

    unsafe {
        SECTIONS = sec_array_ptr;
        MAX_SECTIONS = max_sec;
    }

    let mut allocated_sections = 0;

    // For each usable region, allocate per‑section metadata.
    for region in pmr::iter() {
        let start_pfn = region.base / 4096;
        let end_pfn = (region.base + region.len).div_ceil(4096);

        let start_sec = start_pfn / PAGES_PER_SECTION;
        let end_sec = end_pfn.div_ceil(PAGES_PER_SECTION);

        for sec in start_sec..end_sec {
            unsafe {
                let current_ptr = *SECTIONS.add(sec);
                if current_ptr.is_null() {
                    // Allocate one or more pages for the section's Page array.
                    let pages_needed_per_section = (size_of::<Page>() * PAGES_PER_SECTION).div_ceil(4096);
                    let paddr = crate::mem::ema::alloc(pages_needed_per_section);
                    if paddr.to_raw() == 0 {
                        panic!("PFM: Failed to allocate section {}", sec);
                    }

                    let ptr: *mut Page = paddr.to_virt().to_ptr_mut();
                    *SECTIONS.add(sec) = ptr;

                    // Initialize all Page structures to default.
                    for i in 0..PAGES_PER_SECTION {
                        ptr::write(ptr.add(i), Page::default());
                    }

                    allocated_sections += 1;
                }
            }
        }
    }

    // Set page flags based on region type.
    for region in pmr::iter() {
        let start_pfn = region.base / 4096;
        let end_pfn = (region.base + region.len).div_ceil(4096);

        for pfn in start_pfn..end_pfn {
            if let Some(page) = get_page(pfn) {
                match region.kind {
                    Kind::USABLE => {
                        page.flags.store(PageFlags::FREE.bits(), Ordering::Release);
                    }
                    _ => {
                        page.flags.store(PageFlags::RESERVED.bits(), Ordering::Release);
                    }
                }
            }
        }
    }

    info!(
        "PFM: Initialized. Max PFN: {}, Sections allocated: {}, Max Sections: {}",
        max_pfn, allocated_sections, max_sec
    );
}

// ============================================================================
// PAGE LOOKUP FUNCTIONS
// ============================================================================

/// Returns a reference to the `Page` metadata for the given PFN, if the page exists.
///
/// # Arguments
/// * `pfn` – The physical frame number.
///
/// # Returns
/// `Some(&'static Page)` if the PFN is valid and the section has been allocated,
/// otherwise `None`.
#[inline(always)]
pub fn get_page(pfn: usize) -> Option<&'static Page> {
    let ptr = get_page_ptr(pfn)?;
    Some(unsafe { &*ptr })
}

/// Returns a raw pointer to the `Page` metadata for the given PFN, if it exists.
///
/// This is a lower‑level version of `get_page` that returns a `*mut Page`
/// instead of a reference. Useful when the caller needs to mutate the page
/// without re‑borrowing.
///
/// # Arguments
/// * `pfn` – The physical frame number.
///
/// # Returns
/// `Some(*mut Page)` if the PFN is valid and the section has been allocated,
/// otherwise `None`.
#[inline(always)]
pub fn get_page_ptr(pfn: usize) -> Option<*mut Page> {
    let paddr = pfn * 4096;
    if !crate::mem::kdm::is_mapped(paddr) {
        return None;
    }
    unsafe {
        let sec = pfn / PAGES_PER_SECTION;
        if sec >= MAX_SECTIONS {
            return None;
        }
        let ptr = *SECTIONS.add(sec);
        if ptr.is_null() {
            return None;
        }
        let idx = pfn % PAGES_PER_SECTION;
        Some(ptr.add(idx))
    }
}

/// Converts a physical address (Paddr) to a PFN.
#[inline(always)]
pub fn paddr_to_pfn(paddr: crate::mem::kdm::Paddr) -> usize {
    paddr.to_raw() / 4096
}

/// Returns the `Page` metadata for the page frame containing the given physical address.
#[inline(always)]
pub fn get_page_by_paddr(paddr: crate::mem::kdm::Paddr) -> Option<&'static Page> {
    get_page(paddr_to_pfn(paddr))
}

/// Returns a raw pointer to the `Page` metadata for the page frame containing
/// the given physical address.
#[inline(always)]
pub fn get_page_ptr_by_paddr(paddr: crate::mem::kdm::Paddr) -> Option<*mut Page> {
    get_page_ptr(paddr_to_pfn(paddr))
}

// ============================================================================
// PAGE FRAME WRAPPER
// ============================================================================

/// A safe wrapper around a physical frame number (PFN) that provides methods
/// to access and manipulate the associated page metadata.
///
/// This struct is intended to be used by the buddy allocator and other
/// memory management components to perform atomic operations on page state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageFrame(usize);

impl PageFrame {
    /// Creates a new `PageFrame` from a PFN.
    #[inline(always)]
    pub const fn new(pfn: usize) -> Self {
        Self(pfn)
    }

    /// Creates a `PageFrame` from a physical address (Paddr).
    #[inline(always)]
    pub const fn from_paddr(paddr: crate::mem::kdm::Paddr) -> Self {
        Self(paddr.to_raw() / 4096)
    }

    /// Creates a `PageFrame` from a virtual address (Vaddr).
    #[inline(always)]
    pub fn from_vaddr(vaddr: crate::mem::kdm::Vaddr) -> Self {
        Self::from_paddr(vaddr.to_phys())
    }

    /// Returns the PFN.
    #[inline(always)]
    pub const fn pfn(self) -> usize {
        self.0
    }

    /// Returns the physical address of this page frame.
    #[inline(always)]
    pub const fn paddr(self) -> crate::mem::kdm::Paddr {
        crate::mem::kdm::Paddr::from_raw(self.0 * 4096)
    }

    /// Returns the virtual address (HHDM mapping) of this page frame.
    #[inline(always)]
    pub fn vaddr(self) -> crate::mem::kdm::Vaddr {
        self.paddr().to_virt()
    }

    /// Returns a reference to the `Page` metadata, if it exists.
    #[inline(always)]
    pub fn page(self) -> Option<&'static Page> {
        get_page(self.0)
    }

    /// Returns a raw pointer to the `Page` metadata, if it exists.
    #[inline(always)]
    pub fn page_ptr(self) -> Option<*mut Page> {
        get_page_ptr(self.0)
    }

    /// Returns `true` if the PFN is valid and has metadata.
    #[inline(always)]
    pub fn is_valid(self) -> bool {
        get_page_ptr(self.0).is_some()
    }

    /// Returns the current page flags.
    #[inline(always)]
    pub fn flags(self) -> PageFlags {
        if let Some(page) = self.page() {
            PageFlags::from_bits_truncate(page.flags.load(Ordering::Acquire))
        } else {
            PageFlags::empty()
        }
    }

    /// Sets the page flags to the given value.
    #[inline(always)]
    pub fn set_flags(self, flags: PageFlags) -> Self {
        if let Some(page) = self.page() {
            page.flags.store(flags.bits(), Ordering::Release);
        }
        self
    }

    /// Atomically ORs the given flags into the page's flags.
    #[inline(always)]
    pub fn flags_or(self, flags: PageFlags) -> Self {
        if let Some(page) = self.page() {
            page.flags.fetch_or(flags.bits(), Ordering::AcqRel);
        }
        self
    }

    /// Atomically ANDs the page's flags with the given mask (clears other bits).
    #[inline(always)]
    pub fn flags_and(self, flags: PageFlags) -> Self {
        if let Some(page) = self.page() {
            page.flags.fetch_and(flags.bits(), Ordering::AcqRel);
        }
        self
    }

    /// Atomically clears the given flags.
    #[inline(always)]
    pub fn clear_flags(self, flags: PageFlags) -> Self {
        if let Some(page) = self.page() {
            page.flags.fetch_and(!flags.bits(), Ordering::AcqRel);
        }
        self
    }

    /// Returns `true` if the page is free.
    #[inline(always)]
    pub fn is_free(self) -> bool {
        self.flags().contains(PageFlags::FREE)
    }

    /// Returns `true` if the page is allocated.
    #[inline(always)]
    pub fn is_allocated(self) -> bool {
        self.flags().contains(PageFlags::ALLOCATED)
    }

    /// Returns `true` if the page is reserved.
    #[inline(always)]
    pub fn is_reserved(self) -> bool {
        self.flags().contains(PageFlags::RESERVED)
    }

    /// Returns the current reference count.
    #[inline(always)]
    pub fn count(self) -> u32 {
        if let Some(page) = self.page() {
            page.count.load(Ordering::Acquire)
        } else {
            0
        }
    }

    /// Atomically increments the reference count and returns the new value.
    #[inline(always)]
    pub fn inc_count(self) -> u32 {
        if let Some(page) = self.page() {
            page.count.fetch_add(1, Ordering::AcqRel) + 1
        } else {
            0
        }
    }

    /// Atomically decrements the reference count and returns the new value.
    #[inline(always)]
    pub fn dec_count(self) -> u32 {
        if let Some(page) = self.page() {
            page.count.fetch_sub(1, Ordering::AcqRel) - 1
        } else {
            0
        }
    }

    /// Attempts to atomically change the page state from `FREE` to `ALLOCATED`.
    ///
    /// # Returns
    /// `true` if the transition succeeded, `false` if the page was not free.
    #[inline(always)]
    pub fn try_alloc(self) -> bool {
        if let Some(page) = self.page() {
            let expected = PageFlags::FREE.bits();
            let desired = PageFlags::ALLOCATED.bits();
            page.flags
                .compare_exchange(expected, desired, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
        } else {
            false
        }
    }

    /// Attempts to atomically change the page state from `ALLOCATED` to `FREE`.
    ///
    /// # Returns
    /// `true` if the transition succeeded, `false` if the page was not allocated.
    #[inline(always)]
    pub fn try_free(self) -> bool {
        if let Some(page) = self.page() {
            let expected = PageFlags::ALLOCATED.bits();
            let desired = PageFlags::FREE.bits();
            page.flags
                .compare_exchange(expected, desired, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
        } else {
            false
        }
    }

    /// Returns the buddy order of this page (used by the buddy allocator).
    #[inline(always)]
    pub fn order(self) -> u8 {
        if let Some(page) = self.page() {
            page.order.load(Ordering::Relaxed)
        } else {
            0
        }
    }

    /// Sets the buddy order of this page.
    ///
    /// # Safety
    /// This is an atomic store but does not check any invariants. The caller
    /// must ensure the order is consistent with the page's state and the buddy
    /// allocator's logic.
    #[inline(always)]
    pub unsafe fn set_order(self, order: u8) -> Self {
        if let Some(page_ptr) = self.page_ptr() {
            (unsafe { &*page_ptr }).order.store(order, Ordering::Relaxed);
        }
        self
    }

    /// Returns the private field (used by the allocator for free list linking).
    #[inline(always)]
    pub fn private(self) -> u32 {
        if let Some(page) = self.page() {
            page.private.load(Ordering::Relaxed)
        } else {
            0
        }
    }

    /// Sets the private field.
    ///
    /// # Safety
    /// The caller must ensure that the value is valid and consistent with the
    /// allocator's state.
    #[inline(always)]
    pub unsafe fn set_private(self, private: u32) -> Self {
        if let Some(page_ptr) = self.page_ptr() {
            (unsafe { &*page_ptr }).private.store(private, Ordering::Relaxed);
        }
        self
    }
}

```

### `src/mem/bsa.rs`

```rs
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem::MaybeUninit;
use crate::mem::{pfm, pmr, kdm::Paddr, ema};
use crate::sync::{Nutex, Nitex};
use crate::arch;
use heapless::Vec;

pub const MAX_ORDER: usize = 10;

const PCP_SIZE: usize = 32;

const DMA_END: usize = 16 * 1024 * 1024;      // 16 MiB
const DMA32_END: usize = 4 * 1024 * 1024 * 1024; // 4 GiB

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Dma,
    Dma32,
    Normal,
}

impl Zone {
    #[inline]
    pub fn from_pfn(pfn: usize) -> Self {
        let paddr = pfn * 4096;
        if paddr < DMA_END {
            Zone::Dma
        } else if paddr < DMA32_END {
            Zone::Dma32
        } else {
            Zone::Normal
        }
    }

    #[inline]
    pub const fn index(self) -> usize {
        match self {
            Zone::Dma => 0,
            Zone::Dma32 => 1,
            Zone::Normal => 2,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Zone::Dma => "DMA",
            Zone::Dma32 => "DMA32",
            Zone::Normal => "Normal",
        }
    }
}

struct FreeArea {
    head: Option<usize>,
    count: usize,
}

impl FreeArea {
    const fn new() -> Self {
        Self {
            head: None,
            count: 0,
        }
    }

    fn add(&mut self, pfn: usize) {
        if let Some(page) = pfm::get_page(pfn) {
            page.private.store(
                self.head.unwrap_or(0) as u32,
                Ordering::Release
            );
            self.head = Some(pfn);
            self.count += 1;
        }
    }

    fn remove(&mut self) -> Option<usize> {
        if let Some(pfn) = self.head {
            if let Some(page) = pfm::get_page(pfn) {
                let next = page.private.load(Ordering::Acquire) as usize;
                self.head = if next == 0 { None } else { Some(next) };
                self.count -= 1;
                Some(pfn)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

#[repr(align(64))]
struct PerCpuCache {
    pages: [Vec<usize, PCP_SIZE>; MAX_ORDER],
}

impl PerCpuCache {
    const fn new() -> Self {
        Self {
            pages: [
                Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(),
            ],
        }
    }

    fn try_alloc(&mut self, order: usize) -> Option<usize> {
        if order < MAX_ORDER {
            self.pages[order].pop()
        } else {
            None
        }
    }

    fn try_free(&mut self, order: usize, pfn: usize) -> bool {
        if order < MAX_ORDER && !self.pages[order].is_full() {
            let _ = self.pages[order].push(pfn);
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn is_empty(&self, order: usize) -> bool {
        order >= MAX_ORDER || self.pages[order].is_empty()
    }

    #[allow(dead_code)]
    fn is_full(&self, order: usize) -> bool {
        order < MAX_ORDER && self.pages[order].is_full()
    }

    #[allow(dead_code)]
    fn clear(&mut self, order: usize) {
        if order < MAX_ORDER {
            self.pages[order].clear();
        }
    }
}

struct ZoneInner {
    free_areas: [FreeArea; MAX_ORDER],
    start_pfn: usize,
    end_pfn: usize,
}

impl ZoneInner {
    const fn new(start_pfn: usize, end_pfn: usize) -> Self {
        Self {
            free_areas: [ const { FreeArea::new() }; MAX_ORDER ],
            start_pfn,
            end_pfn,
        }
    }
}

struct ZoneData {
    lock: Nutex<ZoneInner>,
    pcp: [MaybeUninit<Nitex<PerCpuCache>>; arch::MAX_CPUS],
    free_pages: AtomicUsize,
    pcp_pages: AtomicUsize,
}

impl ZoneData {
    const fn new(start_pfn: usize, end_pfn: usize) -> Self {
        Self {
            lock: Nutex::new(ZoneInner::new(start_pfn, end_pfn)),
            pcp: [ const { MaybeUninit::uninit() }; arch::MAX_CPUS ],
            free_pages: AtomicUsize::new(0),
            pcp_pages: AtomicUsize::new(0),
        }
    }

    #[inline]
    unsafe fn pcp(&self, cpu_id: usize) -> &Nitex<PerCpuCache> {
        unsafe { self.pcp[cpu_id].assume_init_ref() }
    }

    fn init_pcp(&self) {
        for i in 0..arch::MAX_CPUS {
            unsafe {
                let ptr = self.pcp[i].as_ptr();
                *(ptr as *mut Nitex<PerCpuCache>).as_mut_unchecked() = Nitex::new(PerCpuCache::new());
            }
        }
    }
}

static ZONES: [ZoneData; 3] = [
    ZoneData::new(0, 0),
    ZoneData::new(0, 0),
    ZoneData::new(0, 0),
];

#[inline]
fn log2_ceil(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let mut order = 0;
    let mut size = 1;
    while size < n {
        size <<= 1;
        order += 1;
    }
    order
}

pub fn init() {
    info!("Initializing BSA");

    for zone in &ZONES {
        zone.init_pcp();
    }

    let (ema_start, ema_end) = ema::get_allocated_range();
    let ema_start_pfn = ema_start / 4096;
    let ema_end_pfn = ema_end.div_ceil(4096);

    let mut zone_boundaries = [(0usize, 0usize); 3];

    let mut max_pfn = 0;
    for region in pmr::iter() {
        let end_pfn = (region.base + region.len) / 4096;
        if end_pfn > max_pfn {
            max_pfn = end_pfn;
        }
    }

    let dma_end_pfn = DMA_END / 4096;
    let dma32_end_pfn = DMA32_END / 4096;

    zone_boundaries[0] = (0, core::cmp::min(max_pfn, dma_end_pfn));
    zone_boundaries[1] = (
        core::cmp::min(max_pfn, dma_end_pfn),
        core::cmp::min(max_pfn, dma32_end_pfn)
    );
    zone_boundaries[2] = (core::cmp::min(max_pfn, dma32_end_pfn), max_pfn);

    for (i, &(start, end)) in zone_boundaries.iter().enumerate() {
        let zone = &ZONES[i];
        let mut inner = zone.lock.lock();
        inner.start_pfn = start;
        inner.end_pfn = end;
        drop(inner);

        let zone_name = match i {
            0 => "DMA",
            1 => "DMA32",
            2 => "Normal",
            _ => "Unknown",
        };

        info!(
            "Zone {} initialized: PFN {} - {} ({} pages)",
            zone_name,
            start,
            end,
            end.saturating_sub(start)
        );
    }

    for region in pmr::iter() {
        let start_pfn = region.base / 4096;
        let end_pfn = (region.base + region.len) / 4096;

        for pfn in start_pfn..end_pfn {
            if let Some(page) = pfm::get_page(pfn) {
                page.flags.store(
                    pfm::PageFlags::ALLOCATED.bits(),
                    Ordering::Release
                );
            }
        }
    }

    if ema_end > ema_start {
        for pfn in ema_start_pfn..ema_end_pfn {
            if let Some(page) = pfm::get_page(pfn) {
                page.flags.store(
                    pfm::PageFlags::RESERVED.bits(),
                    Ordering::Release
                );
            }
        }
        info!(
            "Marked {} EMA pages as RESERVED (PFN {} - {})",
            ema_end_pfn - ema_start_pfn,
            ema_start_pfn,
            ema_end_pfn
        );
    }

    for region in pmr::iter() {
        if region.kind == pmr::Kind::USABLE {
            let start_pfn = region.base / 4096;
            let end_pfn = (region.base + region.len) / 4096;

            let align_mask = (1 << MAX_ORDER) - 1;
            let aligned_start = (start_pfn + align_mask) & !align_mask;
            let aligned_end = end_pfn & !align_mask;

            let mut pfn = aligned_start;
            while pfn < aligned_end {
                let mut order = MAX_ORDER - 1;
                while order > 0 {
                    let block_size = 1 << order;
                    if pfn.is_multiple_of(block_size) && pfn + block_size <= aligned_end {
                        break;
                    }
                    order -= 1;
                }

                let block_size = 1 << order;
                if pfn + block_size <= aligned_end {
                    for i in 0..block_size {
                        if let Some(page) = pfm::get_page(pfn + i) {
                            page.flags.store(
                                pfm::PageFlags::FREE.bits(),
                                Ordering::Release
                            );
                            if i == 0 {
                                page.flags.fetch_or(pfm::PageFlags::BUDDY_HEAD.bits(), Ordering::Release);
                                page.order.store(order as u8, Ordering::Release);
                            }
                        }
                    }

                    let zone = Zone::from_pfn(pfn);
                    let zone_data = &ZONES[zone.index()];
                    let mut inner = zone_data.lock.lock();
                    inner.free_areas[order].add(pfn);
                    zone_data.free_pages.fetch_add(block_size, Ordering::Release);
                    drop(inner);

                    pfn += block_size;
                } else {
                    break;
                }
            }
        }
    }

    let stats = usage();
    info!(
        "Initialized. Free pages: DMA={}, DMA32={}, Normal={}",
        stats[0], stats[1], stats[2]
    );
}

pub fn alloc(count: usize) -> Paddr {
    if count == 0 {
        return Paddr::from_raw(0);
    }

    let order = log2_ceil(count);
    if order >= MAX_ORDER {
        error!("Allocation too large ({} pages, order {})", count, order);
        return Paddr::from_raw(0);
    }

    let cpu_id = arch::current_cpu();

    for zone_idx in [1, 2, 0] {
        let zone = &ZONES[zone_idx];
        
        {
            let pcp = unsafe { zone.pcp(cpu_id) };
            let mut pcp_guard = pcp.lock();
            if let Some(pfn) = pcp_guard.try_alloc(order) {
                zone.pcp_pages.fetch_sub(1 << order, Ordering::Release);
                return Paddr::from_raw(pfn * 4096);
            }
        }

        if let Some(pfn) = alloc_from_zone(zone_idx, order) {
            refill_pcp(zone_idx, order, cpu_id);
            return Paddr::from_raw(pfn * 4096);
        }
    }

    error!("Out of memory (requested {} pages)", count);
    Paddr::from_raw(0)
}

fn refill_pcp(zone_idx: usize, order: usize, cpu_id: usize) {
    let zone = &ZONES[zone_idx];
    let mut inner = zone.lock.lock();
    let pcp = unsafe { zone.pcp(cpu_id) };
    let mut pcp_guard = pcp.lock();

    let target_count = PCP_SIZE / 2;
    let mut count = 0;

    while count < target_count {
        let mut found_order = order;
        while found_order < MAX_ORDER {
            if !inner.free_areas[found_order].is_empty() {
                break;
            }
            found_order += 1;
        }

        if found_order >= MAX_ORDER {
            break;
        }

        let pfn = match inner.free_areas[found_order].remove() {
            Some(pfn) => pfn,
            None => break,
        };

        while found_order > order {
            found_order -= 1;
            let buddy_pfn = pfn + (1 << found_order);

            for i in 0..(1 << found_order) {
                if let Some(page) = pfm::get_page(buddy_pfn + i) {
                    if i == 0 {
                        page.flags.store(
                            (pfm::PageFlags::FREE | pfm::PageFlags::BUDDY_HEAD).bits(),
                            Ordering::Release,
                        );
                        page.order.store(found_order as u8, Ordering::Release);
                    } else {
                        page.flags.store(pfm::PageFlags::FREE.bits(), Ordering::Release);
                    }
                }
            }

            inner.free_areas[found_order].add(buddy_pfn);
        }

        if pcp_guard.try_free(order, pfn) {
            if let Some(page) = pfm::get_page(pfn) {
                page.flags.store(
                    (pfm::PageFlags::ALLOCATED | pfm::PageFlags::BUDDY_HEAD).bits(),
                    Ordering::Release,
                );
                page.order.store(order as u8, Ordering::Release);
            }
            zone.free_pages.fetch_sub(1 << order, Ordering::Release);
            zone.pcp_pages.fetch_add(1 << order, Ordering::Release);
            count += 1;
        } else {
            if let Some(page) = pfm::get_page(pfn) {
                page.flags.store(
                    (pfm::PageFlags::FREE | pfm::PageFlags::BUDDY_HEAD).bits(),
                    Ordering::Release,
                );
                page.order.store(order as u8, Ordering::Release);
            }
            inner.free_areas[order].add(pfn);
            break;
        }
    }
}

fn free_to_zone(zone_idx: usize, mut pfn: usize, mut order: usize) {
    let zone = &ZONES[zone_idx];
    let mut inner = zone.lock.lock();

    if let Some(page) = pfm::get_page(pfn) {
        page.flags.store(
            (pfm::PageFlags::FREE | pfm::PageFlags::BUDDY_HEAD).bits(),
            Ordering::Release,
        );
        page.order.store(order as u8, Ordering::Release);
    }

    while order < MAX_ORDER - 1 {
        let buddy_pfn = pfn ^ (1 << order);

        if let Some(buddy_page) = pfm::get_page(buddy_pfn) {
            let buddy_flags = pfm::PageFlags::from_bits_truncate(
                buddy_page.flags.load(Ordering::Acquire),
            );
            let buddy_order = buddy_page.order.load(Ordering::Acquire) as usize;

            if buddy_flags.contains(pfm::PageFlags::FREE)
                && buddy_flags.contains(pfm::PageFlags::BUDDY_HEAD)
                && buddy_order == order
            {
                remove_from_free_list(&mut inner.free_areas[order], buddy_pfn);

                if buddy_pfn < pfn {
                    pfn = buddy_pfn;
                }

                order += 1;
                continue;
            }
        }

        break;
    }

    if let Some(page) = pfm::get_page(pfn) {
        page.flags.store(
            (pfm::PageFlags::FREE | pfm::PageFlags::BUDDY_HEAD).bits(),
            Ordering::Release,
        );
        page.order.store(order as u8, Ordering::Release);
    }

    inner.free_areas[order].add(pfn);
    zone.free_pages.fetch_add(1 << order, Ordering::Release);
}

fn remove_from_free_list(free_area: &mut FreeArea, target_pfn: usize) {
    if free_area.head == Some(target_pfn) {
        free_area.remove();
        return;
    }

    let mut current = free_area.head;
    let max_steps = free_area.count + 2;
    let mut steps = 0;

    while let Some(pfn) = current {
        steps += 1;
        if steps > max_steps {
            error!(
                "remove_from_free_list: CYCLE DETECTED! target_pfn={}, cache free_area count={}",
                target_pfn, free_area.count
            );
            return;
        }

        if let Some(page) = pfm::get_page(pfn) {
            let next = page.private.load(Ordering::Acquire) as usize;
            if next == target_pfn {
                if let Some(target_page) = pfm::get_page(target_pfn) {
                    let target_next = target_page.private.load(Ordering::Acquire);
                    page.private.store(target_next, Ordering::Release);
                }
                free_area.count -= 1;
                return;
            }
            current = if next == 0 { None } else { Some(next) };
        } else {
            break;
        }
    }

    warn!(
        "remove_from_free_list: PFN {} not found in free list (searched {} entries)",
        target_pfn, steps
    );
}

fn drain_pcp(zone_idx: usize, order: usize, cpu_id: usize) {
    let zone = &ZONES[zone_idx];
    let mut to_free = Vec::<usize, 32>::new();
    
    {
        let pcp = unsafe { zone.pcp(cpu_id) };
        let mut pcp_guard = pcp.lock();
        let drain_count = pcp_guard.pages[order].len() / 2;
        for _ in 0..drain_count {
            if let Some(pfn) = pcp_guard.pages[order].pop() {
                zone.pcp_pages.fetch_sub(1 << order, Ordering::Release);
                let _ = to_free.push(pfn);
            }
        }
    }
    
    for pfn in to_free {
        free_to_zone(zone_idx, pfn, order);
    }
}

pub fn usage() -> [usize; 3] {
    [
        ZONES[0].free_pages.load(Ordering::Acquire) + ZONES[0].pcp_pages.load(Ordering::Acquire),
        ZONES[1].free_pages.load(Ordering::Acquire) + ZONES[1].pcp_pages.load(Ordering::Acquire),
        ZONES[2].free_pages.load(Ordering::Acquire) + ZONES[2].pcp_pages.load(Ordering::Acquire),
    ]
}

pub fn alloc_from_zone_direct(zone: Zone, count: usize) -> Paddr {
    if count == 0 {
        return Paddr::from_raw(0);
    }

    let order = log2_ceil(count);
    if order >= MAX_ORDER {
        return Paddr::from_raw(0);
    }

    let zone_data = &ZONES[zone.index()];
    let cpu_id = arch::current_cpu();

    {
        let pcp = unsafe { zone_data.pcp(cpu_id) };
        let mut pcp_guard = pcp.lock();
        if let Some(pfn) = pcp_guard.try_alloc(order) {
            zone_data.pcp_pages.fetch_sub(1 << order, Ordering::Release);
            return Paddr::from_raw(pfn * 4096);
        }
    }

    if let Some(pfn) = alloc_from_zone(zone.index(), order) {
        refill_pcp(zone.index(), order, cpu_id);
        Paddr::from_raw(pfn * 4096)
    } else {
        Paddr::from_raw(0)
    }
}

fn alloc_from_zone(zone_idx: usize, order: usize) -> Option<usize> {
    let zone = &ZONES[zone_idx];
    let mut inner = zone.lock.lock();

    let mut found_order = order;
    while found_order < MAX_ORDER {
        if !inner.free_areas[found_order].is_empty() {
            break;
        }
        found_order += 1;
    }

    if found_order >= MAX_ORDER {
        return None;
    }

    let pfn = inner.free_areas[found_order].remove()?;

    while found_order > order {
        found_order -= 1;
        let buddy_pfn = pfn + (1 << found_order);

        for i in 0..(1 << found_order) {
            if let Some(page) = pfm::get_page(buddy_pfn + i) {
                if i == 0 {
                    page.flags.store(
                        (pfm::PageFlags::FREE | pfm::PageFlags::BUDDY_HEAD).bits(),
                        Ordering::Release,
                    );
                    page.order.store(found_order as u8, Ordering::Release);
                } else {
                    page.flags.store(pfm::PageFlags::FREE.bits(), Ordering::Release);
                }
            }
        }

        inner.free_areas[found_order].add(buddy_pfn);
    }

    if let Some(page) = pfm::get_page(pfn) {
        page.flags.store(
            (pfm::PageFlags::ALLOCATED | pfm::PageFlags::BUDDY_HEAD).bits(),
            Ordering::Release,
        );
        page.order.store(order as u8, Ordering::Release);
    }

    let block_size = 1 << order;
    zone.free_pages.fetch_sub(block_size, Ordering::Release);

    drop(inner);

    Some(pfn)
}

pub fn free(paddr: Paddr) {
    let pfn = paddr.to_raw() / 4096;

    if let Some(page) = pfm::get_page(pfn) {
        let order = page.order.load(Ordering::Acquire) as usize;
        let zone = Zone::from_pfn(pfn);
        let zone_data = &ZONES[zone.index()];
        let cpu_id = arch::current_cpu();

        {
            let pcp = unsafe { zone_data.pcp(cpu_id) };
            let mut pcp_guard = pcp.lock();
            if pcp_guard.try_free(order, pfn) {
                page.flags.store(
                    (pfm::PageFlags::ALLOCATED | pfm::PageFlags::BUDDY_HEAD).bits(),
                    Ordering::Release,
                );
                zone_data.pcp_pages.fetch_add(1 << order, Ordering::Release);
                return;
            }
        }

        free_to_zone(zone.index(), pfn, order);
        drain_pcp(zone.index(), order, cpu_id);
    }
}

```

### `src/mem/soa.rs`

```rs
use core::alloc::Layout;
use core::ptr::NonNull;
use core::debug_assert;
use crate::mem::{upa, kdm::Vaddr};
use crate::sync::Nutex;

const PAGE_SIZE: usize = 4096;
const CLASS_SIZES: [usize; 9] = [8, 16, 32, 64, 128, 256, 512, 1024, 2048];

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlabState {
    Partial = 0,
    Full = 1,
}

#[repr(C)]
struct SlabHeader {
    free_count: usize,
    free_head: Option<NonNull<u8>>,
    next: Option<NonNull<SlabHeader>>,
    prev: Option<NonNull<SlabHeader>>,
    state: SlabState,
}

struct SlabClassInner {
    partial_slabs: Option<NonNull<SlabHeader>>,
    full_slabs: Option<NonNull<SlabHeader>>,
}

struct SlabClass {
    size: usize,
    obj_per_slab: usize,
    first_obj_offset: usize,
    inner: Nutex<SlabClassInner>,
}

impl SlabClass {
    const fn new(size: usize) -> Self {
        let header_size = size_of::<SlabHeader>();
        let first_obj_offset = (header_size + 7) & !7;
        
        let obj_per_slab = (PAGE_SIZE - first_obj_offset) / size;
        
        Self {
            size,
            obj_per_slab,
            first_obj_offset,
            inner: Nutex::new(SlabClassInner {
                partial_slabs: None,
                full_slabs: None,
            }),
        }
    }

    fn alloc(&self) -> Option<NonNull<u8>> {
        let mut inner = self.inner.lock();

        if let Some(header_ptr) = inner.partial_slabs {
            let header = unsafe { &mut *header_ptr.as_ptr() };
            debug_assert!(header.state == SlabState::Partial);
            debug_assert!(header.free_count > 0);
            
            let ptr = header.free_head.unwrap();
            header.free_head = unsafe { *ptr.cast::<Option<NonNull<u8>>>().as_ptr() };
            header.free_count -= 1;

            if header.free_count == 0 {
                self.remove_from_list(&mut inner.partial_slabs, header_ptr);
                header.state = SlabState::Full;
                self.add_to_list(&mut inner.full_slabs, header_ptr);
            }
            return Some(ptr);
        }

        let paddr = upa::alloc(1);
        if paddr.to_raw() == 0 {
            return None;
        }

        let vaddr = paddr.to_virt().to_raw() as *mut u8;
        let header_ptr = NonNull::new(vaddr as *mut SlabHeader).unwrap();

        unsafe {
            let header = &mut *header_ptr.as_ptr();
            header.next = None;
            header.prev = None;
            header.state = SlabState::Partial;

            let mut current = vaddr.add(self.first_obj_offset);
            let end = vaddr.add(PAGE_SIZE);
            let mut prev_next: Option<NonNull<u8>> = None;

            while current.add(self.size) <= end {
                let node = NonNull::new(current).unwrap();
                node.cast::<Option<NonNull<u8>>>().as_ptr().write(prev_next);
                prev_next = Some(node);
                current = current.add(self.size);
            }

            header.free_head = prev_next;
            header.free_count = self.obj_per_slab;
        }

        self.add_to_list(&mut inner.partial_slabs, header_ptr);

        let header = unsafe { &mut *header_ptr.as_ptr() };
        debug_assert!(header.free_count > 0);
        
        let ptr = header.free_head.unwrap();
        header.free_head = unsafe { *ptr.cast::<Option<NonNull<u8>>>().as_ptr() };
        header.free_count -= 1;

        if header.free_count == 0 {
            self.remove_from_list(&mut inner.partial_slabs, header_ptr);
            header.state = SlabState::Full;
            self.add_to_list(&mut inner.full_slabs, header_ptr);
        }

        Some(ptr)
    }

    fn free(&self, ptr: NonNull<u8>) {
        let mut inner = self.inner.lock();
        
        let slab_base = (ptr.as_ptr() as usize) & !(PAGE_SIZE - 1);
        let header_ptr = NonNull::new(slab_base as *mut SlabHeader).unwrap();

        unsafe {
            let header = &mut *header_ptr.as_ptr();
            
            ptr.cast::<Option<NonNull<u8>>>().as_ptr().write(header.free_head);
            header.free_head = Some(ptr);
            header.free_count += 1;

            if header.free_count == self.obj_per_slab {
                if header.state == SlabState::Partial {
                    self.remove_from_list(&mut inner.partial_slabs, header_ptr);
                } else {
                    self.remove_from_list(&mut inner.full_slabs, header_ptr);
                }
                
                let paddr = Vaddr::from_raw(slab_base).to_phys();
                upa::free(paddr);
            } else if header.free_count == 1 {
                debug_assert!(header.state == SlabState::Full);
                self.remove_from_list(&mut inner.full_slabs, header_ptr);
                header.state = SlabState::Partial;
                self.add_to_list(&mut inner.partial_slabs, header_ptr);
            }
        }
    }

    fn add_to_list(&self, list: &mut Option<NonNull<SlabHeader>>, node: NonNull<SlabHeader>) {
        unsafe {
            let n = &mut *node.as_ptr();
            n.prev = None;
            n.next = *list;
            if let Some(mut head) = *list {
                head.as_mut().prev = Some(node);
            }
            *list = Some(node);
        }
    }

    fn remove_from_list(&self, list: &mut Option<NonNull<SlabHeader>>, node: NonNull<SlabHeader>) {
        unsafe {
            let n = &mut *node.as_ptr();
            
            let is_in_list = if *list == Some(node) {
                true
            } else if let Some(prev) = n.prev {
                prev.as_ref().next == Some(node)
            } else {
                false
            };

            if !is_in_list {
                return;
            }

            if let Some(mut prev) = n.prev {
                prev.as_mut().next = n.next;
            } else {
                *list = n.next;
            }
            if let Some(mut next) = n.next {
                next.as_mut().prev = n.prev;
            }
            
            n.prev = None;
            n.next = None;
        }
    }
}

pub struct Soa {
    classes: [SlabClass; CLASS_SIZES.len()],
}

impl Soa {
    pub const fn new() -> Self {
        Self {
            classes: [
                SlabClass::new(CLASS_SIZES[0]),
                SlabClass::new(CLASS_SIZES[1]),
                SlabClass::new(CLASS_SIZES[2]),
                SlabClass::new(CLASS_SIZES[3]),
                SlabClass::new(CLASS_SIZES[4]),
                SlabClass::new(CLASS_SIZES[5]),
                SlabClass::new(CLASS_SIZES[6]),
                SlabClass::new(CLASS_SIZES[7]),
                SlabClass::new(CLASS_SIZES[8]),
            ],
        }
    }

    fn find_class(&self, layout: Layout) -> Option<usize> {
        if layout.align() > 8 {
            return None;
        }
        let mut size = layout.size();
        if size == 0 {
            size = 1;
        }
        if size > 2048 {
            return None;
        }
        CLASS_SIZES.iter().position(|&s| s >= size)
    }
}

static mut SOA_INSTANCE: Soa = Soa::new();

pub fn init() {
    info!("SOA: Initialized with immediate shrink ({} classes)", CLASS_SIZES.len());
}

#[allow(static_mut_refs)]
pub fn alloc(layout: Layout) -> *mut u8 {
    let soa = unsafe { &SOA_INSTANCE };

    if let Some(class_idx) = soa.find_class(layout)
    && let Some(ptr) = soa.classes[class_idx].alloc() {
        return ptr.as_ptr();
    }

    let pages = layout.size().div_ceil(4096);
    let paddr = upa::alloc(pages);
    if paddr.to_raw() == 0 {
        return core::ptr::null_mut();
    }
    paddr.to_virt().to_raw() as *mut u8
}

#[allow(static_mut_refs)]
pub fn free(ptr: *mut u8, layout: Layout) {
    if ptr.is_null() {
        return;
    }

    let soa = unsafe { &SOA_INSTANCE };

    if layout.align() > 8 || layout.size() > 2048 {
        let vaddr = Vaddr::from_raw(ptr as usize);
        upa::free(vaddr.to_phys());
    } else {
        if let Some(class_idx) = soa.find_class(layout) {
            let ptr_nn = NonNull::new(ptr).unwrap();
            soa.classes[class_idx].free(ptr_nn);
        } else {
            let vaddr = Vaddr::from_raw(ptr as usize);
            upa::free(vaddr.to_phys());
        }
    }
}

#[allow(static_mut_refs)]
pub fn dump_stats() {
    let soa = unsafe { &SOA_INSTANCE };
    info!("--- SOA Statistics ---");
    for (i, class) in soa.classes.iter().enumerate() {
        let inner = class.inner.lock();
        
        let mut partial_count = 0;
        let mut curr = inner.partial_slabs;
        while let Some(node) = curr {
            partial_count += 1;
            curr = unsafe { node.as_ref().next };
        }
        
        let mut full_count = 0;
        curr = inner.full_slabs;
        while let Some(node) = curr {
            full_count += 1;
            curr = unsafe { node.as_ref().next };
        }

        info!(
            "Class {} ({} bytes): {} partial, {} full (obj/slab: {})",
            i, class.size, partial_count, full_count, class.obj_per_slab
        );
    }
    info!("----------------------");
}

```

### `src/mem/vma.rs`

```rs
use alloc::alloc::{alloc, dealloc, Layout};
use core::ptr::NonNull;
use crate::sync::Nutex;

pub const MAX_CANONICAL_ADDR: usize = 0x0000_7FFF_FFFF_FFFF;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct VmaFlags: u64 {
        const READ    = 1 << 0;
        const WRITE   = 1 << 1;
        const EXEC    = 1 << 2;
        const ANON    = 1 << 3;
        const FIXED   = 1 << 4;
        const GROWSUP = 1 << 5;
        const GROWSDN = 1 << 6;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

#[repr(C)]
#[derive(PartialEq)]
pub struct VmaNode {
    pub start: usize,
    pub end: usize,
    pub flags: VmaFlags,
    
    parent: Option<NonNull<VmaNode>>,
    left: Option<NonNull<VmaNode>>,
    right: Option<NonNull<VmaNode>>,
    color: Color,
    
    subtree_max_end: usize,
    subtree_min_start: usize,
    subtree_max_gap: usize,
}

impl VmaNode {
    fn new(start: usize, end: usize, flags: VmaFlags) -> Self {
        Self {
            start,
            end,
            flags,
            parent: None,
            left: None,
            right: None,
            color: Color::Red,
            subtree_max_end: end,
            subtree_min_start: start,
            subtree_max_gap: start,
        }
    }
}

#[inline]
fn update_augmentation(node: &mut NonNull<VmaNode>) {
    unsafe {
        let n = node.as_mut();
        let mut max_end = n.end;
        let mut min_start = n.start;
        let mut max_gap: usize;

        let left_max_end = n.left.map_or(0, |l| l.as_ref().subtree_max_end);
        let left_min_start = n.left.map_or(n.start, |l| l.as_ref().subtree_min_start);
        let left_max_gap = n.left.map_or(0, |l| l.as_ref().subtree_max_gap);

        let right_max_end = n.right.map_or(n.end, |r| r.as_ref().subtree_max_end);
        let right_min_start = n.right.map_or(n.start, |r| r.as_ref().subtree_min_start);
        let right_max_gap = n.right.map_or(0, |r| r.as_ref().subtree_max_gap);

        if left_max_end > max_end { max_end = left_max_end; }
        if right_max_end > max_end { max_end = right_max_end; }

        if left_min_start < min_start { min_start = left_min_start; }
        if right_min_start < min_start { min_start = right_min_start; }

        let gap_before_left = n.left.map_or(n.start, |_| left_min_start);
        let gap_left_to_node = n.start.saturating_sub(left_max_end);
        let gap_node_to_right = right_min_start.saturating_sub(n.end);

        max_gap = gap_before_left;
        if gap_left_to_node > max_gap { max_gap = gap_left_to_node; }
        if gap_node_to_right > max_gap { max_gap = gap_node_to_right; }
        if left_max_gap > max_gap { max_gap = left_max_gap; }
        if right_max_gap > max_gap { max_gap = right_max_gap; }

        n.subtree_max_end = max_end;
        n.subtree_min_start = min_start;
        n.subtree_max_gap = max_gap;
    }
}

struct VmaTree {
    root: Option<NonNull<VmaNode>>,
    cached_hint: usize,
}

unsafe impl Sync for VmaNode {}
unsafe impl Sync for VmaTree {}
unsafe impl Send for VmaNode {}
unsafe impl Send for VmaTree {}

impl VmaTree {
    const fn new() -> Self {
        Self {
            root: None,
            cached_hint: 0x1000_0000_0000,
        }
    }

    fn rotate_left(&mut self, mut x: NonNull<VmaNode>) {
        let mut y = unsafe { x.as_ref().right }.expect("rotate_left on node without right child");
        unsafe {
            x.as_mut().right = y.as_ref().left;
            if let Some(mut y_left) = y.as_ref().left {
                y_left.as_mut().parent = Some(x);
            }
            y.as_mut().parent = x.as_ref().parent;
            if let Some(mut p) = x.as_ref().parent {
                if p.as_ref().left == Some(x) { p.as_mut().left = Some(y); }
                else { p.as_mut().right = Some(y); }
            } else {
                self.root = Some(y);
            }
            y.as_mut().left = Some(x);
            x.as_mut().parent = Some(y);
            update_augmentation(&mut x);
            update_augmentation(&mut y);
        }
    }

    fn rotate_right(&mut self, mut y: NonNull<VmaNode>) {
        let mut x = unsafe { y.as_ref().left }.expect("rotate_right on node without left child");
        unsafe {
            y.as_mut().left = x.as_ref().right;
            if let Some(mut x_right) = x.as_ref().right {
                x_right.as_mut().parent = Some(y);
            }
            x.as_mut().parent = y.as_ref().parent;
            if let Some(mut p) = y.as_ref().parent {
                if p.as_ref().left == Some(y) { p.as_mut().left = Some(x); }
                else { p.as_mut().right = Some(x); }
            } else {
                self.root = Some(x);
            }
            x.as_mut().right = Some(y);
            y.as_mut().parent = Some(x);
            update_augmentation(&mut y);
            update_augmentation(&mut x);
        }
    }

    fn insert(&mut self, mut z: NonNull<VmaNode>) {
        let mut y: Option<NonNull<VmaNode>> = None;
        let mut x = self.root;

        while let Some(curr) = x {
            y = Some(curr);
            unsafe {
                if z.as_ref().start < curr.as_ref().start {
                    x = curr.as_ref().left;
                } else {
                    x = curr.as_ref().right;
                }
            }
        }

        unsafe {
            z.as_mut().parent = y;
            if y.is_none() {
                self.root = Some(z);
            } else if let Some(mut y_node) = y {
                if z.as_ref().start < y_node.as_ref().start {
                    y_node.as_mut().left = Some(z);
                } else {
                    y_node.as_mut().right = Some(z);
                }
            }
        }

        let mut curr = unsafe { z.as_ref().parent };
        while let Some(mut node) = curr {
            update_augmentation(&mut node);
            curr = unsafe { node.as_ref().parent };
        }

        self.insert_fixup(z);
    }

    fn insert_fixup(&mut self, mut z: NonNull<VmaNode>) {
        unsafe {
            while let Some(mut p) = z.as_ref().parent {
                if p.as_ref().color == Color::Black { break; }
                
                let mut gp = p.as_ref().parent.unwrap();
                if p.as_ref() == gp.as_ref().left.unwrap().as_ref() {
                    let y = gp.as_ref().right;
                    if let Some(mut y_node) = y
                    && y_node.as_ref().color == Color::Red {
                        p.as_mut().color = Color::Black;
                        y_node.as_mut().color = Color::Black;
                        gp.as_mut().color = Color::Red;
                        z = gp;
                        continue;
                    }
                    if z == p.as_ref().right.expect("UB") {
                        z = p;
                        self.rotate_left(z);
                        p = z.as_ref().parent.unwrap();
                        gp = p.as_ref().parent.unwrap();
                    }
                    p.as_mut().color = Color::Black;
                    gp.as_mut().color = Color::Red;
                    self.rotate_right(gp);
                } else {
                    let y = gp.as_ref().left;
                    if let Some(mut y_node) = y
                    && y_node.as_ref().color == Color::Red {
                        p.as_mut().color = Color::Black;
                        y_node.as_mut().color = Color::Black;
                        gp.as_mut().color = Color::Red;
                        z = gp;
                        continue;
                    }
                    if z == p.as_ref().left.expect("UB") {
                        z = p;
                        self.rotate_right(z);
                        p = z.as_ref().parent.unwrap();
                        gp = p.as_ref().parent.unwrap();
                    }
                    p.as_mut().color = Color::Black;
                    gp.as_mut().color = Color::Red;
                    self.rotate_left(gp);
                }
            }
            self.root.as_mut().unwrap().as_mut().color = Color::Black;
        }
    }

    fn remove(&mut self, z: NonNull<VmaNode>) {
        let mut y = z;
        let mut y_original_color = unsafe { y.as_ref().color };
        let x: Option<NonNull<VmaNode>>;

        unsafe {
            if z.as_ref().left.is_none() {
                x = z.as_ref().right;
                self.transplant(z, z.as_ref().right);
            } else if z.as_ref().right.is_none() {
                x = z.as_ref().left;
                self.transplant(z, z.as_ref().left);
            } else {
                y = self.tree_minimum(z.as_ref().right.unwrap());
                y_original_color = y.as_ref().color;
                x = y.as_ref().right;

                if y.as_ref().parent == Some(z) {
                    if let Some(mut x_node) = x {
                        x_node.as_mut().parent = Some(y);
                    }
                } else {
                    self.transplant(y, y.as_ref().right);
                    y.as_mut().right = z.as_ref().right;
                    if let Some(mut right) = y.as_ref().right {
                        right.as_mut().parent = Some(y);
                    }
                }
                self.transplant(z, Some(y));
                y.as_mut().left = z.as_ref().left;
                if let Some(mut left) = y.as_ref().left {
                    left.as_mut().parent = Some(y);
                }
                y.as_mut().color = z.as_ref().color;
            }
        }

        let mut curr = x.or(unsafe { y.as_ref().parent });
        while let Some(mut node) = curr {
            update_augmentation(&mut node);
            curr = unsafe { node.as_ref().parent };
        }

        if y_original_color == Color::Black
        && let Some(x_node) = x {
            self.remove_fixup(x_node);
        }
    }

    fn remove_fixup(&mut self, mut x: NonNull<VmaNode>) {
        while Some(x) != self.root && unsafe { x.as_ref().color } == Color::Black {
            let mut parent = unsafe { x.as_ref().parent }.unwrap();
            let is_left = unsafe { parent.as_ref().left == Some(x) };

            let mut w = if is_left {
                unsafe { parent.as_ref().right }.unwrap()
            } else {
                unsafe { parent.as_ref().left }.unwrap()
            };

            if unsafe { w.as_ref().color } == Color::Red {
                unsafe {
                    w.as_mut().color = Color::Black;
                    parent.as_mut().color = Color::Red;
                }
                if is_left { self.rotate_left(parent); } else { self.rotate_right(parent); }
                w = if is_left {
                    unsafe { parent.as_ref().right }.unwrap()
                } else {
                    unsafe { parent.as_ref().left }.unwrap()
                };
            }

            let left_black = unsafe { w.as_ref().left.is_none_or(|n| n.as_ref().color == Color::Black) };
            let right_black = unsafe { w.as_ref().right.is_none_or(|n| n.as_ref().color == Color::Black) };

            if left_black && right_black {
                unsafe { w.as_mut().color = Color::Red };
                x = parent;
            } else {
                if is_left {
                    if right_black {
                        if let Some(mut left) = unsafe { w.as_ref().left } {
                            unsafe { left.as_mut().color = Color::Black };
                        }
                        unsafe { w.as_mut().color = Color::Red };
                        self.rotate_right(w);
                        w = unsafe { parent.as_ref().right }.unwrap();
                    }
                    unsafe {
                        w.as_mut().color = parent.as_ref().color;
                        parent.as_mut().color = Color::Black;
                        if let Some(mut right) = w.as_ref().right {
                            right.as_mut().color = Color::Black;
                        }
                    }
                    self.rotate_left(parent);
                    x = self.root.unwrap();
                } else {
                    if left_black {
                        if let Some(mut right) = unsafe { w.as_ref().right } {
                            unsafe { right.as_mut().color = Color::Black };
                        }
                        unsafe { w.as_mut().color = Color::Red };
                        self.rotate_left(w);
                        w = unsafe { parent.as_ref().left }.unwrap();
                    }
                    unsafe {
                        w.as_mut().color = parent.as_ref().color;
                        parent.as_mut().color = Color::Black;
                        if let Some(mut left) = w.as_ref().left {
                            left.as_mut().color = Color::Black;
                        }
                    }
                    self.rotate_right(parent);
                    x = self.root.unwrap();
                }
            }
        }
        unsafe { x.as_mut().color = Color::Black };
    }

    fn transplant(&mut self, u: NonNull<VmaNode>, v: Option<NonNull<VmaNode>>) {
        unsafe {
            if u.as_ref().parent.is_none() {
                self.root = v;
            } else if let Some(mut p) = u.as_ref().parent {
                if p.as_ref().left == Some(u) {
                    p.as_mut().left = v;
                } else {
                    p.as_mut().right = v;
                }
            }
            if let Some(mut v_node) = v {
                v_node.as_mut().parent = u.as_ref().parent;
            }
        }
    }

    fn tree_minimum(&self, mut node: NonNull<VmaNode>) -> NonNull<VmaNode> {
        while let Some(left) = unsafe { node.as_ref().left } {
            node = left;
        }
        node
    }

    fn find_overlap(&self, addr: usize) -> Option<NonNull<VmaNode>> {
        let mut curr = self.root;
        while let Some(node) = curr {
            let n = unsafe { node.as_ref() };
            if addr >= n.start && addr < n.end {
                return Some(node);
            }
            if let Some(left) = n.left
            && unsafe { left.as_ref().subtree_max_end } > addr {
                curr = n.left;
                continue;
            }
            curr = n.right;
        }
        None
    }

    fn get_unmapped_area(&mut self, size: usize, align: usize, hint: usize) -> Option<usize> {
        if size == 0 || size > MAX_CANONICAL_ADDR {
            return None;
        }

        if let Some(root) = self.root
        && unsafe { root.as_ref().subtree_max_gap } < size {
            let last_end = unsafe { root.as_ref().subtree_max_end };
            if MAX_CANONICAL_ADDR.saturating_sub(last_end) < size {
                return None;
            }
        }

        let mut curr = self.root;
        let mut last_end = 0usize;
        let mut best_addr: Option<usize> = None;

        while let Some(node) = curr {
            let n = unsafe { node.as_ref() };
            
            if n.start > last_end {
                let gap = n.start - last_end;
                if gap >= size {
                    let aligned = (last_end + align - 1) & !(align - 1);
                    if aligned >= last_end && aligned + size <= n.start {
                        let candidate = if hint >= last_end && hint < n.start {
                            let hint_aligned = (hint + align - 1) & !(align - 1);
                            if hint_aligned >= last_end && hint_aligned + size <= n.start {
                                hint_aligned
                            } else {
                                aligned
                            }
                        } else {
                            aligned
                        };
                        
                        if best_addr.is_none() || candidate < best_addr.unwrap() {
                            best_addr = Some(candidate);
                        }
                    }
                }
            }
            
            let go_left = if let Some(left) = n.left {
                (unsafe { left.as_ref().subtree_max_gap }) >= size
            } else {
                false
            };

            if go_left {
                curr = n.left;
            } else {
                let left_max_end = n.left.map_or(0, |l| unsafe { l.as_ref().subtree_max_end });
                last_end = if left_max_end > n.end { left_max_end } else { n.end };
                curr = n.right;
            }
        }
        
        if let Some(root) = self.root {
            let final_end = unsafe { root.as_ref().subtree_max_end };
            if MAX_CANONICAL_ADDR >= final_end {
                let remaining = MAX_CANONICAL_ADDR - final_end;
                if remaining >= size {
                    let aligned = (final_end + align - 1) & !(align - 1);
                    if aligned >= final_end && aligned + size <= MAX_CANONICAL_ADDR {
                        let candidate = if hint >= final_end {
                            let hint_aligned = (hint + align - 1) & !(align - 1);
                            if hint_aligned >= final_end && hint_aligned + size <= MAX_CANONICAL_ADDR {
                                hint_aligned
                            } else {
                                aligned
                            }
                        } else {
                            aligned
                        };
                        
                        if best_addr.is_none() || candidate < best_addr.unwrap() {
                            best_addr = Some(candidate);
                        }
                    }
                }
            }
        }

        if let Some(addr) = best_addr {
            self.cached_hint = addr + size;
            Some(addr)
        } else {
            None
        }
    }
}

pub struct Vmm {
    tree: Nutex<VmaTree>,
}

impl Vmm {
    pub const fn new() -> Self {
        Self {
            tree: Nutex::new(VmaTree::new()),
        }
    }

    pub fn alloc(&self, size: usize, align: usize, flags: VmaFlags, hint: usize) -> Option<usize> {
        let mut guard = self.tree.lock();
        let tree = &mut *guard;

        let target_hint = if flags.contains(VmaFlags::FIXED) || hint != 0 {
            hint
        } else {
            tree.cached_hint
        };

        let start = tree.get_unmapped_area(size, align, target_hint)?;
        let end = start + size;

        if tree.find_overlap(start).is_some() || tree.find_overlap(end - 1).is_some() {
            return None;
        }

        let layout = Layout::new::<VmaNode>();
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return None;
        }

        let node = NonNull::new(ptr as *mut VmaNode).unwrap();
        unsafe {
            node.as_ptr().write(VmaNode::new(start, end, flags));
        }

        tree.insert(node);
        Some(start)
    }

    pub fn free(&self, start: usize) {
        let mut guard = self.tree.lock();
        let tree = &mut *guard;

        let mut curr = tree.root;
        let mut target_node: Option<NonNull<VmaNode>> = None;
        
        while let Some(node) = curr {
            let n = unsafe { node.as_ref() };
            if n.start == start {
                target_node = Some(node);
                break;
            } else if start < n.start {
                curr = n.left;
            } else {
                curr = n.right;
            }
        }

        if let Some(node) = target_node {
            tree.remove(node);
            let layout = Layout::new::<VmaNode>();
            unsafe {
                dealloc(node.as_ptr() as *mut u8, layout);
            }
        }
    }

    pub fn find_overlap(&self, addr: usize) -> Option<&VmaNode> {
        let guard = self.tree.lock();
        guard.find_overlap(addr).map(|ptr| unsafe { ptr.as_ref() })
    }
}

```

### `src/mem/ptm.rs`

```rs
use heapless::Vec;

use crate::arch::paging::{Area, Entry, EntryFlags, Exco, is_huge, tab_from_entry, walk_entry, walk_entry_mut};
use crate::mem::kdm::Paddr;
use crate::mem::pmr::{self, Kind};

fn is_phys_range_contiguous(paddr: Paddr, size: usize) -> bool {
    let start = paddr.to_raw();
    let end = start + size;

    for region in pmr::iter() {
        match region.kind {
            Kind::USABLE | Kind::KERNEL | Kind::BOOTLOADER => {
                let reg_start = region.base;
                let reg_end = region.base + region.len;
                if start >= reg_start && end <= reg_end {
                    return true;
                }
            }
            _ => continue,
        }
    }
    warn!(
        "Physical range {:#X}..{:#X} is not contiguous in a usable region",
        start, end
    );
    false
}

fn is_phys_contiguous(paddr: Paddr, page_size: usize) -> bool {
    is_phys_range_contiguous(paddr, page_size)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PageSize {
    Size4K,
    Size2M,
    Size1G,
}

impl PageSize {
    fn bytes(self) -> usize {
        match self {
            PageSize::Size4K => 4096,
            PageSize::Size2M => 2 << 20,
            PageSize::Size1G => 1 << 30,
        }
    }
}

fn select_page_size(vaddr: usize, paddr: Paddr, size: usize) -> PageSize {
    let align_1g = 1_usize << 30;
    let align_2m = 2_usize << 20;

    if size >= align_1g
        && vaddr & (align_1g - 1) == 0
        && paddr.to_raw() & (align_1g - 1) == 0
        && is_phys_contiguous(paddr, align_1g)
    {
        PageSize::Size1G
    } else if size >= align_2m
        && vaddr & (align_2m - 1) == 0
        && paddr.to_raw() & (align_2m - 1) == 0
        && is_phys_contiguous(paddr, align_2m)
    {
        PageSize::Size2M
    } else {
        PageSize::Size4K
    }
}

pub struct Polen {
    pub exco: Exco,
}

impl Polen {
    pub fn new() -> Self {
        debug!("Polen::new");
        Polen { exco: Exco::new() }
    }

    pub const fn from_exco(exco: Exco) -> Self {
        Polen { exco }
    }

    pub fn reference() -> Self {
        let current = Exco::current();

        let new_root = crate::arch::paging::alloc_tab_zeroed();
        let new_cr3 = crate::mem::kdm::Vaddr::from_ref(new_root).to_phys().to_raw() as u64;

        for i in 0..512 {
            new_root.0[i] = current.root.0[i];
        }

        for i in 0..256 {
            new_root.0[i] = Entry::default();
        }

        let new_exco = Exco {
            cr3: new_cr3,
            root: new_root,
            owned: true,
        };

        Polen { exco: new_exco }
    }

    pub fn map(&mut self, vaddr: usize, paddr: Paddr, size: usize, flags: EntryFlags) {
        self.try_map(vaddr, paddr, size, flags)
            .expect("Polen::map failed");
    }

    pub fn try_map(
        &mut self,
        vaddr: usize,
        paddr: Paddr,
        mut size: usize,
        flags: EntryFlags,
    ) -> Result<(), &'static str> {
        if vaddr & 0xfff != 0 {
            return Err("misaligned virtual address");
        }
        if size == 0 {
            return Err("zero-size mapping");
        }

        trace!("try_map(self, {:p}, {:p}, {}, {:?})", vaddr as *const (), paddr.to_raw() as *const (), size, flags);

        let aligned_size = (size + 4095) & !4095;
        if aligned_size != size {
            warn!(
                "Polen::map: size {:#X} not page-aligned, rounding up to {:#X}",
                size, aligned_size
            );
            size = aligned_size;
        }

        const ALIGN_2M: usize = 2 << 20;
        let first_2m_aligned = (vaddr + ALIGN_2M - 1) & !(ALIGN_2M - 1);
        let head_size = if first_2m_aligned > vaddr {
            let hs = first_2m_aligned - vaddr;
            if hs > size { size } else { hs }
        } else {
            0
        };

        let last_2m_aligned = (vaddr + size) & !(ALIGN_2M - 1);
        let tail_size = if last_2m_aligned > vaddr && last_2m_aligned < vaddr + size {
            (vaddr + size) - last_2m_aligned
        } else {
            0
        };

        let middle_start = vaddr + head_size;
        let middle_size = size - head_size - tail_size;

        if head_size > 0 {
            let mut head_vaddr = vaddr;
            let mut head_paddr = paddr;
            let head_bytes = head_size;
            let mut remaining = head_bytes;
            while remaining > 0 {
                let step = 4096;
                self.map_4k_block(head_vaddr, head_paddr, flags)?;
                head_vaddr += step;
                head_paddr = Paddr::from_raw(head_paddr.to_raw() + step);
                remaining -= step;
            }
        }

        if middle_size > 0 {
            let mut mid_vaddr = middle_start;
            let mut mid_paddr = Paddr::from_raw(paddr.to_raw() + head_size);
            let mut mid_rem = middle_size;
            while mid_rem > 0 {
                let ps = select_page_size(mid_vaddr, mid_paddr, mid_rem);
                let ps_bytes = ps.bytes();
                // Step is min(ps_bytes, mid_rem) – but for huge pages we require full size.
                let step = if ps_bytes > mid_rem { mid_rem } else { ps_bytes };

                match ps {
                    PageSize::Size1G => {
                        if step == ps_bytes {
                            if let Ok(entry) = walk_entry_mut(self.exco.root, mid_vaddr, 2, false)
                            && entry.is_present() && is_huge(entry) {
                                debug!("  split 1G (blocking 1G map)");
                                split_and_retry(self, mid_vaddr, ps)?;
                                continue;
                            }
                            self.exco.try_map1g(mid_vaddr, mid_paddr, flags)?;
                            mid_vaddr += step;
                            mid_paddr = Paddr::from_raw(mid_paddr.to_raw() + step);
                            mid_rem -= step;
                            continue;
                        }
                    }
                    PageSize::Size2M => {
                        if step == ps_bytes {
                            if let Ok(entry) = walk_entry_mut(self.exco.root, mid_vaddr, 1, false)
                            && entry.is_present() && is_huge(entry) {
                                debug!("  split 2M (blocking 2M map)");
                                split_and_retry(self, mid_vaddr, ps)?;
                                continue;
                            }
                            self.exco.try_map2m(mid_vaddr, mid_paddr, flags)?;
                            mid_vaddr += step;
                            mid_paddr = Paddr::from_raw(mid_paddr.to_raw() + step);
                            mid_rem -= step;
                            continue;
                        }
                    }
                    PageSize::Size4K => {
                        let step_4k = if step > 4096 { 4096 } else { step };
                        self.map_4k_block(mid_vaddr, mid_paddr, flags)?;
                        mid_vaddr += step_4k;
                        mid_paddr = Paddr::from_raw(mid_paddr.to_raw() + step_4k);
                        mid_rem -= step_4k;
                        continue;
                    }
                }

                let step_4k = if step > 4096 { 4096 } else { step };
                self.map_4k_block(mid_vaddr, mid_paddr, flags)?;
                mid_vaddr += step_4k;
                mid_paddr = Paddr::from_raw(mid_paddr.to_raw() + step_4k);
                mid_rem -= step_4k;
            }
        }

        if tail_size > 0 {
            let mut tail_vaddr = last_2m_aligned;
            let mut tail_paddr = Paddr::from_raw(paddr.to_raw() + size - tail_size);
            let mut tail_rem = tail_size;
            while tail_rem > 0 {
                let step = 4096;
                self.map_4k_block(tail_vaddr, tail_paddr, flags)?;
                tail_vaddr += step;
                tail_paddr = Paddr::from_raw(tail_paddr.to_raw() + step);
                tail_rem -= step;
            }
        }

        Ok(())
    }

    pub fn map_4k_block(
        &mut self,
        vaddr: usize,
        paddr: Paddr,
        flags: EntryFlags,
    ) -> Result<(), &'static str> {
        loop {
            match self.exco.try_map4k(vaddr, paddr, flags) {
                Ok(_) => return Ok(()),
                Err(_) => {
                    if let Ok(_entry) = walk_entry_mut(self.exco.root, vaddr, 1, false) {
                        let base = vaddr & !(0x1f_ffff);
                        debug!("  map_4k_block: split 2M at {:#X}", base);
                        self.exco.try_split2m(base)?;
                    } else if let Ok(_entry) = walk_entry_mut(self.exco.root, vaddr, 2, false) {
                        let base = vaddr & !(0x3fff_ffff);
                        debug!("  map_4k_block: split 1G at {:#X}", base);
                        self.exco.try_split1g(base)?;
                    } else {
                        return Err("map_4k_block: cannot resolve blocking page");
                    }
                }
            }
        }
    }

    pub fn remap(&mut self, vaddr: usize, size: usize, new_flags: EntryFlags) {
        self.try_remap(vaddr, size, new_flags)
            .expect("Polen::remap failed");
    }

    pub fn try_remap(
        &mut self,
        mut vaddr: usize,
        mut size: usize,
        new_flags: EntryFlags,
    ) -> Result<(), &'static str> {
        if vaddr & 0xfff != 0 || size == 0 {
            return Err("misaligned or zero-size remap");
        }

        debug!("Polen::remap [ {:#X} .. {:#X} ) -> flags {:?}", vaddr, vaddr + size, new_flags);

        while size > 0 {
            if let Ok(entry) = walk_entry_mut(self.exco.root, vaddr, 2, false)
            && entry.is_present() && is_huge(entry) {
                let base = vaddr & !(0x3fff_ffff);
                debug!("  remap: split 1G at {:#X}", base);
                self.exco.try_split1g(base)?;
                continue;
            }
            if let Ok(entry) = walk_entry_mut(self.exco.root, vaddr, 1, false)
            && entry.is_present() && is_huge(entry) {
                let base = vaddr & !(0x1f_ffff);
                debug!("  remap: split 2M at {:#X}", base);
                self.exco.try_split2m(base)?;
                continue;
            }

            let entry = walk_entry_mut(self.exco.root, vaddr, 0, false)?;
            if !entry.is_present() {
                return Err("address not mapped");
            }
            let paddr = entry.address();
            *entry = Entry::new(paddr, new_flags | EntryFlags::PRESENT);
            crate::arch::paging::flush_tlb(vaddr);

            vaddr += 4096;
            size = size.saturating_sub(4096);
        }

        Ok(())
    }

    pub fn unmap(&mut self, vaddr: usize, size: usize) {
        self.try_unmap(vaddr, size).expect("Polen::unmap failed");
    }

    pub fn try_unmap(
        &mut self,
        mut vaddr: usize,
        mut size: usize,
    ) -> Result<(), &'static str> {
        if vaddr & 0xfff != 0 || size == 0 {
            return Err("misaligned or zero-size unmap");
        }

        info!("Polen::unmap [ {:#X} .. {:#X} ) ({} bytes)", vaddr, vaddr + size, size);

        while size > 0 {
            if let Ok(entry) = walk_entry_mut(self.exco.root, vaddr, 2, false)
            && entry.is_present() && is_huge(entry) {
                let base = vaddr & !(0x3fff_ffff);
                if base != vaddr || size < (1 << 30) {
                    debug!("  unmap: partial split 1G at {:#X}", base);
                    self.exco.try_split1g(base)?;
                    continue;
                }
            }
            if let Ok(entry) = walk_entry_mut(self.exco.root, vaddr, 1, false)
            && entry.is_present() && is_huge(entry) {
                let base = vaddr & !(0x1f_ffff);
                if base != vaddr || size < (2 << 20) {
                    debug!("  unmap: partial split 2M at {:#X}", base);
                    self.exco.try_split2m(base)?;
                    continue;
                }
            }

            self.exco.try_unmap(vaddr)?;

            vaddr += 4096;
            size = size.saturating_sub(4096);
        }

        Ok(())
    }

    pub fn merge_range(&mut self, start: usize, size: usize) {
        let end = start + size;
        debug!("Polen::merge_range [ {:#X} .. {:#X} )", start, end);

        if size < (2 << 20) {
            return;
        }

        let two_m = 2 << 20;
        let two_m_mask = !(two_m - 1);
        let first_2m = (start + two_m - 1) & two_m_mask;
        let last_2m = (end - 1) & two_m_mask;

        let mut vaddr = first_2m;
        while vaddr <= last_2m {
            let _ = self.exco.try_merge2m(vaddr);
            vaddr += two_m;
        }

        if size >= (1 << 30) {
            let one_g = 1 << 30;
            let one_g_mask = !(one_g - 1);
            let first_1g = (start + one_g - 1) & one_g_mask;
            let last_1g = (end - 1) & one_g_mask;

            let mut vaddr_gb = first_1g;
            while vaddr_gb <= last_1g {
                let _ = self.exco.try_merge1g(vaddr_gb);
                vaddr_gb += one_g;
            }
        }
    }

    pub fn query(&self, vaddr: usize) -> Option<(Paddr, EntryFlags)> {
        trace!("Polen::query {:#X}", vaddr);

        if let Ok(entry) = walk_entry(self.exco.root, vaddr, 0)
        && entry.is_present() {
            let offset = vaddr & 0xfff;
            let paddr = Paddr::from_raw(entry.address().to_raw() + offset);
            trace!("  -> 4K page: phys {:#016X} flags {:?}", paddr.to_raw(), entry.flags());
            return Some((paddr, entry.flags()));
        }

        if let Ok(entry) = walk_entry(self.exco.root, vaddr, 1)
        && entry.is_present() && is_huge(entry) {
            let offset = vaddr & 0x1f_ffff;
            let paddr = Paddr::from_raw(entry.address().to_raw() + offset);
            trace!("  -> 2M page: phys {:#016X} flags {:?}", paddr.to_raw(), entry.flags());
            return Some((paddr, entry.flags()));
        }

        if let Ok(entry) = walk_entry(self.exco.root, vaddr, 2)
        && entry.is_present() && is_huge(entry) {
            let offset = vaddr & 0x3fff_ffff;
            let paddr = Paddr::from_raw(entry.address().to_raw() + offset);
            trace!("  -> 1G page: phys {:#016X} flags {:?}", paddr.to_raw(), entry.flags());
            return Some((paddr, entry.flags()));
        }

        trace!("  -> not mapped");
        None
    }

    pub fn report<const N: usize>(&self) -> Vec<Area, N> {
        self.exco.report()
    }

    #[inline(always)]
    pub unsafe fn activate(&self) {
        unsafe {
            self.exco.activate()
        }
    }

    pub fn mark_user_cow(&mut self) {
        for pml4_idx in 0..256 {
            let pml4_entry = &mut self.exco.root.0[pml4_idx];
            if !pml4_entry.is_present() || is_huge(pml4_entry) { continue; }
            
            let pdpt = tab_from_entry(pml4_entry);
            for pdpt_idx in 0..512 {
                let pdpt_entry = &mut pdpt.0[pdpt_idx];
                if !pdpt_entry.is_present() || is_huge(pdpt_entry) { continue; }
                
                let pd = tab_from_entry(pdpt_entry);
                for pd_idx in 0..512 {
                    let pd_entry = &mut pd.0[pd_idx];
                    if !pd_entry.is_present() || is_huge(pd_entry) { continue; }
                    
                    let pt = tab_from_entry(pd_entry);
                    for pt_idx in 0..512 {
                        let pt_entry = &mut pt.0[pt_idx];
                        // Если страница присутствует и доступна для записи
                        if pt_entry.is_present() && pt_entry.is_writable() {
                            let mut flags = pt_entry.flags();
                            flags.remove(EntryFlags::WRITABLE);
                            flags.insert(EntryFlags::COPY_ON_WRITE);
                            pt_entry.set_flags(flags);
                        }
                    }
                }
            }
        }
        unsafe { self.exco.activate(); } 
    }
}

fn split_and_retry(polen: &mut Polen, vaddr: usize, ps: PageSize) -> Result<(), &'static str> {
    match ps {
        PageSize::Size1G => {
            polen.exco.try_split1g(vaddr)?;
        }
        PageSize::Size2M => {
            polen.exco.try_split2m(vaddr)?;
        }
        PageSize::Size4K => {}
    }
    Ok(())
}

```

### `src/sync/nutex.rs`

```rs
//! # Interrupt-Disabling Mutex (Nutex)
//!
//! A mutual exclusion primitive that **disables interrupts** while the lock is held.
//! This ensures that the critical section is not interrupted, making it safe to use
//! in interrupt handlers and other contexts where preemption must be prevented.
//!
//! ## Overview
//!
//! The `Nutex` (Non‑preemptive Mutex) is a spinlock‑based mutex that disables
//! interrupts on the current CPU when the lock is acquired, and restores them
//! when the lock is released. This guarantees that:
//! - The critical section is atomic with respect to interrupts on the current CPU.
//! - No interrupt handler can run while the lock is held, preventing deadlocks
//!   where an interrupt handler tries to acquire the same lock.
//! - The lock is safe to use from both task context and interrupt context.
//!
//! ## Characteristics
//!
//! - **Interrupt disabling**: Interrupts are disabled (`cli`) when the lock is
//!   acquired and restored to their previous state when the lock is released.
//! - **Spinlock**: If the lock is already held, the CPU spins in a tight loop
//!   (`spin_loop`) until the lock is released.
//! - **IRQ state restoration**: The `NutexGuard` saves the previous interrupt
//!   state (whether interrupts were enabled) and restores it on drop. This
//!   ensures that interrupts are only disabled if they were enabled before,
//!   and are not incorrectly left disabled.
//! - **No recursion**: The `Nutex` does not support recursive locking. Attempting
//!   to lock the same `Nutex` from the same CPU will cause a deadlock.
//!
//! ## Usage
//!
//! ```ignore
//! use crate::sync::Nutex;
//!
//! static SHARED_DATA: Nutex<u32> = Nutex::new(0);
//!
//! fn critical_section() {
//!     let mut guard = SHARED_DATA.lock();
//!     *guard += 1;  // Interrupts are disabled here
//!     // Guard is dropped, interrupts are restored
//! }
//! ```
//!
//! ## Safety
//!
//! - This mutex is `Send` and `Sync` when `T` is `Send`.
//! - The lock is safe to use from multiple CPUs, but the user must ensure that
//!   the protected data is not accessed concurrently outside the lock.
//! - Interrupts are disabled on the current CPU, but other CPUs are not affected.
//! - The `lock` and `try_lock` methods use `cli` to disable interrupts, and the
//!   guard restores the previous interrupt state. This is safe because:
//!   - Interrupts are not re‑enabled until the guard is dropped.
//!   - The saved state ensures that interrupts are not incorrectly enabled if
//!     they were disabled before locking.
//!
//! ## Comparison with Other Synchronization Primitives
//!
//! | Primitive | Interrupts Disabled | Use Case |
//! |-----------|---------------------|----------|
//! | `Mutex`   | No                  | Data shared between tasks (threads) on multiple CPUs, but not in interrupt context. |
//! | `Nutex`   | Yes                 | Data that can be accessed from both task and interrupt context. |
//! | `Litex`   | Yes                 | Similar to `Nutex` but with an `unsafe inner()` method for raw access. |
//! | `Nitex`   | Yes                 | A lock‑free (or rather, interrupt‑only) mutex that only disables interrupts, no spinning. Actually `Nitex` does not spin; it just disables interrupts. |
//!
//! ## Implementation Details
//!
//! The lock uses `AtomicBool` as the spinlock flag. On acquisition:
//! 1. The current interrupt state is saved using `pushfq`/`pop`.
//! 2. Interrupts are disabled with `cli`.
//! 3. The lock flag is set to `true` using a spinloop with `compare_exchange_weak`.
//! 4. On release, the lock flag is cleared (`store(false)`), and the saved
//!    interrupt state is restored (if interrupts were previously enabled, `sti` is
//!    executed).

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
    arch::asm
};

// ============================================================================
// NUTEX STRUCTURE
// ============================================================================

/// An interrupt‑disabling spinlock mutex.
///
/// This mutex disables interrupts when locked and restores them when unlocked.
/// It is safe to use in interrupt handlers and task contexts.
///
/// # Type Parameters
/// * `T` – The type of the data protected by the mutex.
///
/// # Examples
/// ```ignore
/// static COUNTER: Nutex<usize> = Nutex::new(0);
///
/// fn increment() {
///     let mut guard = COUNTER.lock();
///     *guard += 1;
/// }
/// ```
#[derive(Debug)]
pub struct Nutex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

// Safety: Nutex is Send and Sync if T is Send.
unsafe impl<T: Send> Send for Nutex<T> {}
unsafe impl<T: Send> Sync for Nutex<T> {}

impl<T> Nutex<T> {
    /// Creates a new `Nutex` in the unlocked state.
    ///
    /// # Arguments
    /// * `t` – The initial value to protect.
    pub const fn new(t: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(t),
        }
    }

    /// Acquires the lock, disabling interrupts and spinning until the lock is available.
    ///
    /// This function:
    /// 1. Saves the current interrupt state (whether interrupts were enabled).
    /// 2. Disables interrupts with `cli`.
    /// 3. Spins until the lock flag is acquired.
    /// 4. Returns a guard that will release the lock and restore the interrupt
    ///    state when dropped.
    ///
    /// # Returns
    /// A `NutexGuard` that dereferences to the protected data.
    ///
    /// # Deadlocks
    /// This function will deadlock if the lock is already held by the current CPU.
    pub fn lock(&self) -> NutexGuard<'_, T> {
        // Save the current interrupt state before we disable interrupts.
        let rflags: u64;
        unsafe {
            asm!(
                "pushfq",
                "pop {0}",
                out(reg) rflags,
                options(nomem, preserves_flags)
            );
            asm!("cli", options(nomem, nostack, preserves_flags));
        }

        // Spin until we acquire the lock.
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }

        NutexGuard {
            mutex: self,
            saved_if: (rflags & (1 << 9)) != 0,  // Bit 9 is the IF flag in RFLAGS.
        }
    }

    /// Attempts to acquire the lock without spinning.
    ///
    /// If the lock is free, it is acquired, interrupts are disabled, and a guard
    /// is returned. If the lock is already held, `None` is returned immediately.
    ///
    /// # Returns
    /// `Some(NutexGuard)` if the lock was acquired, `None` otherwise.
    pub fn try_lock(&self) -> Option<NutexGuard<'_, T>> {
        // Save the current interrupt state and disable interrupts.
        let rflags: u64;
        unsafe {
            asm!(
                "pushfq",
                "pop {0}",
                out(reg) rflags,
                options(nomem, preserves_flags)
            );
            asm!("cli", options(nomem, nostack, preserves_flags));
        }

        // Try to acquire the lock with a single CAS attempt.
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(NutexGuard {
                mutex: self,
                saved_if: (rflags & (1 << 9)) != 0,
            })
        } else {
            unsafe {
                if (rflags & (1 << 9)) != 0 {
                    asm!("sti", options(nomem, nostack, preserves_flags));
                }
            }
            None
        }
    }
}

// ============================================================================
// GUARD STRUCTURE
// ============================================================================

/// A guard that holds the `Nutex` lock and provides access to the protected data.
///
/// The guard dereferences to `T` (via `Deref` and `DerefMut`) and releases the
/// lock and restores the interrupt state when dropped.
pub struct NutexGuard<'a, T> {
    mutex: &'a Nutex<T>,
    saved_if: bool,  // Whether interrupts were enabled before locking.
}

impl<T> core::ops::Deref for NutexGuard<'_, T> {
    type Target = T;

    /// Returns a reference to the protected data.
    ///
    /// # Safety
    /// The lock is held and interrupts are disabled, so the data is not being
    /// mutated by other threads or interrupt handlers.
    fn deref(&self) -> &T {
        unsafe { self.mutex.data.get().as_mut_unchecked() }
    }
}

impl<T> core::ops::DerefMut for NutexGuard<'_, T> {
    /// Returns a mutable reference to the protected data.
    ///
    /// # Safety
    /// The lock is held exclusively and interrupts are disabled.
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for NutexGuard<'_, T> {
    /// Releases the lock and restores the interrupt state.
    ///
    /// When the guard is dropped:
    /// 1. The lock flag is cleared with `Release` ordering.
    /// 2. If interrupts were enabled before locking, `sti` is executed.
    fn drop(&mut self) {
        // Release the lock.
        self.mutex.lock.store(false, Ordering::Release);

        // Restore the interrupt state if it was previously enabled.
        unsafe {
            if self.saved_if {
                asm!("sti", options(nomem, nostack, preserves_flags));
            }
        }
    }
}

```

### `src/sync/mutex.rs`

```rs
//! # Simple Spinlock (Mutex)
//!
//! This module provides a basic spinlock‑based mutual exclusion primitive,
//! `Mutex<T>`. It is a straightforward implementation that uses a single
//! `AtomicBool` as the lock flag and spins (busy‑waits) when the lock is
//! already held.
//!
//! ## Overview
//!
//! The `Mutex<T>` is the simplest locking primitive in the kernel. It is
//! suitable for protecting data that is accessed from multiple CPU cores but
//! where the critical sections are short and the lock is not held for long
//! periods.
//!
//! Unlike `Nutex` and `Litex`, this mutex **does not disable interrupts**.
//! This makes it unsuitable for use in interrupt handlers or in contexts where
//! the lock might be taken by an interrupt handler that interrupts a lock‑holder.
//! For such cases, use `Nutex` or `Litex`.
//!
//! ## Characteristics
//!
//! - **Spinlock**: The lock spins in a tight loop (`spin_loop`) until the lock
//!   is acquired.
//! - **No interrupt disabling**: Interrupts remain enabled while the lock is
//!   held. This means the lock is not safe for use in interrupt handlers or
//!   with code that can be preempted by interrupts.
//! - **Fairness**: No fairness guarantees; it is a simple test‑and‑set lock.
//! - **Atomic operations**: Uses `AtomicBool` with `Acquire`/`Release` ordering
//!   to ensure memory visibility.
//!
//! ## Usage
//!
//! ```ignore
//! use crate::sync::Mutex;
//!
//! static MY_DATA: Mutex<u32> = Mutex::new(0);
//!
//! fn increment() {
//!     let mut guard = MY_DATA.lock();
//!     *guard += 1;
//! }
//! ```
//!
//! The `lock()` method returns a `MutexGuard` that dereferences to the inner
//! data. When the guard goes out of scope, the lock is automatically released.
//!
//! ## Safety
//!
//! - This mutex is `Send` and `Sync` when `T` is `Send`.
//! - The lock is safe to use from multiple CPUs, but the user must ensure that
//!   the protected data is not accessed concurrently outside the lock.
//! - Because interrupts are not disabled, the lock must not be used in code
//!   that can be preempted by interrupt handlers that also try to acquire the
//!   same lock (deadlock risk).
//!
//! ## Comparison with Other Synchronization Primitives
//!
//! | Primitive | Interrupts Disabled | Use Case |
//! |-----------|---------------------|----------|
//! | `Mutex`   | No                  | Data shared between tasks (threads) on multiple CPUs, but not in interrupt context. |
//! | `Nutex`   | Yes                 | Data that can be accessed from both task and interrupt context. |
//! | `Litex`   | Yes                 | Similar to `Nutex` but with an `unsafe inner()` method for raw access. |
//! | `Nitex`   | Yes                 | A lock‑free (or rather, interrupt‑only) mutex that only disables interrupts, no spinning. Actually `Nitex` does not spin; it just disables interrupts. |
//!
//! ## Implementation Details
//!
//! The lock uses `compare_exchange_weak` in a loop to attempt to set the flag
//! from `false` to `true`. If it fails, it calls `spin_loop` to yield the CPU
//! briefly before retrying. The `try_lock` method attempts a single CAS and
//! returns `None` if it fails.

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering}
};

// ============================================================================
// MUTEX STRUCTURE
// ============================================================================

/// A spinlock‑based mutual exclusion primitive.
///
/// This struct protects a value of type `T` with a spinlock. The lock is
/// acquired by calling `lock()`, which returns a guard that releases the lock
/// when dropped.
///
/// # Type Parameters
/// * `T` – The type of the data protected by the mutex.
///
/// # Examples
/// ```ignore
/// static COUNTER: Mutex<usize> = Mutex::new(0);
///
/// fn increment() {
///     let mut guard = COUNTER.lock();
///     *guard += 1;
/// }
/// ```
pub struct Mutex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

// Safety: Mutex is Send and Sync if T is Send.
unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// Creates a new `Mutex` in the unlocked state.
    ///
    /// # Arguments
    /// * `t` – The initial value to protect.
    pub const fn new(t: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(t),
        }
    }

    /// Acquires the lock, spinning until it is available.
    ///
    /// This function will spin in a loop until the lock is acquired. Once
    /// acquired, it returns a `MutexGuard` that provides access to the data.
    ///
    /// # Returns
    /// A `MutexGuard` that dereferences to the protected data.
    ///
    /// # Panics
    /// This function does not panic.
    ///
    /// # Deadlocks
    /// This function will deadlock if the lock is already held by the current
    /// CPU (i.e., recursive locking is not supported).
    pub fn lock(&self) -> MutexGuard<'_, T> {
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }

        MutexGuard { mutex: self }
    }

    /// Attempts to acquire the lock without spinning.
    ///
    /// If the lock is currently free, it is acquired and a guard is returned.
    /// Otherwise, `None` is returned.
    ///
    /// # Returns
    /// `Some(MutexGuard)` if the lock was acquired, `None` otherwise.
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(MutexGuard { mutex: self })
        } else {
            None
        }
    }
}

// ============================================================================
// GUARD STRUCTURE
// ============================================================================

/// A guard that holds the mutex lock and provides access to the protected data.
///
/// The guard dereferences to `T` (via `Deref` and `DerefMut`) and releases the
/// lock when dropped.
pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> core::ops::Deref for MutexGuard<'_, T> {
    type Target = T;

    /// Returns a reference to the protected data.
    ///
    /// # Safety
    /// The lock is held, so the data is not being mutated by other threads.
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> core::ops::DerefMut for MutexGuard<'_, T> {
    /// Returns a mutable reference to the protected data.
    ///
    /// # Safety
    /// The lock is held exclusively, so no other thread can access the data.
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    /// Releases the lock.
    ///
    /// When the guard is dropped, the lock flag is set to `false` with `Release`
    /// ordering, ensuring that all writes to the protected data are visible
    /// to the next lock acquirer.
    fn drop(&mut self) {
        self.mutex.lock.store(false, Ordering::Release);
    }
}

```

### `src/sync/nitex.rs`

```rs
//! # Interrupt-Only Mutex (Nitex)
//!
//! A mutual exclusion primitive that **only disables interrupts** and does **not spin**.
//! This is the simplest form of mutual exclusion for single‑CPU critical sections
//! where preemption by interrupts must be prevented, but no other CPU can contend
//! for the lock (or the lock is only ever used on one CPU).
//!
//! ## Overview
//!
//! The `Nitex` (Non‑interruptible, no‑spin mutex) is a lock that:
//! - Disables interrupts when acquired, and restores them when released.
//! - Does **not** spin if the lock is already held; it simply disables interrupts
//!   and assumes that the lock will be released quickly by the current CPU.
//! - Is **not** a spinlock; it does not use atomic operations or busy‑waiting.
//! - Is intended for use on **single‑CPU** systems or in situations where the
//!   lock is guaranteed to be uncontended on the current CPU (e.g., per‑CPU data
//!   structures that are only accessed from interrupt handlers on the same CPU).
//!
//! ## Characteristics
//!
//! - **Interrupt disabling**: Disables interrupts (`cli`) on lock acquisition and
//!   restores the previous state on release.
//! - **No spinning**: Does not loop or use atomic compare‑exchange; it simply
//!   disables interrupts and assumes the lock is free. This means it is **not**
//!   safe for use on multi‑CPU systems where another CPU might hold the lock.
//! - **Unsafe inner access**: Provides an `unsafe inner()` method to get a
//!   mutable reference to the data without locking (use with extreme caution).
//! - **No atomic operations**: The lock state is not tracked; the lock is
//!   effectively a "disable interrupts" marker.
//!
//! ## Usage
//!
//! This primitive is typically used for per‑CPU data that is only accessed by
//! code running on that CPU, such as:
//! - Per‑CPU runqueues.
//! - Per‑CPU timers.
//! - Per‑CPU statistics.
//!
//! It is not suitable for data shared between multiple CPU cores.
//!
//! ```ignore
//! use crate::sync::Nitex;
//!
//! static PER_CPU_DATA: Nitex<u32> = Nitex::new(0);
//!
//! fn access() {
//!     let mut guard = PER_CPU_DATA.lock();
//!     *guard += 1;
//!     // Interrupts are re‑enabled when guard is dropped.
//! }
//! ```
//!
//! ## Safety
//!
//! - The `lock()` method disables interrupts but does **not** check if the lock
//!   is already held. It is the caller's responsibility to ensure that the lock
//!   is not used recursively on the same CPU (which would leave interrupts
//!   disabled forever, as the guard would not be dropped before the second lock).
//! - The `inner()` method allows unsafe access to the data without any locking.
//!   It must only be used when the caller knows that no other code is accessing
//!   the data concurrently (e.g., during early boot).
//! - This primitive is **not** `Sync` or `Send` for multi‑CPU safety. It is
//!   intended for per‑CPU usage only.
//!
//! ## Comparison with Other Synchronization Primitives
//!
//! | Primitive | Interrupts Disabled | Spins | Multi‑CPU Safe |
//! |-----------|---------------------|-------|----------------|
//! | `Mutex`   | No                  | Yes   | Yes            |
//! | `Nutex`   | Yes                 | Yes   | Yes            |
//! | `Litex`   | Yes                 | Yes   | Yes            |
//! | `Nitex`   | Yes                 | No    | No (per‑CPU only) |
//!
//! ## Implementation Details
//!
//! The `Nitex` is simply a wrapper around `UnsafeCell<T>` that provides a
//! `lock()` method which disables interrupts and returns a guard. The guard
//! restores interrupts on drop. There is no lock flag; the lock is "held"
//! by virtue of interrupts being disabled, preventing any interrupt handler
//! from running and potentially accessing the data.

use core::{
    cell::UnsafeCell,
    arch::asm
};

// ============================================================================
// NITEX STRUCTURE
// ============================================================================

/// An interrupt‑only mutex that disables interrupts but does not spin.
///
/// This primitive is only safe for use on a single CPU core and must not be
/// used for data shared across multiple CPUs.
///
/// # Type Parameters
/// * `T` – The type of the data protected by the mutex.
///
/// # Examples
/// ```ignore
/// static MY_DATA: Nitex<usize> = Nitex::new(0);
///
/// fn increment() {
///     let mut guard = MY_DATA.lock();
///     *guard += 1;
/// }
/// ```
pub struct Nitex<T> {
    data: UnsafeCell<T>,
}

impl<T: Clone> Clone for Nitex<T> {
    /// Clones the data by locking the mutex and copying the inner value.
    ///
    /// # Safety
    /// This acquires the lock (disables interrupts) to read the data safely.
    fn clone(&self) -> Self {
        Self::new((unsafe { &*self.data.get() }).clone())
    }
}

// Safety: Nitex is Send and Sync if T is Send, but only if the caller ensures
// that the lock is only used on a single CPU.
unsafe impl<T: Send> Send for Nitex<T> {}
unsafe impl<T: Send> Sync for Nitex<T> {}

impl<T> Nitex<T> {
    /// Creates a new `Nitex` with the given initial value.
    pub const fn new(t: T) -> Self {
        Self { data: UnsafeCell::new(t) }
    }

    /// Returns a mutable reference to the inner data **without locking**.
    ///
    /// # Safety
    /// This is unsafe because it bypasses the mutex. The caller must ensure
    /// that no other code is accessing the data concurrently.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn inner(&self) -> &mut T {
        unsafe {
            self.data.as_mut_unchecked()
        }
    }

    /// Acquires the lock by disabling interrupts.
    ///
    /// This function disables interrupts (`cli`) and returns a guard. The guard
    /// will re‑enable interrupts (restoring the previous state) when dropped.
    ///
    /// # Panics
    /// This function does not panic.
    ///
    /// # Deadlocks
    /// If this function is called while interrupts are already disabled, it
    /// will simply disable them again (no effect) and the guard will restore
    /// the saved state. However, if called recursively (while the lock is already
    /// held by the same CPU), interrupts will be re‑enabled when the inner guard
    /// is dropped, potentially corrupting the data.
    pub fn lock(&self) -> NitexGuard<'_, T> {
        let rflags: u64;
        unsafe {
            asm!(
                "pushfq",
                "pop {0}",
                out(reg) rflags,
                options(nomem, preserves_flags)
            );
            asm!("cli", options(nomem, nostack, preserves_flags));
        }

        NitexGuard { mutex: self, saved_if: (rflags & (1 << 9)) != 0 }
    }
}

// ============================================================================
// GUARD STRUCTURE
// ============================================================================

/// A guard that holds the `Nitex` lock and provides access to the protected data.
///
/// The guard dereferences to `T` (via `Deref` and `DerefMut`) and restores
/// the interrupt state when dropped.
pub struct NitexGuard<'a, T> {
    mutex: &'a Nitex<T>,
    saved_if: bool,
}

impl<T> core::ops::Deref for NitexGuard<'_, T> {
    type Target = T;

    /// Returns a reference to the protected data.
    ///
    /// # Safety
    /// Interrupts are disabled, so the data is not being mutated by interrupt
    /// handlers on the same CPU.
    fn deref(&self) -> &T {
        unsafe { self.mutex.data.get().as_ref_unchecked() }
    }
}

impl<T> core::ops::DerefMut for NitexGuard<'_, T> {
    /// Returns a mutable reference to the protected data.
    ///
    /// # Safety
    /// Interrupts are disabled, so the data is safe to mutate.
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for NitexGuard<'_, T> {
    /// Restores the interrupt state (re‑enables interrupts if they were enabled
    /// before locking).
    fn drop(&mut self) {
        unsafe {
            if self.saved_if {
                asm!("sti", options(nomem, nostack, preserves_flags));
            }
        }
    }
}

```

### `src/sync/rwlock.rs`

```rs
//! # Read-Write Lock (RwLock)
//!
//! A spinlock‑based reader‑writer lock that allows multiple concurrent readers
//! or a single exclusive writer. This is a fundamental synchronization primitive
//! for data structures that are frequently read but only occasionally written.
//!
//! ## Overview
//!
//! The `RwLock<T>` provides two locking modes:
//! - **Read lock** (`read()`): Allows multiple readers to access the data
//!   concurrently. Readers spin while a writer holds the lock.
//! - **Write lock** (`write()`): Provides exclusive access to the data. Writers
//!   spin until no readers or writers hold the lock.
//!
//! The lock state is represented by a single `AtomicUsize`:
//! - `0` : Unlocked.
//! - `WRITER_BIT` (highest bit): Write‑locked.
//! - Any other value: Read‑locked, with the lower bits counting the number of
//!   active readers.
//!
//! ## Characteristics
//!
//! - **Spinlock‑based**: Both readers and writers spin (busy‑wait) until the
//!   lock is available. This is suitable for short critical sections.
//! - **Fairness**: No fairness guarantees; readers and writers contend equally.
//!   This could lead to writer starvation if there is a steady stream of readers.
//! - **Interrupts**: This lock does **not** disable interrupts. It is not safe
//!   to use in interrupt handlers if the same lock can be acquired in task context.
//!   For interrupt‑safe rwlocks, use `Nutex` or `Litex` wrapping a custom implementation.
//! - **No recursion**: Recursive locking is not supported. A writer attempting
//!   to acquire a read lock or vice versa will deadlock.
//!
//! ## Usage
//!
//! ```ignore
//! use crate::sync::RwLock;
//!
//! static DATA: RwLock<Vec<u32>> = RwLock::new(Vec::new());
//!
//! fn read_data() {
//!     let guard = DATA.read();
//!     println!("Length: {}", guard.len());
//! }
//!
//! fn write_data() {
//!     let mut guard = DATA.write();
//!     guard.push(42);
//! }
//! ```
//!
//! ## Safety
//!
//! - The lock is `Send` and `Sync` when `T` is `Send`.
//! - The `read()` and `write()` methods use atomic operations to manage the state.
//! - The lock is safe for use on multi‑CPU systems because all state updates
//!   are atomic with appropriate memory ordering.
//! - The `try_read()` and `try_write()` methods provide non‑blocking attempts.
//!
//! ## Implementation Details
//!
//! The lock state is stored in an `AtomicUsize`:
//! - **Writer bit**: The most significant bit (`1 << (usize::BITS - 1)`).
//!   When set, the lock is write‑locked.
//! - **Reader count**: The lower bits (excluding the writer bit) store the
//!   number of active readers.
//!
//! ### Read Lock (shared)
//! 1. Spin while the writer bit is set.
//! 2. Atomically increment the reader count (with overflow check).
//! 3. If CAS fails, retry from step 1.
//!
//! ### Write Lock (exclusive)
//! 1. Spin while the state is not `0`.
//! 2. Atomically set the state to `WRITER_BIT`.
//! 3. If CAS fails, retry from step 1.
//!
//! ### Unlocking
//! - **Read unlock**: Decrement the reader count (`fetch_sub(1)`).
//! - **Write unlock**: Set the state to `0`.
//!
//! ## Comparison with Other Primitives
//!
//! | Primitive | Readers | Writers | Interrupt‑Safe | Use Case |
//! |-----------|---------|---------|----------------|----------|
//! | `Mutex`   | No      | Yes     | No             | Single exclusive access, not interrupt context. |
//! | `Nutex`   | No      | Yes     | Yes            | Exclusive access with interrupt safety. |
//! | `RwLock`  | Yes     | Yes     | No             | Read‑heavy workloads, not interrupt context. |
//! | `Litex`   | No      | Yes     | Yes            | Exclusive access with interrupt safety and unsafe inner. |

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
    hint,
};

// ============================================================================
// CONSTANTS
// ============================================================================

/// The bit used to indicate that the lock is write‑locked.
/// This is the most significant bit of a `usize`.
const WRITER_BIT: usize = 1 << (usize::BITS - 1);

// ============================================================================
// RWLOCK STRUCTURE
// ============================================================================

/// A reader‑writer lock that spins until the lock is available.
///
/// This lock allows multiple readers or a single writer at any time.
///
/// # Type Parameters
/// * `T` – The type of the data protected by the lock.
///
/// # Examples
/// ```ignore
/// static SHARED: RwLock<Vec<u32>> = RwLock::new(vec![]);
///
/// fn read() {
///     let guard = SHARED.read();
///     println!("Length: {}", guard.len());
/// }
///
/// fn write() {
///     let mut guard = SHARED.write();
///     guard.push(42);
/// }
/// ```
pub struct RwLock<T> {
    state: AtomicUsize,
    data: UnsafeCell<T>,
}

// Safety: RwLock is Send and Sync if T is Send.
unsafe impl<T: Send> Send for RwLock<T> {}
unsafe impl<T: Send> Sync for RwLock<T> {}

impl<T> RwLock<T> {
    /// Creates a new `RwLock` in the unlocked state.
    ///
    /// # Arguments
    /// * `t` – The initial value to protect.
    pub const fn new(t: T) -> Self {
        Self {
            state: AtomicUsize::new(0),
            data: UnsafeCell::new(t),
        }
    }

    /// Returns a mutable reference to the inner data **without locking**.
    ///
    /// # Safety
    /// This is unsafe because it bypasses the lock. The caller must ensure that
    /// no other code is accessing the data concurrently.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn inner(&self) -> &mut T {
        unsafe {
            self.data.as_mut_unchecked()
        }
    }

    /// Acquires a shared (read) lock, spinning until it is available.
    ///
    /// This function will spin in a loop until the lock can be acquired for reading.
    /// It returns a `RwLockReadGuard` that dereferences to `T` and releases the
    /// lock when dropped.
    ///
    /// # Returns
    /// A `RwLockReadGuard` that provides shared access to the data.
    ///
    /// # Deadlocks
    /// This function will deadlock if the lock is already held by the current
    /// thread for writing.
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        loop {
            let old = self.state.load(Ordering::Acquire);
            // If the writer bit is set, spin.
            if old & WRITER_BIT != 0 {
                hint::spin_loop();
                continue;
            }
            // Try to increment the reader count.
            let new = old + 1;
            // The increment must not overflow into the writer bit.
            debug_assert!(new & WRITER_BIT == 0);
            if self
                .state
                .compare_exchange_weak(old, new, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
            {
                return RwLockReadGuard { lock: self };
            }
            // CAS failed, retry.
        }
    }

    /// Attempts to acquire a shared (read) lock without spinning.
    ///
    /// If the lock is available for reading, it is acquired and a guard is returned.
    /// Otherwise, `None` is returned immediately.
    ///
    /// # Returns
    /// `Some(RwLockReadGuard)` if the lock was acquired, `None` otherwise.
    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        let old = self.state.load(Ordering::Acquire);
        if old & WRITER_BIT != 0 {
            return None;
        }
        let new = old + 1;
        if new & WRITER_BIT == 0
            && self
                .state
                .compare_exchange(old, new, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
        {
            Some(RwLockReadGuard { lock: self })
        } else {
            None
        }
    }

    /// Acquires an exclusive (write) lock, spinning until it is available.
    ///
    /// This function will spin in a loop until the lock can be acquired for writing.
    /// It returns a `RwLockWriteGuard` that dereferences to `&mut T` and releases
    /// the lock when dropped.
    ///
    /// # Returns
    /// A `RwLockWriteGuard` that provides exclusive access to the data.
    ///
    /// # Deadlocks
    /// This function will deadlock if the lock is already held by the current
    /// thread for reading or writing.
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        loop {
            let old = self.state.load(Ordering::Acquire);
            // If the lock is not free, spin.
            if old != 0 {
                hint::spin_loop();
                continue;
            }
            // Try to set the writer bit.
            if self
                .state
                .compare_exchange_weak(0, WRITER_BIT, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
            {
                return RwLockWriteGuard { lock: self };
            }
        }
    }

    /// Attempts to acquire an exclusive (write) lock without spinning.
    ///
    /// If the lock is free, it is acquired and a guard is returned.
    /// Otherwise, `None` is returned immediately.
    ///
    /// # Returns
    /// `Some(RwLockWriteGuard)` if the lock was acquired, `None` otherwise.
    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        let old = self.state.load(Ordering::Acquire);
        if old != 0 {
            return None;
        }
        if self
            .state
            .compare_exchange(0, WRITER_BIT, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
        {
            Some(RwLockWriteGuard { lock: self })
        } else {
            None
        }
    }
}

// ============================================================================
// READ GUARD
// ============================================================================

/// A guard that holds a shared (read) lock on an `RwLock`.
///
/// The guard dereferences to `T` (via `Deref`) and releases the lock when dropped.
pub struct RwLockReadGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<T> core::ops::Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    /// Returns a reference to the protected data.
    ///
    /// # Safety
    /// The read lock is held, so the data is not being mutated.
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> Drop for RwLockReadGuard<'_, T> {
    /// Releases the read lock by decrementing the reader count.
    fn drop(&mut self) {
        // Decrement the reader count with Release ordering to ensure that all
        // reads are visible before any future writes.
        self.lock.state.fetch_sub(1, Ordering::Release);
    }
}

// ============================================================================
// WRITE GUARD
// ============================================================================

/// A guard that holds an exclusive (write) lock on an `RwLock`.
///
/// The guard dereferences to `&mut T` (via `DerefMut`) and releases the lock
/// when dropped.
pub struct RwLockWriteGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<T> core::ops::Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    /// Returns a reference to the protected data.
    ///
    /// # Safety
    /// The write lock is held exclusively, so no other access is possible.
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> core::ops::DerefMut for RwLockWriteGuard<'_, T> {
    /// Returns a mutable reference to the protected data.
    ///
    /// # Safety
    /// The write lock is held exclusively, so mutation is safe.
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for RwLockWriteGuard<'_, T> {
    /// Releases the write lock by clearing the state.
    fn drop(&mut self) {
        // Release the write lock with Release ordering.
        self.lock.state.store(0, Ordering::Release);
    }
}

// ============================================================================
// TESTS (only run in std environment)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use core::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn smoke() {
        let lock = RwLock::new(42);
        {
            let r = lock.read();
            assert_eq!(*r, 42);
        }
        {
            let mut w = lock.write();
            *w = 43;
        }
        assert_eq!(*lock.read(), 43);
    }

    #[test]
    fn try_lock() {
        let lock = RwLock::new(0);
        assert!(lock.try_read().is_some());
        assert!(lock.try_write().is_some());
        // Write lock held
        let _w = lock.write();
        assert!(lock.try_read().is_none());
        assert!(lock.try_write().is_none());
        drop(_w);
        // Read lock held
        let _r = lock.read();
        assert!(lock.try_read().is_some());
        assert!(lock.try_write().is_none());
    }

    #[test]
    fn concurrent_readers() {
        let lock = RwLock::new(0);
        let counter = AtomicU32::new(0);
        let mut handles = vec![];
        for _ in 0..10 {
            let lock = &lock;
            let counter = &counter;
            handles.push(std::thread::spawn(move || {
                for _ in 0..100 {
                    let _g = lock.read();
                    counter.fetch_add(1, Ordering::Relaxed);
                    // Simulate some work
                    core::hint::spin_loop();
                    counter.fetch_sub(1, Ordering::Relaxed);
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn writer_exclusive() {
        let lock = RwLock::new(0);
        let counter = AtomicU32::new(0);
        let mut handles = vec![];
        // One writer
        handles.push(std::thread::spawn(move || {
            for _ in 0..50 {
                let _g = lock.write();
                counter.fetch_add(1, Ordering::Relaxed);
                core::hint::spin_loop();
                counter.fetch_sub(1, Ordering::Relaxed);
            }
        }));
        // Several readers
        for _ in 0..5 {
            let lock = &lock;
            let counter = &counter;
            handles.push(std::thread::spawn(move || {
                for _ in 0..50 {
                    let _g = lock.read();
                    // Readers should see counter at 0 if writer exclusive
                    assert_eq!(counter.load(Ordering::Relaxed), 0);
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
    }
}

```

### `src/sync/litex.rs`

```rs
//! # Litex – Interrupt-Disabling Spinlock with Unsafe Inner Access
//!
//! A mutual exclusion primitive that combines the features of `Nutex` (interrupt
//! disabling) with an additional `unsafe inner()` method for raw access to the
//! protected data without locking. It is a spinlock that disables interrupts
//! when acquired and restores them when released.
//!
//! ## Overview
//!
//! The `Litex` is very similar to `Nutex` but provides an `unsafe inner()` method
//! that returns a mutable reference to the data without acquiring the lock. This
//! is useful in situations where the caller can guarantee that no other code is
//! accessing the data concurrently (e.g., during early boot, or when the lock is
//! already held and the caller wants to bypass the guard).
//!
//! ## Characteristics
//!
//! - **Interrupt disabling**: Disables interrupts (`cli`) on lock acquisition and
//!   restores the previous state on release.
//! - **Spinlock**: Spins in a tight loop (`spin_loop`) if the lock is already held.
//! - **Unsafe inner access**: Provides an `unsafe inner()` method to get a mutable
//!   reference to the data without locking. This is useful for initialization or
//!   when the lock is already known to be held.
//! - **No recursion**: Does not support recursive locking; attempting to lock the
//!   same `Litex` from the same CPU will cause a deadlock.
//!
//! ## Usage
//!
//! ```ignore
//! use crate::sync::Litex;
//!
//! static SHARED: Litex<u32> = Litex::new(0);
//!
//! // Safe locking:
//! let mut guard = SHARED.lock();
//! *guard += 1;
//!
//! // Unsafe inner access (only safe if no concurrent access):
//! unsafe {
//!     *SHARED.inner() = 42;
//! }
//! ```
//!
//! ## Safety
//!
//! - The `inner()` method is unsafe because it bypasses the lock. The caller must
//!   ensure that no other code is accessing the data concurrently (e.g., during
//!   early boot, or when the lock is already held).
//! - The lock is `Send` and `Sync` when `T` is `Send`.
//! - Interrupts are disabled on the current CPU, but other CPUs are not affected.
//! - The saved interrupt state ensures that interrupts are restored correctly
//!   on drop, even if they were previously disabled.
//!
//! ## Comparison with Other Synchronization Primitives
//!
//! | Primitive | Interrupts Disabled | Spins | Unsafe Inner | Multi‑CPU Safe |
//! |-----------|---------------------|-------|--------------|----------------|
//! | `Mutex`   | No                  | Yes   | No           | Yes            |
//! | `Nutex`   | Yes                 | Yes   | No           | Yes            |
//! | `Litex`   | Yes                 | Yes   | Yes          | Yes            |
//! | `Nitex`   | Yes                 | No    | Yes          | No (per‑CPU)   |
//!
//! ## Implementation Details
//!
//! The lock uses `AtomicBool` as the spinlock flag. On acquisition:
//! 1. The current interrupt state is saved using `pushfq`/`pop`.
//! 2. Interrupts are disabled with `cli`.
//! 3. The lock flag is set to `true` using a spinloop with `compare_exchange_weak`.
//! 4. On release, the lock flag is cleared (`store(false)`), and the saved
//!    interrupt state is restored (if interrupts were previously enabled, `sti` is
//!    executed).
//!
//! The `inner()` method simply returns a mutable reference to the `UnsafeCell`
//! data without any atomic operations.

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
    arch::asm
};

// ============================================================================
// LITEX STRUCTURE
// ============================================================================

/// An interrupt‑disabling spinlock with unsafe inner access.
///
/// # Type Parameters
/// * `T` – The type of the data protected by the lock.
///
/// # Examples
/// ```ignore
/// static COUNTER: Litex<usize> = Litex::new(0);
///
/// fn increment() {
///     let mut guard = COUNTER.lock();
///     *guard += 1;
/// }
///
/// // Unsafe, but safe if called during early boot:
/// unsafe {
///     *COUNTER.inner() = 100;
/// }
/// ```
#[derive(Debug)]
pub struct Litex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

// Safety: Litex is Send and Sync if T is Send.
unsafe impl<T: Send> Send for Litex<T> {}
unsafe impl<T: Send> Sync for Litex<T> {}

impl<T> Litex<T> {
    /// Creates a new `Litex` in the unlocked state.
    ///
    /// # Arguments
    /// * `t` – The initial value to protect.
    pub const fn new(t: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(t),
        }
    }

    /// Returns a mutable reference to the inner data **without locking**.
    ///
    /// # Safety
    /// This is unsafe because it bypasses the lock. The caller must ensure that
    /// no other code is accessing the data concurrently. This is typically safe
    /// during early boot (single‑threaded) or when the lock is already held.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn inner(&self) -> &mut T {
        unsafe {
            self.data.as_mut_unchecked()
        }
    }

    /// Acquires the lock, disabling interrupts and spinning until the lock is available.
    ///
    /// This function:
    /// 1. Saves the current interrupt state.
    /// 2. Disables interrupts with `cli`.
    /// 3. Spins until the lock flag is acquired.
    /// 4. Returns a guard that will release the lock and restore the interrupt
    ///    state when dropped.
    ///
    /// # Returns
    /// A `LitexGuard` that dereferences to the protected data.
    ///
    /// # Deadlocks
    /// This function will deadlock if the lock is already held by the current CPU.
    pub fn lock(&self) -> LitexGuard<'_, T> {
        // Save the current interrupt state before we disable interrupts.
        let rflags: u64;
        unsafe {
            asm!(
                "pushfq",
                "pop {0}",
                out(reg) rflags,
                options(nomem, preserves_flags)
            );
            asm!("cli", options(nomem, nostack, preserves_flags));
        }

        // Spin until we acquire the lock.
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }

        LitexGuard {
            mutex: self,
            saved_if: (rflags & (1 << 9)) != 0,  // Bit 9 is the IF flag in RFLAGS.
        }
    }

    /// Attempts to acquire the lock without spinning.
    ///
    /// If the lock is free, it is acquired, interrupts are disabled, and a guard
    /// is returned. If the lock is already held, `None` is returned immediately.
    ///
    /// # Returns
    /// `Some(LitexGuard)` if the lock was acquired, `None` otherwise.
    pub fn try_lock(&self) -> Option<LitexGuard<'_, T>> {
        // Try to acquire the lock with a single CAS attempt.
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            // Save the current interrupt state and disable interrupts.
            let rflags: u64;
            unsafe {
                asm!(
                    "pushfq",
                    "pop {0}",
                    out(reg) rflags,
                    options(nomem, preserves_flags)
                );
                asm!("cli", options(nomem, nostack, preserves_flags));
            }
            Some(LitexGuard {
                mutex: self,
                saved_if: (rflags & (1 << 9)) != 0,
            })
        } else {
            None
        }
    }
}

// ============================================================================
// GUARD STRUCTURE
// ============================================================================

/// A guard that holds the `Litex` lock and provides access to the protected data.
///
/// The guard dereferences to `T` (via `Deref` and `DerefMut`) and releases the
/// lock and restores the interrupt state when dropped.
pub struct LitexGuard<'a, T> {
    mutex: &'a Litex<T>,
    saved_if: bool,  // Whether interrupts were enabled before locking.
}

impl<T> core::ops::Deref for LitexGuard<'_, T> {
    type Target = T;

    /// Returns a reference to the protected data.
    ///
    /// # Safety
    /// The lock is held and interrupts are disabled, so the data is safe to read.
    fn deref(&self) -> &T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> core::ops::DerefMut for LitexGuard<'_, T> {
    /// Returns a mutable reference to the protected data.
    ///
    /// # Safety
    /// The lock is held exclusively and interrupts are disabled.
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for LitexGuard<'_, T> {
    /// Releases the lock and restores the interrupt state.
    fn drop(&mut self) {
        // Release the lock.
        self.mutex.lock.store(false, Ordering::Release);

        // Restore the interrupt state if it was previously enabled.
        unsafe {
            if self.saved_if {
                asm!("sti", options(nomem, nostack, preserves_flags));
            }
        }
    }
}

```

### `src/sync/barrier.rs`

```rs
//! # Barrier – A Simple Spin Barrier (Flag)
//!
//! A minimal synchronization primitive that acts as a one‑time barrier or gate.
//! It allows one or more tasks to wait until the barrier is "opened" by another task.
//!
//! ## Overview
//!
//! `Barrier` is a simple flag‑based synchronization primitive. It has two states:
//! - **Closed** (`false`): Tasks calling `wait()` will spin‑loop until the flag
//!   becomes `true`.
//! - **Open** (`true`): Tasks calling `wait()` will return immediately; the
//!   barrier has been passed.
//!
//! This primitive is useful for coordinating the boot sequence of multiple
//! CPU cores, where the BSP (Bootstrap Processor) initializes subsystems and
//! then signals APs (Application Processors) to proceed. In the kernel, it is
//! used by the `Barrier!` macro to create barriers like `ARCH_INIT`, `MEM_INIT`,
//! `LATE_INIT`, and `DEV_INIT`.
//!
//! ## Characteristics
//!
//! - **Spin‑wait**: `wait()` spins in a tight loop (`spin_loop`) until the flag
//!   is open. This is suitable for short‑duration waits in early boot, where
//!   other synchronization primitives (like wait queues) are not yet available.
//! - **One‑time use**: Once opened, the barrier cannot be closed again (though
//!   `close()` is provided, it is typically not used).
//! - **No locking**: The flag is an `AtomicBool` with `Acquire`/`Release` ordering
//!   to ensure visibility across CPUs.
//! - **No sleeping**: `wait()` does not block or yield; it busy‑waits.
//!
//! ## Usage
//!
//! ```ignore
//! use crate::sync::Barrier;
//!
//! static BARRIER: Barrier = Barrier::new();
//!
//! // On BSP:
//! BARRIER.open();  // Signal that initialization is complete.
//!
//! // On AP:
//! BARRIER.wait();  // Spin until BSP opens the barrier.
//! ```
//!
//! The `Barrier!` macro creates multiple static `Barrier` instances:
//! ```ignore
//! Barrier! { ARCH_INIT MEM_INIT LATE_INIT DEV_INIT }
//! ```
//! This expands to:
//! ```ignore
//! static ARCH_INIT: Barrier = Barrier::new();
//! static MEM_INIT: Barrier = Barrier::new();
//! // etc.
//! ```
//!
//! ## Safety
//!
//! - The `AtomicBool` operations use `Acquire`/`Release` ordering to ensure
//!   that all writes performed before `open()` are visible to any CPU that
//!   observes the flag as `true` (via `wait()`).
//! - The primitive is `Sync` and can be accessed from multiple CPUs safely.
//! - `wait()` does not disable interrupts; it is safe to use in interrupt
//!   handlers, though spinning in an interrupt handler is generally discouraged.
//!
//! ## Comparison with Other Primitives
//!
//! | Primitive | Purpose | Sleeps | Spin | One‑time |
//! |-----------|---------|--------|------|----------|
//! | `Barrier`   | Barrier | No     | Yes  | Yes      |
//! | `WaitQueue` | Blocking queue | Yes (via scheduler) | No | No |
//! | `Mutex`   | Mutual exclusion | No (spins) | Yes | No |
//! | `Litex`   | Mutual exclusion with interrupt disable | No (spins) | Yes | No |

use core::sync::atomic::{AtomicBool, Ordering};
use core::hint;

// ============================================================================
// Barrier STRUCTURE
// ============================================================================

/// A simple spin barrier (flag) that can be opened once.
///
/// Tasks waiting on the barrier will spin until it is opened.
pub struct Barrier {
    open: AtomicBool,
}

impl Barrier {
    /// Creates a new `Barrier` in the closed state.
    pub const fn new() -> Self {
        Self {
            open: AtomicBool::new(false),
        }
    }

    /// Opens the barrier, allowing all waiting tasks to proceed.
    ///
    /// This stores `true` with `Release` ordering, ensuring that all previous
    /// writes are visible to tasks that later observe the flag as `true`.
    pub fn open(&self) {
        self.open.store(true, Ordering::Release);
    }

    /// Closes the barrier (sets the flag to `false`).
    ///
    /// This is rarely used; typically barriers are only opened once.
    pub fn close(&self) {
        self.open.store(false, Ordering::Release);
    }

    /// Returns `true` if the barrier is open.
    ///
    /// This loads the flag with `Acquire` ordering to ensure that any writes
    /// performed before `open()` are visible to the caller.
    pub fn is_open(&self) -> bool {
        self.open.load(Ordering::Acquire)
    }

    /// Spins until the barrier is opened.
    ///
    /// This function will loop, calling `spin_loop()` on each iteration, until
    /// the flag is set to `true` by another task.
    ///
    /// # Examples
    /// ```ignore
    /// static BARRIER: Barrier = Barrier::new();
    ///
    /// // In AP boot:
    /// BARRIER.wait();  // Spin until BSP opens.
    /// ```
    pub fn wait(&self) {
        while !self.is_open() {
            hint::spin_loop();
        }
    }
}

```

### `src/arch/mod.rs`

```rs
//! # Architecture‑Specific Code (x86_64)
//!
//! This module acts as the architecture abstraction layer for the kernel.
//! It re‑exports all architecture‑specific functionality from the `amd64`
//! submodule and provides a small set of generic helper functions that are
//! independent of the underlying CPU architecture.
//!
//! ## Overview
//!
//! The kernel is designed to be portable across multiple CPU architectures.
//! However, the current implementation targets only x86_64. The `arch` module
//! serves as a facade that abstracts the low‑level hardware details.
//! All architecture‑dependent code is contained within the `amd64` submodule,
//! and this module re‑exports its public interface.
//!
//! When porting the kernel to another architecture (e.g., ARM64, RISC‑V), this
//! module would conditionally include the appropriate submodule and provide
//! the same public API, allowing the rest of the kernel to remain architecture‑
//! agnostic.
//!
//! ## Re‑exports
//!
//! The module re‑exports all public items from `amd64`, including:
//! - CPU initialisation (`init_bsp`, `init_ap`, `early_init`)
//! - Interrupt handling (IDT, exceptions)
//! - Memory management (paging, per‑CPU data, GDT)
//! - System calls
//! - Timers (HPET, APIC)
//! - ACPI support
//! - Trap frame
//!
//! ## Helper Functions
//!
//! The module provides a simple `blocking_sleep` function that busy‑waits for
//! a given number of seconds. This is used for early‑boot delays and should be
//! replaced with a proper scheduler‑based sleep in the future.

// ============================================================================
// ARCHITECTURE SELECTION
// ============================================================================

#[cfg(target_arch = "x86_64")]
mod amd64;

// ============================================================================
// RE‑EXPORTS
// ============================================================================

#[cfg(target_arch = "x86_64")]
pub use amd64::*;

// ============================================================================
// GENERIC HELPERS
// ============================================================================

/// Busy‑waits for a given number of seconds.
///
/// This function is a simple blocking sleep that spins in a tight loop,
/// repeatedly checking the system time. It is intended for use during early
/// boot, before the scheduler is fully initialised and proper sleep primitives
/// are available.
///
/// # Arguments
/// * `s` – The number of seconds to sleep.
///
/// # Note
/// This function is not suitable for long‑duration sleeps or production use,
/// as it consumes CPU resources and does not yield to other tasks. It should
/// be replaced with a scheduler‑based sleep mechanism in the future.
///
/// # Examples
/// ```ignore
/// // Wait for 2 seconds during device initialisation.
/// crate::arch::blocking_sleep(2.0);
/// ```
pub fn blocking_sleep(s: f32) {
    let start = get_time_from_boot_s();
    while get_time_from_boot_s() < start + s {
        core::hint::spin_loop();
    }
}

```

### `src/arch/amd64/percpu.rs`

```rs
//! # Per‑CPU Data Management
//!
//! This module provides infrastructure for per‑CPU data structures on x86_64.
//! Each CPU core has its own dedicated data region, accessible via the `gs`
//! segment register. This allows efficient, lock‑free access to CPU‑local
//! data such as the current CPU ID, kernel stack top, and user stack pointer.
//!
//! ## Overview
//!
//! On multi‑core systems, many data structures need to be per‑CPU to avoid
//! contention and to maintain correctness. Examples include:
//! - Current CPU ID.
//! - Kernel stack top for the current task (for syscall entry).
//! - User stack pointer (for returning to user mode).
//! - Per‑CPU runqueues and caches.
//!
//! The x86_64 architecture provides the `gs` segment register, which can be
//! set to point to a per‑CPU data region via the `IA32_GS_BASE` and
//! `IA32_KERNEL_GS_BASE` MSRs. This module manages those MSRs and provides
//! a convenient interface to the per‑CPU data.
//!
//! ## Structure
//!
//! The per‑CPU data is defined in the `PerCpu` struct:
//!
//! ```text
//! struct PerCpu {
//!     user_rsp: u64,           // User stack pointer (for syscall return)
//!     kernel_stack_top: u64,   // Kernel stack top (for syscall entry)
//!     cpu_id: usize,           // CPU ID (0 .. MAX_CPUS-1)
//! }
//! ```
//!
//! This structure is stored in a static array `PERCPU_AREA`, one entry per CPU.
//! The size of the array is `MAX_CPUS` (currently 64). The structure is
//! cache‑line aligned (64 bytes) to prevent false sharing.
//!
//! ## Accessing Per‑CPU Data
//!
//! The per‑CPU data for the current CPU is accessed via `percpu::current()`.
//! This function:
//! 1. Calls `arch::current_cpu()` to get the current CPU ID (using `rdpid`
//!    or `rdmsr(IA32_TSC_AUX)`).
//! 2. Returns a mutable reference to `PERCPU_ARRAY[cpu_id]`.
//!
//! The `gs` segment is set up by the `percpu::init_syscall_gs()` function,
//! which writes `IA32_KERNEL_GS_BASE` to point to the per‑CPU data for that
//! core. The `swapgs` instruction is then used by the syscall entry/exit
//! handlers to switch between the user `gs` and the kernel `gs`.
//!
//! ## Initialization
//!
//! - **BSP**: `percpu::init()` is called during `arch::init_bsp()`. It sets
//!   `cpu_id` for CPU 0.
//! - **APs**: `percpu::init()` is called during `arch::init_ap()` for each AP.
//!   It sets `cpu_id` for that core.
//! - **Syscall GS**: `percpu::init_syscall_gs(cpu_id, kernel_stack_top)` is
//!   called during both BSP and AP init. It sets `IA32_KERNEL_GS_BASE` to
//!   point to the per‑CPU data for that core.
//!
//! ## Safety
//!
//! - The `PERCPU_ARRAY` is `static mut` and is accessed via raw pointers in
//!   `current()`. This is safe because each CPU accesses only its own entry,
//!   and the array is never deallocated.
//! - The MSR writes (`wrmsr`) are privileged operations and require that the
//!   kernel is running in Ring 0.
//! - The `gs` segment is used in interrupt and syscall handlers; the `swapgs`
//!   instruction must be used correctly to switch between user and kernel GS.

use crate::arch::current_cpu;
use crate::arch::MAX_CPUS;

// ============================================================================
// PER‑CPU STRUCTURE
// ============================================================================

/// Per‑CPU data structure.
///
/// This structure holds CPU‑local information. It is aligned to 64 bytes
/// to avoid false sharing between CPU cores.
///
/// # Fields
/// - `user_rsp`: The user‑mode stack pointer. Set when returning to user space.
/// - `kernel_stack_top`: The top of the kernel stack for the current task.
///   Used by the syscall entry handler to switch to the kernel stack.
/// - `cpu_id`: The ID of the current CPU (0 .. MAX_CPUS-1).
#[repr(C, align(64))]
#[derive(Debug)]
pub struct PerCpu {
    pub user_rsp: u64,
    pub kernel_stack_top: u64,
    pub cpu_id: usize,
}

impl PerCpu {
    /// Creates a new, zero‑initialized `PerCpu` structure.
    pub const fn new() -> Self {
        Self {
            cpu_id: 0,
            kernel_stack_top: 0,
            user_rsp: 0,
        }
    }
}

// ============================================================================
// GLOBAL PER‑CPU ARRAY
// ============================================================================

/// Static array of per‑CPU data, one entry per CPU.
///
/// This array is indexed by CPU ID. The entries are initialized to zero
/// and are filled in during CPU initialization.
///
/// # Safety
/// This is `static mut`; it is written during early boot (single‑threaded)
/// and read thereafter in a per‑CPU manner.
static mut PERCPU_AREA: [PerCpu; MAX_CPUS] = [const { PerCpu::new() }; MAX_CPUS];

// ============================================================================
// INITIALIZATION
// ============================================================================

/// Initializes the per‑CPU data for the current CPU.
///
/// This function sets the `cpu_id` field of the current CPU's `PerCpu` entry.
/// It is called during both BSP and AP initialization.
///
/// # Safety
/// This function uses `current_cpu()` to get the CPU ID and mutably borrows
/// the `PERCPU_ARRAY` entry. It is safe because it is called only once per CPU
/// during boot, with interrupts disabled.
pub fn init() {
    current().cpu_id = current_cpu();
}

// ============================================================================
// PUBLIC API
// ============================================================================

/// Returns a mutable reference to the per‑CPU data for the current CPU.
///
/// This function:
/// 1. Calls `arch::current_cpu()` to get the current CPU ID.
/// 2. Returns a reference to `PERCPU_ARRAY[cpu_id]`.
///
/// # Panics
/// Panics if the CPU ID is out of bounds (>= MAX_CPUS).
///
/// # Safety
/// The returned reference is `'static` and mutable. It is safe because:
/// - Each CPU accesses only its own entry.
/// - The array is never deallocated or moved.
/// - The reference is used only within the context of the current CPU.
#[inline(always)]
pub fn current() -> &'static mut PerCpu {
    let cpu_id = current_cpu();
    debug_assert!(
        cpu_id < MAX_CPUS,
        "CPU ID {} exceeds MAX_CPUS ({})",
        cpu_id,
        MAX_CPUS
    );

    #[allow(static_mut_refs)]
    unsafe {
        &mut *PERCPU_AREA.as_mut_ptr().add(cpu_id)
    }
}

/// Sets up the `IA32_KERNEL_GS_BASE` MSR for the current CPU.
///
/// This function:
/// 1. Writes the per‑CPU data address for the current CPU into `IA32_KERNEL_GS_BASE`.
/// 2. Also sets `kernel_stack_top` in the per‑CPU data.
///
/// The `IA32_KERNEL_GS_BASE` MSR holds the base address of the kernel's `gs`
/// segment. On syscall entry, the `swapgs` instruction switches from the
/// user `gs` to this kernel `gs`, making the per‑CPU data accessible.
///
/// # Arguments
/// * `cpu_id` – The CPU core index.
/// * `kernel_stack_top` – The top of the kernel stack for this CPU.
///
/// # Panics
/// Panics if `cpu_id >= MAX_CPUS`.
///
/// # Safety
/// This function uses `wrmsr` to write an MSR, which is a privileged operation.
pub fn init_syscall_gs(cpu_id: usize, kernel_stack_top: u64) {
    unsafe {
        PERCPU_AREA[cpu_id].kernel_stack_top = kernel_stack_top;
        // IA32_KERNEL_GS_BASE (0xC0000102) points to our per‑CPU data.
        crate::arch::wrmsr(0xC0000102, &PERCPU_AREA[cpu_id] as *const _ as u64);
    }
}

/// Sets the kernel stack top in the per‑CPU data for the current CPU.
///
/// This is used by the scheduler when switching to a new task to update
/// the kernel stack pointer that will be used on the next syscall or interrupt.
///
/// # Arguments
/// * `stack_top` – The top address of the kernel stack (the stack grows down).
pub fn set_kernel_stack(stack_top: u64) {
    current().kernel_stack_top = stack_top;
}

```

### `src/arch/amd64/paging.rs`

```rs
//! # x86_64 Paging (4‑Level Page Tables)
//!
//! This module implements the kernel's 4‑level paging infrastructure for the x86_64
//! architecture. It provides low‑level page table manipulation, including mapping,
//! unmapping, and the use of huge pages (2 MiB and 1 GiB) for performance.
//!
//! ## Overview
//!
//! The x86_64 architecture uses 4 levels of page tables:
//! - **PML4** (Page Map Level 4) – top level, 512 entries, each pointing to a PDPT.
//! - **PDPT** (Page Directory Pointer Table) – second level, 512 entries, each
//!   pointing to a PD (or a 1 GiB huge page).
//! - **PD** (Page Directory) – third level, 512 entries, each pointing to a PT
//!   (or a 2 MiB huge page).
//! - **PT** (Page Table) – fourth level, 512 entries, each pointing to a 4 KiB page.
//!
//! Each entry is a 64‑bit value that contains the physical address of the next
//! level table (or the page frame) and various flags (present, writable, user,
//! no‑execute, etc.).
//!
//! ## Key Abstractions
//!
//! - **`Entry`**: A 64‑bit page table entry with methods to read/write address and flags.
//! - **`Tab`**: A page table (array of 512 `Entry`s), aligned to 4 KiB.
//! - **`Exco`**: An execution context that holds a CR3 value and a reference to
//!   the root PML4 table. It provides methods for mapping, unmapping, splitting,
//!   merging, and reporting the page table structure.
//! - **`Area`**: A contiguous range of virtual addresses with the same flags,
//!   used for reporting mapped regions.
//!
//! ## Huge Pages
//!
//! The kernel supports 2 MiB and 1 GiB huge pages to reduce TLB pressure and
//! improve performance. Huge pages are used when:
//! - The virtual address is aligned to the huge page size.
//! - The physical address is aligned to the huge page size.
//! - The memory region is contiguous (checked via PMR).
//!
//! The `Exco` provides `map2m`/`map1g` and `try_merge2m`/`try_merge1g` methods
//! to manage huge pages. The `split2m`/`split1g` methods break huge pages into
//! smaller pages when needed (e.g., for partial unmapping or changing permissions).
//!
//! ## Allocation and Freeing of Page Tables
//!
//! Page tables are allocated from the physical memory allocator (`upa`) and are
//! zero‑initialised. The `alloc_tab_zeroed()` function allocates a 4 KiB page
//! and returns a mutable reference to it as a `Tab`. The `free_tab()` function
//! frees a page table back to the allocator.
//!
//! ## Walking the Page Tables
//!
//! The `walk_entry` and `walk_entry_mut` functions traverse the page tables for
//! a given virtual address, returning the entry at the specified level. The
//! `walk_entry_mut` variant can optionally create missing intermediate tables.
//!
//! ## Execution Context (`Exco`)
//!
//! An `Exco` represents a page table context (an address space). It contains:
//! - `cr3`: The physical address of the root PML4 (loaded into CR3).
//! - `root`: A mutable reference to the PML4 table.
//! - `owned`: Whether this context owns the page tables (for cleanup).
//!
//! The `Exco` provides safe methods to manipulate the page tables and to
//! activate the context (load CR3). It also supports duplication (`dup()`)
//! for creating a new address space (copy‑on‑write is handled separately).
//!
//! ## Reporting and Debugging
//!
//! The `report<const N>` method traverses the page tables and returns a
//! `Vec<Area, N>` of contiguous mapped regions, grouped by flags. This is
//! useful for debugging and for the `Vmm` module to track memory usage.
//!
//! ## Safety
//!
//! - Most functions in this module are `unsafe` because they manipulate
//!   page tables, which directly affect memory access and CPU behaviour.
//! - The `Exco::activate()` function uses inline assembly to load CR3.
//! - The `walk_entry_mut` function uses raw pointers to modify page tables.
//! - The `alloc_tab_zeroed` and `free_tab` functions interact with the
//!   physical memory allocator.
//! - The `transmute` in `try_merge` is safe because the functions are
//!   called with the correct arguments.

use crate::mem::kdm::Paddr;

// ============================================================================
// PAGE TABLE ENTRY FLAGS
// ============================================================================

bitflags! {
    /// Flags for page table entries.
    ///
    /// These bits control access permissions, caching, and other attributes
    /// of the page or page table.
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct EntryFlags: u64 {
        /// The page is present in memory.
        const PRESENT         = 1 <<  0;
        /// The page is writable (for kernel mode, or user if `USER_ACCESSIBLE`).
        const WRITABLE        = 1 <<  1;
        /// The page is accessible from user mode (CPL 3).
        const USER_ACCESSIBLE = 1 <<  2;
        /// Write‑through caching (vs. write‑back).
        const WRITE_THROUGH   = 1 <<  3;
        /// Cache disabled for this page.
        const CACHE_DISABLE   = 1 <<  4;
        /// The page has been accessed (set by hardware).
        const ACCESSED        = 1 <<  5;
        /// The page has been written to (set by hardware).
        const DIRTY           = 1 <<  6;
        /// The entry points to a huge page (2 MiB or 1 GiB).
        const HUGE_PAGE       = 1 <<  7;
        /// The page is global (not flushed on CR3 switch).
        const GLOBAL          = 1 <<  8;
        /// Execute disable (NX bit) – the page cannot be executed.
        const NO_EXECUTE      = 1 << 63;

        // Kernel‑specific software‑managed flags (stored in available bits).
        /// Copy‑on‑write flag (used by the scheduler).
        const COPY_ON_WRITE   = 1 << 52;
        /// File‑mapped flag (for mmap).
        const FILE_MAPPED     = 1 << 53;
        /// Swapped flag (page is swapped out).
        const SWAPPED         = 1 << 54;
    }
}

// ============================================================================
// PAGE TABLE ENTRY
// ============================================================================

/// A 64‑bit page table entry.
///
/// This struct wraps a `u64` and provides methods to manipulate the address
/// and flags of the entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Entry(u64);

impl Entry {
    /// Mask for the physical address bits (bits 12‑51).
    pub const ADDRESS_MASK: u64 = 0x000f_ffff_ffff_f000;

    /// Mask for the available bits (bits 52‑62).
    pub const AVAILABLE_MASK: u64 = 0x7ff0_0000_0000_0e00;

    /// Shift for the physical address (12 bits).
    pub const ADDRESS_SHIFT: u32 = 12;

    /// Creates a new entry with the given physical address and flags.
    #[inline]
    pub const fn new(physical_address: Paddr, flags: EntryFlags) -> Self {
        let addr_part = physical_address.to_raw() as u64 & Self::ADDRESS_MASK;
        Self(addr_part | flags.bits())
    }

    /// Returns the raw `u64` value of the entry.
    #[inline]
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    /// Returns the flags of the entry.
    #[inline]
    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.0)
    }

    /// Sets the flags of the entry, preserving the address bits.
    #[inline]
    pub fn set_flags(&mut self, flags: EntryFlags) {
        self.0 = (self.0 & !EntryFlags::all().bits()) | flags.bits();
    }

    /// Returns the physical address stored in the entry.
    #[inline]
    pub fn address(&self) -> Paddr {
        Paddr::from_raw((self.0 & Self::ADDRESS_MASK) as usize)
    }

    /// Sets the physical address of the entry, preserving the flags.
    #[inline]
    pub fn set_address(&mut self, paddr: Paddr) {
        let addr = paddr.to_raw() as u64;
        self.0 = (self.0 & !Self::ADDRESS_MASK) | (addr & Self::ADDRESS_MASK);
    }

    /// Returns `true` if the entry is present.
    #[inline]
    pub fn is_present(&self) -> bool {
        self.flags().contains(EntryFlags::PRESENT)
    }

    /// Returns `true` if the entry is writable.
    #[inline]
    pub fn is_writable(&self) -> bool {
        self.flags().contains(EntryFlags::WRITABLE)
    }

    /// Returns `true` if the page is executable (NX bit is not set).
    #[inline]
    pub fn is_executable(&self) -> bool {
        !self.flags().contains(EntryFlags::NO_EXECUTE)
    }
}

impl From<u64> for Entry {
    #[inline]
    fn from(raw: u64) -> Self {
        Entry(raw)
    }
}

impl From<Entry> for u64 {
    #[inline]
    fn from(entry: Entry) -> Self {
        entry.0
    }
}

impl Default for Entry {
    #[inline]
    fn default() -> Self {
        Entry(0)
    }
}

// ============================================================================
// PAGE TABLE STRUCTURE
// ============================================================================

use core::ops::{Index, IndexMut};
use heapless::Vec;

use crate::mem::kdm::Vaddr;
use crate::mem::upa;

// ----------------------------------------------------------------------------
// HELPERS: INDEX FUNCTIONS
// ----------------------------------------------------------------------------

#[inline]
fn pml4_i(vaddr: usize) -> usize {
    (vaddr >> 39) & 0x1ff
}

#[inline]
fn pdpt_i(vaddr: usize) -> usize {
    (vaddr >> 30) & 0x1ff
}

#[inline]
fn pd_i(vaddr: usize) -> usize {
    (vaddr >> 21) & 0x1ff
}

#[inline]
fn pt_i(vaddr: usize) -> usize {
    (vaddr >> 12) & 0x1ff
}

// ----------------------------------------------------------------------------
// MASK CONSTANTS
// ----------------------------------------------------------------------------

const MASK_4K: usize = 0xfff;
const MASK_2M: usize = 0x1f_ffff;
const MASK_1G: usize = 0x3fff_ffff;

// ----------------------------------------------------------------------------
// PAGE TABLE ALLOCATION / DEALLOCATION
// ----------------------------------------------------------------------------

/// Allocates a zero‑initialised page table (4 KiB).
///
/// This function allocates a physical page from the UPA, maps it via the HHDM,
/// and returns a mutable reference to it as a `Tab`. The table is zeroed.
///
/// # Panics
/// Panics if the allocation fails.
pub fn alloc_tab_zeroed() -> &'static mut Tab {
    let paddr = upa::alloc(1);
    let vaddr = paddr.to_virt();
    let tab: &'static mut Tab = vaddr.to_ref_mut();
    for e in tab.0.iter_mut() {
        *e = Entry::default();
    }
    tab
}

/// Frees a page table by physical address.
///
/// # Arguments
/// * `paddr` – The physical address of the table to free.
fn free_tab(paddr: Paddr) {
    upa::free(paddr);
}

/// Returns a mutable reference to the table pointed to by an entry.
///
/// # Safety
/// The entry must point to a valid, mapped table.
pub fn tab_from_entry(entry: &Entry) -> &'static mut Tab {
    entry.address().to_virt().to_ref_mut()
}

/// Returns an immutable reference to the table pointed to by an entry.
///
/// # Safety
/// The entry must point to a valid, mapped table.
pub fn tab_from_entry_const(entry: &Entry) -> &Tab {
    entry.address().to_virt().to_ref()
}

/// Returns `true` if the entry is a huge page (2 MiB or 1 GiB).
#[inline]
pub fn is_huge(entry: &Entry) -> bool {
    entry.flags().contains(EntryFlags::HUGE_PAGE)
}

// ----------------------------------------------------------------------------
// TLB FUNCTIONS
// ----------------------------------------------------------------------------

/// Invalidates a single TLB entry for the given virtual address.
#[inline]
pub fn flush_tlb(vaddr: usize) {
    unsafe {
        core::arch::asm!("invlpg [{}]", in(reg) vaddr, options(nostack, preserves_flags));
    }
}

/// Invalidates all TLB entries (by reloading CR3).
#[allow(dead_code)]
pub fn flush_all(cr3: u64) {
    debug!("TLB flush-all CR3 {:#018X}", cr3);
    unsafe {
        core::arch::asm!("mov cr3, {}", in(reg) cr3, options(nostack));
    }
}

// ============================================================================
// PAGE TABLE WALKING
// ============================================================================

/// Walks the page tables and returns a mutable reference to the entry
/// at the given virtual address and level.
///
/// # Arguments
/// * `root` – The root PML4 table.
/// * `vaddr` – The virtual address to walk.
/// * `level_hint` – The level to return: 0 = PT entry, 1 = PD entry, 2 = PDPT entry.
/// * `create` – Whether to create missing intermediate tables.
///
/// # Returns
/// `Ok(&mut Entry)` if the entry exists (or was created), `Err` otherwise.
///
/// # Errors
/// - `PML4 entry not present` (if `create` is false and the PML4 entry is missing).
/// - `PDPT entry not present` (if `create` is false and the PDPT entry is missing).
/// - `PD entry not present` (if `create` is false and the PD entry is missing).
/// - `1 GiB huge page encountered` (if a 1 GiB page blocks the walk).
/// - `2 MiB huge page encountered` (if a 2 MiB page blocks the walk).
pub fn walk_entry_mut(
    root: &mut Tab,
    vaddr: usize,
    level_hint: u8,
    create: bool,
) -> Result<&mut Entry, &'static str> {
    let pml4_idx = pml4_i(vaddr);
    let pdpt_idx = pdpt_i(vaddr);
    let pd_idx   = pd_i(vaddr);
    let pt_idx   = pt_i(vaddr);

    let pml4_entry = &mut root.0[pml4_idx];
    if !pml4_entry.is_present() {
        if !create {
            return Err("PML4 entry not present");
        }
        let new_tab = alloc_tab_zeroed();
        *pml4_entry = Entry::new(
            Vaddr::from_ref(new_tab).to_phys(),
            EntryFlags::PRESENT | EntryFlags::WRITABLE,
        );
        debug!("Created PML4[{}] at {:#018X}", pml4_idx, pml4_entry.address().to_raw());
    }

    let pdpt = tab_from_entry(pml4_entry);
    let pdpt_entry = &mut pdpt.0[pdpt_idx];

    if level_hint == 2 {
        return Ok(pdpt_entry);
    }

    if !pdpt_entry.is_present() {
        if !create {
            return Err("PDPT entry not present");
        }
        let new_tab = alloc_tab_zeroed();
        *pdpt_entry = Entry::new(
            Vaddr::from_ref(new_tab).to_phys(),
            EntryFlags::PRESENT | EntryFlags::WRITABLE,
        );
        debug!("Created PDPT[{}] for vaddr {:#X}", pdpt_idx, vaddr);
    }
    if is_huge(pdpt_entry) {
        return Err("1 GiB huge page encountered - split first");
    }

    let pd = tab_from_entry(pdpt_entry);
    let pd_entry = &mut pd.0[pd_idx];

    if level_hint == 1 {
        return Ok(pd_entry);
    }

    if !pd_entry.is_present() {
        if !create {
            return Err("PD entry not present");
        }
        let new_tab = alloc_tab_zeroed();
        *pd_entry = Entry::new(
            Vaddr::from_ref(new_tab).to_phys(),
            EntryFlags::PRESENT | EntryFlags::WRITABLE,
        );
    }
    if is_huge(pd_entry) {
        return Err("2 MiB huge page encountered - split first");
    }

    let pt = tab_from_entry(pd_entry);
    let pt_entry = &mut pt.0[pt_idx];

    if !pt_entry.is_present() && !create {
        return Err("PT entry not present");
    }

    Ok(pt_entry)
}

/// Walks the page tables and returns an immutable reference to the entry
/// at the given virtual address and level.
///
/// # Arguments
/// * `root` – The root PML4 table.
/// * `vaddr` – The virtual address to walk.
/// * `level_hint` – The level to return: 0 = PT entry, 1 = PD entry, 2 = PDPT entry.
///
/// # Returns
/// `Ok(&Entry)` if the entry exists, `Err` otherwise.
pub fn walk_entry(
    root: &Tab,
    vaddr: usize,
    level_hint: u8,
) -> Result<&Entry, &'static str> {
    let pml4_idx = pml4_i(vaddr);
    let pdpt_idx = pdpt_i(vaddr);
    let pd_idx   = pd_i(vaddr);
    let pt_idx   = pt_i(vaddr);

    let pml4_entry = &root.0[pml4_idx];
    if !pml4_entry.is_present() {
        return Err("PML4 entry not present");
    }
    if is_huge(pml4_entry) {
        return Err("PML4 huge - unsupported");
    }

    let pdpt = tab_from_entry_const(pml4_entry);
    let pdpt_entry = &pdpt.0[pdpt_idx];
    if !pdpt_entry.is_present() {
        return Err("PDPT entry not present");
    }
    if level_hint == 2 {
        return Ok(pdpt_entry);
    }
    if is_huge(pdpt_entry) {
        return Err("1 GiB huge");
    }

    let pd = tab_from_entry_const(pdpt_entry);
    let pd_entry = &pd.0[pd_idx];
    if !pd_entry.is_present() {
        return Err("PD entry not present");
    }
    if level_hint == 1 {
        return Ok(pd_entry);
    }
    if is_huge(pd_entry) {
        return Err("2 MiB huge");
    }

    let pt = tab_from_entry_const(pd_entry);
    let pt_entry = &pt.0[pt_idx];
    if !pt_entry.is_present() {
        return Err("PT entry not present");
    }
    Ok(pt_entry)
}

// ============================================================================
// PAGE TABLE STRUCTURE (Tab)
// ============================================================================

/// A page table (4 KiB aligned array of 512 entries).
#[repr(align(4096))]
pub struct Tab(pub [Entry; 512]);

impl Tab {
    /// Creates a new, zero‑initialised page table.
    pub const fn new() -> Self {
        Self([
            const {
                Entry::new(
                    Paddr::from_raw(0),
                    EntryFlags::empty()
                )
            };
            512
        ])
    }
}

impl Index<usize> for Tab {
    type Output = Entry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Tab {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

// ============================================================================
// AREA (for reporting mapped regions)
// ============================================================================

/// A contiguous virtual address range with the same flags.
#[derive(Clone, Copy)]
pub struct Area {
    pub start: usize,
    pub count: usize,
    pub flags: EntryFlags,
}

impl core::fmt::Display for Area {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("Area at {:X} of {} KiB: {:?}", self.start, (self.count + 1023) >> 10, self.flags))
    }
}

// ============================================================================
// EXECUTION CONTEXT (Exco)
// ============================================================================

/// An execution context for a set of page tables.
///
/// This struct holds the CR3 value and a reference to the root PML4 table.
/// It provides methods for mapping, unmapping, and other page table operations.
pub struct Exco {
    pub cr3: u64,
    pub root: &'static mut Tab,
    pub owned: bool,
}

impl Exco {
    // ------------------------------------------------------------------------
    // CONSTRUCTORS
    // ------------------------------------------------------------------------

    /// Creates a new `Exco` from a root table, CR3, and ownership flag.
    pub const fn from_root(root: &'static mut Tab, cr3: u64, owned: bool) -> Self {
        Exco { cr3, root, owned }
    }

    /// Creates a new empty address space (allocates a new PML4).
    pub fn new() -> Self {
        let root = alloc_tab_zeroed();
        let cr3 = Vaddr::from_ref(&*root).to_phys().to_raw() as u64;
        info!("Exco::new   CR3 {:#018X} owned", cr3);
        Exco { cr3, root, owned: true }
    }

    /// Duplicates the current address space (copy‑on‑write style).
    ///
    /// This creates a new PML4 and copies all entries from the current table.
    /// Child tables are recursively duplicated.
    pub fn dup(&self) -> Self {
        let (root, cr3) = Self::dup_table(self.root);
        info!("Exco::dup   CR3 {:#018X} (src CR3 {:#018X})", cr3 as u64, self.cr3);
        Exco { cr3: cr3 as u64, root, owned: true }
    }

    /// Internal recursive function to duplicate a table.
    fn dup_table(table: &Tab) -> (&'static mut Tab, usize) {
        let new = alloc_tab_zeroed();
        for (i, entry) in table.0.iter().enumerate() {
            if entry.is_present() && !is_huge(entry) {
                let child = tab_from_entry(entry);
                let (_, child_paddr) = Self::dup_table(child);
                new.0[i] = Entry::new(Paddr::from_raw(child_paddr), entry.flags());
            } else if entry.is_present() {
                new.0[i] = *entry;
            }
        }
        let paddr = Vaddr::from_ref(&*new).to_phys().to_raw();
        (new, paddr)
    }

    /// Returns the current CPU's page table context.
    pub fn current() -> Self {
        let cr3_raw: u64;
        unsafe {
            core::arch::asm!("mov {}, cr3", out(reg) cr3_raw);
        }
        let phys = (cr3_raw & 0x000f_ffff_ffff_f000) as usize;
        let vaddr = Paddr::from_raw(phys).to_virt();
        let root: &'static mut Tab = vaddr.to_ref_mut();
        Exco { cr3: cr3_raw, root, owned: false }
    }

    /// Clears the page table (replaces it with a zeroed table).
    ///
    /// If the context is owned, the old tables are freed.
    pub fn clean(&mut self) {
        if self.owned {
            Self::free_table(self.root, true);
            self.root = alloc_tab_zeroed();
        } else {
            for e in self.root.0.iter_mut() {
                *e = Entry::default();
            }
        }
        self.cr3 = Vaddr::from_ref(&*self.root).to_phys().to_raw() as u64;
    }

    /// Recursively frees a page table and its children.
    fn free_table(table: &mut Tab, free_self: bool) {
        let paddr = Vaddr::from_ref(&*table).to_phys();
        for entry in table.0.iter_mut() {
            if entry.is_present() && !is_huge(entry) {
                Self::free_table(tab_from_entry(entry), true);
            }
            *entry = Entry::default();
        }
        if free_self {
            free_tab(paddr);
        }
    }

    // ------------------------------------------------------------------------
    // REPORTING
    // ------------------------------------------------------------------------

    /// Reports all mapped regions in the address space.
    ///
    /// # Type Parameters
    /// * `N` – The maximum number of `Area`s to return.
    ///
    /// # Returns
    /// A `Vec<Area, N>` of contiguous mapped regions, grouped by flags.
    pub fn report<const N: usize>(&self) -> Vec<Area, N> {
        debug!("Exco::report<{}> start", N);
        let mut areas: Vec<Area, N> = Vec::new();
        let mut total_pages: usize = 0;
        for pml4_idx in 0..512 {
            let pml4_entry = &self.root.0[pml4_idx];
            if !pml4_entry.is_present() {
                continue;
            }
            let base_pml4 = if pml4_idx >= 256 {
                0xFFFF000000000000 | (pml4_idx << 39)
            } else {
                pml4_idx << 39
            };
            if is_huge(pml4_entry) {
                continue;
            }
            let pdpt = tab_from_entry(pml4_entry);
            for pdpt_idx in 0..512 {
                let pdpt_entry = &pdpt.0[pdpt_idx];
                if !pdpt_entry.is_present() {
                    continue;
                }
                let base_pdpt = base_pml4 | (pdpt_idx << 30);
                if is_huge(pdpt_entry) {
                    try_push_area(&mut areas, base_pdpt, 1 << 20, pdpt_entry.flags());
                    total_pages += 1 << 20;
                    continue;
                }
                let pd = tab_from_entry(pdpt_entry);
                for pd_idx in 0..512 {
                    let pd_entry = &pd.0[pd_idx];
                    if !pd_entry.is_present() {
                        continue;
                    }
                    let base_pd = base_pdpt | (pd_idx << 21);
                    if is_huge(pd_entry) {
                        try_push_area(&mut areas, base_pd, 512, pd_entry.flags());
                        total_pages += 512;
                        continue;
                    }
                    let pt = tab_from_entry(pd_entry);
                    for pt_idx in 0..512 {
                        let pt_entry = &pt.0[pt_idx];
                        if !pt_entry.is_present() {
                            continue;
                        }
                        let base_pt = base_pd | (pt_idx << 12);
                        try_push_area(&mut areas, base_pt, 1, pt_entry.flags());
                        total_pages += 1;
                    }
                }
            }
        }
        debug!("Exco::report done: {} areas, {} 4K pages mapped", areas.len(), total_pages);
        areas
    }

    // ------------------------------------------------------------------------
    // MAPPING
    // ------------------------------------------------------------------------

    /// Maps a 4 KiB page (convenience wrapper).
    pub fn map4k(&mut self, vaddr: usize, paddr: Paddr, flags: EntryFlags) {
        self.try_map4k(vaddr, paddr, flags).expect("map4k failed");
    }

    /// Maps a 2 MiB huge page (convenience wrapper).
    pub fn map2m(&mut self, vaddr: usize, paddr: Paddr, flags: EntryFlags) {
        self.try_map2m(vaddr, paddr, flags).expect("map2m failed");
    }

    /// Maps a 1 GiB huge page (convenience wrapper).
    pub fn map1g(&mut self, vaddr: usize, paddr: Paddr, flags: EntryFlags) {
        self.try_map1g(vaddr, paddr, flags).expect("map1g failed");
    }

    /// Tries to map a 4 KiB page.
    ///
    /// # Errors
    /// - If the virtual address is not 4 KiB aligned.
    /// - If the page table walk fails.
    pub fn try_map4k(&mut self, vaddr: usize, paddr: Paddr, flags: EntryFlags) -> Result<(), &'static str> {
        if vaddr & MASK_4K != 0 {
            return Err("vaddr not 4 KiB-aligned");
        }
        let entry = walk_entry_mut(self.root, vaddr, 0, true)?;
        *entry = Entry::new(paddr, flags | EntryFlags::PRESENT);
        flush_tlb(vaddr);
        Ok(())
    }

    /// Tries to map a 2 MiB huge page.
    ///
    /// # Errors
    /// - If the virtual address is not 2 MiB aligned.
    /// - If the physical address is not 2 MiB aligned.
    pub fn try_map2m(&mut self, vaddr: usize, paddr: Paddr, flags: EntryFlags) -> Result<(), &'static str> {
        if vaddr & MASK_2M != 0 {
            return Err("vaddr not 2 MiB-aligned");
        }
        if paddr.to_raw() & MASK_2M != 0 {
            return Err("paddr not 2 MiB-aligned");
        }
        debug!("map2m vaddr {:#X} -> phys {:#016X} flags {:?}", vaddr, paddr.to_raw(), flags);
        let entry = walk_entry_mut(self.root, vaddr, 1, true)?;
        *entry = Entry::new(paddr, flags | EntryFlags::PRESENT | EntryFlags::HUGE_PAGE);
        flush_tlb(vaddr);
        Ok(())
    }

    /// Tries to map a 1 GiB huge page.
    ///
    /// # Errors
    /// - If the virtual address is not 1 GiB aligned.
    /// - If the physical address is not 1 GiB aligned.
    pub fn try_map1g(&mut self, vaddr: usize, paddr: Paddr, flags: EntryFlags) -> Result<(), &'static str> {
        if vaddr & MASK_1G != 0 {
            return Err("vaddr not 1 GiB-aligned");
        }
        if paddr.to_raw() & MASK_1G != 0 {
            return Err("paddr not 1 GiB-aligned");
        }
        debug!("map1g vaddr {:#X} -> phys {:#016X} flags {:?}", vaddr, paddr.to_raw(), flags);
        let entry = walk_entry_mut(self.root, vaddr, 2, true)?;
        *entry = Entry::new(paddr, flags | EntryFlags::PRESENT | EntryFlags::HUGE_PAGE);
        flush_tlb(vaddr);
        Ok(())
    }

    // ------------------------------------------------------------------------
    // UNMAPPING
    // ------------------------------------------------------------------------

    /// Unmaps a 4 KiB page (convenience wrapper).
    pub fn unmap(&mut self, vaddr: usize) {
        self.try_unmap(vaddr).expect("unmap failed");
    }

    /// Tries to unmap a 4 KiB page.
    ///
    /// # Errors
    /// - If the address is not mapped.
    pub fn try_unmap(&mut self, vaddr: usize) -> Result<(), &'static str> {
        if let Ok(entry) = walk_entry_mut(self.root, vaddr, 0, false)
        && entry.is_present() {
            *entry = Entry::default();
            flush_tlb(vaddr);
            return Ok(());
        }

        if let Ok(entry) = walk_entry_mut(self.root, vaddr, 1, false)
        && entry.is_present() && is_huge(entry) {
            debug!("unmap 2M huge page at {:#X}", vaddr & !MASK_2M);
            *entry = Entry::default();
            flush_tlb(vaddr & !MASK_2M);
            return Ok(());
        }

        if let Ok(entry) = walk_entry_mut(self.root, vaddr, 2, false)
        && entry.is_present() && is_huge(entry) {
            debug!("unmap 1G huge page at {:#X}", vaddr & !MASK_1G);
            *entry = Entry::default();
            flush_tlb(vaddr & !MASK_1G);
            return Ok(());
        }

        Err("address not mapped")
    }

    // ------------------------------------------------------------------------
    // SPLITTING HUGE PAGES
    // ------------------------------------------------------------------------

    /// Splits a 2 MiB huge page (convenience wrapper).
    pub fn split2m(&mut self, vaddr: usize) {
        self.try_split2m(vaddr).expect("split2m failed");
    }

    /// Splits a 1 GiB huge page (convenience wrapper).
    pub fn split1g(&mut self, vaddr: usize) {
        self.try_split1g(vaddr).expect("split1g failed");
    }

    /// Tries to split a 2 MiB huge page into 4 KiB pages.
    ///
    /// # Errors
    /// - If the virtual address is not 2 MiB aligned.
    /// - If the entry is not a 2 MiB huge page.
    pub fn try_split2m(&mut self, vaddr: usize) -> Result<(), &'static str> {
        if vaddr & MASK_2M != 0 {
            return Err("vaddr not 2 MiB-aligned");
        }

        let entry = walk_entry_mut(self.root, vaddr, 1, false)?;
        if !entry.is_present() || !is_huge(entry) {
            return Err("not a 2 MiB huge page");
        }

        let base_paddr = entry.address();
        let flags = entry.flags() - EntryFlags::HUGE_PAGE;

        debug!("split2m {:#X} (phys {:#016X}) -> 512x4K", vaddr, base_paddr.to_raw());

        let pt = alloc_tab_zeroed();
        let pt_paddr = Vaddr::from_ref(&*pt).to_phys();

        for i in 0..512 {
            let page_paddr = Paddr::from_raw(base_paddr.to_raw() + (i << 12));
            pt.0[i] = Entry::new(page_paddr, flags);
        }

        *entry = Entry::new(pt_paddr, EntryFlags::PRESENT | EntryFlags::WRITABLE);
        flush_tlb(vaddr);
        Ok(())
    }

    /// Tries to split a 1 GiB huge page into 2 MiB pages.
    ///
    /// # Errors
    /// - If the virtual address is not 1 GiB aligned.
    /// - If the entry is not a 1 GiB huge page.
    pub fn try_split1g(&mut self, vaddr: usize) -> Result<(), &'static str> {
        if vaddr & MASK_1G != 0 {
            return Err("vaddr not 1 GiB-aligned");
        }

        let entry = walk_entry_mut(self.root, vaddr, 2, false)?;
        if !entry.is_present() || !is_huge(entry) {
            return Err("not a 1 GiB huge page");
        }

        let base_paddr = entry.address();
        let flags = entry.flags() - EntryFlags::HUGE_PAGE;

        debug!("split1g {:#X} (phys {:#016X}) -> 512x2M", vaddr, base_paddr.to_raw());

        let pd = alloc_tab_zeroed();
        let pd_paddr = Vaddr::from_ref(&*pd).to_phys();

        for i in 0..512 {
            let page_paddr = Paddr::from_raw(base_paddr.to_raw() + (i << 21));
            pd.0[i] = Entry::new(page_paddr, flags | EntryFlags::HUGE_PAGE);
        }

        *entry = Entry::new(pd_paddr, EntryFlags::PRESENT | EntryFlags::WRITABLE);
        flush_tlb(vaddr);
        Ok(())
    }

    // ------------------------------------------------------------------------
    // MERGING INTO HUGE PAGES
    // ------------------------------------------------------------------------

    /// Merges 4 KiB pages into a 2 MiB huge page (convenience wrapper).
    pub fn merge2m(&mut self, vaddr: usize) {
        self.try_merge2m(vaddr).expect("merge2m failed");
    }

    /// Merges 2 MiB pages into a 1 GiB huge page (convenience wrapper).
    pub fn merge1g(&mut self, vaddr: usize) {
        self.try_merge1g(vaddr).expect("merge1g failed");
    }

    /// Tries to merge 512 consecutive 4 KiB pages into a 2 MiB huge page.
    ///
    /// # Errors
    /// - If the virtual address is not 2 MiB aligned.
    /// - If the PD entry is not a table pointer.
    /// - If the pages are not contiguous or have inconsistent flags.
    pub fn try_merge2m(&mut self, vaddr: usize) -> Result<(), &'static str> {
        if vaddr & MASK_2M != 0 {
            return Err("vaddr not 2 MiB-aligned");
        }

        let pd_entry = walk_entry_mut(self.root, vaddr, 1, false)?;
        if !pd_entry.is_present() || is_huge(pd_entry) {
            return Err("PD entry is not a table pointer");
        }

        let pt = tab_from_entry(pd_entry);
        try_coalesce_into_2m(pt, vaddr, pd_entry)
    }

    /// Tries to merge 512 consecutive 2 MiB pages into a 1 GiB huge page.
    ///
    /// # Errors
    /// - If the virtual address is not 1 GiB aligned.
    /// - If the PDPT entry is not a table pointer.
    /// - If the pages are not contiguous or have inconsistent flags.
    pub fn try_merge1g(&mut self, vaddr: usize) -> Result<(), &'static str> {
        if vaddr & MASK_1G != 0 {
            return Err("vaddr not 1 GiB-aligned");
        }

        let pdpt_entry = walk_entry_mut(self.root, vaddr, 2, false)?;
        if !pdpt_entry.is_present() || is_huge(pdpt_entry) {
            return Err("PDPT entry is not a table pointer");
        }

        let pd = tab_from_entry(pdpt_entry);
        try_coalesce_into_1g(pd, vaddr, pdpt_entry)
    }

    // ------------------------------------------------------------------------
    // ACTIVATION
    // ------------------------------------------------------------------------

    /// Loads this page table context (writes CR3).
    ///
    /// # Safety
    /// This function uses inline assembly to write CR3. The caller must ensure
    /// that the context is valid and that the CPU supports the CR3 value.
    #[inline(always)]
    pub unsafe fn activate(&self) {
        unsafe {
            core::arch::asm!("mov cr3, {}", in(reg) self.cr3, options(nostack, preserves_flags));
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS FOR REPORTING
// ============================================================================

/// Tries to push an area to the report vector, merging if possible.
fn try_push_area<const N: usize>(
    areas: &mut Vec<Area, N>,
    vaddr: usize,
    count: usize,
    flags: EntryFlags,
) {
    if let Some(last) = areas.last_mut()
    && last.flags == flags && last.start + last.count * 4096 == vaddr {
        last.count += count;
        return;
    }
    let _ = areas.push(Area { start: vaddr, count, flags });
}

// ============================================================================
// COALESCING FUNCTIONS (FOR MERGING)
// ============================================================================

/// Internal function to coalesce a PT into a 2 MiB huge page.
fn try_coalesce_into_2m(
    pt: &mut Tab,
    vaddr: usize,
    pd_entry: &mut Entry,
) -> Result<(), &'static str> {
    let first_flags = pt.0[0].flags() - EntryFlags::ACCESSED - EntryFlags::DIRTY;
    let first_paddr = pt.0[0].address();

    if !pt.0[0].is_present() {
        return Err("first PT entry not present");
    }

    for i in 1..512 {
        if !pt.0[i].is_present() {
            return Err("PT entry not present");
        }
        let flags = pt.0[i].flags() - EntryFlags::ACCESSED - EntryFlags::DIRTY;
        if flags != first_flags {
            return Err("inconsistent flags across PT entries");
        }
        let expected_paddr = Paddr::from_raw(first_paddr.to_raw() + (i << 12));
        if pt.0[i].address().to_raw() != expected_paddr.to_raw() {
            return Err("non-contiguous physical addresses");
        }
    }

    let huge_flags = first_flags | EntryFlags::HUGE_PAGE | EntryFlags::PRESENT;
    *pd_entry = Entry::new(first_paddr, huge_flags);

    let pt_paddr = Vaddr::from_ref(&*pt).to_phys();
    free_tab(pt_paddr);

    info!("Merged 2 MiB at {:#X} (phys {:#016X})", vaddr, first_paddr.to_raw());
    Ok(())
}

/// Internal function to coalesce a PD into a 1 GiB huge page.
fn try_coalesce_into_1g(
    pd: &mut Tab,
    vaddr: usize,
    pdpt_entry: &mut Entry,
) -> Result<(), &'static str> {
    let first_flags = pd.0[0].flags() - EntryFlags::ACCESSED - EntryFlags::DIRTY;
    let first_paddr = pd.0[0].address();

    if !pd.0[0].is_present() || !is_huge(&pd.0[0]) {
        return Err("first PD entry not a 2 MiB huge page");
    }

    for i in 1..512 {
        if !pd.0[i].is_present() || !is_huge(&pd.0[i]) {
            return Err("PD entry not a 2 MiB huge page");
        }
        let flags = pd.0[i].flags() - EntryFlags::ACCESSED - EntryFlags::DIRTY;
        if flags != first_flags {
            return Err("inconsistent flags across PD entries");
        }
        let expected_paddr = Paddr::from_raw(first_paddr.to_raw() + (i << 21));
        if pd.0[i].address().to_raw() != expected_paddr.to_raw() {
            return Err("non-contiguous physical addresses");
        }
    }

    let huge_flags = first_flags | EntryFlags::HUGE_PAGE | EntryFlags::PRESENT;
    *pdpt_entry = Entry::new(first_paddr, huge_flags);

    let pd_paddr = Vaddr::from_ref(&*pd).to_phys();
    free_tab(pd_paddr);

    info!("Merged 1 GiB at {:#X} (phys {:#016X})", vaddr, first_paddr.to_raw());
    Ok(())
}

```

### `src/arch/amd64/idt.rs`

```rs
//! # Interrupt Descriptor Table (IDT) and Exception Handling
//!
//! This module manages the x86_64 Interrupt Descriptor Table (IDT), which defines
//! the handlers for all exceptions and interrupts. It sets up the gate descriptors
//! for CPU exceptions, hardware interrupts (including the timer and IPI), and
//! provides the low‑level interrupt handler functions.
//!
//! ## Overview
//!
//! The IDT is a table of 256 entries, each containing the address of an interrupt
//! handler (ISR), along with privilege and type information. When an interrupt or
//! exception occurs, the CPU looks up the corresponding entry in the IDT and
//! transfers control to the handler.
//!
//! This module defines handlers for:
//! - **CPU Exceptions** (0‑19): Divide error, page fault, double fault, etc.
//! - **Hardware Interrupts**: Timer (vector 32), IPI (vector 128), etc.
//! - **Software Interrupts**: Yield (vector 33), used by the scheduler.
//!
//! ## Handler Types
//!
//! The IDT uses two types of handlers:
//! - **Exception handlers**: Handle CPU‑generated exceptions (page faults, GPF, etc.).
//!   These are defined with the `x86-interrupt` ABI and receive an `InterruptStackFrame`.
//! - **Interrupt handlers**: Handle external interrupts (timer, IPI). These are also
//!   defined with the `x86-interrupt` ABI.
//! - **Naked wrappers**: The timer handler is a naked function that saves the
//!   context and calls the scheduler. This is required because the scheduler
//!   needs to access and modify the trap frame.
//!
//! ## Exception Handling
//!
//! Most exceptions are treated as critical and cause a kernel panic. The handlers
//! log the exception details (error code, RIP, RSP, etc.) and then panic. The
//! following exceptions are handled specially:
//! - **Page Fault**: Delegated to the scheduler's `handle_page_fault` for
//!   demand paging and copy‑on‑write.
//! - **Double Fault**: Uses an Interrupt Stack Table (IST) entry to avoid
//!   stack corruption.
//! - **Breakpoint**: Logged as a warning (useful for debugging).
//! - **Debug**: Logged as a warning.
//!
//! ## Interrupt Handlers
//!
//! - **Timer (vector 32)**: Calls `timer_wrapper`, which saves the context and
//!   invokes `sched::timer_tick` to update vruntime and reschedule.
//! - **Yield (vector 33)**: Calls `yield_wrapper`, which invokes `sched::reschedule`
//!   to perform a voluntary context switch.
//! - **IPI (vector 128)**: Logs the IPI reception and sends an EOI to the APIC.
//!
//! ## Interrupt Stack Table (IST)
//!
//! The double fault handler uses IST entry 1, which provides a dedicated stack
//! for handling double faults. This prevents a stack overflow from corrupting
//! the handler's own stack. Other IST entries are reserved for future use.
//!
//! ## Initialisation
//!
//! - **BSP**: `idt::init_bsp()` is called during `arch::init_bsp()`. It sets up
//!   all handlers and loads the IDT via `lidt`.
//! - **APs**: `idt::init_ap()` is called during `arch::init_ap()`. It loads the
//!   same IDT (the table is shared across all CPUs).
//!
//! ## Safety
//!
//! - The IDT is a `static mut` and is modified during early boot (single‑threaded).
//! - The naked wrapper functions use inline assembly to manipulate the stack and
//!   registers; they are carefully written to preserve the ABI.
//! - The `transmute` calls for the timer and yield handlers are required because
//!   the scheduler functions have a different signature from the IDT handler type.
//!
//! ## Layout
//!
//! | Vector | Description                | Handler                     |
//! |--------|----------------------------|-----------------------------|
//! | 0      | Divide Error               | divide_error_handler        |
//! | 1      | Debug                      | debug_handler               |
//! | 2      | NMI                        | nmi_handler                 |
//! | 3      | Breakpoint                 | breakpoint_handler          |
//! | 4      | Overflow                   | overflow_handler            |
//! | 5      | Bound Range Exceeded       | bound_range_exceeded_handler|
//! | 6      | Invalid Opcode             | invalid_opcode_handler      |
//! | 7      | Device Not Available       | device_not_available_handler|
//! | 8      | Double Fault (IST1)        | double_fault_handler        |
//! | 10     | Invalid TSS                | invalid_tss_handler         |
//! | 11     | Segment Not Present        | segment_not_present_handler |
//! | 12     | Stack Segment Fault        | stack_segment_fault_handler |
//! | 13     | General Protection Fault   | general_protection_fault_handler |
//! | 14     | Page Fault                 | page_fault_handler          |
//! | 16     | x87 FPU Exception          | x87_fpu_exception_handler   |
//! | 17     | Alignment Check            | alignment_check_handler     |
//! | 18     | Machine Check              | machine_check_handler       |
//! | 19     | SIMD Floating Point        | simd_floating_point_handler |
//! | 32     | Timer (APIC)               | timer_wrapper (naked)       |
//! | 33     | Yield (software)           | yield_wrapper (naked)       |
//! | 128    | IPI                        | ipi_handler                 |

use crate::arch::current_cpu;
use x86_64::structures::idt::{
    EntryOptions, InterruptDescriptorTable, InterruptStackFrame,
    PageFaultErrorCode
};
use x86_64::registers::control::Cr2;

// ============================================================================
// CONSTANTS
// ============================================================================

/// IPI (Inter‑Processor Interrupt) vector.
///
/// This vector is used for sending IPIs between CPUs, typically for TLB shootdown
/// or rescheduling requests.
pub const IPI_VECTOR: u8 = 128;

/// Timer interrupt vector (APIC timer).
///
/// The APIC timer is programmed to fire at this vector on each tick (10 ms).
pub const TIMER_VECTOR: u8 = 32;

// ============================================================================
// GLOBAL IDT
// ============================================================================

/// The global Interrupt Descriptor Table.
///
/// This table is shared by all CPU cores. It is `static mut` because it is
/// modified during early boot (single‑threaded) and read‑only thereafter.
pub static mut GLOBAL_IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

// ============================================================================
// HELPER: PRINT TRAP FRAME
// ============================================================================

/// Prints the contents of an `InterruptStackFrame` for debugging.
///
/// This function is called from exception handlers to log the CPU state at
/// the time of the exception.
fn print_frame(frame: &InterruptStackFrame) {
    error!(
        "\n  RIP: {:#018X}\n  CS : {:#08X}\n  RFLAGS: {:#018X}\n  RSP: {:#018X}\n  SS : {:#08X}",
        frame.instruction_pointer.as_u64(),
        frame.code_segment.0,
        frame.cpu_flags,
        frame.stack_pointer.as_u64(),
        frame.stack_segment.0,
    );
}

// ============================================================================
// EXCEPTION HANDLERS
// ============================================================================

/// Divide error (vector 0).
///
/// Occurs when a division by zero or an overflow in division is attempted.
extern "x86-interrupt" fn divide_error_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 0: DIVIDE_ERROR on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Divide Error");
}

/// Debug (vector 1).
///
/// Triggered by the `int3` instruction or single‑step debugging.
extern "x86-interrupt" fn debug_handler(frame: InterruptStackFrame) {
    warn!("EXCEPTION 1: DEBUG on CPU#{}", current_cpu());
    print_frame(&frame);
}

/// Non‑Maskable Interrupt (vector 2).
///
/// Typically triggered by hardware errors.
extern "x86-interrupt" fn nmi_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 2: NMI on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: NMI");
}

/// Breakpoint (vector 3).
///
/// Triggered by the `int3` instruction; used for debugging.
extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
    warn!("EXCEPTION 3: BREAKPOINT on CPU#{}", current_cpu());
    print_frame(&frame);
}

/// Overflow (vector 4).
///
/// Triggered by the `into` instruction when the overflow flag is set.
extern "x86-interrupt" fn overflow_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 4: OVERFLOW on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Overflow");
}

/// Bound range exceeded (vector 5).
///
/// Triggered by the `bound` instruction when the index is out of range.
extern "x86-interrupt" fn bound_range_exceeded_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 5: BOUND_RANGE_EXCEEDED on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Bound Range Exceeded");
}

/// Invalid opcode (vector 6).
///
/// Occurs when the CPU tries to execute an invalid instruction.
extern "x86-interrupt" fn invalid_opcode_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 6: INVALID_OPCODE on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Invalid Opcode");
}

/// Device not available (vector 7).
///
/// Occurs when an x87 FPU or SIMD instruction is executed without the device
/// being present (or with CR0.EM set).
extern "x86-interrupt" fn device_not_available_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 7: DEVICE_NOT_AVAILABLE on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Device Not Available");
}

/// Double fault (vector 8) – uses IST1.
///
/// Occurs when an exception occurs while trying to deliver another exception.
/// This uses a dedicated Interrupt Stack Table entry to avoid stack corruption.
extern "x86-interrupt" fn double_fault_handler(frame: InterruptStackFrame, error_code: u64) -> ! {
    error!("!!! CRITICAL EXCEPTION 8: DOUBLE_FAULT on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: Double Fault");
}

/// Invalid TSS (vector 10).
extern "x86-interrupt" fn invalid_tss_handler(frame: InterruptStackFrame, error_code: u64) {
    error!("!!! CRITICAL EXCEPTION 10: INVALID_TSS on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: Invalid TSS");
}

/// Segment not present (vector 11).
extern "x86-interrupt" fn segment_not_present_handler(frame: InterruptStackFrame, error_code: u64) {
    error!("!!! CRITICAL EXCEPTION 11: SEGMENT_NOT_PRESENT on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: Segment Not Present");
}

/// Stack segment fault (vector 12).
extern "x86-interrupt" fn stack_segment_fault_handler(frame: InterruptStackFrame, error_code: u64) {
    error!("!!! CRITICAL EXCEPTION 12: STACK_SEGMENT_FAULT on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: Stack Segment Fault");
}

/// General protection fault (vector 13).
extern "x86-interrupt" fn general_protection_fault_handler(frame: InterruptStackFrame, error_code: u64) {
    error!("!!! CRITICAL EXCEPTION 13: GENERAL_PROTECTION_FAULT on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: General Protection Fault");
}

/// Page fault (vector 14).
///
/// This handler delegates to `sched::handle_page_fault` to handle demand paging,
/// copy‑on‑write, and segmentation faults.
extern "x86-interrupt" fn page_fault_handler(frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    let cr2 = Cr2::read().unwrap().as_u64() as usize;
    let rip = frame.instruction_pointer.as_u64();

    let is_user = (frame.code_segment.0 & 0x3) != 0;

    crate::sched::handle_page_fault(cr2, error_code.bits(), rip, is_user);
}

/// x87 FPU exception (vector 16).
extern "x86-interrupt" fn x87_fpu_exception_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 16: X87_FPU_EXCEPTION on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: x87 FPU Exception");
}

/// Alignment check (vector 17).
extern "x86-interrupt" fn alignment_check_handler(frame: InterruptStackFrame, error_code: u64) {
    error!("!!! CRITICAL EXCEPTION 17: ALIGNMENT_CHECK on CPU#{} (Error Code: {:#X})", current_cpu(), error_code);
    print_frame(&frame);
    panic!("Unhandled critical exception: Alignment Check");
}

/// Machine check (vector 18) – does not return.
extern "x86-interrupt" fn machine_check_handler(frame: InterruptStackFrame) -> ! {
    error!("!!! CRITICAL EXCEPTION 18: MACHINE_CHECK on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: Machine Check");
}

/// SIMD floating point exception (vector 19).
extern "x86-interrupt" fn simd_floating_point_handler(frame: InterruptStackFrame) {
    error!("!!! CRITICAL EXCEPTION 19: SIMD_FLOATING_POINT on CPU#{}", current_cpu());
    print_frame(&frame);
    panic!("Unhandled critical exception: SIMD Floating Point");
}

// ============================================================================
// INTERRUPT HANDLERS
// ============================================================================

/// IPI (Inter‑Processor Interrupt) handler (vector 128).
///
/// Logs the IPI reception and sends an EOI to the APIC.
extern "x86-interrupt" fn ipi_handler(_frame: InterruptStackFrame) {
    warn!("IPI received on CPU#{}", current_cpu());
    crate::arch::acpi::eoi();
}

// ============================================================================
// HELPER: SET ENTRY OPTIONS
// ============================================================================

/// Sets the common options for an IDT entry: present, Ring 0, and optional IST.
///
/// # Arguments
/// * `entry` – The IDT entry to configure.
/// * `ist_index` – Optional IST index (1..7) for the entry.
fn set_entry_options(entry: &mut EntryOptions, ist_index: Option<u16>) {
    entry.set_present(true);
    entry.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
    if let Some(index) = ist_index {
        unsafe { entry.set_stack_index(index); }
    }
}

// ============================================================================
// INITIALISATION
// ============================================================================

/// Initialises the IDT for the BSP (Bootstrap Processor).
///
/// This function:
/// 1. Sets up all exception handlers (vectors 0‑19).
/// 2. Sets up the timer handler (vector 32) with a naked wrapper.
/// 3. Sets up the yield handler (vector 33) with a naked wrapper.
/// 4. Sets up the IPI handler (vector 128).
/// 5. Loads the IDT with `lidt`.
///
/// # Safety
/// This function modifies the global IDT and performs `lidt`. It is called
/// during early boot with interrupts disabled.
pub fn init_bsp() {
    info!("Initializing exception handlers for BSP...");

    #[allow(static_mut_refs)]
    let idt = unsafe { &mut GLOBAL_IDT };

    // Set up exception handlers (vectors 0‑19).
    set_entry_options(idt.divide_error.set_handler_fn(divide_error_handler), None);
    set_entry_options(idt.debug.set_handler_fn(debug_handler), None);
    set_entry_options(idt.non_maskable_interrupt.set_handler_fn(nmi_handler), None);
    set_entry_options(idt.breakpoint.set_handler_fn(breakpoint_handler), None);
    set_entry_options(idt.overflow.set_handler_fn(overflow_handler), None);
    set_entry_options(idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded_handler), None);
    set_entry_options(idt.invalid_opcode.set_handler_fn(invalid_opcode_handler), None);
    set_entry_options(idt.device_not_available.set_handler_fn(device_not_available_handler), None);

    // Double fault uses IST1.
    set_entry_options(idt.double_fault.set_handler_fn(double_fault_handler), Some(1));

    set_entry_options(idt.invalid_tss.set_handler_fn(invalid_tss_handler), None);
    set_entry_options(idt.segment_not_present.set_handler_fn(segment_not_present_handler), None);
    set_entry_options(idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler), None);
    set_entry_options(idt.general_protection_fault.set_handler_fn(general_protection_fault_handler), None);
    set_entry_options(idt.page_fault.set_handler_fn(page_fault_handler), None);

    set_entry_options(idt.x87_floating_point.set_handler_fn(x87_fpu_exception_handler), None);
    set_entry_options(idt.alignment_check.set_handler_fn(alignment_check_handler), None);
    set_entry_options(idt.machine_check.set_handler_fn(machine_check_handler), None);
    set_entry_options(idt.simd_floating_point.set_handler_fn(simd_floating_point_handler), None);

    // Timer handler (vector 32) – uses a naked wrapper.
    set_entry_options(
        idt[TIMER_VECTOR].set_handler_fn(unsafe {
            core::mem::transmute(crate::arch::timer::timer_wrapper as *const ())
        }),
        None,
    );

    // Yield handler (vector 33) – uses a naked wrapper.
    set_entry_options(
        idt[TIMER_VECTOR + 1].set_handler_fn(unsafe {
            core::mem::transmute(crate::sched::yield_wrapper as *const ())
        }),
        None,
    );

    // IPI handler (vector 128).
    set_entry_options(idt[IPI_VECTOR].set_handler_fn(ipi_handler), None);

    // Load the IDT.
    idt.load();

    info!("Loaded successfully on BSP.");
}

/// Initialises the IDT for an AP (Application Processor).
///
/// This function simply loads the global IDT (which was already set up by the BSP).
///
/// # Safety
/// This function performs `lidt` and is called during AP boot with interrupts disabled.
pub fn init_ap() {
    info!("Loading for AP...");
    #[allow(static_mut_refs)]
    unsafe { GLOBAL_IDT.load() }
    info!("Loaded successfully on AP.");
}

```

### `src/arch/amd64/gdt.rs`

```rs
//! # Global Descriptor Table (GDT) and Task State Segment (TSS)
//!
//! This module manages the x86_64 Global Descriptor Table (GDT), which defines
//! segmentation and protection rings, along with the Task State Segment (TSS)
//! used for kernel stack switching and interrupt handling.
//!
//! ## Overview
//!
//! On x86_64, the GDT is used primarily for:
//! - Defining code and data segments for both kernel (Ring 0) and user (Ring 3).
//! - Providing the Task State Segment (TSS), which contains the kernel stack
//!   pointer (`rsp0`) for each CPU core.
//! - Supporting Interrupt Stack Tables (IST) for handling exceptions and
//!   interrupts with separate stacks.
//!
//! ## Structure
//!
//! The GDT is a table of 64‑bit segment descriptors. This implementation defines:
//!
//! - **Null Descriptor** (index 0): Required by the x86 architecture.
//! - **Kernel Code** (index 1): 64‑bit, Ring 0, readable/executable.
//! - **Kernel Data** (index 2): Ring 0, writable data segment.
//! - **User Code** (index 3): 64‑bit, Ring 3, readable/executable.
//! - **User Data** (index 4): Ring 3, writable data segment.
//! - **TSS Descriptors** (starting at index 5): One per CPU core, each pointing
//!   to a per‑CPU TSS structure.
//!
//! ## Per‑CPU TSS
//!
//! Each CPU core has its own TSS, which stores:
//! - `rsp0`: The kernel stack pointer for when the CPU switches from user to
//!   kernel mode (e.g., on interrupts or system calls).
//! - `ist1..ist7`: Interrupt Stack Table entries, used by the IDT for
//!   exceptions that need a dedicated stack (e.g., double fault).
//!
//! The TSS for each core is stored in a static array `TSS_TABLE`, indexed by
//! CPU ID.
//!
//! ## Initialization
//!
//! - **BSP**: `gdt::init_bsp()` is called during BSP initialization. It sets up
//!   the TSS for CPU 0, loads the GDT, and loads the TSS selector into `tr`.
//! - **APs**: `gdt::init_ap(cpu_id)` is called for each AP. It sets up the TSS
//!   for that CPU, loads the GDT (the same GDT is shared), and loads the TSS
//!   selector.
//!
//! ## Stack Switching
//!
//! The `set_kernel_stack(cpu_id, stack_top)` function updates the `rsp0` field
//! of the TSS for a given CPU. This is used by the scheduler to set the kernel
//! stack for the current task.
//!
//! ## Selectors
//!
//! Segment selectors are defined as constants:
//! - `KERNEL_CODE_SELECTOR`: 0x08
//! - `KERNEL_DATA_SELECTOR`: 0x10
//! - `USER_CODE_SELECTOR`: 0x18
//! - `USER_DATA_SELECTOR`: 0x20
//! - `tss_selector(cpu_id)`: 0x28 + (cpu_id * 16)
//!
//! The TSS selectors are spaced 16 bytes apart to accommodate the 16‑byte
//! TSS descriptor (two 64‑bit entries).
//!
//! ## Safety
//!
//! This module uses `static mut` for the GDT, TSS table, and other structures.
//! These are accessed during early boot (single‑threaded) and later via per‑CPU
//! operations that are safe because each CPU writes to its own TSS entry.
//! The `load()` and `set_tss()` functions use unsafe inline assembly to perform
//! privileged operations (`lgdt`, `ltr`, `mov ds`, etc.).

use super::MAX_CPUS;

// ============================================================================
// SEGMENT SELECTORS
// ============================================================================

/// Kernel code segment selector (Ring 0, executable).
pub const KERNEL_CODE_SELECTOR: u16 = 0x08;

/// Kernel data segment selector (Ring 0, writable).
pub const KERNEL_DATA_SELECTOR: u16 = 0x10;

/// User code segment selector (Ring 3, executable).
pub const USER_CODE_SELECTOR: u16 = 0x18;

/// User data segment selector (Ring 3, writable).
pub const USER_DATA_SELECTOR: u16 = 0x20;

/// Returns the TSS segment selector for a given CPU.
///
/// TSS descriptors start at index 5, with each descriptor taking 2 entries
/// (16 bytes). The selector is `(index << 3) | RPL (0)`.
#[inline]
pub const fn tss_selector(cpu_id: usize) -> u16 {
    (0x28 + (cpu_id * 16)) as u16
}

// ============================================================================
// TASK STATE SEGMENT (TSS)
// ============================================================================

/// Task State Segment (TSS) structure.
///
/// This structure holds the stack pointers for the CPU, used for privilege
/// level transitions and interrupt handling.
///
/// # Fields
/// - `reserved_1`: Must be zero.
/// - `rsp0`: Stack pointer for Ring 0 (kernel mode).
/// - `rsp1`: Stack pointer for Ring 1 (not used).
/// - `rsp2`: Stack pointer for Ring 2 (not used).
/// - `reserved_2`: Must be zero.
/// - `ist1..ist7`: Interrupt Stack Table entries (one per IST index).
/// - `reserved_3`, `reserved_4`, `iomap_base`: Must be zero for 64‑bit mode.
#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Tss {
    pub reserved_1: u32,
    pub rsp0: u64,
    pub rsp1: u64,
    pub rsp2: u64,
    pub reserved_2: u64,
    pub ist1: u64,
    pub ist2: u64,
    pub ist3: u64,
    pub ist4: u64,
    pub ist5: u64,
    pub ist6: u64,
    pub ist7: u64,
    pub reserved_3: u64,
    pub reserved_4: u16,
    pub iomap_base: u16,
}

// ============================================================================
// GLOBAL DESCRIPTOR TABLE (GDT)
// ============================================================================

/// The GDT structure, containing all segment descriptors.
///
/// The GDT is an array of 64‑bit descriptors. The total size is:
/// - 5 static descriptors (null, kernel code/data, user code/data)
/// - 2 descriptors per CPU for TSS (each TSS descriptor is 16 bytes)
#[repr(C, align(8))]
pub struct Gdt {
    entries: [u64; 5 + (MAX_CPUS * 2)],
}

/// Descriptor Table Register (GDTR / IDTR) structure.
///
/// This is loaded with `lgdt` or `lidt`.
#[repr(C, packed)]
pub struct Dtr {
    pub limit: u16,
    pub base: u64,
}

impl Gdt {
    /// Creates a new GDT with default segment descriptors.
    ///
    /// The descriptors are set up as follows:
    /// - **Null**: All zero.
    /// - **Kernel Code**: Present, Ring 0, 64‑bit, readable, executable.
    /// - **Kernel Data**: Present, Ring 0, writable.
    /// - **User Code**: Present, Ring 3, 64‑bit, readable, executable.
    /// - **User Data**: Present, Ring 3, writable.
    ///
    /// TSS descriptors are zero‑initialized and must be set with `set_tss()`.
    pub const fn new() -> Self {
        let mut entries = [0u64; 5 + (MAX_CPUS * 2)];

        // 0. Null descriptor (required by x86)
        entries[0] = 0x0000000000000000;

        // 1. Kernel Code (64-bit, present, ring 0, readable)
        // Base: 0, Limit: 0, Access: 0x9A (1001 1010), Flags: 0x20 (L=1)
        entries[1] = 0x00209A0000000000;

        // 2. Kernel Data (64-bit, present, ring 0, writable)
        // Base: 0, Limit: 0, Access: 0x92 (1001 0010), Flags: 0x00
        entries[2] = 0x0000920000000000;

        // 3. User Code (64-bit, present, ring 3, readable)
        // Base: 0, Limit: 0, Access: 0xFA (1111 1010), Flags: 0x20 (L=1)
        entries[3] = 0x0020FA0000000000;

        // 4. User Data (64-bit, present, ring 3, writable)
        // Base: 0, Limit: 0, Access: 0xF2 (1111 0010), Flags: 0x00
        entries[4] = 0x0000F20000000000;

        Self { entries }
    }

    /// Loads the GDT and reloads segment registers.
    ///
    /// This function:
    /// 1. Executes `lgdt` to load the GDT base and limit.
    /// 2. Reloads `ds`, `es`, `ss`, `fs`, `gs` with the kernel data selector.
    /// 3. Performs a far jump to reload `cs` with the kernel code selector.
    ///
    /// # Safety
    /// This function uses inline assembly and requires that the GDT is valid.
    pub unsafe fn load(&'static self) {
        let dtr = Dtr {
            limit: (size_of_val(&self.entries) - 1) as u16,
            base: self as *const _ as u64,
        };

        unsafe {
            core::arch::asm!(
                "lgdt [{0}]",
                in(reg) &dtr,
                options(readonly, nostack, preserves_flags)
            );

            Self::reload_segments();
        }
    }

    /// Reloads segment registers after a GDT load.
    ///
    /// This function:
    /// 1. Loads `ds`, `es`, `ss`, `fs`, `gs` with the kernel data selector.
    /// 2. Pushes the kernel code selector and a return address, then executes
    ///    `retfq` to perform a far return, reloading `cs`.
    #[inline]
    unsafe fn reload_segments() {
        unsafe {
            core::arch::asm!(
                "mov ds, {0:x}",
                "mov es, {0:x}",
                "mov ss, {0:x}",
                "mov fs, {0:x}",
                "mov gs, {0:x}",

                "push {1:r}",
                "lea {2}, [rip + 2f]",
                "push {2}",
                "retfq",
                "2:",
                in(reg) KERNEL_DATA_SELECTOR,
                in(reg) KERNEL_CODE_SELECTOR,
                out(reg) _,
                options(nostack, preserves_flags)
            );
        }
    }

    /// Sets the TSS descriptor for a given CPU.
    ///
    /// This function writes a 16‑byte TSS descriptor into the GDT at the
    /// appropriate slot. The descriptor includes:
    /// - Base address of the TSS.
    /// - Limit (size of the TSS).
    /// - Access flags (type 9, present).
    /// - Flags (G=0, L=0 for 64‑bit TSS).
    ///
    /// # Arguments
    /// * `cpu_id` – The CPU core index (0..MAX_CPUS-1).
    /// * `tss_ptr` – Pointer to the TSS structure.
    ///
    /// # Panics
    /// Panics if `cpu_id >= MAX_CPUS`.
    pub fn set_tss(&mut self, cpu_id: usize, tss_ptr: *const Tss) {
        assert!(cpu_id < MAX_CPUS, "CPU ID {} exceeds MAX_CPUS ({})", cpu_id, MAX_CPUS);

        let base = tss_ptr as u64;
        let limit = (size_of::<Tss>() - 1) as u64;

        // Access: present (0x80) | type (0x9) = 0x89
        let access: u64 = 0x89;
        // Flags: G=0 (limit in bytes), L=0 (for 64-bit TSS)
        let flags: u64 = 0x00;

        // First 64‑bit word of the descriptor.
        // Bits:
        //  0‑15:   Limit (low)
        //  16‑39:  Base (low)
        //  40‑47:  Access byte
        //  48‑51:  Limit (high)
        //  52‑55:  Flags
        //  56‑63:  Base (mid)
        let low = (limit & 0xFFFF)
                | ((base & 0xFFFFFF) << 16)
                | ((access & 0xFF) << 40)
                | (((limit >> 16) & 0xF) << 48)
                | ((flags & 0xF) << 52)
                | ((base >> 24) << 56);

        // Second 64‑bit word: Base (high 32 bits).
        let high = base >> 32;

        let tss_idx = 5 + (cpu_id * 2);
        self.entries[tss_idx] = low;
        self.entries[tss_idx + 1] = high;
    }
}

// ============================================================================
// GLOBAL GDT AND TSS TABLES
// ============================================================================

/// The global GDT, shared by all CPU cores.
///
/// This is `static mut` because it is modified during early boot and then
/// remains immutable thereafter.
pub static mut GLOBAL_GDT: Gdt = Gdt::new();

/// The per‑CPU TSS table.
///
/// Each CPU has its own TSS, stored in this static array.
static mut TSS_TABLE: [Tss; MAX_CPUS] = [Tss {
    reserved_1: 0, rsp0: 0, rsp1: 0, rsp2: 0, reserved_2: 0,
    ist1: 0, ist2: 0, ist3: 0, ist4: 0, ist5: 0, ist6: 0, ist7: 0,
    reserved_3: 0, reserved_4: 0, iomap_base: 0,
}; MAX_CPUS];

// ============================================================================
// PUBLIC API
// ============================================================================

/// Sets the kernel stack pointer (`rsp0`) for a given CPU.
///
/// This updates the TSS `rsp0` field, which is used when transitioning from
/// user mode to kernel mode (e.g., on interrupts or syscalls).
///
/// # Arguments
/// * `cpu_id` – The CPU core index.
/// * `stack_top` – The top address of the kernel stack (the stack grows down).
///
/// # Panics
/// Panics if `cpu_id >= MAX_CPUS`.
pub fn set_kernel_stack(cpu_id: usize, stack_top: u64) {
    assert!(cpu_id < MAX_CPUS, "CPU ID {} exceeds MAX_CPUS", cpu_id);
    unsafe {
        TSS_TABLE[cpu_id].rsp0 = stack_top;
    }
}

/// Sets an Interrupt Stack Table (IST) entry for a given CPU.
///
/// The IST provides dedicated stacks for specific interrupts (e.g., double fault).
/// This function sets one of the 7 IST entries.
///
/// # Arguments
/// * `cpu_id` – The CPU core index.
/// * `ist_index` – The IST index (1..7).
/// * `stack_top` – The top address of the stack for this IST.
///
/// # Panics
/// Panics if `cpu_id >= MAX_CPUS` or `ist_index` is not in 1..7.
pub fn set_ist(cpu_id: usize, ist_index: usize, stack_top: u64) {
    assert!(cpu_id < MAX_CPUS, "CPU ID {} exceeds MAX_CPUS", cpu_id);
    assert!((1..=7).contains(&ist_index), "IST index must be 1..=7");

    unsafe {
        match ist_index {
            1 => TSS_TABLE[cpu_id].ist1 = stack_top,
            2 => TSS_TABLE[cpu_id].ist2 = stack_top,
            3 => TSS_TABLE[cpu_id].ist3 = stack_top,
            4 => TSS_TABLE[cpu_id].ist4 = stack_top,
            5 => TSS_TABLE[cpu_id].ist5 = stack_top,
            6 => TSS_TABLE[cpu_id].ist6 = stack_top,
            7 => TSS_TABLE[cpu_id].ist7 = stack_top,
            _ => unreachable!(),
        }
    }
}

/// Initializes the GDT and TSS for the Bootstrap Processor (BSP, CPU 0).
///
/// This function:
/// 1. Sets up the TSS for CPU 0.
/// 2. Loads the GDT (`lgdt`).
/// 3. Loads the TSS selector (`ltr`) with `tss_selector(0)`.
///
/// # Safety
/// This is called during early boot with interrupts disabled.
pub fn init_bsp() {
    info!("Initializing for BSP (CPU#0)");
    unsafe {
        #[allow(static_mut_refs)]
        GLOBAL_GDT.set_tss(0, &raw const TSS_TABLE[0]);
        #[allow(static_mut_refs)]
        GLOBAL_GDT.load();

        core::arch::asm!(
            "ltr {0:x}",
            in(reg) tss_selector(0),
            options(nostack, preserves_flags)
        );
    }
    info!("BSP initialized successfully.");
}

/// Initializes the GDT and TSS for an Application Processor (AP).
///
/// This function:
/// 1. Sets up the TSS for the specified CPU.
/// 2. Loads the GDT (`lgdt`).
/// 3. Loads the TSS selector (`ltr`) with `tss_selector(cpu_id)`.
///
/// # Arguments
/// * `cpu_id` – The CPU core index (1..MAX_CPUS-1).
///
/// # Panics
/// Panics if `cpu_id >= MAX_CPUS`.
///
/// # Safety
/// This is called during AP boot with interrupts disabled.
pub fn init_ap(cpu_id: usize) {
    info!("Initializing for AP (CPU#{})", cpu_id);

    unsafe {
        #[allow(static_mut_refs)]
        GLOBAL_GDT.set_tss(cpu_id, &raw const TSS_TABLE[cpu_id]);
        #[allow(static_mut_refs)]
        GLOBAL_GDT.load();

        core::arch::asm!(
            "ltr {0:x}",
            in(reg) tss_selector(cpu_id),
            options(nostack, preserves_flags)
        );
    }

    info!("AP initialized successfully (Selector: {:#X})", tss_selector(cpu_id));
}

```

### `src/arch/amd64/acpi.rs`

```rs
//! # ACPI Subsystem (x86_64)
//!
//! This module provides the interface to the ACPI (Advanced Configuration and Power Interface)
//! tables and hardware, including the MADT (Multiple APIC Description Table), HPET, and APIC
//! (Advanced Programmable Interrupt Controller) initialization. It is responsible for
//! discovering and initialising the Local APIC and I/O APIC, parsing the ACPI tables for
//! processor topology, and providing functions for inter‑processor interrupts (IPIs).
//!
//! ## Overview
//!
//! ACPI is the standard for hardware discovery and power management on x86_64 systems.
//! The kernel uses ACPI to:
//! - Enumerate all CPU cores (via the MADT).
//! - Locate the Local APIC (LAPIC) and I/O APIC base addresses.
//! - Initialise the APIC timer and calibrate it using the HPET.
//! - Send IPIs to other cores for bootstrapping and inter‑core communication.
//!
//! ## Structure
//!
//! The module is divided into three sub‑modules:
//! - **`lapic`**: Local APIC programming (MMIO registers, EOI, timer, IPI sending).
//! - **`handler`**: An implementation of `acpi::Handler` that provides the ACPI library
//!   with a way to map physical memory, read/write I/O ports, and handle AML operations.
//! - **`acpi.rs` (this file)**: High‑level ACPI initialisation, table parsing, and IPI
//!   functions that use the `lapic` module and the global `TABLES` lazy‑static.
//!
//! ## Global State
//!
//! - **`TABLES`**: A `lazy_static` holding the parsed ACPI tables, obtained from the RSDP.
//! - **`TOTAL_CPUS`**: A `static mut` counter set during ACPI init to the number of CPUs.
//! - **`LAPIC_PHYS_ADDR`**: A `static mut` holding the physical address of the Local APIC.
//!
//! ## Initialisation Flow
//!
//! 1. **BSP**:
//!    - `acpi::init_bsp()` is called from `arch::late_init_bsp()`.
//!    - It calls `lapic::init()`, which maps the Local APIC into the kernel's virtual
//!      address space and parses the MADT to set `TOTAL_CPUS`.
//!    - It then calls `timer::init_bsp()` to set up the HPET mapping.
//!
//! 2. **All CPUs (BSP + APs)**:
//!    - `acpi::init()` is called from `arch::late_init()`.
//!    - It calls `lapic::enable()` to enable the Local APIC (set SVR, mask LVT entries).
//!    - It calls `timer::init()` to calibrate the APIC timer using the HPET.
//!    - Finally, interrupts are enabled with `sti`.
//!
//! ## IPI Functions
//!
//! - **`send_ipi(target_apic_id, vector, mode)`**: Sends an IPI with a specific delivery mode.
//! - **`send_fixed_ipi(target_apic_id, vector)`**: Convenience wrapper for a fixed IPI.
//! - **`eoi()`**: Writes to the EOI register of the Local APIC to signal the end of
//!   interrupt processing.
//!
//! ## Safety
//!
//! - The module uses `unsafe` to access MMIO registers of the Local APIC and HPET.
//! - The `TABLES` lazy‑static uses a `static mut` for the RSDP address, which is set
//!   during early boot.
//! - The `limine!` macro creates a static request for the RSDP, which is guaranteed
//!   by the bootloader.

use crate::{arch::timer, mem::kdm::Vaddr};

// ============================================================================
// SUBMODULES
// ============================================================================

pub mod lapic;
pub mod handler;

// ============================================================================
// RE-EXPORTS FROM ACPI CRATE
// ============================================================================

pub use ::acpi::platform::Processor;

// ============================================================================
// LIMINE REQUEST FOR RSDP
// ============================================================================

/// Limine request for the RSDP (Root System Description Pointer).
///
/// The RSDP is the entry point to the ACPI tables. It is provided by the
/// bootloader and is used by the ACPI library to parse all other tables.
limine! { RSDP <= RsdpRequest }

// ============================================================================
// LAZY‑STATIC ACPI TABLES
// ============================================================================

/// Parsed ACPI tables, lazily initialised from the RSDP.
///
/// The `AcpiTables` struct provides access to all ACPI tables (MADT, HPET,
/// DSDT, FADT, etc.) and includes a handler (`Hdl`) for platform‑specific
/// operations (memory mapping, I/O access).
///
/// # Panics
/// Panics if the RSDP response is unavailable or if table parsing fails.
lazy_static! {
    pub static ref TABLES: acpi::AcpiTables<handler::Hdl> = unsafe {
        acpi::AcpiTables::from_rsdp(
            handler::Hdl,
            Vaddr::from_raw(
                RSDP
                    .response()
                    .expect("Can't obtain RSDP")
                    .address as usize
                ).to_phys().to_raw()
        ).expect("Failed to parse ACPI tables")
    };
}

// ============================================================================
// GLOBAL STATE
// ============================================================================

/// Total number of CPU cores detected by ACPI.
///
/// This is set by `lapic::init()` and used by the scheduler and other subsystems.
/// It is `static mut` because it is written during early boot (single‑threaded)
/// and read thereafter.
pub static mut TOTAL_CPUS: usize = 0;

/// Physical address of the Local APIC.
///
/// This is set by `lapic::init()` and used internally for mapping.
pub static mut LAPIC_PHYS_ADDR: usize = 0;

/// Spurious interrupt vector used by the Local APIC.
pub const SPURIOUS_VECTOR: u8 = 0xFF;

// ============================================================================
// ACPI INITIALISATION
// ============================================================================

/// Initialises the Local APIC and HPET on the BSP.
///
/// This is called early in BSP initialisation, before memory management is
/// fully set up. It:
/// 1. Calls `lapic::init()` to parse the MADT and map the Local APIC.
/// 2. Calls `timer::init_bsp()` to map the HPET.
pub fn init_bsp() {
    lapic::init();
    timer::init_bsp();
}

/// Initialises the APIC and timer on all CPUs (BSP and APs).
///
/// This is called after memory management and the device model are ready.
/// It:
/// 1. Calls `lapic::enable()` to enable the Local APIC (set SVR, mask LVTs).
/// 2. Calls `timer::init()` to calibrate the APIC timer.
///
/// After this function returns, interrupts are enabled globally.
pub fn init() {
    lapic::enable();
    timer::init();
}

// ============================================================================
// END OF INTERRUPT (EOI)
// ============================================================================

/// Sends an End‑Of‑Interrupt (EOI) to the Local APIC.
///
/// This function writes a zero to the EOI register, signalling that the
/// current interrupt has been handled. It must be called at the end of
/// every interrupt handler.
///
/// # Safety
/// This function performs a volatile write to an MMIO register.
#[inline(always)]
pub fn eoi() {
    *lapic::LocalApic::new().eoi() = 0;
}

// ============================================================================
// INTER‑PROCESSOR INTERRUPTS (IPIs)
// ============================================================================

/// Delivery modes for IPIs.
///
/// These are the bits that are OR‑ed into the ICR (Interrupt Command Register)
/// to specify the delivery semantics of the IPI.
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum DeliveryMode {
    /// Deliver the interrupt to the target processor(s).
    Fixed        = 0b000 << 8,
    /// Deliver to the processor with the lowest priority.
    LowestPri    = 0b001 << 8,
    /// System Management Interrupt.
    Smi          = 0b010 << 8,
    /// Non‑Maskable Interrupt.
    Nmi          = 0b100 << 8,
    /// INIT IPI (reset the target processor).
    Init         = 0b101 << 8,
    /// Startup IPI (used for AP boot).
    StartUp      = 0b110 << 8,
}

/// Constants for the Interrupt Command Register (ICR).
pub const ICR_LEVEL_ASSERT:  u32 = 1 << 14;   // Assert the interrupt (vs. deassert).
pub const ICR_DEST_MODE_PHYS: u32 = 0 << 11;  // Physical destination mode (APIC ID).
pub const ICR_DEST_MODE_LOG:  u32 = 1 << 11;  // Logical destination mode.

/// Sends an IPI to a target APIC ID.
///
/// This function:
/// 1. Waits for the ICR to become free (bit 12 of ICR low is cleared).
/// 2. Writes the target APIC ID to the ICR high register.
/// 3. Writes the vector, delivery mode, and flags to the ICR low register,
///    which sends the IPI.
///
/// # Arguments
/// * `target_apic_id` – The APIC ID of the target CPU.
/// * `vector` – The interrupt vector to deliver.
/// * `mode` – The delivery mode.
#[inline]
pub fn send_ipi(target_apic_id: u32, vector: u8, mode: DeliveryMode) {
    let lapic = lapic::LocalApic::new();

    // Wait for the ICR to be free (bit 12 is the "delivery status" bit).
    while (*lapic.iclo() & (1 << 12)) != 0 {
        core::hint::spin_loop();
    }

    // Set the target APIC ID in the high register.
    *lapic.ichi() = target_apic_id << 24;

    // Set the vector, mode, and flags in the low register.
    let icr_low = (vector as u32)
        |   (mode as u32)
        |   ICR_DEST_MODE_PHYS
        |   ICR_LEVEL_ASSERT;

    *lapic.iclo() = icr_low;
}

/// Sends a fixed IPI (delivery mode = Fixed) to a target APIC ID.
///
/// This is a convenience wrapper around `send_ipi` with `DeliveryMode::Fixed`.
///
/// # Arguments
/// * `target_apic_id` – The APIC ID of the target CPU.
/// * `vector` – The interrupt vector to deliver.
#[inline]
pub fn send_fixed_ipi(target_apic_id: u32, vector: u8) {
    send_ipi(target_apic_id, vector, DeliveryMode::Fixed);
}

```

### `src/arch/amd64/mod.rs`

```rs
//! # x86_64 Architecture Support
//!
//! This module provides all architecture‑specific code for the x86_64 target.
//! It implements CPU initialization, interrupt handling, memory management,
//! system calls, timers, and ACPI support.
//!
//! ## Overview
//!
//! The architecture module is organized into several sub‑modules, each
//! responsible for a distinct aspect of the x86_64 platform:
//!
//! - **ACPI**: Advanced Configuration and Power Interface – discovers and
//!   manages hardware resources (APIC, HPET, CPUs, etc.).
//! - **GDT**: Global Descriptor Table – defines segmentation and TSS entries.
//! - **IDT**: Interrupt Descriptor Table – handles exceptions and interrupts.
//! - **Paging**: 4‑level page tables (PML4, PDPT, PD, PT) with support for
//!   huge pages (2 MiB, 1 GiB) and merging/splitting.
//! - **Timer**: HPET and APIC timer calibration and management.
//! - **Trap**: Trap frame definition for context switching.
//! - **Syscall**: System call entry point (via `syscall` instruction).
//! - **Per‑CPU**: Per‑CPU data structures (via `gs` segment).
//!
//! ## Initialization Flow
//!
//! The architecture subsystem is initialized in phases:
//!
//! 1. **Early Init** (`early_init`, `early_init_bs`)
//!    - Called very early in the boot process, before paging is fully set up.
//!    - Reads CPUID, APIC ID, detects support for `rdpid`.
//!    - Sets up the `IA32_TSC_AUX` MSR for per‑CPU identification.
//!    - Initializes the per‑CPU data structure.
//!
//! 2. **BSP Init** (`init_bsp`)
//!    - Called on the bootstrap processor (CPU 0) after early init.
//!    - Initializes GDT, IDT, and per‑CPU GS base.
//!    - Sets up the TSS and interrupt stack tables.
//!
//! 3. **Late Init** (`late_init_bsp`, `late_init`)
//!    - Called after memory management is initialized.
//!    - Initializes ACPI (MADT, HPET) and the APIC timer.
//!    - Enables interrupts (`sti`).
//!
//! 4. **AP Init** (`init_ap`)
//!    - Called on each Application Processor (AP) after the BSP has opened
//!      the appropriate Fueue barriers.
//!    - Initializes GDT, IDT, and per‑CPU GS base for the AP.
//!
//! ## CPU Identification
//!
//! Each CPU core is identified by its APIC ID. The kernel uses the `rdpid`
//! instruction (if available) or reads the `IA32_TSC_AUX` MSR to get the
//! current CPU's ID. This is used for per‑CPU data access and logging.
//!
//! ## Safety
//!
//! This module contains a significant amount of unsafe code, including:
//! - Inline assembly for privileged instructions (`rdmsr`, `wrmsr`, `cpuid`, etc.).
//! - Manipulation of the GDT, IDT, and TSS via raw pointers.
//! - Naked functions for interrupt and syscall entry points.
//! - Access to `static mut` data (e.g., `GLOBAL_GDT`, `GLOBAL_IDT`).
//!
//! The unsafe operations are required for kernel‑level hardware control and
//! are carefully encapsulated in safe interfaces.

// ============================================================================
// SUBMODULES
// ============================================================================

pub mod acpi;     // ACPI tables, MADT, HPET, APIC
pub mod idt;      // Interrupt Descriptor Table, exception handlers
pub mod gdt;      // Global Descriptor Table, TSS, segmentation
pub mod percpu;   // Per‑CPU data (via GS base)
pub mod paging;   // 4‑level paging, huge pages, page table manipulation
pub mod timer;    // HPET and APIC timer calibration
pub mod trap;     // Trap frame definition
pub mod syscall;  // System call entry point

// ============================================================================
// IMPORTS
// ============================================================================

use core::arch::x86_64;
use core::arch::asm;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
pub use syscall::init as init_syscall;

// ============================================================================
// MSR CONSTANTS
// ============================================================================

/// IA32_TSC_AUX MSR – stores the APIC ID for `rdtscp` and `rdpid` fallback.
const IA32_TSC_AUX: u32 = 0xC0000103;

// ============================================================================
// CPUID CONSTANTS
// ============================================================================

const CPUID_MAX_LEAF: u32 = 0x00;
const CPUID_PROC_INFO: u32 = 0x01;
const CPUID_X2APIC: u32 = 0x0B;
const CPUID_EXT_FEATURES: u32 = 0x07;

// ============================================================================
// CPUID HELPERS
// ============================================================================

/// Result of a `cpuid` instruction.
#[derive(Debug, Clone, Copy)]
pub struct CpuidResult {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

/// Executes the `cpuid` instruction.
///
/// # Arguments
/// * `leaf` – The main CPUID leaf.
/// * `subleaf` – The sub‑leaf (ECX value).
///
/// # Returns
/// A `CpuidResult` containing the values of EAX, EBX, ECX, EDX.
#[inline]
pub fn cpuid(leaf: u32, subleaf: u32) -> CpuidResult {
    let res = x86_64::__cpuid_count(leaf, subleaf);
    CpuidResult {
        eax: res.eax,
        ebx: res.ebx,
        ecx: res.ecx,
        edx: res.edx,
    }
}

// ============================================================================
// MSR HELPERS
// ============================================================================

/// Reads a Model‑Specific Register (MSR).
///
/// # Safety
/// The caller must ensure that the MSR is valid and accessible.
#[inline]
pub unsafe fn rdmsr(msr: u32) -> u64 {
    let (lo, hi): (u32, u32);
    unsafe {
        asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") lo,
            out("edx") hi,
            options(nostack, preserves_flags),
        );
    }
    ((hi as u64) << 32) | (lo as u64)
}

/// Writes a Model‑Specific Register (MSR).
///
/// # Safety
/// The caller must ensure that the MSR is valid and writable.
#[inline]
pub unsafe fn wrmsr(msr: u32, value: u64) {
    let lo = value as u32;
    let hi = (value >> 32) as u32;
    unsafe {
        asm!(
            "wrmsr",
            in("ecx") msr,
            in("eax") lo,
            in("edx") hi,
            options(nostack, preserves_flags),
        );
    }
}

// ============================================================================
// CPU DETECTION
// ============================================================================

/// Returns the maximum CPUID leaf supported by the CPU.
#[inline]
fn max_cpuid_leaf() -> u32 {
    cpuid(CPUID_MAX_LEAF, 0).eax
}

/// Reads the APIC ID of the current CPU.
///
/// Uses `x2APIC` if available, otherwise falls back to the legacy APIC ID.
fn read_apic_id() -> u32 {
    let max_leaf = max_cpuid_leaf();

    let x2apic_supported = if max_leaf >= CPUID_PROC_INFO {
        let r = cpuid(CPUID_PROC_INFO, 0);
        (r.ecx & (1 << 21)) != 0
    } else {
        false
    };

    if x2apic_supported && max_leaf >= CPUID_X2APIC {
        let r = cpuid(CPUID_X2APIC, 0);
        return r.edx;
    }

    if max_leaf >= CPUID_PROC_INFO {
        let r = cpuid(CPUID_PROC_INFO, 0);
        return (r.ebx >> 24) & 0xFF;
    }

    0
}

/// Checks if the `rdpid` instruction is available.
fn has_rdpid() -> bool {
    let max_leaf = max_cpuid_leaf();
    if max_leaf < CPUID_EXT_FEATURES {
        return false;
    }
    let r = cpuid(CPUID_EXT_FEATURES, 0);
    (r.ecx & (1 << 22)) != 0
}

/// Reads the current CPU's ID using `rdpid` (if available).
///
/// # Note
/// This is a raw instruction; the result is the APIC ID stored in `IA32_TSC_AUX`.
#[inline(always)]
fn rdpid_raw() -> usize {
    let id: u64;
    unsafe {
        asm!(
            "rdpid {}",
            out(reg) id,
            options(nostack, preserves_flags),
        );
    }
    id as usize
}

// ============================================================================
// GLOBAL STATE
// ============================================================================

/// Whether `rdpid` is available on the current CPU.
static RDPID_AVAILABLE: AtomicBool = AtomicBool::new(false);

/// Maximum number of supported CPU cores.
///
/// This is a compile‑time limit; if the system has more cores, the kernel
/// will panic.
pub const MAX_CPUS: usize = 64;

/// Time since boot in milliseconds (updated by timer tick on BSP).
pub static TIME_FROM_BOOT: AtomicU64 = AtomicU64::new(0);

// ============================================================================
// INITIALIZATION FUNCTIONS
// ============================================================================

/// Early initialization for the BSP (Bootstrap Processor).
///
/// This is a thin wrapper around `early_init` for symmetry with APs.
pub fn early_init_bs() {
    early_init();
}

/// Early initialization for all CPUs (BSP and APs).
///
/// This function:
/// 1. Reads the APIC ID.
/// 2. Writes it to the `IA32_TSC_AUX` MSR.
/// 3. Detects support for `rdpid`.
/// 4. Initializes the per‑CPU data structure with the CPU ID.
/// 5. Logs the CPU's APIC ID and `rdpid` support.
/// 6. Panics if the CPU ID exceeds `MAX_CPUS`.
///
/// # Panics
/// If the CPU ID >= `MAX_CPUS`, the kernel halts.
pub fn early_init() {
    let apic_id = read_apic_id();

    unsafe {
        wrmsr(IA32_TSC_AUX, apic_id as u64);
    }

    let rdpid_ok = has_rdpid();
    RDPID_AVAILABLE.store(rdpid_ok, Ordering::Release);

    let cpu_id = current_cpu();

    let pcpu = percpu::current();
    pcpu.cpu_id = cpu_id;

    crate::info!(
        "APIC ID = {}, RDPID = {}",
        apic_id,
        if rdpid_ok { "yes" } else { "no" }
    );

    if cpu_id > MAX_CPUS - 1 {
        error!("Too high CPU detected. Gonna sleep (Zzz...)");
        unsafe {
            core::arch::asm! {
                "2:",
                "cli",
                "hlt",
                "jmp 2b"
            }
        }
        unreachable!()
    }
}

/// Returns the ID of the current CPU.
///
/// If `rdpid` is available, uses the `rdpid` instruction.
/// Otherwise, reads the `IA32_TSC_AUX` MSR.
#[inline(always)]
pub fn current_cpu() -> usize {
    if RDPID_AVAILABLE.load(Ordering::Acquire) {
        rdpid_raw()
    } else {
        unsafe { rdmsr(IA32_TSC_AUX) as usize }
    }
}

/// Full initialization for the BSP.
///
/// This function is called after early init and before memory management.
/// It initializes:
/// - Per‑CPU data (`percpu::init`)
/// - GDT (`gdt::init_bsp`)
/// - IDT (`idt::init_bsp`)
/// - Per‑CPU GS base (`percpu::init_syscall_gs`)
pub fn init_bsp() {
    percpu::init();
    gdt::init_bsp();
    idt::init_bsp();
    percpu::init_syscall_gs(0, 0);
}

/// Full initialization for APs.
///
/// This function is called on each AP after waiting for `ARCH_INIT`.
/// It initializes:
/// - GDT (`gdt::init_ap`)
/// - IDT (`idt::init_ap`) – reuses the BSP's IDT
/// - Per‑CPU GS base (`percpu::init_syscall_gs`)
pub fn init_ap() {
    gdt::init_ap(current_cpu());
    idt::init_ap();
    percpu::init_syscall_gs(0, 0);
}

/// Late initialization for the BSP.
///
/// This is called after memory management and the device model are initialized.
/// It initializes ACPI and the HPET timer.
pub fn late_init_bsp() {
    acpi::init_bsp();
}

/// Late initialization for all CPUs.
///
/// This function:
/// 1. Initializes ACPI (APIC, timer) via `acpi::init()`.
/// 2. Enables interrupts (`sti`).
///
/// Interrupts are enabled here, after all exception handlers and the timer
/// are set up.
pub fn late_init() {
    acpi::init();
    unsafe {
        core::arch::asm! {
            "sti"
        }
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Returns the number of CPUs detected by ACPI.
///
/// # Safety
/// This uses a `static mut` value set by `acpi::init_bsp`.
pub fn num_cpus() -> usize {
    #[allow(static_mut_refs)]
    unsafe {
        acpi::TOTAL_CPUS
    }
}

/// Returns the time since boot in milliseconds.
#[inline]
pub fn get_time_from_boot() -> u64 {
    TIME_FROM_BOOT.load(Ordering::Relaxed)
}

/// Returns the time since boot in seconds (as a floating‑point value).
#[inline]
pub fn get_time_from_boot_s() -> f32 {
    get_time_from_boot() as f32 / 1000.0
}

/// Halts the system.
///
/// This function never returns; it enters an infinite loop with `hlt`.
pub fn exit() -> ! {
    loop {
        unsafe {
            core::arch::asm! {
                "2:",
                "cli",
                "hlt",
                "jmp 2b",
            }
        }
    }
}

```

### `src/arch/amd64/trap.rs`

```rs
//! # Trap Frame (x86_64)
//!
//! This module defines the `TrapFrame` structure, which captures the CPU state
//! at the moment of an interrupt, exception, or system call. It is used by the
//! interrupt handlers, the scheduler, and the syscall dispatcher to save and
//! restore the context of a task.
//!
//! ## Overview
//!
//! When an interrupt or exception occurs, the CPU automatically pushes certain
//! registers onto the stack (the "hardware‑saved" part). The kernel's interrupt
//! handlers then push the remaining general‑purpose registers (the "software‑
//! saved" part) to form a complete `TrapFrame`. The same structure is used for
//! system calls, where the `syscall_entry` wrapper builds a trap frame manually.
//!
//! ## Layout
//!
//! The `TrapFrame` is laid out as follows:
//!
//! ```text
//! +------------------+  <-- lower addresses (top of stack)
//! | rax              |  software‑saved (15 registers)
//! | rbx              |
//! | rcx              |
//! | rdx              |
//! | rsi              |
//! | rdi              |
//! | rbp              |
//! | r8               |
//! | r9               |
//! | r10              |
//! | r11              |
//! | r12              |
//! | r13              |
//! | r14              |
//! | r15              |
//! +------------------+
//! | rip              |  hardware‑saved (5 registers)
//! | cs               |
//! | rflags           |
//! | rsp              |
//! | ss               |
//! +------------------+  <-- higher addresses (original stack)
//! ```
//!
//! The hardware‑saved part is pushed by the CPU automatically:
//! - On interrupts/exceptions: the CPU pushes `SS`, `RSP`, `RFLAGS`, `CS`, `RIP`
//!   (and sometimes an error code, which is handled separately).
//! - On `syscall`: the CPU does not push these; the wrapper builds them manually.
//!
//! The software‑saved part is pushed by the handler (or wrapper) to preserve all
//! general‑purpose registers that may be clobbered.
//!
//! ## Usage
//!
//! - **Interrupt handlers**: The `timer_wrapper` and `yield_wrapper` functions
//!   save the context into a `TrapFrame` and pass it to the scheduler.
//! - **Syscall dispatcher**: The `syscall_entry` wrapper builds a trap frame
//!   and passes it to `sched::syscall_dispatcher`.
//! - **Scheduler**: During context switching, the scheduler saves the current
//!   task's trap frame and loads the next task's trap frame.
//! - **Exception handlers**: The IDT exception handlers receive a similar
//!   frame (via the `x86-interrupt` ABI) and may convert it to a `TrapFrame`
//!   or use it directly.
//!
//! ## Safety
//!
//! The `TrapFrame` is `repr(C)` and is accessed via raw pointers in assembly
//! and in the scheduler. The layout must match the assembly code exactly.
//! Changing the order or size of fields will break the interrupt handlers
//! and context switching.

// ============================================================================
// TRAP FRAME STRUCTURE
// ============================================================================

/// A complete snapshot of the CPU state at the time of an interrupt, exception,
/// or system call.
///
/// This struct is `repr(C)` to guarantee a stable layout, matching the
/// assembly code that builds and restores the frame.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapFrame {
    // ========================================================================
    // Software‑saved registers (pushed by the kernel)
    // ========================================================================

    /// RAX – general‑purpose register, often used as the syscall number and
    /// return value.
    pub rax: u64,
    /// RBX – general‑purpose register, saved but not used for syscalls.
    pub rbx: u64,
    /// RCX – general‑purpose register; on `syscall` entry, it holds the user
    /// return address (`RIP`).
    pub rcx: u64,
    /// RDX – general‑purpose register, often used as the third syscall argument.
    pub rdx: u64,
    /// RSI – general‑purpose register, used as the second syscall argument.
    pub rsi: u64,
    /// RDI – general‑purpose register, used as the first syscall argument.
    pub rdi: u64,
    /// RBP – base pointer, saved for stack frame debugging.
    pub rbp: u64,
    /// R8 – general‑purpose register, used as the fifth syscall argument.
    pub r8: u64,
    /// R9 – general‑purpose register, used as the sixth syscall argument.
    pub r9: u64,
    /// R10 – general‑purpose register, used as the fourth syscall argument
    /// (the `syscall` instruction clobbers RCX and R11, so R10 is used instead
    /// of RCX for the fourth argument).
    pub r10: u64,
    /// R11 – general‑purpose register; on `syscall` entry, it holds the user
    /// `RFLAGS`.
    pub r11: u64,
    /// R12 – general‑purpose register, callee‑saved.
    pub r12: u64,
    /// R13 – general‑purpose register, callee‑saved.
    pub r13: u64,
    /// R14 – general‑purpose register, callee‑saved.
    pub r14: u64,
    /// R15 – general‑purpose register, callee‑saved.
    pub r15: u64,

    // ========================================================================
    // Hardware‑saved registers (pushed by the CPU or manually constructed)
    // ========================================================================

    /// Instruction pointer – the address to return to after the interrupt.
    pub rip: u64,
    /// Code segment selector (with RPL) – indicates the privilege level of
    /// the interrupted context (e.g., `0x08 | 0` for kernel, `0x18 | 3` for user).
    pub cs: u64,
    /// RFLAGS register – contains CPU flags (interrupt flag, direction flag, etc.).
    pub rflags: u64,
    /// Stack pointer – the user or kernel stack pointer at the time of the
    /// interrupt.
    pub rsp: u64,
    /// Stack segment selector – used with `RSP` to form the full stack address.
    pub ss: u64,
}

```

### `src/arch/amd64/timer.rs`

```rs
//! # HPET and APIC Timer Management (x86_64)
//!
//! This module manages the High Precision Event Timer (HPET) and the Local APIC
//! timer on x86_64 systems. The APIC timer is used as the primary system tick
//! source, while the HPET is used for calibration during early boot.
//!
//! ## Overview
//!
//! The kernel uses two timers:
//! - **HPET**: A high‑resolution timer that is used to calibrate the APIC timer.
//!   The HPET is memory‑mapped and provides a stable counter with a known
//!   frequency (usually 10 MHz or higher).
//! - **APIC Timer**: A per‑CPU timer that generates periodic interrupts. It is
//!   calibrated against the HPET to determine the number of ticks per 10 ms.
//!
//! ## Calibration Process
//!
//! The calibration is performed during `timer::init()`:
//!
//! 1. The HPET is disabled and reset to zero.
//! 2. The APIC timer is set to one‑shot mode with a maximum count (`!0`).
//! 3. The HPET is enabled.
//! 4. The kernel spins, waiting for a fixed number of HPET ticks (1 second).
//! 5. The APIC timer's current count is read and subtracted from the initial
//!    maximum value to determine the number of APIC ticks in 1 second.
//! 6. The result is stored in `TICKS_PER_10MS` (divided by 100 to get 10 ms ticks).
//! 7. The APIC timer is set to periodic mode with the calibrated count.
//!
//! ## Timer Interrupt
//!
//! The APIC timer fires at vector `TIMER_VECTOR` (32). The interrupt handler
//! (`timer_wrapper`) is a naked function that saves the CPU context and calls
//! `sched::timer_tick()`, which updates the system time and performs scheduling.
//!
//! ## HPET Mapping
//!
//! The HPET is mapped into the kernel's virtual address space at a fixed address
//! (`HPET_VMA`) during `init_bsp()`. The mapping uses cache‑disabled and write‑
//! through attributes to ensure correct timing.
//!
//! ## Safety
//!
//! - The HPET and APIC registers are accessed via MMIO and MSRs, which are
//!   privileged operations.
//! - The `timer_wrapper` is a naked function that uses inline assembly to save
//!   and restore the CPU context.
//! - The calibration function spins with interrupts disabled; this is safe
//!   because it is called before interrupts are enabled.

use core::hint::unlikely;

use crate::{arch::paging::EntryFlags, mem::kdm::{Paddr, Vaddr}, sync::Nutex};
use core::arch::naked_asm;
use core::sync::atomic::{AtomicU64, Ordering};

// ============================================================================
// GLOBAL STATE
// ============================================================================

/// The number of APIC timer ticks per 10 ms, calibrated at boot.
static TICKS_PER_10MS: AtomicU64 = AtomicU64::new(0);

// ============================================================================
// TIMER WRAPPER (NAKED INTERRUPT HANDLER)
// ============================================================================

/// Naked interrupt wrapper for the APIC timer.
///
/// This function is called on vector 32. It:
/// 1. Saves the CPU context (including `swapgs` if coming from user mode).
/// 2. Calls `sched::timer_tick` with the trap frame.
/// 3. Restores the context and returns via `iretq`.
///
/// # Safety
/// This is a naked function that manipulates the stack and registers directly.
#[unsafe(naked)]
pub unsafe extern "C" fn timer_wrapper() -> ! {
    naked_asm!(
        // If we came from user mode (RPL 3), swap GS.
        "mov rax, [rsp + 8]",
        "and rax, 3",
        "cmp rax, 3",
        "jne 1f",
        "swapgs",
        "1:",

        // Save all general‑purpose registers on the stack.
        "push r15", "push r14", "push r13", "push r12",
        "push r11", "push r10", "push r9", "push r8",
        "push rbp", "push rdi", "push rsi", "push rdx",
        "push rcx", "push rbx", "push rax",

        // Call the scheduler tick handler with the trap frame.
        "mov rdi, rsp",
        "call {scheduler_tick}",

        // Restore all general‑purpose registers.
        "pop rax", "pop rbx", "pop rcx", "pop rdx",
        "pop rsi", "pop rdi", "pop rbp", "pop r8",
        "pop r9", "pop r10", "pop r11", "pop r12",
        "pop r13", "pop r14", "pop r15",

        // If we came from user mode, swap GS back.
        "mov rax, [rsp + 8]",
        "and rax, 3",
        "cmp rax, 3",
        "jne 2f",
        "swapgs",
        "2:",

        // Return from interrupt.
        "iretq",

        scheduler_tick = sym crate::sched::timer_tick,
    );
}

// ============================================================================
// HPET CONSTANTS AND STRUCTURE
// ============================================================================

/// Virtual address where the HPET is mapped.
///
/// The HPET is mapped to a fixed address just below the LAPIC mapping.
const HPET_VMA: usize = 0xFFFFFFFFFFFFE000;

/// HPET register offsets.
struct HpetOffsets;
impl HpetOffsets {
    const HPET_CAP: usize = 0x000;     // Capabilities register (RO)
    const HPET_CFG: usize = 0x010;     // Configuration register (RW)
    const HPET_COUNTER: usize = 0x0F0; // Main counter (RW)
}

/// A handle to the HPET.
///
/// This struct provides methods to access the HPET registers via MMIO.
#[derive(Debug, Clone, Copy)]
pub struct Hpet;

impl Hpet {
    /// Enable bit for the HPET configuration register.
    pub const ENABLE: u32 = 1;

    /// Disables the HPET (clears the enable bit).
    #[inline(always)]
    pub fn disable(&self) {
        *self.cfg() &= !Hpet::ENABLE;
    }

    /// Enables the HPET (sets the enable bit).
    #[inline(always)]
    pub fn enable(&self) {
        *self.cfg() |= Hpet::ENABLE;
    }

    /// Resets the HPET counter to zero.
    #[inline(always)]
    pub fn reset(&self) {
        *self.counter() = 0;
    }

    // ---- Register accessors ----

    /// Returns a reference to the capabilities register (read‑only).
    #[inline(always)]
    pub fn cap(&self) -> u64 {
        *Vaddr::from_raw(HPET_VMA + HpetOffsets::HPET_CAP).to_ref::<u64>()
    }

    /// Returns a mutable reference to the configuration register.
    #[inline(always)]
    pub fn cfg(&self) -> &mut u32 {
        Vaddr::from_raw(HPET_VMA + HpetOffsets::HPET_CFG).to_ref_mut::<u32>()
    }

    /// Returns a mutable reference to the main counter register.
    #[inline(always)]
    pub fn counter(&self) -> &mut u64 {
        Vaddr::from_raw(HPET_VMA + HpetOffsets::HPET_COUNTER).to_ref_mut::<u64>()
    }
}

/// The global HPET instance, protected by a `Nutex`.
pub static INSTANCE: Nutex<Hpet> = Nutex::new(Hpet);

// ============================================================================
// HPET INITIALISATION
// ============================================================================

/// Initialises the HPET on the BSP.
///
/// This function:
/// 1. Parses the ACPI HPET table to get the physical base address.
/// 2. Maps the HPET MMIO region into the kernel's virtual address space at
///    `HPET_VMA` using a 4 KiB page with cache‑disabled and write‑through flags.
///
/// # Panics
/// - If the HPET table is not found.
/// - If the mapping fails.
pub fn init_bsp() {
    let hpet_info = acpi::HpetInfo::new(&super::acpi::TABLES).expect("Failed to parse HPET table");
    let hpet_base_paddr = hpet_info.base_address;

    info!("HPET found at physical address: {:p}", hpet_base_paddr as *const ());

    match crate::mem::PTM.lock().map_4k_block(
        HPET_VMA,
        Paddr::from_raw(hpet_base_paddr),
        EntryFlags::PRESENT
            | EntryFlags::WRITABLE
            | EntryFlags::CACHE_DISABLE
            | EntryFlags::WRITE_THROUGH
    ) {
        Ok(_) => {},
        Err(e) => panic!("Can't map HPET: {}", e)
    };
}

// ============================================================================
// APIC TIMER CALIBRATION AND INITIALISATION
// ============================================================================

/// Calibrates and initialises the APIC timer on all CPUs.
///
/// This function is called from `acpi::init()` after the HPET is mapped.
/// It performs the calibration on the BSP (CPU 0) and sets the APIC timer
/// to periodic mode with the calibrated count.
///
/// The calibration process:
/// 1. Reads the HPET period (in femtoseconds) from the capabilities register.
/// 2. Computes the number of HPET ticks in 1 second.
/// 3. Disables and resets the HPET.
/// 4. Sets the APIC timer to one‑shot mode with a maximum count.
/// 5. Enables the HPET and spins until 1 second has elapsed.
/// 6. Reads the remaining APIC timer count and computes elapsed ticks.
/// 7. Stores the elapsed ticks in `TICKS_PER_10MS` (dividing by 100).
/// 8. Sets the APIC timer to periodic mode and the initial count.
///
/// # Panics
/// - If the HPET period is zero (should never happen).
/// - If the calibration fails (e.g., the APIC timer does not fire).
///
/// # Safety
/// This function performs MMIO and MSR writes. It is called with interrupts
/// disabled and is single‑threaded on the BSP.
pub fn init() {
    // Sync guaranteed, so we can temporarily go direct (no sync primitives).
    let inst = Hpet;
    let lapic = super::acpi::lapic::LocalApic;

    // Read the HPET period from the capabilities register.
    let cap = inst.cap();
    let period_fs = cap >> 32;
    if unlikely(period_fs == 0) {
        panic!("HPET period is 0, cannot calibrate!")
    }

    // Target: 1 second (10^12 femtoseconds).
    let target_fs = 1_000_000_000_000u64;
    let hpet_ticks_to_wait = target_fs / period_fs;

    // Prepare the APIC timer.
    inst.disable();
    inst.reset();

    // Set divisor to x16 (code 3) and timer to one‑shot mode.
    *lapic.div() = 3;               // x16
    *lapic.lvt_timer() = 0x00010000; // oneshot, masked initially
    *lapic.icr() = !0;              // maximum initial value

    // Start the HPET.
    inst.enable();

    // Wait for the HPET to reach the target count.
    let start_hpet = *inst.counter();
    while (*inst.counter() - start_hpet) < hpet_ticks_to_wait {
        core::hint::spin_loop();
    }

    inst.disable();

    // Read the remaining APIC timer count and compute elapsed ticks.
    let cur_lapic = *lapic.ccr();
    let elapsed = !0 - cur_lapic;

    // Store the number of ticks per 10 ms (for the scheduler).
    TICKS_PER_10MS.store(elapsed as u64, Ordering::Relaxed);

    info!("APIC timer calibrated: {} ticks per 10ms", elapsed);

    // Set the APIC timer to periodic mode with the calibrated count.
    *lapic.lvt_timer() = (1 << 17) | (crate::arch::idt::TIMER_VECTOR as u32); // periodic
    *lapic.icr() = elapsed;
}

// ============================================================================
// PUBLIC API
// ============================================================================

/// Returns the number of APIC timer ticks per 10 ms.
///
/// This value is used by the scheduler to convert real time (10 ms ticks)
/// into virtual runtime increments.
#[inline]
pub fn get_ticks_per_10ms() -> u64 {
    TICKS_PER_10MS.load(Ordering::Relaxed)
}

```

### `src/arch/amd64/syscall.rs`

```rs
//! # System Call Entry (x86_64)
//!
//! This module implements the low‑level system call entry point for the x86_64
//! architecture, using the `syscall` instruction. It sets up the necessary MSRs
//! to enable fast system calls from user mode and provides the assembly wrapper
//! that switches to the kernel stack, saves the context, and dispatches the
//! system call to the scheduler.
//!
//! ## Overview
//!
//! On x86_64, the `syscall` instruction is the preferred mechanism for making
//! system calls from user space (Ring 3) to the kernel (Ring 0). It provides a
//! fast, low‑overhead entry point that is supported on all modern CPUs.
//!
//! The `syscall` instruction:
//! - Loads `RIP` from `IA32_LSTAR` MSR.
//! - Loads `CS` and `SS` from `IA32_STAR` MSR.
//! - Switches to Ring 0.
//! - Saves the return address in `RCX` and the old RFLAGS in `R11`.
//!
//! ## MSR Configuration
//!
//! The following MSRs are configured during `syscall::init()`:
//!
//! - **`IA32_EFER`** (0xC0000080): The `SCE` (System Call Extensions) bit is set
//!   to enable the `syscall` instruction in 64‑bit mode.
//! - **`IA32_STAR`** (0xC0000081): The upper 32 bits hold the kernel CS/SS
//!   selectors; the lower 32 bits hold the user CS/SS selectors.
//!   - Kernel CS = `0x08`, Kernel SS = `0x10`
//!   - User CS = `0x18`, User SS = `0x20`
//! - **`IA32_LSTAR`** (0xC0000082): Holds the address of the syscall entry point
//!   (`syscall_entry`).
//! - **`IA32_FMASK`** (0xC0000084): Sets the RFLAGS mask applied during `syscall`;
//!   we mask out bits 8 and 9 (TF and IF) to disable single‑step and interrupts.
//!
//! ## Syscall Entry Wrapper (`syscall_entry`)
//!
//! The entry point is a **naked** function written in inline assembly. It:
//! 1. **Swaps GS** (via `swapgs`) to switch from the user `gs` base to the
//!    kernel `gs` base (which points to per‑CPU data).
//! 2. **Saves the user stack pointer** (`RSP`) and switches to the kernel stack
//!    (stored in `gs:[8]`).
//! 3. **Saves all general‑purpose registers** on the kernel stack, building a
//!    `TrapFrame` that can be passed to the dispatcher.
//! 4. **Calls the syscall dispatcher** (`sched::syscall_dispatcher`) with a
//!    pointer to the trap frame.
//! 5. **Restores all registers** (except `RCX` and `R11`, which are used by
//!    `sysret` to restore the user RIP and RFLAGS).
//! 6. **Swaps GS back** and executes `sysret` to return to user mode.
//!
//! ## Syscall Dispatcher
//!
//! The dispatcher is implemented in the scheduler module (`sched::syscall_dispatcher`).
//! It retrieves the current process and delegates to its `syscall_handler` function,
//! which interprets the syscall number and arguments.
//!
//! ## Safety
//!
//! - The `syscall_entry` function is a naked function that manipulates the stack
//!   and registers directly. It must be written with extreme care to avoid
//!   corrupting the CPU state.
//! - The `swapgs` instruction is used to switch between user and kernel GS.
//!   It must be paired correctly with `swapgs` on the return path.
//! - The kernel stack pointer is read from `gs:[8]`, which is set up by the
//!   scheduler when switching tasks.
//! - The MSR writes (`wrmsr`) are privileged operations that require the kernel
//!   to be running in Ring 0.

use core::arch::naked_asm;
use crate::arch::{rdmsr, wrmsr};

// ============================================================================
// MSR CONSTANTS
// ============================================================================

/// Extended Feature Enable Register (EFER) – enables `syscall` in 64‑bit mode.
pub const IA32_EFER: u32 = 0xC0000080;

/// System Call Target Address Register (STAR) – holds the CS/SS selectors.
/// - Upper 32 bits: Kernel CS (bits 32‑47) and Kernel SS (bits 48‑63).
/// - Lower 32 bits: User CS (bits 0‑15) and User SS (bits 16‑31).
pub const IA32_STAR: u32 = 0xC0000081;

/// System Call Target Address Register (LSTAR) – holds the RIP of the syscall handler.
pub const IA32_LSTAR: u32 = 0xC0000082;

/// System Call Flag Mask (FMASK) – masks RFLAGS bits during `syscall`.
/// We mask out TF (bit 8) and IF (bit 9) to disable single‑step and interrupts.
pub const IA32_FMASK: u32 = 0xC0000084;

// ============================================================================
// SYSCALL ENTRY (NAKED FUNCTION)
// ============================================================================

/// The entry point for all system calls.
///
/// This function is called by the CPU via the `syscall` instruction. It is a
/// naked function written in inline assembly that:
/// 1. Switches GS from user to kernel.
/// 2. Saves the user stack pointer and switches to the kernel stack.
/// 3. Saves all registers on the kernel stack (building a trap frame).
/// 4. Calls the system call dispatcher (`sched::syscall_dispatcher`).
/// 5. Restores registers and returns to user mode via `sysret`.
///
/// # Registers at Entry
///
/// On entry via `syscall`:
/// - `RCX` = user `RIP` (return address)
/// - `R11` = user `RFLAGS`
/// - `RAX` = system call number
/// - `RDI`, `RSI`, `RDX`, `R10`, `R8`, `R9` = arguments
///
/// # Safety
/// This is a naked function that manipulates the stack and registers directly.
/// It must not be called directly; it is only invoked by the CPU via the
/// `syscall` instruction.
#[unsafe(naked)]
pub unsafe extern "C" fn syscall_entry() -> ! {
    naked_asm!(
        // --------------------------------------------------------------------
        // 1. Switch from user GS to kernel GS (per‑CPU data).
        // --------------------------------------------------------------------
        "swapgs",

        // --------------------------------------------------------------------
        // 2. Save the user stack pointer and switch to the kernel stack.
        //    The kernel stack top is stored in gs:[8] (offset 8 in PerCpu).
        // --------------------------------------------------------------------
        "mov rbx, rsp",             // Save user RSP in RBX (will be saved later).
        "mov rsp, gs:[8]",          // Load kernel stack top from per‑CPU data.

        // --------------------------------------------------------------------
        // 3. Allocate space for the trap frame on the kernel stack.
        //    The trap frame layout matches the TrapFrame struct:
        //    RAX, RBX, RCX, RDX, RSI, RDI, RBP, R8, R9, R10, R11, R12, R13, R14, R15,
        //    then the hardware‑saved part: RIP, CS, RFLAGS, RSP, SS.
        // --------------------------------------------------------------------
        "sub rsp, 160",             // 15 general‑purpose registers (8 bytes each) = 120 bytes,
                                    // plus 5 hardware fields (40 bytes) = 160 bytes total.

        // Save all general‑purpose registers to the trap frame.
        "mov [rsp + 0], rax",
        "mov [rsp + 8], rbx",       // User RSP (saved earlier).
        "mov [rsp + 16], rcx",      // User RIP (from syscall).
        "mov [rsp + 24], rdx",
        "mov [rsp + 32], rsi",
        "mov [rsp + 40], rdi",
        "mov [rsp + 48], rbp",
        "mov [rsp + 56], r8",
        "mov [rsp + 64], r9",
        "mov [rsp + 72], r10",
        "mov [rsp + 80], r11",      // User RFLAGS (from syscall).
        "mov [rsp + 88], r12",
        "mov [rsp + 96], r13",
        "mov [rsp + 104], r14",
        "mov [rsp + 112], r15",

        // Save the hardware‑saved fields of the trap frame.
        "mov [rsp + 120], rcx",     // RIP (from syscall).
        "mov [rsp + 128], 0x18",    // CS (user code selector + RPL 3).
        "mov [rsp + 136], r11",     // RFLAGS (from syscall).
        "mov [rsp + 144], rbx",     // RSP (user stack).
        "mov [rsp + 152], 0x20",    // SS (user data selector + RPL 3).

        // --------------------------------------------------------------------
        // 4. Call the syscall dispatcher.
        //    RDI = pointer to the trap frame (RSP).
        // --------------------------------------------------------------------
        "mov rdi, rsp",
        "call {syscall_dispatcher}",

        // --------------------------------------------------------------------
        // 5. Restore registers (except RCX and R11, which are restored by sysret).
        // --------------------------------------------------------------------
        "mov rax, [rsp + 0]",
        "mov rbx, [rsp + 8]",
        "mov rdx, [rsp + 24]",
        "mov rsi, [rsp + 32]",
        "mov rdi, [rsp + 40]",
        "mov rbp, [rsp + 48]",
        "mov r8, [rsp + 56]",
        "mov r9, [rsp + 64]",
        "mov r10, [rsp + 72]",
        "mov r12, [rsp + 88]",
        "mov r13, [rsp + 96]",
        "mov r14, [rsp + 104]",
        "mov r15, [rsp + 112]",

        // Restore RCX (user RIP) and R11 (user RFLAGS) for sysret.
        "mov rcx, [rsp + 120]",
        "mov r11, [rsp + 136]",

        // Restore the user stack pointer (RSP) from the trap frame.
        "mov rsp, [rsp + 144]",

        // --------------------------------------------------------------------
        // 6. Switch back to user GS and return via sysret.
        // --------------------------------------------------------------------
        "swapgs",
        "sysret",

        // The dispatcher symbol (defined in the scheduler module).
        syscall_dispatcher = sym crate::sched::syscall_dispatcher,
    );
}

// ============================================================================
// SYSCALL INITIALISATION
// ============================================================================

/// Initialises the system call infrastructure.
///
/// This function sets up the MSRs required for `syscall`:
/// 1. **IA32_EFER**: Sets the `SCE` bit (System Call Extensions) to enable
///    `syscall` in 64‑bit mode.
/// 2. **IA32_STAR**: Sets the CS and SS selectors for both kernel and user mode.
///    - Kernel CS = `0x08` (GDT index 1)
///    - Kernel SS = `0x10` (GDT index 2)
///    - User CS = `0x18` (GDT index 3)
///    - User SS = `0x20` (GDT index 4)
/// 3. **IA32_LSTAR**: Sets the address of `syscall_entry`.
/// 4. **IA32_FMASK**: Masks RFLAGS bits 8 and 9 (TF and IF) to disable
///    single‑step and interrupts during syscall handling.
///
/// # Safety
/// This function uses `wrmsr` to write to privileged MSRs. It is called during
/// early boot with interrupts disabled.
pub fn init() {
    // Enable the System Call Extensions (SCE) bit in EFER.
    let efer = unsafe { rdmsr(IA32_EFER) };
    unsafe { wrmsr(IA32_EFER, efer | 1); }

    // Set STAR: Kernel CS (0x08) at bits 48‑63, Kernel SS at bits 32‑47,
    // User CS (0x18) at bits 16‑31, User SS (0x20) at bits 0‑15.
    let star = (0x08u64 << 48) | (0x08u64 << 32);
    unsafe { wrmsr(IA32_STAR, star); }

    // Set LSTAR to the address of the syscall entry point.
    unsafe { wrmsr(IA32_LSTAR, syscall_entry as *const () as u64); }

    // Set FMASK to mask TF (bit 8) and IF (bit 9).
    // 0x300 = bits 8 and 9 set.
    unsafe { wrmsr(IA32_FMASK, 0x300); }
}

```

### `src/arch/amd64/acpi/lapic.rs`

```rs
//! # Local APIC (LAPIC) Programming
//!
//! This module provides low‑level access to the x86_64 Local APIC (Advanced Programmable
//! Interrupt Controller). The Local APIC is a per‑CPU interrupt controller that handles
//! local interrupts (timer, performance counters, thermal events) and receives IPIs
//! (Inter‑Processor Interrupts) from other cores.
//!
//! ## Overview
//!
//! The Local APIC is memory‑mapped into the physical address space. Its registers are
//! accessed via MMIO at a base address provided by the ACPI MADT (Multiple APIC
//! Description Table). The kernel maps this physical address into the virtual address
//! space at a fixed location (`LOCAL_APIC_VMA`).
//!
//! ## Registers
//!
//! The Local APIC has several key registers:
//! - **ID Register**: The APIC ID of the current CPU.
//! - **Version Register**: The version of the APIC.
//! - **Task Priority Register (TPR)**: Controls interrupt priority.
//! - **End Of Interrupt (EOI)**: Writing to this register signals that an interrupt
//!   has been handled.
//! - **Spurious Interrupt Vector (SVR)**: Enables the APIC and sets the spurious
//!   interrupt vector.
//! - **Interrupt Command Register (ICR)**: Used to send IPIs to other CPUs.
//! - **Local Vector Table (LVT)**: Configures local interrupts (timer, LINT0, LINT1,
//!   error).
//! - **Timer Registers**: Control the APIC timer (divisor, initial count, current count).
//!
//! ## Initialisation
//!
//! The LAPIC is initialised in two phases:
//! 1. **`lapic::init()`**: Called on the BSP during `acpi::init_bsp()`. It parses
//!    the MADT to find the LAPIC physical address, maps it into the kernel's address
//!    space, and sets the global `TOTAL_CPUS` and `LAPIC_PHYS_ADDR`.
//! 2. **`lapic::enable()`**: Called on all CPUs during `acpi::init()`. It enables
//!    the LAPIC by setting the SVR register, masks all LVT entries, and disables
//!    the timer pending interrupts.
//!
//! ## IPI Sending
//!
//! The `LocalApic` struct provides methods to access the ICR registers:
//! - `iclo()`: Returns a mutable reference to the ICR low register.
//! - `ichi()`: Returns a mutable reference to the ICR high register.
//!
//! IPIs are sent by writing the target APIC ID to the high register and the vector
//! and delivery mode to the low register. The `acpi::send_ipi` function wraps this
//! process.
//!
//! ## Timer
//!
//! The APIC timer is used as the system tick source. It is programmed by writing to:
//! - **Divisor Configuration Register (DCR)**: Sets the divider for the timer.
//! - **Initial Count Register (ICR)**: Sets the initial count.
//! - **Current Count Register (CCR)**: Reads the current count.
//! - **LVT Timer Register**: Configures the timer mode (oneshot or periodic) and
//!   the interrupt vector.
//!
//! The timer is calibrated against the HPET in `timer::init()`.
//!
//! ## Safety
//!
//! - All register access is performed via volatile MMIO reads and writes using
//!   raw pointers. This is safe because the registers are mapped to known physical
//!   addresses and are read/write without side effects (except for EOI and ICR).
//! - The `LocalApic` struct is `Clone` and `Copy` and provides safe wrappers around
//!   unsafe pointer operations.
//! - The `init()` and `enable()` functions use `static mut` variables and are
//!   called during early boot (single‑threaded) or with interrupts disabled.

use crate::{arch::paging::EntryFlags, mem::kdm::{Paddr, Vaddr}};

// ============================================================================
// CONSTANTS
// ============================================================================

/// Virtual address where the Local APIC is mapped.
///
/// The kernel maps the LAPIC MMIO region to the top of the virtual address space,
/// just below the HPET mapping. This address is fixed and used by all CPUs.
const LOCAL_APIC_VMA: usize = 0xFFFFFFFFFFFFF000;

// ============================================================================
// LAPIC REGISTER OFFSETS
// ============================================================================

/// Offsets (in bytes) from the LAPIC base address for each register.
#[allow(dead_code)]
struct RegOffsets;

impl RegOffsets {
    const LAPIC_ID:        usize = 0x020;
    const LAPIC_VERSION:   usize = 0x030;
    const LAPIC_TPR:       usize = 0x080;
    const LAPIC_EOI:       usize = 0x0B0;
    const LAPIC_SVR:       usize = 0x0F0;
    const LAPIC_ICR_LOW:   usize = 0x300;
    const LAPIC_ICR_HIGH:  usize = 0x310;
    const LAPIC_LVT_TIMER: usize = 0x320;
    const LAPIC_LVT_LINT0: usize = 0x350;
    const LAPIC_LVT_LINT1: usize = 0x360;
    const LAPIC_LVT_ERROR: usize = 0x370;
    const LAPIC_TIMER_DCR: usize = 0x3E0;
    const LAPIC_TIMER_ICR: usize = 0x380;
    const LAPIC_TIMER_CCR: usize = 0x390;
}

// ============================================================================
// LOCAL APIC STRUCTURE
// ============================================================================

/// A handle to the Local APIC.
///
/// This struct provides methods to read and write the LAPIC registers using
/// MMIO. It is a zero‑sized type (ZST) because the registers are accessed via
/// fixed virtual addresses.
///
/// # Examples
/// ```ignore
/// let lapic = LocalApic::new();
/// let id = lapic.id();          // Read the APIC ID.
/// *lapic.eoi() = 0;            // Signal end of interrupt.
/// *lapic.iclo() = vector;      // Send an IPI.
/// ```
#[derive(Debug, Clone, Copy)]
pub struct LocalApic;

impl LocalApic {
    /// Returns a handle to the Local APIC.
    ///
    /// The handle is a ZST and can be created cheaply.
    #[inline(always)]
    pub const fn new() -> Self { INSTANCE }

    // ========================================================================
    // REGISTER ACCESSORS
    // ========================================================================

    /// Returns a reference to the APIC ID register (read‑only).
    #[inline(always)]
    pub fn id(&self) -> u32 {
        *Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_ID).to_ref::<u32>()
    }

    /// Returns a reference to the APIC version register (read‑only).
    #[inline(always)]
    pub fn version(&self) -> u32 {
        *Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_VERSION).to_ref::<u32>()
    }

    /// Returns a mutable reference to the Task Priority Register (TPR).
    #[inline(always)]
    pub fn tpr(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_TPR).to_ref_mut()
    }

    /// Returns a mutable reference to the End‑Of‑Interrupt (EOI) register.
    ///
    /// Writing any value to this register signals that the current interrupt
    /// has been handled. Typically, we write `0`.
    #[inline(always)]
    pub fn eoi(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_EOI).to_ref_mut()
    }

    /// Returns a mutable reference to the Spurious Interrupt Vector (SVR) register.
    ///
    /// The SVR enables the LAPIC (bit 8) and sets the spurious interrupt vector.
    #[inline(always)]
    pub fn svr(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_SVR).to_ref_mut()
    }

    /// Returns a mutable reference to the Interrupt Command Register (ICR) low word.
    ///
    /// This register is used to send IPIs. It must be written after the high word.
    #[inline(always)]
    pub fn iclo(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_ICR_LOW).to_ref_mut()
    }

    /// Returns a mutable reference to the Interrupt Command Register (ICR) high word.
    ///
    /// This register holds the target APIC ID for the IPI.
    #[inline(always)]
    pub fn ichi(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_ICR_HIGH).to_ref_mut()
    }

    // ========================================================================
    // LVT (Local Vector Table) ACCESSORS
    // ========================================================================

    /// Returns a mutable reference to the LVT Timer register.
    ///
    /// Configures the APIC timer mode (oneshot/periodic) and the interrupt vector.
    #[inline(always)]
    pub fn lvt_timer(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_LVT_TIMER).to_ref_mut()
    }

    /// Returns a mutable reference to the LVT LINT0 register.
    #[inline(always)]
    pub fn lvt_lint0(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_LVT_LINT0).to_ref_mut()
    }

    /// Returns a mutable reference to the LVT LINT1 register.
    #[inline(always)]
    pub fn lvt_lint1(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_LVT_LINT1).to_ref_mut()
    }

    /// Returns a mutable reference to the LVT Error register.
    #[inline(always)]
    pub fn lvt_error(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_LVT_ERROR).to_ref_mut()
    }

    // ========================================================================
    // TIMER REGISTERS
    // ========================================================================

    /// Returns a mutable reference to the Timer Divisor Configuration Register (DCR).
    #[inline(always)]
    pub fn div(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_TIMER_DCR).to_ref_mut()
    }

    /// Returns a mutable reference to the Timer Initial Count Register (ICR).
    #[inline(always)]
    pub fn icr(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_TIMER_ICR).to_ref_mut()
    }

    /// Returns a mutable reference to the Timer Current Count Register (CCR).
    #[inline(always)]
    pub fn ccr(&self) -> &mut u32 {
        Vaddr::from_raw(LOCAL_APIC_VMA + RegOffsets::LAPIC_TIMER_CCR).to_ref_mut()
    }
}

/// The singleton instance of the Local APIC.
static INSTANCE: LocalApic = LocalApic;

// ============================================================================
// INITIALISATION FUNCTIONS
// ============================================================================

/// Initialises the Local APIC on the BSP.
///
/// This function:
/// 1. Parses the ACPI tables to get the interrupt model (MADT).
/// 2. Extracts the Local APIC physical address.
/// 3. Maps the LAPIC MMIO region into the kernel's virtual address space at
///    `LOCAL_APIC_VMA`.
/// 4. Sets the global `TOTAL_CPUS` to the number of processors found in the MADT.
/// 5. Stores the LAPIC physical address for later use.
///
/// # Panics
/// - If the interrupt model is not APIC (e.g., x2APIC or other unsupported modes).
/// - If the LAPIC mapping fails.
///
/// # Safety
/// - This function uses the `TABLES` lazy‑static, which must already be initialised.
/// - It performs MMIO mapping using `PTM.lock()` (which is safe because it's
///   called during early boot, single‑threaded).
pub fn init() {
    // Get the interrupt model from the ACPI tables.
    let interrupt_model =
        acpi::platform::InterruptModel::new(&super::TABLES).expect("Failed to parse interrupt model (MADT)");

    // Extract the list of application processors.
    let aps = match interrupt_model.1 {
        Some(pi) => pi.application_processors,
        None => panic!("Can't obtain CPU topology from ACPI"),
    };

    // Set the total number of CPUs (BSP + APs).
    unsafe {
        super::TOTAL_CPUS = aps.len() + 1;
    }

    // Extract the Local APIC physical address.
    let local_apic_address = match interrupt_model.0 {
        acpi::platform::InterruptModel::Apic(x) => {
            x.local_apic_address
        },
        _ => panic!("Unsupported host interrupt model"),
    };

    // Map the Local APIC into the kernel's virtual address space.
    match crate::mem::PTM.lock().map_4k_block(
        LOCAL_APIC_VMA,
        Paddr::from_raw(local_apic_address as usize),
        EntryFlags::PRESENT
            | EntryFlags::WRITABLE
            | EntryFlags::WRITE_THROUGH
            | EntryFlags::CACHE_DISABLE
    ) {
        Ok(_) => {},
        Err(e) => panic!("Can't map LAPIC: {}", e)
    };

    // Store the physical address globally.
    unsafe {
        super::LAPIC_PHYS_ADDR = local_apic_address as usize;
    }
}

/// Enables the Local APIC on the current CPU.
///
/// This function:
/// 1. Masks all LVT entries (timer, LINT0, LINT1, error) by setting the mask bit.
/// 2. Sets the SVR register to enable the APIC (bit 8) and sets the spurious
///    interrupt vector to `SPURIOUS_VECTOR`.
///
/// This must be called on every CPU (BSP and APs) after the LAPIC has been
/// initialised and mapped.
///
/// # Safety
/// - This function performs MMIO writes to the LAPIC registers.
/// - It is safe to call from multiple CPUs because each CPU has its own LAPIC
///   and the registers are per‑CPU.
pub fn enable() {
    let lapic = LocalApic::new();

    // Mask all LVT entries to prevent spurious interrupts.
    *lapic.lvt_timer()  = 1 << 16;
    *lapic.lvt_lint0()  = 1 << 16;
    *lapic.lvt_lint1()  = 1 << 16;
    *lapic.lvt_error()  = 1 << 16;

    // Enable the APIC (bit 8) and set the spurious interrupt vector.
    *lapic.svr() = (1u32 << 8) | (super::SPURIOUS_VECTOR as u32);
}

```

### `src/arch/amd64/acpi/handler.rs`

```rs
//! # ACPI Handler Implementation
//!
//! This module provides the platform‑specific handler for the ACPI library (`acpi` crate).
//! The `Hdl` struct implements the `acpi::Handler` trait, which allows the ACPI library
//! to perform platform‑dependent operations such as memory mapping, I/O port access,
//! PCI configuration space access, and AML (ACPI Machine Language) runtime support.
//!
//! ## Overview
//!
//! The ACPI library requires a handler to interact with the hardware. Most of the
//! handler methods are not needed for the kernel's initialisation; only a few are
//! essential:
//!
//! - **`map_physical_region`**: Maps a physical memory region (e.g., ACPI tables)
//!   into the kernel's virtual address space so that the library can parse them.
//! - **`unmap_physical_region`**: No‑op; the kernel does not need to unmap ACPI
//!   regions because they are mapped permanently.
//!
//! Other methods (I/O access, PCI access, AML mutexes, etc.) are stubbed out with
//! `unimplemented!()` because they are not required for the kernel's current
//! functionality. They would need to be implemented for full ACPI runtime support
//! (e.g., for power management or device enumeration).
//!
//! ## Memory Mapping Strategy
//!
//! The `map_physical_region` method uses the kernel's HHDM (High Half Direct Map)
//! to map physical addresses directly to virtual addresses. This is achieved by:
//! 1. Converting the physical address to a virtual address via `Paddr::to_virt()`.
//! 2. Returning the virtual address as a `NonNull<T>` to the ACPI library.
//!
//! This is efficient because the HHDM maps all physical memory linearly into the
//! kernel's address space, so no additional page table entries are needed.
//!
//! ## Safety
//!
//! - The handler methods use unsafe code to cast virtual addresses to pointers.
//! - The `map_physical_region` method assumes that the physical address is already
//!   mapped (which is true because the HHDM covers all physical memory).
//! - The `unmap_physical_region` method is a no‑op because the kernel does not
//!   unmap ACPI memory.
//!
//! ## Future Work
//!
//! To fully support ACPI runtime features (e.g., device power management, thermal
//! monitoring, battery status), the following methods would need to be implemented:
//! - `read_io_*` / `write_io_*`: PIO access.
//! - `read_pci_*` / `write_pci_*`: PCI configuration space access.
//! - `acquire` / `release`: AML mutex support.
//! - `sleep` / `stall`: Timing operations.
//! - `create_mutex`, `handle_debug`, `handle_fatal_error`: AML runtime support.

use crate::mem::kdm::Paddr;

// ============================================================================
// HANDLER STRUCTURE
// ============================================================================

/// The ACPI handler for the kernel.
///
/// This is a zero‑sized type (ZST) that implements the `acpi::Handler` trait.
/// It provides the platform‑specific operations required by the ACPI library.
#[derive(Clone, Copy, Debug)]
pub struct Hdl;

// ============================================================================
// ACPI HANDLER IMPLEMENTATION
// ============================================================================

impl acpi::Handler for Hdl {
    // ========================================================================
    // AML SYNCHRONIZATION (unimplemented)
    // ========================================================================

    /// Acquires an AML mutex.
    ///
    /// # Note
    /// Not implemented; this would be needed for full AML runtime support.
    fn acquire(&self, _mutex: acpi::Handle, _timeout: u16) -> Result<(), acpi::aml::AmlError> {
        unimplemented!()
    }

    /// Triggers an AML breakpoint.
    ///
    /// # Note
    /// Not implemented; used for debugging AML code.
    fn breakpoint(&self) {
        unimplemented!()
    }

    /// Creates an AML mutex.
    ///
    /// # Note
    /// Not implemented; would need to be implemented for AML mutex support.
    fn create_mutex(&self) -> acpi::Handle {
        unimplemented!()
    }

    /// Handles an AML debug output.
    ///
    /// # Note
    /// Not implemented; AML `Debug` objects are ignored.
    fn handle_debug(&self, _object: &acpi::aml::object::Object) {
        // No‑op: ignore AML debug output.
    }

    /// Handles an AML fatal error.
    ///
    /// # Note
    /// Not implemented; fatal errors would panic or halt the system.
    fn handle_fatal_error(&self, _fatal_type: u8, _fatal_code: u32, _fatal_arg: u64) {
        unimplemented!()
    }

    // ========================================================================
    // MEMORY MAPPING
    // ========================================================================

    /// Maps a physical memory region into the kernel's virtual address space.
    ///
    /// This function uses the HHDM to convert the physical address to a virtual
    /// address. The region is assumed to be already mapped by the HHDM.
    ///
    /// # Arguments
    /// * `physical_address` – The physical address of the region.
    /// * `size` – The size of the region (unused, but required by the trait).
    ///
    /// # Returns
    /// A `PhysicalMapping` struct containing the virtual address and metadata.
    ///
    /// # Safety
    /// This function performs a raw pointer cast from a physical address to a
    /// virtual address. It assumes that the HHDM is set up and that the physical
    /// address is valid and mapped.
    unsafe fn map_physical_region<T>(&self, physical_address: usize, size: usize) -> acpi::PhysicalMapping<Self, T> {
        use core::ptr::NonNull;
        acpi::PhysicalMapping {
            physical_start: physical_address,
            virtual_start: unsafe { NonNull::<T>::new_unchecked(Paddr::from_raw(physical_address).to_virt().to_ptr_mut()) },
            region_length: size,
            mapped_length: size,
            handler: *self,
        }
    }

    /// Unmaps a previously mapped physical region.
    ///
    /// # Note
    /// This is a no‑op because the kernel does not unmap ACPI regions. The HHDM
    /// keeps them permanently mapped.
    fn unmap_physical_region<T>(_region: &acpi::PhysicalMapping<Self, T>) {
        // No‑op: ACPI regions are not unmapped.
    }

    // ========================================================================
    // TIMING (unimplemented)
    // ========================================================================

    /// Returns the number of nanoseconds since boot.
    ///
    /// # Note
    /// Not implemented; would need to read the HPET or TSC.
    fn nanos_since_boot(&self) -> u64 {
        unimplemented!()
    }

    /// Sleeps for a given number of milliseconds.
    ///
    /// # Note
    /// Not implemented; would need to use the APIC timer or HPET.
    fn sleep(&self, _milliseconds: u64) {
        unimplemented!()
    }

    /// Stalls (busy‑waits) for a given number of microseconds.
    ///
    /// # Note
    /// Not implemented; could be implemented with a tight loop reading the TSC.
    fn stall(&self, _microseconds: u64) {
        unimplemented!()
    }

    // ========================================================================
    // I/O PORT ACCESS (unimplemented)
    // ========================================================================

    fn read_io_u8(&self, _port: u16) -> u8 {
        unimplemented!()
    }
    fn read_io_u16(&self, _port: u16) -> u16 {
        unimplemented!()
    }
    fn read_io_u32(&self, _port: u16) -> u32 {
        unimplemented!()
    }
    fn write_io_u8(&self, _port: u16, _value: u8) {
        unimplemented!()
    }
    fn write_io_u16(&self, _port: u16, _value: u16) {
        unimplemented!()
    }
    fn write_io_u32(&self, _port: u16, _value: u32) {
        unimplemented!()
    }

    // ========================================================================
    // MEMORY-MAPPED I/O ACCESS (unimplemented)
    // ========================================================================

    fn read_u8(&self, _address: usize) -> u8 {
        unimplemented!()
    }
    fn read_u16(&self, _address: usize) -> u16 {
        unimplemented!()
    }
    fn read_u32(&self, _address: usize) -> u32 {
        unimplemented!()
    }
    fn read_u64(&self, _address: usize) -> u64 {
        unimplemented!()
    }
    fn write_u8(&self, _address: usize, _value: u8) {
        unimplemented!()
    }
    fn write_u16(&self, _address: usize, _value: u16) {
        unimplemented!()
    }
    fn write_u32(&self, _address: usize, _value: u32) {
        unimplemented!()
    }
    fn write_u64(&self, _address: usize, _value: u64) {
        unimplemented!()
    }

    // ========================================================================
    // PCI CONFIGURATION SPACE ACCESS (unimplemented)
    // ========================================================================

    fn read_pci_u8(&self, _address: acpi::PciAddress, _offset: u16) -> u8 {
        unimplemented!()
    }
    fn read_pci_u16(&self, _address: acpi::PciAddress, _offset: u16) -> u16 {
        unimplemented!()
    }
    fn read_pci_u32(&self, _address: acpi::PciAddress, _offset: u16) -> u32 {
        unimplemented!()
    }
    fn write_pci_u8(&self, _address: acpi::PciAddress, _offset: u16, _value: u8) {
        unimplemented!()
    }
    fn write_pci_u16(&self, _address: acpi::PciAddress, _offset: u16, _value: u16) {
        unimplemented!()
    }
    fn write_pci_u32(&self, _address: acpi::PciAddress, _offset: u16, _value: u32) {
        unimplemented!()
    }

    // ========================================================================
    // AML MUTEX RELEASE (unimplemented)
    // ========================================================================

    /// Releases an AML mutex.
    ///
    /// # Note
    /// Not implemented; would need to be implemented for AML mutex support.
    fn release(&self, _mutex: acpi::Handle) {
        unimplemented!()
    }
}

```

### `src/sched/mod.rs`

```rs
//! # Scheduler Subsystem (EEVDF)
//!
//! This module implements the kernel scheduler based on the **Earliest Eligible Virtual Deadline First (EEVDF)** algorithm.
//! It manages task execution, preemption, and CPU time distribution across all cores.
//!
//! ## Overview
//!
//! The scheduler is responsible for:
//!
//! - **Task Management**: Creating, running, blocking, and terminating tasks.
//! - **CPU Scheduling**: Selecting the next task to run on each CPU using the EEVDF policy.
//! - **Synchronization**: Providing primitives for waiting and waking tasks.
//! - **Process Support**: Managing process IDs, address spaces, and system call handling.
//!
//! ## Key Concepts
//!
//! - **Task**: A schedulable unit of execution, either kernel or user mode. Each task has a
//!   `TaskId`, priority (`Priority`), weight, virtual runtime (`vruntime`), and deadline.
//! - **EEVDF**: Tasks are scheduled based on a virtual deadline. Each task gets a time slice
//!   (`slice`) and is assigned a deadline = `vruntime + slice`. The scheduler picks the task
//!   with the earliest deadline that is eligible (`vruntime <= min_vruntime`).
//! - **Runqueue**: Per-CPU queue of runnable tasks, organized by deadline in a `BTreeSet`.
//!   Each runqueue also tracks the current task and the minimum vruntime.
//! - **Process**: A container for tasks, address space, VMAs, and syscall handler.
//!   Each task belongs to a process (`Arc<Process>`).
//! - **WaitQueue**: A list of tasks waiting for an event; used for `sleep`/`wakeup`.
//! - **Zombie Reaping**: Tasks that exit become zombies and are reaped by the `reaper` task.
//!
//! ## Scheduling Algorithm (EEVDF)
//!
//! 1. Each task has a `vruntime` (accumulated virtual runtime) and a `slice` (time quota).
//! 2. When scheduled, the task runs until its `vruntime` reaches its `deadline` (`deadline = vruntime + slice`).
//! 3. The task with the smallest `deadline` among runnable tasks is selected (Earliest Deadline First).
//! 4. To prevent starvation, the runqueue maintains a `min_vruntime`; tasks with `vruntime` below it are
//!    considered eligible and get priority.
//! 5. On each timer tick (10 ms), the current task's `vruntime` is updated by:
//!    `delta_vruntime = delta_real_time * (NICE_0_WEIGHT / weight)`
//!    (where weight depends on priority, and `NICE_0_WEIGHT = 1024`).
//! 6. If the task's `vruntime >= deadline`, a new deadline is computed: `deadline = vruntime + slice`.
//!
//! ## CPU Affinity
//!
//! Each task can specify a preferred CPU (`cpu_affinity`). If `None`, it can run on any core.
//! The scheduler respects affinity when selecting a runqueue.
//!
//! ## Synchronization Primitives
//!
//! - **`sleep(wq)`**: Moves the current task to a wait queue and yields the CPU.
//! - **`wakeup(wq)`**: Wakes up one task from the wait queue, making it runnable.
//! - **`wait_child(id)`**: Waits for a specific child task to exit.
//! - **`wait_any()`**: Waits for any child task to exit.
//!
//! ## System Calls (Native ABI)
//!
//! - **sys_yield** (0): Yield the CPU voluntarily.
//! - **sys_exit** (1): Exit the current process with a code.
//!
//! ## Page Fault Handling
//!
//! The scheduler handles page faults via `handle_page_fault`:
//! - **Copy-on-Write (CoW)**: If a write fault occurs on a page with `COPY_ON_WRITE` flag,
//!   a private copy is created.
//! - **Demand Paging**: If the fault address falls within a VMA, a new physical page is allocated
//!   and mapped with appropriate permissions.
//! - **Segmentation Fault**: If the fault cannot be resolved, the process is terminated with SIGSEGV.
//!
//! ## Initialization
//!
//! `sched::init(ticks_per_10ms)` is called by the BSP after all subsystems are ready.
//! It initializes the per-CPU runqueues, creates an idle task for each CPU, and sets up
//! the current task. The scheduler is then ready to run.
//!
//! ## Safety
//!
//! - The runqueues are protected by `Nitex` (interrupt‑disabling spinlocks) to ensure
//!   safe concurrent access from multiple CPUs.
//! - Task registry (`TASK_REGISTRY`) is protected by a `Nutex` (also interrupt‑disabling).
//! - The scheduler uses inline assembly for context switching and interrupt handling.
//! - The `yield_wrapper` and `timer_wrapper` are naked functions that manipulate the stack
//!   and trap frames directly.

// ============================================================================
// SUBMODULES
// ============================================================================

pub mod task;   // Task structure, TaskId, Priority, Context
pub mod rq;     // Runqueue (per-CPU), EEVDF logic
pub mod wq;     // WaitQueue for task sleeping
pub mod proc;   // Process structure, address space, VMM, root FS

// ============================================================================
// IMPORTS
// ============================================================================

use crate::arch::trap::TrapFrame;
use crate::mem::vma::VmaFlags;
use crate::sched::proc::Process;
use crate::sync::Nutex;
use crate::vfs::RootRef;
use alloc::sync::Arc;
use alloc::{boxed::Box, collections::btree_map::BTreeMap};
use task::{Task, TaskId, TaskState, Priority};
use rq::RUNQUEUES;
use wq::WaitQueue;
use core::arch::naked_asm;
use core::sync::atomic::{AtomicU64, Ordering};
use crate::arch::paging::EntryFlags;

// ============================================================================
// GLOBAL STATE
// ============================================================================

/// Global registry of all tasks (including zombies).
///
/// This map is used to look up tasks by ID, re-parent orphans, and reap zombies.
pub static TASK_REGISTRY: Nutex<BTreeMap<TaskId, Box<Task>>> = Nutex::new(BTreeMap::new());

/// Wait queue for tasks waiting for any child to exit.
pub static EXIT_WQ: Nutex<WaitQueue> = Nutex::new(WaitQueue::new());

/// Ticks per millisecond (derived from HPET calibration).
static TICKS_PER_MS: AtomicU64 = AtomicU64::new(0);

// ============================================================================
// INITIALIZATION
// ============================================================================

/// Initializes the scheduler.
///
/// # Arguments
/// * `ticks_per_10ms` – Number of APIC timer ticks per 10 ms (calibrated in `timer::init`).
///
/// # Operations
/// 1. Stores `ticks_per_10ms / 10` as ticks per ms.
/// 2. For each CPU, allocates a kernel stack and creates an idle task.
/// 3. Inserts the idle task into the CPU's runqueue and sets it as current.
/// 4. Sets the kernel stack for the BSP's per‑CPU data.
///
/// # Panics
/// If stack allocation fails (should not happen).
pub fn init(ticks_per_10ms: u64) {
    TICKS_PER_MS.store(ticks_per_10ms / 10, Ordering::Release);

    for (cpu, _) in RUNQUEUES.iter().enumerate().take(crate::arch::num_cpus()) {
        let stack = allocate_kernel_stack(16 * 1024);
        let idle = Task::new_kernel(idle_task, stack, Priority(19), "idle");
        let mut rq = RUNQUEUES[cpu].lock();
        rq.set_current(idle.id);
        rq.insert(idle);

        // Set kernel stack for the BSP (CPU 0) – other CPUs will set it when they start.
        if cpu == crate::arch::current_cpu() {
            crate::arch::percpu::set_kernel_stack(stack as u64);
        }
    }

    crate::info!("Scheduler initialized with EEVDF");
}

/// Allocates a kernel stack of the given size.
///
/// Returns the top address of the stack (the stack grows downward).
fn allocate_kernel_stack(size: usize) -> usize {
    let pages = size.div_ceil(4096);
    let paddr = crate::mem::upa::alloc(pages);
    if paddr.to_raw() == 0 {
        panic!("Failed to allocate kernel stack");
    }
    paddr.to_virt().to_raw() + size
}

/// Idle task – runs when no other task is runnable.
///
/// It simply halts the CPU, waiting for interrupts.
fn idle_task() {
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

// ============================================================================
// TASK SPAWNING
// ============================================================================

/// Spawns a new kernel task.
///
/// # Arguments
/// * `entry` – The function to run (must never return, or call `exit`).
/// * `priority` – The priority (niceness) of the task.
/// * `name` – Static name for debugging.
/// * `root` – Optional `RootRef` (VFS root) for the process.
///
/// # Returns
/// The `TaskId` of the newly spawned task.
///
/// # Notes
/// - The task's process is cloned from the current process (if any) or a default
///   process. The `root` is set if provided.
/// - The task is inserted into the runqueue of the current CPU.
/// - The parent is set to the current task.
pub fn spawn_kernel_task(entry: fn(), priority: Priority, name: &'static str, root: Option<RootRef>) -> TaskId {
    let stack = allocate_kernel_stack(32 * 1024);
    let mut task = Task::new_kernel(entry, stack, priority, name);

    if let Some(x) = root {
        let mut proc;
        if let Some(p) = current_process() {
            proc = (*p).clone()
        } else {
            proc = Process::new();
        }
        proc.roots = x;
        task.process = Arc::new(proc);
    }

    let cpu = crate::arch::current_cpu();
    let rq = RUNQUEUES[cpu].lock();
    task.parent = rq.current_task_id();
    drop(rq);

    let id = task.id;
    RUNQUEUES[cpu].lock().insert(task);
    id
}

// ============================================================================
// TASK EXIT & WAITING
// ============================================================================

/// Terminates the current task with the given exit code.
///
/// This function:
/// 1. Removes the task from its CPU's runqueue.
/// 2. Re‑parents any children to the init task (TaskId(1)).
/// 3. Marks the task as `Zombie` and stores it in the global task registry.
/// 4. Wakes up any waiters (via `EXIT_WQ`).
/// 5. Yields the CPU (never returns).
///
/// # Note
/// The function never returns; it yields and eventually the task is reaped.
pub fn exit(code: i32) -> ! {
    let cpu = crate::arch::current_cpu();
    let mut rq = RUNQUEUES[cpu].lock();
    let current_id = rq.current_task_id().unwrap();

    debug!(
        "Exiting task {} (PID {}) with code {}",
        current_id.0,
        current_process().unwrap_or(Arc::new(Process::new())).pid,
        code,
    );

    let mut task = rq.remove(current_id).unwrap();
    rq.clear_current();
    drop(rq);

    let init_id = TaskId(1);
    {
        let mut registry = TASK_REGISTRY.lock();
        for t in registry.values_mut() {
            if t.parent == Some(current_id) {
                t.parent = Some(init_id);
            }
        }
    }

    task.state = TaskState::Zombie;
    task.exit_code = code;

    TASK_REGISTRY.lock().insert(current_id, task);

    wakeup(&EXIT_WQ);

    yield_now();

    loop {
        unsafe {
            core::arch::asm! {
                "hlt"
            }
        }
    }
}

/// Yields the CPU voluntarily (calls `int 33`).
#[inline(always)]
pub fn yield_now() {
    unsafe {
        core::arch::asm!("int 33");
    }
}

/// Puts the current task to sleep on a wait queue.
///
/// The task is removed from the runqueue and added to the wait queue.
/// It will be woken by `wakeup` on the same wait queue.
pub fn sleep(wq: &Nutex<WaitQueue>) {
    let cpu = crate::arch::current_cpu();
    let mut rq = RUNQUEUES[cpu].lock();
    let current_id = rq.current_task_id().unwrap();
    if let Some(mut task) = rq.remove(current_id) {
        task.state = TaskState::Sleeping;
        wq.lock().sleep(task.id);
        rq.insert(task);
    }
    drop(rq);
    yield_now();
}

/// Wakes up one task from a wait queue.
///
/// The task is removed from the wait queue and made runnable.
pub fn wakeup(wq: &Nutex<WaitQueue>) {
    let cpu = crate::arch::current_cpu();
    let mut rq = RUNQUEUES[cpu].lock();

    if let Some(task_id) = wq.lock().wakeup_one()
    && let Some(mut task) = rq.remove(task_id) {
        task.state = TaskState::Runnable;
        rq.insert(task);
    }
}

/// Waits for a specific child task to exit.
///
/// # Returns
/// The exit code of the child.
///
/// # Notes
/// This function blocks the current task until the child becomes a zombie,
/// then removes it from the registry and returns its exit code.
pub fn wait_child(child_id: TaskId) -> i32 {
    loop {
        let mut registry = TASK_REGISTRY.lock();
        if let Some(task) = registry.get(&child_id)
        && task.state == TaskState::Zombie {
            let code = task.exit_code;
            registry.remove(&child_id);
            return code;
        }
        drop(registry);
        sleep(&EXIT_WQ);
    }
}

/// Waits for any child task to exit.
///
/// # Returns
/// `Some((TaskId, exit_code))` if a zombie child exists, or waits indefinitely.
///
/// # Notes
/// The zombie is removed from the registry before returning.
pub fn wait_any() -> Option<(TaskId, i32)> {
    loop {
        let mut registry = TASK_REGISTRY.lock();

        let zombie_id = registry.iter()
            .find(|(_, t)| t.state == TaskState::Zombie)
            .map(|(id, _)| *id);

        if let Some(id) = zombie_id {
            let task = registry.remove(&id).unwrap();
            return Some((id, task.exit_code));
        }
        drop(registry);
        sleep(&EXIT_WQ);
    }
}

// ============================================================================
// SCHEDULER CORE
// ============================================================================

/// Timer tick handler – called by the APIC timer interrupt (vector 32).
///
/// This function:
/// 1. Updates the system time (on BSP only).
/// 2. Calls `reschedule` to perform scheduling decisions.
pub fn timer_tick(frame: &mut TrapFrame) {
    if crate::arch::current_cpu() == 0 {
        crate::arch::TIME_FROM_BOOT.fetch_add(10, Ordering::Relaxed);
    }

    reschedule(frame);
}

/// Reschedules the current CPU.
///
/// This is the heart of the scheduler:
/// 1. Sends EOI to the APIC.
/// 2. Updates the current task's vruntime (using `rq.update_vruntime`).
/// 3. Picks the next task (using `rq.pick_next`).
/// 4. If the picked task is different from the current:
///    - Saves the current task's FPU state and trap frame.
///    - Loads the new task's FPU state and trap frame.
///    - If the process ID changed, switches CR3 to the new address space.
///    - Sets the current task in the runqueue.
/// 5. If the same task continues, updates its trap frame.
///
/// # Arguments
/// * `frame` – The trap frame from the interrupt (used to save/restore context).
///
/// # Safety
/// This function uses inline assembly to switch CR3 and modify registers.
pub fn reschedule(frame: &mut TrapFrame) {
    crate::arch::acpi::eoi();

    let cpu = crate::arch::current_cpu();
    let mut rq = RUNQUEUES[cpu].lock();

    let ticks_per_10ms = crate::arch::timer::get_ticks_per_10ms();
    rq.update_vruntime(ticks_per_10ms);

    let current_id = rq.current_task_id();
    let next_id = rq.pick_next();

    if let Some(next_id) = next_id {
        if Some(next_id) != current_id {
            if let Some(curr_id) = current_id
            && let Some(old_task) = rq.tasks_mut().get_mut(&curr_id) {
                old_task.ctx.frame = *frame;
                if old_task.state == TaskState::Running {
                    old_task.state = TaskState::Runnable;
                }
                unsafe { core::arch::x86_64::_fxsave64(old_task.ctx.fpu_state.area.as_mut_ptr()); }
            }

            if let Some(new_task) = rq.tasks_mut().get_mut(&next_id) {
                new_task.state = TaskState::Running;
                *frame = new_task.ctx.frame;
                unsafe { core::arch::x86_64::_fxrstor64(new_task.ctx.fpu_state.area.as_ptr()); }

                let cpu = crate::arch::current_cpu();
                crate::arch::gdt::set_kernel_stack(cpu, new_task.kernel_stack_top as u64);
                crate::arch::percpu::set_kernel_stack(new_task.kernel_stack_top as u64);

                if let Some(curr_id) = current_id {
                    let old_pid = unsafe { RUNQUEUES[cpu].inner() }.tasks().get(&curr_id).unwrap().process.pid;
                    let new_pid = new_task.process.pid;

                    if old_pid != new_pid {
                        let new_cr3 = new_task.process.address_space.lock().exco.cr3;
                        unsafe {
                            core::arch::asm!(
                                "mov cr3, {}",
                                in(reg) new_cr3,
                                options(nostack, preserves_flags)
                            );
                        }
                    }
                } else {
                    let new_cr3 = new_task.process.address_space.lock().exco.cr3;
                    unsafe {
                        core::arch::asm!("mov cr3, {}", in(reg) new_cr3, options(nostack, preserves_flags));
                    }
                }

                rq.set_current(next_id);
            }
        } else {
            if let Some(curr_id) = current_id
            && let Some(curr_task) = rq.tasks_mut().get_mut(&curr_id) {
                curr_task.ctx.frame = *frame;
            }
        }
    }
}

/// Naked interrupt wrapper for `yield_now` (vector 33).
///
/// This function is called via `int 33` and performs the same context
/// save/restore as the timer interrupt, then calls `reschedule`.
#[unsafe(naked)]
pub unsafe extern "C" fn yield_wrapper() -> ! {
    naked_asm!(
        "mov rax, [rsp + 8]",
        "and rax, 3",
        "cmp rax, 3",
        "jne 1f",
        "swapgs",
        "1:",

        "push r15", "push r14", "push r13", "push r12",
        "push r11", "push r10", "push r9", "push r8",
        "push rbp", "push rdi", "push rsi", "push rdx",
        "push rcx", "push rbx", "push rax",

        "mov rdi, rsp",
        "call {scheduler_tick}",

        "pop rax", "pop rbx", "pop rcx", "pop rdx",
        "pop rsi", "pop rdi", "pop rbp", "pop r8",
        "pop r9", "pop r10", "pop r11", "pop r12",
        "pop r13", "pop r14", "pop r15",

        "mov rax, [rsp + 8]",
        "and rax, 3",
        "cmp rax, 3",
        "jne 2f",
        "swapgs",
        "2:",

        "iretq",

        scheduler_tick = sym reschedule,
    );
}

// ============================================================================
// PROCESS & SYSCALL SUPPORT
// ============================================================================

/// Returns the current process (if any).
pub fn current_process() -> Option<Arc<Process>> {
    let cpu = crate::arch::current_cpu();
    let rq = RUNQUEUES[cpu].lock();
    if let Some(id) = rq.current_task_id()
    && let Some(task) = rq.tasks().get(&id) {
        return Some(task.process.clone());
    }
    None
}

/// Native system call handler (calls `syscall_dispatcher`).
///
/// This is the default syscall handler for processes.
/// It interprets `rax` as the syscall number and `rdi`, `rsi`, `rdx` as arguments.
pub fn native_syscall_handler(frame: &mut TrapFrame) {
    match frame.rax {
        0 => {
            // sys_yield
            yield_now();
            frame.rax = 0;
        }
        1 => {
            // sys_exit
            let code = frame.rdi as i32;
            crate::info!("[Native SFD] Process {} exiting with code {}", current_process().unwrap().pid, code);
            exit(code);
        }
        _ => {
            crate::warn!("[Native SFD] Unknown syscall: {}", frame.rax);
            frame.rax = u64::MAX; // -ENOSYS
        }
    }
}

/// Syscall dispatcher – called from the syscall entry point.
///
/// It retrieves the current process and delegates to its `syscall_handler`.
pub fn syscall_dispatcher(frame: &mut TrapFrame) {
    let proc = match current_process() {
        Some(p) => p,
        None => {
            crate::error!("Syscall from unknown context!");
            frame.rax = u64::MAX; // -ENOSYS
            return;
        }
    };

    (proc.syscall_handler)(frame);
}

// ============================================================================
// PAGE FAULT HANDLING
// ============================================================================

/// Handles page faults (from the page fault handler in IDT).
///
/// # Arguments
/// * `addr` – The faulting virtual address.
/// * `error_code` – Page fault error code (bits: present, write, user, etc.)
/// * `rip` – Instruction pointer that caused the fault.
/// * `_is_user` – Whether the fault occurred in user mode.
///
/// # Handling
/// - **Copy‑on‑Write**: If the fault is a write to a `COPY_ON_WRITE` page,
///   a private copy is made and the mapping is updated.
/// - **Demand Paging**: If the address falls within a VMA that allows access,
///   a new physical page is allocated and mapped.
/// - **Segmentation Fault**: Otherwise, the process is terminated with exit code 139 (SIGSEGV).
///
/// # Panics
/// - If the fault occurs in kernel mode (not user) and cannot be resolved.
/// - If the current process is unknown.
pub fn handle_page_fault(addr: usize, error_code: u64, rip: u64, _is_user: bool) {
    let is_present = (error_code & 0x1) != 0;
    let is_write   = (error_code & 0x2) != 0;

    // Intentional no-panic on kernel segfaults; if from module, just kill it.
    // if !is_user {
    //     panic!("KERNEL PAGE FAULT at {:#X} (code: {:#X}) RIP: {:#X}", addr, error_code, rip);
    // }

    let proc = match current_process() {
        Some(p) => p,
        None => panic!("Page fault in unknown context (no current process)"),
    };

    // Copy‑on‑Write
    if is_present && is_write {
        let ptm = proc.address_space.lock();
        if let Some((paddr, flags)) = ptm.query(addr & !0xFFF)
        && flags.contains(EntryFlags::COPY_ON_WRITE) {
            drop(ptm);

            let new_paddr = crate::mem::upa::alloc(1);
            if new_paddr.to_raw() == 0 { panic!("OOM during CoW"); }

            let src = paddr.to_virt().to_ptr::<u8>();
            let dst = new_paddr.to_virt().to_ptr_mut::<u8>();
            unsafe { core::ptr::copy_nonoverlapping(src, dst, 4096); }

            let mut ptm = proc.address_space.lock();
            let mut new_flags = flags;
            new_flags.remove(EntryFlags::COPY_ON_WRITE);
            new_flags.insert(EntryFlags::WRITABLE);

            let _ = ptm.try_unmap(addr & !0xFFF, 4096);
            ptm.map_4k_block(addr & !0xFFF, new_paddr, new_flags).unwrap();
            return;
        }
    }

    // Demand paging
    if !is_present {
        let vmm = proc.vmm.lock();
        if let Some(vma) = vmm.find_overlap(addr) {
            // VMA access rights check
            let is_write_vma = vma.flags.contains(VmaFlags::WRITE);
            if is_write && !is_write_vma {
                drop(vmm);
                crate::info!("Process {} SEGFAULT: Write to Read-Only VMA at {:#X}", proc.pid, addr);
                exit(139);
            }
            drop(vmm); // free the lock before allocation

            // allocate phys page
            let paddr = crate::mem::upa::alloc(1);
            if paddr.to_raw() == 0 { panic!("OOM during Demand Paging"); }

            // zero it out
            let vaddr_ptr = paddr.to_virt().to_ptr_mut::<u8>();
            unsafe { core::ptr::write_bytes(vaddr_ptr, 0, 4096); }

            // map to address space
            let mut ptm = proc.address_space.lock();
            let mut flags = EntryFlags::PRESENT | EntryFlags::USER_ACCESSIBLE;
            if is_write_vma {
                flags |= EntryFlags::WRITABLE;
            }
            ptm.map_4k_block(addr & !0xFFF, paddr, flags).unwrap();
            return;
        }
    }

    // Segfault actually
    crate::info!("Process {} SEGFAULT at {:#X} (RIP: {:#X}, code: {:#X})", proc.pid, addr, rip, error_code);
    exit(139);
}

```

### `src/sched/task.rs`

```rs
// src/sched/task.rs
use crate::{arch::{gdt::{USER_CODE_SELECTOR, USER_DATA_SELECTOR}, trap::TrapFrame}, sched::proc::Process};
use core::sync::atomic::{AtomicU64, Ordering};
use alloc::{boxed::Box, sync::Arc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Runnable,
    Running,
    Sleeping,
    Blocked,
    Zombie,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Priority(pub i32);

impl Priority {
    pub const fn nice_to_weight(nice: i32) -> u64 {
        // weight = 1024 * 1.25^(-nice)
        const WEIGHTS: [u64; 40] = [
            88761, 71755, 58481, 46236, 37617, 30483, 24513, 19862,
            16124, 13031, 10550, 8546, 6912, 5594, 4519, 3659,
            2958, 2389, 1934, 1563, 1274, 1024, 833, 672,
            546, 441, 356, 287, 232, 187, 151, 122,
            98, 79, 64, 51, 41, 33, 27, 22,
        ];
        let nice = nice.clamp(-20, 19);
        WEIGHTS[(nice + 20) as usize]
    }
}

#[derive(Clone)]
pub struct Task {
    pub id: TaskId,
    pub state: TaskState,
    pub vruntime: u64,
    pub deadline: u64,
    pub weight: u64,
    pub slice: u64,           // Requested time slice in virtual ticks
    pub ctx: Context,
    pub kernel_stack: usize,  // Top of kernel stack for this task
    pub user_stack: usize,    // Top of user stack (if user task)
    pub cpu_affinity: Option<usize>,  // None = any CPU
    pub name: &'static str,
    pub parent: Option<TaskId>,
    pub exit_code: i32,
    pub process: Arc<Process>,
    pub kernel_stack_top: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Context {
    pub frame: TrapFrame,
    pub fpu_state: FpuState,
}

#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct FpuState {
    pub area: [u8; 512],  // FXSAVE area (SSE/SSE2)
    pub initialized: bool,
}

impl Default for FpuState {
    fn default() -> Self {
        Self {
            area: [0; 512],
            initialized: false,
        }
    }
}

static NEXT_TASK_ID: AtomicU64 = AtomicU64::new(1);

impl Task {pub fn new_user(
        entry: usize,
        user_stack_top: usize,
        kernel_stack_top: usize,
        priority: Priority,
        name: &'static str,
    ) -> Box<Self> {
        let id = TaskId(NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed));
        let mut frame = unsafe { core::mem::zeroed::<TrapFrame>() };
        
        let initial_kern_rsp = kernel_stack_top - 8;
        unsafe {
            *(initial_kern_rsp as *mut u64) = 0;
        }
        
        frame.rip = entry as u64;
        frame.rsp = user_stack_top as u64; // User-space stack
        frame.cs = USER_CODE_SELECTOR as u64 | 3; // USER_CODE_SELECTOR | RING_3
        frame.ss = USER_DATA_SELECTOR as u64 | 3; // USER_DATA_SELECTOR | RING_3
        frame.rflags = 0x202; // IF=1
        
        Box::new(Self {
            id,
            state: TaskState::Runnable,
            vruntime: 0,
            deadline: 0,
            weight: Priority::nice_to_weight(priority.0),
            slice: 10_000,
            ctx: Context {
                frame,
                fpu_state: FpuState::default(),
            },
            kernel_stack: kernel_stack_top,
            user_stack: user_stack_top,
            cpu_affinity: None,
            name,
            parent: None,
            exit_code: -1,
            process: Arc::new(Process::new()),
            kernel_stack_top: 0,
        })
    }

    pub fn new_kernel(
        entry: fn(),
        kernel_stack_top: usize,
        priority: Priority,
        name: &'static str,
    ) -> Box<Self> {
        let id = TaskId(NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed));
        let mut frame = unsafe { core::mem::zeroed::<TrapFrame>() };
        
        let initial_rsp = kernel_stack_top - 8;
        unsafe {
            *(initial_rsp as *mut u64) = 0; 
        }
        
        frame.rip = entry as *const () as u64;
        frame.rsp = initial_rsp as u64;
        frame.cs = 0x08;  // KERNEL_CODE_SELECTOR
        frame.ss = 0x10;  // KERNEL_DATA_SELECTOR
        frame.rflags = 0x202;  // IF=1
        
        Box::new(Self {
            id,
            state: TaskState::Runnable,
            vruntime: 0,
            deadline: 0,
            weight: Priority::nice_to_weight(priority.0),
            slice: 10_000,
            ctx: Context {
                frame,
                fpu_state: FpuState::default(),
            },
            kernel_stack: kernel_stack_top,
            user_stack: 0,
            cpu_affinity: None,
            name,
            parent: None,
            exit_code: -1,
            process: Arc::new(Process::new()),
            kernel_stack_top: 0,
        })
    }
}

```

### `src/sched/rq.rs`

```rs
//! # EEVDF Runqueue
//!
//! This module implements the per‑CPU runqueue for the EEVDF (Earliest Eligible
//! Virtual Deadline First) scheduler. It manages the set of runnable tasks on
//! a single CPU core, handles task insertion and removal, and selects the next
//! task to run according to the EEVDF policy.
//!
//! ## Overview
//!
//! The runqueue is the heart of the EEVDF scheduler on each CPU. It maintains:
//! - A `BTreeMap<TaskId, Box<Task>>` for fast lookup by ID.
//! - A `BTreeSet<TaskKey>` ordered by deadline for efficient selection of the
//!   next task.
//! - The `min_vruntime` value used for eligibility checks.
//! - The current task (if any).
//! - The total load of the runqueue (sum of task weights).
//!
//! ## EEVDF Scheduling
//!
//! In the EEVDF algorithm, each task has:
//! - `vruntime`: The accumulated virtual runtime.
//! - `deadline`: The virtual deadline (`vruntime + slice`).
//! - `weight`: The task's weight (derived from priority).
//!
//! The scheduler selects the task with the earliest deadline that is **eligible**
//! (`vruntime <= min_vruntime`). If no eligible task exists, it picks the task
//! with the smallest `vruntime` (to ensure fairness and prevent starvation).
//!
//! The `min_vruntime` is updated periodically to ensure progress; it is the
//! minimum `vruntime` among all tasks in the runqueue.
//!
//! ## Task Key
//!
//! Tasks are ordered in the `by_deadline` set by:
//! 1. `deadline` (primary key) – earliest deadline first.
//! 2. `vruntime` (secondary key) – for tie‑breaking.
//! 3. `TaskId` (tertiary key) – to ensure a deterministic total order.
//!
//! This ordering ensures that the first element in the set is always the task
//! with the earliest deadline.
//!
//! ## Update Vruntime
//!
//! On each timer tick (10 ms), the current task's `vruntime` is updated:
//! ```text
//! delta_vruntime = delta_real_time * (NICE_0_WEIGHT / weight)
//! ```
//!
//! Where `NICE_0_WEIGHT = 1024`. A task with higher priority (lower nice value)
//! has a higher weight, so its `vruntime` advances more slowly, allowing it to
//! run more frequently.
//!
//! If the task's `vruntime` reaches its `deadline`, a new deadline is assigned:
//! ```text
//! deadline = vruntime + slice
//! ```
//!
//! ## Per‑CPU Runqueues
//!
//! The runqueue is stored in a `Nitex` (interrupt‑disabling spinlock) to
//! protect against concurrent access from interrupts and other CPUs. Each CPU
//! has its own runqueue, allowing the scheduler to scale with the number of cores.
//!
//! The `RUNQUEUES` array is indexed by CPU ID and is initialized statically.
//!
//! ## Safety
//!
//! - The `by_deadline` set and `tasks` map are kept in sync by the `insert`
//!   and `remove` methods.
//! - The `update_vruntime` method removes and re‑inserts the current task
//!   to update its position in the deadline set.
//! - The `pick_next` method iterates over the deadline set and returns the
//!   first eligible (or lowest `vruntime`) runnable task.

use alloc::{boxed::Box, collections::{BTreeMap, BTreeSet}};
use super::task::{Task, TaskId};
use crate::{sched::task::TaskState, sync::Nitex};

// ============================================================================
// CONSTANTS
// ============================================================================

/// Weight for a task with niceness 0 (baseline).
const NICE_0_WEIGHT: u64 = 1024;

// ============================================================================
// TASK KEY (for ordering by deadline)
// ============================================================================

/// A key used to order tasks in the `by_deadline` `BTreeSet`.
///
/// The ordering is:
/// 1. `deadline` (primary) – earliest deadline first.
/// 2. `vruntime` (secondary) – for tie‑breaking.
/// 3. `id` (tertiary) – to ensure a deterministic total order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TaskKey {
    deadline: u64,
    vruntime: u64,
    id: TaskId,
}

impl Ord for TaskKey {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.deadline
            .cmp(&other.deadline)
            .then_with(|| self.vruntime.cmp(&other.vruntime))
            .then_with(|| self.id.0.cmp(&other.id.0))
    }
}

impl PartialOrd for TaskKey {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// ============================================================================
// RUNQUEUE STRUCTURE
// ============================================================================

/// The per‑CPU runqueue, implementing the EEVDF scheduling algorithm.
///
/// # Fields
/// - `tasks`: A map from `TaskId` to the `Box<Task>` for fast lookup.
/// - `by_deadline`: A set of `TaskKey` ordered by deadline for efficient
///   selection of the next task.
/// - `min_vruntime`: The minimum virtual runtime among all tasks in the
///   runqueue. Used for eligibility checks.
/// - `current`: The `TaskId` of the currently running task (if any).
/// - `load`: The total weight of all tasks in the runqueue.
pub struct Runqueue {
    tasks: BTreeMap<TaskId, Box<Task>>,
    by_deadline: BTreeSet<TaskKey>,
    min_vruntime: u64,
    current: Option<TaskId>,
    load: u64,
}

impl Runqueue {
    /// Creates a new, empty runqueue.
    pub const fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            by_deadline: BTreeSet::new(),
            min_vruntime: 0,
            current: None,
            load: 0,
        }
    }

    // ========================================================================
    // ACCESSORS
    // ========================================================================

    /// Returns an immutable reference to the task map.
    pub fn tasks(&self) -> &BTreeMap<TaskId, Box<Task>> {
        &self.tasks
    }

    /// Returns a mutable reference to the task map.
    ///
    /// # Safety
    /// The caller must ensure that any modifications to the task map are
    /// also reflected in the `by_deadline` set (e.g., by calling `insert`
    /// or `remove` on the runqueue).
    pub fn tasks_mut(&mut self) -> &mut BTreeMap<TaskId, Box<Task>> {
        &mut self.tasks
    }

    /// Returns the currently running task, if any.
    pub fn current_task(&self) -> Option<&Task> {
        self.current.and_then(|id| self.tasks.get(&id)).map(|b| b.as_ref())
    }

    /// Returns a mutable reference to the currently running task, if any.
    pub fn current_task_mut(&mut self) -> Option<&mut Task> {
        self.current.and_then(|id| self.tasks.get_mut(&id)).map(|b| b.as_mut())
    }

    /// Sets the current task ID.
    pub fn set_current(&mut self, id: TaskId) {
        self.current = Some(id);
    }

    /// Returns the ID of the current task, if any.
    pub fn current_task_id(&self) -> Option<TaskId> {
        self.current
    }

    /// Returns the total load (sum of weights) of the runqueue.
    pub fn load(&self) -> u64 {
        self.load
    }

    /// Returns the number of tasks in the runqueue.
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// Returns `true` if the runqueue is empty.
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Clears the current task (used when exiting).
    pub fn clear_current(&mut self) {
        self.current = None;
    }

    // ========================================================================
    // TASK INSERTION AND REMOVAL
    // ========================================================================

    /// Inserts a task into the runqueue.
    ///
    /// This method:
    /// 1. Ensures the task's `vruntime` is at least `min_vruntime` (to prevent
    ///    tasks from "going back in time").
    /// 2. If the task has no deadline, sets it to `vruntime + slice`.
    /// 3. Creates a `TaskKey` for the task.
    /// 4. Adds the task to both the `tasks` map and the `by_deadline` set.
    /// 5. Updates the load.
    pub fn insert(&mut self, task: Box<Task>) {
        let mut t = task;

        // Ensure the task's vruntime is not below the current minimum.
        if t.vruntime < self.min_vruntime {
            t.vruntime = self.min_vruntime;
        }

        // Assign a deadline if not already set.
        if t.deadline == 0 {
            t.deadline = t.vruntime + t.slice;
        }

        let key = TaskKey {
            deadline: t.deadline,
            vruntime: t.vruntime,
            id: t.id,
        };

        self.load += t.weight;
        self.by_deadline.insert(key);
        self.tasks.insert(t.id, t);
    }

    /// Removes a task from the runqueue by its ID.
    ///
    /// # Returns
    /// `Some(Box<Task>)` if the task was found and removed, `None` otherwise.
    pub fn remove(&mut self, id: TaskId) -> Option<Box<Task>> {
        if let Some(task) = self.tasks.remove(&id) {
            let key = TaskKey {
                deadline: task.deadline,
                vruntime: task.vruntime,
                id: task.id,
            };
            self.by_deadline.remove(&key);
            self.load -= task.weight;
            Some(task)
        } else {
            None
        }
    }

    // ========================================================================
    // VIRTUAL RUNTIME UPDATE
    // ========================================================================

    /// Updates the `vruntime` of the current task and advances `min_vruntime`.
    ///
    /// This is called on each timer tick (typically 10 ms).
    ///
    /// # Arguments
    /// * `delta_ms` – The real time elapsed in milliseconds (e.g., 10 ms).
    ///
    /// # Algorithm
    /// 1. If there is a current task, remove it from the runqueue.
    /// 2. Compute `delta_vruntime = delta_ms * (NICE_0_WEIGHT / weight)`.
    /// 3. Add `delta_vruntime` to the task's `vruntime`.
    /// 4. If `vruntime >= deadline`, assign a new deadline: `vruntime + slice`.
    /// 5. Re‑insert the task into the runqueue.
    /// 6. Update `min_vruntime` to the minimum `vruntime` among all tasks.
    ///
    /// # Notes
    /// The `delta_vruntime` calculation uses `u128` to avoid overflow and
    /// maintain precision. The result is cast back to `u64` after the division.
    pub fn update_vruntime(&mut self, delta_ms: u64) {
        if let Some(curr_id) = self.current {
            // Remove the current task from the runqueue.
            if let Some(mut task) = self.remove(curr_id) {
                // delta_ms is the real time in milliseconds (e.g., 10 ms).
                // vruntime += delta_ms * (NICE_0_WEIGHT / weight)
                let delta_vruntime = (delta_ms as u128 * NICE_0_WEIGHT as u128 / task.weight as u128) as u64;
                task.vruntime += delta_vruntime;

                // If the task has exhausted its slice, assign a new deadline.
                if task.vruntime >= task.deadline {
                    task.deadline = task.vruntime + task.slice;
                }

                // Re‑insert the task.
                self.insert(task);
            }
        }
        self.advance_min_vruntime();
    }

    /// Advances `min_vruntime` to the minimum `vruntime` among all tasks.
    ///
    /// This ensures that tasks with `vruntime` below the minimum are considered
    /// eligible (they get priority).
    fn advance_min_vruntime(&mut self) {
        if let Some(min_key) = self.by_deadline.iter().min_by_key(|k| k.vruntime)
        && min_key.vruntime > self.min_vruntime {
            self.min_vruntime = min_key.vruntime;
        }
    }

    // ========================================================================
    // TASK SELECTION
    // ========================================================================

    /// Picks the next task to run according to the EEVDF algorithm.
    ///
    /// The algorithm:
    /// 1. If the runqueue is empty, return `None`.
    /// 2. First, look for an **eligible** task: one where `vruntime <= min_vruntime`.
    ///    Return the first such task (which will have the earliest deadline).
    /// 3. If no eligible task exists, fall back to the task with the smallest
    ///    `vruntime` (to ensure fairness and prevent starvation).
    ///
    /// # Returns
    /// `Some(TaskId)` of the next task to run, or `None` if the runqueue is empty.
    ///
    /// # Notes
    /// Only tasks in the `Runnable` state are considered. Tasks that are
    /// sleeping, blocked, or zombie are skipped.
    pub fn pick_next(&mut self) -> Option<TaskId> {
        if self.by_deadline.is_empty() {
            return None;
        }

        // First pass: find an eligible task (vruntime <= min_vruntime).
        for key in self.by_deadline.iter() {
            if let Some(task) = self.tasks.get(&key.id)
            && task.state == TaskState::Runnable && key.vruntime <= self.min_vruntime {
                return Some(key.id);
            }
        }

        // Second pass: fall back to the task with the smallest vruntime.
        let mut best_id = None;
        let mut min_vr = u64::MAX;
        for key in self.by_deadline.iter() {
            if let Some(task) = self.tasks.get(&key.id)
            && task.state == TaskState::Runnable && key.vruntime < min_vr {
                min_vr = key.vruntime;
                best_id = Some(key.id);
            }
        }
        best_id
    }
}

// ============================================================================
// PER‑CPU RUNQUEUES
// ============================================================================

/// Static array of per‑CPU runqueues.
///
/// Each CPU core has its own runqueue, protected by a `Nitex` (interrupt‑
/// disabling spinlock). The array is indexed by CPU ID and is sized to
/// `MAX_CPUS` from the architecture module.
///
/// # Safety
/// The runqueues are `static` and are accessed from multiple CPUs. The `Nitex`
/// lock ensures safe concurrent access.
pub static RUNQUEUES: [Nitex<Runqueue>; crate::arch::MAX_CPUS]
=   [const { Nitex::new(Runqueue::new()) }; crate::arch::MAX_CPUS];

```

### `src/sched/wq.rs`

```rs
//! # Wait Queue (for Task Sleeping)
//!
//! This module provides a simple wait queue implementation used by the scheduler
//! to put tasks to sleep and wake them up when certain events occur. Wait queues
//! are the primary mechanism for blocking tasks that are waiting for resources,
//! I/O completion, or other conditions.
//!
//! ## Overview
//!
//! A wait queue is a container that holds a list of `TaskId`s that are currently
//! sleeping and waiting for an event. The queue is protected by a mutex (typically
//! a `Nutex`) to ensure safe concurrent access from multiple CPUs.
//!
//! ## Operations
//!
//! - **`sleep(task_id)`**: Adds a task to the wait queue. The task is expected to
//!   be removed from its runqueue before calling this function.
//! - **`wakeup_one()`**: Wakes up the first task in the queue (FIFO order). The
//!   task is removed from the queue and should be made runnable by the caller.
//! - **`wakeup_all()`**: Wakes up all tasks in the queue, returning a vector of
//!   their IDs.
//! - **`is_empty()`**: Returns `true` if the queue has no waiting tasks.
//!
//! ## Usage Pattern
//!
//! The typical usage pattern for a wait queue is:
//!
//! 1. A task needs to wait for an event (e.g., I/O completion, lock availability).
//! 2. It removes itself from its runqueue (`rq.remove(current_id)`).
//! 3. It marks its state as `Sleeping`.
//! 4. It calls `wq.lock().sleep(task.id)` to add itself to the wait queue.
//! 5. It re‑inserts itself into the runqueue (as sleeping tasks are still
//!    present in the runqueue's `tasks` map, just not in the `by_deadline` set).
//!    Actually, in our implementation, tasks are removed entirely and re‑inserted
//!    when woken.
//! 6. It calls `yield_now()` to trigger a context switch.
//!
//! When the event occurs:
//! 1. Another task (or interrupt handler) calls `wakeup(wq)`.
//! 2. `wakeup` locks the wait queue, removes the first task, and makes it runnable.
//! 3. The task is re‑inserted into the runqueue.
//!
//! ## FIFO Ordering
//!
//! The wait queue is a `VecDeque`, which provides FIFO (first‑in, first‑out)
//! ordering. This ensures fairness: tasks that have been waiting the longest
//! are woken first.
//!
//! ## Safety
//!
//! - The wait queue is protected by an external mutex (usually a `Nutex`).
//!   It is the caller's responsibility to acquire the lock before calling
//!   any methods.
//! - The tasks stored in the queue are identified by `TaskId`. The caller
//!   must ensure that the tasks are valid and exist in the global task registry.
//! - The `sleep` function holds the lock for the entire duration and does
//!   not yield, so the critical section is short.

use alloc::collections::VecDeque;
use crate::kmsg;

use super::task::TaskId;

// ============================================================================
// WAIT QUEUE STRUCTURE
// ============================================================================

/// A queue of sleeping tasks, waiting for an event.
///
/// The queue stores `TaskId`s in FIFO order. It is typically wrapped in a
/// `Nutex` or `Mutex` for safe concurrent access.
///
/// # Examples
///
/// ```ignore
/// use crate::sync::Nutex;
/// use crate::sched::wq::WaitQueue;
///
/// static MY_WQ: Nutex<WaitQueue> = Nutex::new(WaitQueue::new());
///
/// // In a task that wants to sleep:
/// let mut wq = MY_WQ.lock();
/// wq.sleep(task_id);
/// drop(wq);
/// crate::sched::yield_now();
///
/// // In the task that wakes it up:
/// let mut wq = MY_WQ.lock();
/// if let Some(id) = wq.wakeup_one() {
///     // Make the task runnable again.
///     crate::sched::wakeup(&MY_WQ);
/// }
/// ```
pub struct WaitQueue {
    waiters: VecDeque<TaskId>,
}

impl WaitQueue {
    /// Creates a new, empty wait queue.
    pub const fn new() -> Self {
        Self {
            waiters: VecDeque::new(),
        }
    }

    /// Adds a task to the end of the wait queue.
    ///
    /// The task is pushed to the back of the queue, ensuring FIFO ordering.
    ///
    /// # Arguments
    /// * `task_id` – The ID of the task to add.
    ///
    /// # Note
    /// This function also locks the KMSG sink registry to prevent the scheduler
    /// from escaping the lock and causing race conditions. This is a temporary
    /// workaround and should be replaced with a proper solution.
    pub fn sleep(&mut self, task_id: TaskId) {
        self.waiters.push_back(task_id);
        // Lock KMSG sinks to prevent the scheduler from escaping.
        let _ = kmsg::SINKS.lock();
    }

    /// Wakes up the first task in the queue (FIFO order).
    ///
    /// The task is removed from the front of the queue and returned.
    ///
    /// # Returns
    /// `Some(TaskId)` if the queue was not empty, otherwise `None`.
    pub fn wakeup_one(&mut self) -> Option<TaskId> {
        self.waiters.pop_front()
    }

    /// Wakes up all tasks in the queue.
    ///
    /// All tasks are removed from the queue and returned as a `Vec`.
    ///
    /// # Returns
    /// A `Vec<TaskId>` containing all tasks that were waiting.
    pub fn wakeup_all(&mut self) -> alloc::vec::Vec<TaskId> {
        self.waiters.drain(..).collect()
    }

    /// Returns `true` if the wait queue is empty.
    pub fn is_empty(&self) -> bool {
        self.waiters.is_empty()
    }
}

```

### `src/sched/proc.rs`

```rs
//! # Process Management
//!
//! This module defines the `Process` structure, which represents a user‑space process
//! in the kernel. A process is a container for tasks (threads) that share a common
//! address space, virtual memory areas, filesystem root, and syscall handler.
//!
//! ## Overview
//!
//! In the kernel, a **process** is the primary unit of resource ownership. Each process
//! has:
//!
//! - A unique process ID (`pid`).
//! - An optional parent process (`parent`).
//! - An address space (page tables) shared by all its threads.
//! - A virtual memory manager (`Vmm`) that tracks the process's memory regions.
//! - A list of threads (`threads`) that belong to the process.
//! - A syscall handler function that interprets system calls for the process.
//! - A root filesystem view (`RootRef`) for the process's mount namespace.
//! - A security level (`level`) for privilege separation.
//!
//! A process is **not** a schedulable entity by itself; instead, each process has
//! one or more tasks (threads) that are scheduled independently. All tasks in a
//! process share the same address space, VMM, and root filesystem.
//!
//! ## Relationship to Tasks
//!
//! ```text
//! Process (pid: 42)
//!   ├── Task (tid: 100)  <-- main thread
//!   ├── Task (tid: 101)  <-- worker thread
//!   └── Task (tid: 102)  <-- worker thread
//! ```
//!
//! Each `Task` contains an `Arc<Process>` that links it to its owning process.
//! This allows multiple tasks to share the same address space and resources.
//!
//! ## Address Space
//!
//! The process's address space is represented by a `Polen` (page table manager)
//! wrapped in an `Arc<Nutex<Polen>>`. This allows:
//! - Safe concurrent access from multiple threads.
//! - Copy‑on‑write semantics for `fork()` (cloning the address space).
//! - Isolation between processes (different CR3 values).
//!
//! ## Virtual Memory Areas (VMAs)
//!
//! The process's `Vmm` (Virtual Memory Manager) tracks all mapped memory regions
//! (VMAs) in a Red‑Black tree. This is used for:
//! - Demand paging: handling page faults by mapping pages from VMAs.
//! - Memory mapping: `mmap` and `munmap` operations.
//! - Access control: checking that faults occur within valid VMAs with correct permissions.
//!
//! ## Cloning
//!
//! Processes can be cloned (for `fork()`). The `Clone` implementation for `Process`
//! creates a new process with:
//! - A new, unique PID.
//! - The parent PID set to the original process's PID.
//! - A cloned address space (copy‑on‑write).
//! - A cloned VMM.
//! - The same syscall handler.
//! - A cloned root filesystem reference.
//! - An empty thread list.
//!
//! The actual cloning of the address space is deferred to the page table manager
//! (`Polen::dup()`), which performs a shallow copy of the page tables with
//! copy‑on‑write semantics.
//!
//! ## PID Allocation
//!
//! Process IDs are allocated from a global counter (`NEXT`), protected by a
//! `Litex` (interrupt‑disabling spinlock). The counter starts at 0 and is
//! incremented for each new process.
//!
//! ## Syscall Handler
//!
//! Each process has a `syscall_handler` function pointer. This allows different
//! processes to have different system call ABIs (e.g., native Linux‑style syscalls,
//! or a custom microkernel IPC interface). The default handler is
//! `sched::native_syscall_handler`.
//!
//! ## Root Filesystem
//!
//! Each process has a `RootRef` (an `Arc<RootReg>`) that defines its mount
//! namespace. This allows processes to have isolated views of the filesystem
//! (e.g., chroot, containers). By default, each process gets a new `RootReg`
//! that is independent of other processes.
//!
//! ## Security Level
//!
//! The `level` field represents the process's security level (a simple numeric
//! privilege level). This is used by the VFS to enforce access control:
//! files with `LEVEL_READ`, `LEVEL_WRITE`, or `LEVEL_EXEC` flags can only be
//! accessed by processes with a matching or higher level.
//!
//! ## Safety
//!
//! - The `Process` struct uses `Arc` for shared ownership of the address space
//!   and VMM. This ensures that the resources are not freed while any task is
//!   still using them.
//! - The `address_space` is wrapped in a `Nutex` (interrupt‑disabling spinlock)
//!   to protect against concurrent modifications from multiple threads.
//! - The `NEXT` PID counter uses a `Litex` for safe, interrupt‑safe increments.
//! - The `syscall_handler` is a function pointer that must be safe to call
//!   from interrupt context (it is called from the syscall dispatcher, which
//!   runs in the context of the calling thread).

use alloc::sync::Arc;
use alloc::vec::Vec;
use crate::arch::trap::TrapFrame;
use crate::mem::ptm::Polen;
use crate::mem::vma::Vmm;
use crate::sync::{Litex, Nutex};
use crate::vfs::{RootRef, RootReg};

// ============================================================================
// TYPE ALIASES
// ============================================================================

/// A process ID, which is a 32‑bit unsigned integer.
pub type ProcId = u32;

// ============================================================================
// PROCESS STRUCTURE
// ============================================================================

/// A user‑space process, representing a running program with its own address space.
///
/// A process is a container for tasks (threads) that share the same memory
/// space, filesystem view, and system call handler.
///
/// # Fields
/// * `pid` – The unique process ID.
/// * `parent` – The optional PID of the parent process (for `wait()` and reaping).
/// * `address_space` – The page tables for this process, shared by all its threads.
/// * `vmm` – The virtual memory manager, tracking all mapped regions (VMAs).
/// * `threads` – The list of task IDs that belong to this process.
/// * `syscall_handler` – The function called to handle system calls from this process.
/// * `roots` – The filesystem mount namespace (root registry).
/// * `level` – The security level of the process.
///
/// # Examples
/// ```ignore
/// let proc = Process::default();
/// let task = Task::new_user(entry, stack_top, kernel_stack_top, priority, "my_task");
/// // The task's process will be set to `proc`.
/// ```
pub struct Process {
    /// The unique process ID.
    pub pid: ProcId,

    /// The PID of the parent process, if any.
    pub parent: Option<ProcId>,

    /// The address space (page tables) for this process.
    ///
    /// This is shared among all threads in the process. It is wrapped in an `Arc`
    /// and a `Nutex` for safe concurrent access from multiple CPUs.
    pub address_space: Arc<Nutex<Polen>>,

    /// The virtual memory manager for this process.
    ///
    /// Tracks all mapped regions (VMAs) and is used for demand paging and
    /// memory management operations.
    pub vmm: Arc<Nutex<Vmm>>,

    /// The list of task IDs that belong to this process.
    pub threads: Vec<super::task::TaskId>,

    /// The system call handler for this process.
    ///
    /// This function pointer determines how system calls are interpreted.
    /// The default is `sched::native_syscall_handler`.
    pub syscall_handler: fn(&mut TrapFrame),

    /// The mount namespace (root registry) for this process.
    ///
    /// This defines which filesystems are mounted and where. Processes can
    /// have isolated mount namespaces for container‑like behaviour.
    pub roots: RootRef,

    /// The security level of the process.
    ///
    /// Used by the VFS to enforce access control on files with level‑based
    /// permissions (`LEVEL_READ`, `LEVEL_WRITE`, `LEVEL_EXEC`).
    pub level: u16,
}

// ============================================================================
// PID ALLOCATOR
// ============================================================================

/// The global PID counter, protected by a `Litex` (interrupt‑disabling spinlock).
static NEXT: Litex<u32> = Litex::new(0);

/// Allocates the next available PID.
///
/// This function locks the global counter, reads the current value, increments
/// it (using unsafe access to the inner data), and returns the old value.
///
/// # Returns
/// The next PID (starting from 0).
///
/// # Safety
/// The `unsafe` block is used to write to the inner data of the `Litex`.
/// This is safe because the lock is held for the duration of the operation.
fn next() -> u32 {
    let next = NEXT.lock();
    let rv = *next;
    unsafe { *NEXT.inner() = rv + 1 }
    rv
}

// ============================================================================
// PROCESS IMPLEMENTATION
// ============================================================================

impl Process {
    /// Creates a new `Process` with default values.
    ///
    /// The default process has:
    /// - A new, unique PID.
    /// - No parent.
    /// - A new address space (via `Polen::reference()`).
    /// - A new VMM.
    /// - An empty thread list.
    /// - The native syscall handler (`sched::native_syscall_handler`).
    /// - A new, empty root registry.
    /// - Security level 0.
    ///
    /// # Examples
    /// ```ignore
    /// let proc = Process::default();
    /// assert_eq!(proc.pid, 0);
    /// ```
    pub fn new() -> Self {
        // FIXME: triple fault before entry point when opt-level > 0
        Self {
            pid: next(),
            parent: None,
            address_space: Arc::new(Nutex::new(Polen::reference())),
            vmm: Arc::new(Nutex::new(Vmm::new())),
            threads: Vec::new(),
            syscall_handler: crate::sched::native_syscall_handler,
            roots: RootRef::new(RootReg::new()),
            level: 0,
        }
    }
}

// ============================================================================
// CLONE IMPLEMENTATION
// ============================================================================

impl Clone for Process {
    /// Creates a new process by cloning an existing one.
    ///
    /// This is used for `fork()`‑like operations. The new process:
    /// - Gets a new, unique PID.
    /// - Sets its parent to the original process's PID.
    /// - Shares the same address space (copy‑on‑write).
    /// - Shares the same VMM.
    /// - Gets an empty thread list.
    /// - Uses the same syscall handler.
    /// - Shares the same root filesystem reference.
    /// - Uses the same security level.
    ///
    /// # Returns
    /// A new `Process` that is a clone of the current process.
    ///
    /// # Note
    /// The address space and VMM are shared via `Arc`. This means that
    /// modifications to the page tables or VMAs will be visible to both
    /// processes. For true copy‑on‑write semantics, the address space
    /// should be duplicated lazily when the first write occurs.
    fn clone(&self) -> Self {
        Self {
            pid: next(),
            parent: Some(self.pid),
            address_space: self.address_space.clone(),
            vmm: self.vmm.clone(),
            threads: vec![],
            syscall_handler: self.syscall_handler,
            roots: self.roots.clone(),
            level: self.level,
        }
    }
}

// ============================================================================
// FORMATTING TRAITS
// ============================================================================

impl core::fmt::Display for Process {
    /// Formats the process for debugging and logging.
    ///
    /// The display includes the PID, parent PID, CR3 (address space), and
    /// the number of threads.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "Process {{ pid: {}, parent: {:?}, address_space: {}, threads (len): {} }}",
            self.pid,
            self.parent,
            self.address_space.lock().exco.cr3,
            self.threads.len(),
        ))
    }
}

```

### `src/vfs/mod.rs`

```rs
//! # Virtual File System (VFS)
//!
//! The VFS subsystem provides a unified interface for file operations across different
//! filesystem types. It abstracts the underlying storage and allows multiple filesystems
//! to be mounted, accessed, and managed consistently.
//!
//! ## Architecture Overview
//!
//! The VFS is designed around several key abstractions:
//!
//! - **Inode**: Represents a file, directory, or other filesystem object. Each inode
//!   stores metadata (size, permissions, timestamps, etc.) and a reference to its
//!   containing filesystem (`MetaBlock`). Inodes are identified by `InodeId`.
//!
//! - **MetaBlock**: A mount point or filesystem instance. It holds a pointer to a
//!   filesystem‑specific `FsVtable` (function table) and an opaque `data` pointer
//!   that points to the filesystem's internal state. It is wrapped in a `RwLock`
//!   for concurrent access.
//!
//! - **FsVtable**: A static table of function pointers that implement the filesystem's
//!   operations: `lookup`, `readdir`, `read`, `write`, `truncate`, `unlink`, `link`,
//!   `new`, `get`, `get_mut`. Each filesystem provides its own implementation.
//!
//! - **RootReg / RootRef**: Manages the mount points (root entries) for a process.
//!   Each process has a `RootRef` (an `Arc<RootReg>`) that maps mount point names
//!   (e.g., `"/"`, `"/proc"`) to `InodeId`s. This allows processes to have different
//!   namespace views.
//!
//! - **InodeId**: A tuple `(u32, u32)` where the first component is an inode number
//!   (unique within the filesystem) and the second is a `MetaBlock` identifier. It
//!   provides safe methods to retrieve the inode from the global registry.
//!
//! ## Filesystem Registration
//!
//! Each filesystem type (e.g., `PvfsMb` for the purely virtual filesystem) defines:
//! - A data structure that holds the filesystem's internal state (e.g., a `BTreeMap`
//!   of inodes).
//! - An instance of `FsVtable` with function pointers to the implementation.
//! - A mechanism to create a new `MetaBlock` by calling `new_mblock(id, &VTABLE, &state)`.
//!
//! The `MetaBlock` is then registered in the global `MBLK_REG` via `reg_mblk()`,
//! which returns a unique ID. This ID is used in `InodeId` to locate the filesystem.
//!
//! ## Inode Operations
//!
//! The module provides high‑level functions that operate on `InodeId`:
//! - `lookup(id, name)` – find a child inode by name (for directories).
//! - `listdir(id)` – list all entries in a directory.
//! - `read(id, offset, buf)` – read data from a file.
//! - `write(id, offset, buf)` – write data to a file.
//! - `link(id, name, new_id)` – create a new directory entry.
//! - `new(mb, inode, kind)` – create a new inode in a filesystem.
//!
//! These functions obtain the `Inode` from the `InodeId`, retrieve the `MetaBlock`,
//! and call the corresponding method from the `FsVtable`.
//!
//! ## Global Inode Registry
//!
//! The VFS maintains a global registry of `MetaBlock` instances in `MBLK_REG`,
//! a `RwLock<BTreeMap<u32, MetaBlock>>`. This registry is used to look up
//! the `MetaBlock` for a given ID. Inodes themselves are stored inside the
//! filesystem's internal data structures (e.g., in `PvfsMb.reg`), not in a
//! global table.
//!
//! ## Purely Virtual Filesystem (PVFS)
//!
//! `PvfsMb` is a simple, in‑memory filesystem that does not require a block device.
//! It stores files as `Vec<u8>` and directories as `Vec<(String, InodeId)>`.
//! It implements all `FsVtable` operations and is used early in boot for testing
//! and as a basis for other virtual filesystems (e.g., `procfs`, `sysfs`).
//!
//! ## Mount Points and Root Namespace
//!
//! Each process has a `RootRef` (shared reference-counted `RootReg`) that maps
//! mount point names (e.g., `"root"`, `"proc"`) to `InodeId`s. This allows
//! processes to have isolated views of the filesystem namespace. The `RootReg`
//! is protected by a `Litex` (interrupt‑disabling spinlock) for safe concurrent
//! access.
//!
//! ## Error Handling
//!
//! Most VFS operations return a `Result` with an `Error` enum (`Unknown`,
//! `NotAFile`, `OutOfBounds`, `NoEntry`, `NotADirectory`, `Found`). Errors are
//! propagated to the caller, which typically handles them by logging or returning
//! an appropriate userspace error code.

// ============================================================================
// SUBMODULES
// ============================================================================

mod inode;   // Inode, InodeId, MetaBlock registration
mod root;    // RootReg, RootRef – mount point management
mod pvfs;    // Purely virtual filesystem (PVFS)
mod err;     // VFS error types
mod mb;      // MetaBlock, FsVtable definitions

// ============================================================================
// RE-EXPORTS
// ============================================================================

pub use inode::*;
pub use root::*;
pub use pvfs::*;
pub use err::*;
pub use mb::*;

// ============================================================================
// VFS HIGH‑LEVEL OPERATIONS
// ============================================================================

use alloc::{collections::btree_map::BTreeMap, string::String};

/// Looks up a child inode by name in a directory.
///
/// # Arguments
/// * `this` – The directory inode ID.
/// * `name` – The name to look up.
///
/// # Returns
/// `Some(InodeId)` if the entry exists, `None` otherwise.
pub fn lookup(this: &InodeId, name: String) -> Option<InodeId> {
    if let Some(i) = this.get() {
        return (unsafe { i.mblock.inner().vtable().lookup })(i, name)
    }
    None
}

/// Lists all entries in a directory.
///
/// # Arguments
/// * `this` – The directory inode ID.
///
/// # Returns
/// A `BTreeMap<String, InodeId>` mapping entry names to inode IDs.
pub fn listdir(this: &InodeId) -> BTreeMap<String, InodeId> {
    let mut rv = BTreeMap::<String, InodeId>::new();

    if let Some(i) = this.get() {
        for ofs in 0.. {
            if let Some((name, id)) = (unsafe { i.mblock.inner().vtable().readdr })(i, ofs) {
                rv.insert(name, id);
            } else {
                break
            }
        }
    }
    rv
}

/// Reads data from a file at a given offset.
///
/// # Arguments
/// * `this` – The file inode ID.
/// * `offset` – The byte offset to start reading from.
/// * `buf` – The buffer to fill with data.
///
/// # Returns
/// The number of bytes read, or an `Error`.
pub fn read(this: &InodeId, offset: usize, buf: &mut [u8]) -> Result<usize, Error> {
    if let Some(i) = this.get() {
        return (unsafe { i.mblock.inner().vtable().read })(i, offset, buf)
    }
    Err(Error::Unknown)
}

/// Reads the entire contents of a file into a `String`.
///
/// # Arguments
/// * `this` – The file inode ID.
///
/// # Returns
/// `Ok(String)` if successful, `Err(())` on failure.
pub fn read_to_string(this: &InodeId) -> Result<String, ()> {
    if let Some(i) = this.get() {
        let mut buf = [0u8].repeat(i.size as usize);
        if read(this, 0, &mut buf).is_err() {
            return Err(())
        }
        if let Ok(s) = String::from_utf8(buf) {
            return Ok(s)
        }
    }
    Err(())
}

/// Writes data to a file at a given offset.
///
/// # Arguments
/// * `this` – The file inode ID.
/// * `offset` – The byte offset to start writing at.
/// * `buf` – The data to write.
///
/// # Returns
/// The number of bytes written, or an `Error`.
pub fn write(this: &InodeId, offset: usize, buf: &[u8]) -> Result<usize, Error> {
    if let Some(i) = this.get() {
        return (unsafe { i.mblock.inner().vtable().write })(i, offset, buf)
    }
    Err(Error::Unknown)
}

/// Creates a new directory entry (link) in a directory.
///
/// # Arguments
/// * `this` – The directory inode ID.
/// * `name` – The name of the new entry.
/// * `new` – The inode ID to link to.
///
/// # Returns
/// `Ok(())` on success, or an `Error`.
pub fn link(this: &InodeId, name: String, new: InodeId) -> Result<(), Error> {
    if let Some(i) = this.get_mut() {
        return (unsafe { i.mblock.inner().vtable().link })(i, name, new)
    }
    Err(Error::Unknown)
}

/// Creates a new inode in a filesystem.
///
/// # Arguments
/// * `mb` – The `MetaBlock` representing the filesystem.
/// * `inode` – The inode to create (must have `kind`, `flags`, etc. set).
/// * `kind` – The kind of inode (e.g., `File`, `Directory`).
///
/// # Returns
/// The new `InodeId`, or an `Error`.
pub fn new(mb: &MetaBlock, inode: Inode, kind: Kind) -> Result<InodeId, Error> {
    (unsafe { mb.inner().vtable().new })(mb, inode, kind)
}

```

### `src/vfs/inode.rs`

```rs
//! # VFS Inode Management
//!
//! This module defines the core inode structure, inode identifiers, and the global
//! registry of filesystem instances (`MetaBlock`s). Inodes represent filesystem
//! objects (files, directories, sockets, etc.) and are the primary interface for
//! VFS operations.
//!
//! ## Overview
//!
//! An **inode** is a metadata structure that describes a filesystem object. It
//! contains:
//! - **Identity**: `id` (`InodeId`), which is a combination of an inode number
//!   (unique within the filesystem) and a `MetaBlock` ID.
//! - **Type**: `kind` (`Kind`), indicating whether it's a file, directory, socket,
//!   virtual device, or symlink.
//! - **Permissions**: `flags` (`Flags`), which encode POSIX‑style read/write/execute
//!   permissions for user, group, and others, plus a security level.
//! - **Metadata**: `size`, `uid`, `gid`, `atime`, `mtime`, `ctime`.
//! - **Parent**: `parent` (`InodeId`), linking to the containing directory.
//! - **Filesystem binding**: `mblock` (`&'static MetaBlock`), a reference to the
//!   filesystem instance that owns this inode.
//! - **Name**: `name` (`String`), the entry name (used for lookup).
//! - **Private data**: `private` (`[u8; 32]`), a small buffer for filesystem‑specific
//!   usage.
//!
//! ## Inode Identifiers (`InodeId`)
//!
//! An `InodeId` is a tuple `(u32, u32)`:
//! - **First component**: The inode number, unique within the filesystem.
//! - **Second component**: The `MetaBlock` ID, which identifies the filesystem
//!   instance in the global registry.
//!
//! The `InodeId` provides methods `get()` and `get_mut()` that look up the
//! `MetaBlock` from the global registry and then fetch the inode from the
//! filesystem's internal data structures. This provides a safe way to obtain
//! a reference to the inode.
//!
//! ## MetaBlock Registry
//!
//! The global registry (`MBLK_REG`) is a `RwLock<BTreeMap<u32, MetaBlock>>` that
//! maps `MetaBlock` IDs to their instances. This registry is used by `InodeId`
//! to locate the filesystem when performing operations.
//!
//! - `reg_mblk(mb)`: Registers a new `MetaBlock` and returns its ID.
//! - `unreg_mblk(id)`: Unregisters a `MetaBlock` by ID.
//! - `get_mblk(id)`: Returns a reference to the `MetaBlock` for a given ID.
//!
//! ## Inode Flags (`Flags`)
//!
//! The `Flags` struct uses bitflags to encode:
//! - Directory flag (`DIR`).
//! - Read/write/execute permissions for user, group, and others.
//! - Level‑based permissions (for security levels).
//! - A 16‑bit security level stored in the upper bits.
//!
//! ## Empty VTable
//!
//! The `EMPTY_VTABLE` is a static `FsVtable` that panics on any operation. It is
//! used as a placeholder when an inode is created before being properly associated
//! with a filesystem.

use alloc::{borrow::ToOwned, collections::btree_map::BTreeMap, string::String};

use crate::{sync::RwLock, vfs::{FsVtable, MetaBlock, new_mblock}};

// ============================================================================
// INODE FLAGS
// ============================================================================

extrum! {
    /// Permissions and attributes for an inode.
    ///
    /// These flags encode POSIX‑like permissions and a security level.
    #[derive(Clone, Copy, PartialEq)]
    pub enum Flags: u64 {
        // Directory flag
        // The inode is a directory.
        DIR         = 1 << 0    ,

        // User owner rights
        USER_READ   = 1 << 1    ,
        USER_WRITE  = 1 << 2    ,
        USER_EXEC   = 1 << 3    ,

        // Group owner rights
        GROUP_READ  = 1 << 4    ,
        GROUP_WRITE = 1 << 5    ,
        GROUP_EXEC  = 1 << 6    ,

        // Others rights
        OTHER_READ  = 1 << 7    ,
        OTHER_WRITE = 1 << 8    ,
        OTHER_EXEC  = 1 << 9    ,

        // Level-defined rights
        LEVEL_READ  = 1 << 10   ,
        LEVEL_WRITE = 1 << 11   ,
        LEVEL_EXEC  = 1 << 12   ,
    }
}

impl Flags {
    /// Returns the security level stored in the upper bits of the flags.
    pub fn level(self) -> u16 { (self.0 >> 48) as u16 }

    /// Sets the security level in the upper bits of the flags.
    pub fn set_level(&mut self, level: u16) {
        self.0 &= !0 << 16 >> 16;
        self.0 |= (level as u64) << 48;
    }
}

implement_display![Flags];

// ============================================================================
// INODE IDENTIFIER
// ============================================================================

/// A unique identifier for an inode.
///
/// The first component is an inode number (unique within the filesystem),
/// and the second is the `MetaBlock` ID (identifying the filesystem instance).
#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct InodeId(pub u32, pub u32);

impl InodeId {
    /// Returns an immutable reference to the inode, if it exists.
    ///
    /// This function looks up the `MetaBlock` from the registry using the
    /// second component of the ID, then calls the `get` method of the
    /// filesystem's vtable to retrieve the inode.
    ///
    /// # Returns
    /// `Some(&'static Inode)` if the inode exists, otherwise `None`.
    pub fn get(self) -> Option<&'static Inode> {
        if let Some(mb) = get_mblk(self.1)
        && let Some(rv) = (mb.read().vtable().get)(mb, self.0) {
            return Some(rv)
        }
        None
    }

    /// Returns a mutable reference to the inode, if it exists.
    ///
    /// Similar to `get()`, but returns a mutable reference for in‑place updates.
    pub fn get_mut(self) -> Option<&'static mut Inode> {
        if let Some(mb) = get_mblk(self.1)
        && let Some(rv) = (mb.read().vtable().get_mut)(mb, self.0) {
            return Some(rv)
        }
        None
    }
}

// ============================================================================
// INODE KIND
// ============================================================================

/// The type of a filesystem object.
#[repr(C)]
pub enum Kind {
    /// Unknown or uninitialized.
    Unknown     ,
    /// A regular file.
    File        ,
    /// A directory (can contain entries).
    Directory   ,
    /// A UNIX domain socket.
    Socket      ,
    /// A virtual/device file (e.g., `/dev/null`).
    Virtual     ,
    /// A symbolic link.
    SymLink     ,
}

// ============================================================================
// INODE STRUCTURE
// ============================================================================

/// Metadata for a filesystem object.
///
/// This structure is stored in the filesystem's internal data structures
/// (e.g., in `PvfsMb.reg`). It is aligned to 128 bytes for cache efficiency.
#[repr(C, align(128))]
pub struct Inode {
    /// Unique identifier for this inode.
    pub id      :                InodeId    ,
    /// The kind of object (file, directory, etc.).
    pub kind    :                Kind       ,
    /// Permissions and attributes.
    pub flags   :                Flags      ,
    /// Size of the file in bytes (for directories, number of entries).
    pub size    :                u64        ,
    /// Owner user ID.
    pub uid     :                u16        ,
    /// Owner group ID.
    pub gid     :                u16        ,
    /// Last access time (in seconds since epoch).
    pub atime   :                u64        ,
    /// Last modification time.
    pub mtime   :                u64        ,
    /// Last status change time.
    pub ctime   :                u64        ,
    /// Parent inode (the containing directory).
    pub parent  :                InodeId    ,
    /// Reference to the filesystem instance that owns this inode.
    pub mblock  :       &'static MetaBlock  ,
    /// Entry name (used for lookups).
    pub name    :                String     ,
    /// Filesystem‑specific private data (32 bytes).
    pub private :                [u8; 32]   ,
}

// ============================================================================
// EMPTY VTABLE AND META BLOCK
// ============================================================================

/// A dummy filesystem vtable that panics on any operation.
///
/// This is used as a placeholder for inodes that are not yet associated with
/// a real filesystem.
pub static EMPTY_VTABLE: FsVtable = FsVtable {
    lookup  :|_,_  |panic!("empty FS vtable"),
    readdr  :|_,_  |panic!("empty FS vtable"),
    read    :|_,_,_|panic!("empty FS vtable"),
    write   :|_,_,_|panic!("empty FS vtable"),
    trunc   :|_,_  |panic!("empty FS vtable"),
    unlink  :|_,   |panic!("empty FS vtable"),
    link    :|_,_,_|panic!("empty FS vtable"),
    new     :|_,_,_|panic!("empty FS vtable"),
    get     :|_,_  |panic!("empty FS vtable"),
    get_mut :|_,_  |panic!("empty FS vtable"),
};

lazy_static! {
    /// A dummy `MetaBlock` associated with `EMPTY_VTABLE`.
    pub static ref EMPTY_MBLOCK: MetaBlock = new_mblock(!0, &EMPTY_VTABLE, &mut ());
}

// ============================================================================
// INODE DEFAULT CONSTRUCTOR
// ============================================================================

impl Default for Inode {
    /// Creates a new, default‑initialized inode.
    ///
    /// The inode is assigned `id = InodeId(0, 0)`, `kind = Kind::Unknown`,
    /// and all other fields are zero or empty. The `mblock` points to the
    /// global `EMPTY_MBLOCK`.
    fn default() -> Self {
        Self {
            id      : InodeId(0, 0)     ,
            kind    : Kind::Unknown     ,
            flags   : Flags::from_raw(0),
            size    : 0                 ,
            uid     : 0                 ,
            gid     : 0                 ,
            atime   : 0                 ,
            mtime   : 0                 ,
            ctime   : 0                 ,
            parent  : InodeId(0, 0)     ,
            mblock  : &*EMPTY_MBLOCK    ,
            name    : "".to_owned()     ,
            private : [0u8; 32]         ,
        }
    }
}

impl Inode {
    /// Public constructor for a new, default inode.
    ///
    /// Use this to create a new inode before adding it to a filesystem.
    pub fn new() -> Self { Self::default() }
}

// ============================================================================
// GLOBAL META BLOCK REGISTRY
// ============================================================================

lazy_static! {
    /// Global registry of filesystem instances (`MetaBlock`).
    ///
    /// This is a `RwLock` that protects a `BTreeMap` mapping `MetaBlock` IDs to
    /// their instances, along with a counter for the next available ID.
    ///
    /// The registry is seeded with the `EMPTY_MBLOCK` at ID `!0`.
    static ref MBLK_REG: RwLock<(BTreeMap<u32, MetaBlock>, u32)> = {
        let v: RwLock<(BTreeMap<u32, MetaBlock>, u32)> = RwLock::new((BTreeMap::new(), 0u32));
        let _ = v.write().0.insert(!0, RwLock::new(EMPTY_MBLOCK.read().clone()));
        v
    };
}

/// Registers a new `MetaBlock` in the global registry.
///
/// # Arguments
/// * `mb` – The `MetaBlock` to register.
///
/// # Returns
/// A unique ID that can be used in `InodeId` to refer to this filesystem.
pub fn reg_mblk(mb: MetaBlock) -> u32 {
    let mut rn = MBLK_REG.write();
    let rv = rn.1;
    mb.write().id = rv;
    rn.0.insert(rv, mb);
    rn.1 += 1;
    rv
}

/// Unregisters a `MetaBlock` from the global registry.
///
/// # Arguments
/// * `id` – The ID of the `MetaBlock` to remove.
///
/// # Returns
/// `Ok(MetaBlock)` if the registry entry was found and removed, `Err(())` otherwise.
pub fn unreg_mblk(id: u32) -> Result<MetaBlock, ()> {
    MBLK_REG.write().0.remove(&id).ok_or(())
}

/// Retrieves a `MetaBlock` from the registry by ID.
///
/// # Arguments
/// * `id` – The ID of the `MetaBlock`.
///
/// # Returns
/// `Some(&'static MetaBlock)` if the ID exists, otherwise `None`.
///
/// # Safety
/// This function uses unsafe code to obtain a reference with static lifetime.
/// The caller must ensure that the `MetaBlock` is not freed while the reference
/// is held (the registry holds ownership).
pub fn get_mblk(id: u32) -> Option<&'static MetaBlock> {
    if MBLK_REG.read().0.contains_key(&id) {
        return Some(&unsafe { MBLK_REG.inner() }.0[&id])
    }
    None
}

/// Retrieves a mutable reference to a `MetaBlock` from the registry by ID.
///
/// # Safety
/// This function uses unsafe code; the caller must ensure proper synchronization.
pub fn get_inode_mut(id: u32) -> Option<&'static mut MetaBlock> {
    if MBLK_REG.read().0.contains_key(&id) {
        return unsafe { MBLK_REG.inner() }.0.get_mut(&id)
    }
    None
}

```

### `src/vfs/root.rs`

```rs
//! # VFS Root Registry (Mount Points)
//!
//! This module manages the mount point namespace for processes. Each process has a
//! `RootRef` (an `Arc<RootReg>`) that maps mount point names (e.g., `"/"`, `"/proc"`,
//! `"/dev"`) to `InodeId`s. This allows processes to have isolated filesystem views
//! and supports mount namespaces.
//!
//! ## Overview
//!
//! The root registry is the process‑level equivalent of a mount table. It associates
//! a name (e.g., `"root"`, `"proc"`, `"tmp"`) with an `InodeId` representing the
//! root inode of that filesystem. All path resolution starts from this registry.
//!
//! ## Structure
//!
//! - **`RootReg`**: The actual registry, which is a `Litex<BTreeMap<String, InodeId>>`.
//!   The `Litex` is an interrupt‑disabling spinlock that protects the map.
//! - **`RootRef`**: A type alias for `Arc<RootReg>`, allowing shared ownership of
//!   the registry across tasks and processes. This enables efficient cloning and
//!   sharing of mount namespaces.
//!
//! ## Operations
//!
//! - **`add_root(name, inode)`**: Inserts or overwrites a mount point. Returns the
//!   previous value, if any.
//! - **`add_new_root(name, inode)`**: Inserts a mount point only if it does not
//!   already exist. Returns `Ok(())` on success, `Err(inode)` if the name is taken.
//! - **`pop_root(name)`**: Removes a mount point and returns its `InodeId`, if it existed.
//! - **`Index<String>`**: Allows direct indexing, e.g., `roots["proc"]`.
//!
//! ## Cloning
//!
//! `RootReg` implements `Clone` by creating a new `RootReg` and copying the
//! contents of the map from the source. This creates a deep copy of the mount
//! namespace, allowing for isolated namespaces while still sharing the underlying
//! `Arc<RootReg>` when desired.
//!
//! ## Safety
//!
//! - The `Litex` lock disables interrupts during critical sections, ensuring that
//!   operations on the map are atomic with respect to interrupts and other CPUs.
//! - The `unsafe` code in `clone_from` and `Index` is used to access the inner
//!   map after locking, which is safe because the lock is held.
//! - The `RootRef` type alias (`Arc<RootReg>`) provides safe shared ownership.

// ============================================================================
// IMPORTS
// ============================================================================

use core::ops::Index;

use alloc::{collections::btree_map::BTreeMap, string::String, sync::Arc};

use crate::{sync::Litex, vfs::InodeId};

// ============================================================================
// ROOT REGISTRY
// ============================================================================

/// A process‑local mount point registry.
///
/// This struct holds a `BTreeMap` that maps mount point names (e.g., `"root"`,
/// `"proc"`) to `InodeId`s. The map is protected by a `Litex` (interrupt‑disabling
/// spinlock) for safe concurrent access from multiple CPUs.
///
/// # Examples
/// ```ignore
/// let roots = RootReg::new();
/// let root_inode = InodeId(0, 0);
/// roots.add_root("root".to_string(), root_inode);
///
/// let proc_inode = InodeId(1, 0);
/// roots.add_new_root("proc".to_string(), proc_inode).unwrap();
///
/// assert_eq!(roots["proc"], proc_inode);
/// ```
pub struct RootReg(Litex<BTreeMap<String, InodeId>>);

impl RootReg {
    /// Creates a new, empty root registry.
    pub fn new() -> Self {
        Self(Litex::new(BTreeMap::new()))
    }

    /// Inserts a mount point, overwriting any existing entry with the same name.
    ///
    /// # Arguments
    /// * `name` – The mount point name (e.g., `"root"`).
    /// * `inode` – The `InodeId` of the root inode of the filesystem.
    ///
    /// # Returns
    /// `Some(old_inode)` if an entry with the same name already existed,
    /// otherwise `None`.
    pub fn add_root(&self, name: String, inode: InodeId) -> Option<InodeId> {
        self.0.lock().insert(name, inode)
    }

    /// Inserts a mount point only if the name is not already taken.
    ///
    /// # Arguments
    /// * `name` – The mount point name.
    /// * `inode` – The `InodeId` of the root inode.
    ///
    /// # Returns
    /// `Ok(())` if the insertion succeeded, or `Err(inode)` if the name was
    /// already present.
    pub fn add_new_root(&self, name: String, inode: InodeId) -> Result<(), InodeId> {
        let mut reg = self.0.lock();

        // Safe: we hold the lock
        if !unsafe { self.0.inner() }.contains_key(&name) {
            reg.insert(name, inode);
            return Ok(())
        }

        Err(inode)
    }

    /// Removes a mount point by name.
    ///
    /// # Arguments
    /// * `name` – The mount point name to remove.
    ///
    /// # Returns
    /// `Some(inode)` if the entry existed and was removed, otherwise `None`.
    pub fn pop_root(&self, name: String) -> Option<InodeId> {
        self.0.lock().remove(&name)
    }
}

impl Default for RootReg {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// CLONE IMPLEMENTATION
// ============================================================================

impl Clone for RootReg {
    /// Creates a deep copy of the mount registry.
    ///
    /// This is useful for creating isolated mount namespaces for new processes.
    /// The clone acquires locks on both the source and destination maps and
    /// copies all entries.
    fn clone(&self) -> Self {
        let rv = Self::new();

        let _g1 = self.0.lock();

        {
            let _g2 = rv.0.lock();

            unsafe { rv.0.inner().iter().clone_from(&self.0.inner().iter()) };
        }

        rv
    }

    /// Clones another `RootReg` into this one, clearing existing entries.
    ///
    /// This is a more efficient way to replace the contents of a `RootReg`
    /// with another's contents.
    fn clone_from(&mut self, source: &Self)
    where
        Self: core::marker::Destruct,
    {
        self.0.lock().iter().clone_from(&source.0.lock().iter());
    }
}

// ============================================================================
// INDEX TRAIT IMPLEMENTATION
// ============================================================================

impl Index<String> for RootReg {
    type Output = InodeId;

    /// Indexes into the registry by mount point name.
    ///
    /// # Panics
    /// Panics if the name does not exist in the registry.
    fn index(&self, index: String) -> &Self::Output {
        let _ = self.0.lock();
        unsafe { self.0.inner() }.get(&index).unwrap()
    }
}

// ============================================================================
// TYPE ALIAS
// ============================================================================

/// An atomically reference‑counted root registry.
///
/// This type alias simplifies sharing of mount namespaces across tasks and processes.
/// Multiple tasks can share the same `RootRef` to have a consistent view of the
/// filesystem namespace, or each can have its own copy for isolation.
pub type RootRef = Arc<RootReg>;

```

### `src/vfs/pvfs.rs`

```rs
//! # Purely Virtual Filesystem (PVFS)
//!
//! The PVFS is a simple, in‑memory filesystem that does not rely on any block
//! device or persistent storage. It is used primarily for:
//! - Early boot testing and validation of the VFS subsystem.
//! - Temporary file storage in ramdisks (e.g., `tmpfs`‑like functionality).
//!
//! ## Architecture
//!
//! PVFS stores all data in memory, using a global `BTreeMap` protected by a
//! `Litex` (interrupt‑disabling spinlock). The map associates `InodeId`s with
//! `Data` variants:
//!
//! - `Data::File(Vec<u8>)`: A regular file with its contents stored as a byte vector.
//! - `Data::Dir(Vec<(String, InodeId)>)`: A directory, storing a list of entries
//!   (name and inode ID).
//!
//! ## Filesystem Structure
//!
//! The PVFS is managed through a `PvfsMb` struct, which holds:
//! - `reg`: A `BTreeMap<u32, Inode>` mapping inode numbers to inode metadata.
//! - `nxt`: The next available inode number.
//!
//! Each inode stores its `id`, `kind`, `flags`, `size`, and other metadata. The
//! actual file data or directory entries are stored separately in the global `DATA`
//! map, keyed by the inode's full `InodeId`.
//!
//! ## Global Data Map
//!
//! ```text
//! DATA: Litex<BTreeMap<InodeId, Data>>
//! ```
//!
//! This map is the actual storage backend for PVFS. It is protected by a `Litex`
//! spinlock (which disables interrupts) to ensure safe concurrent access.
//!
//! ## VTable Operations
//!
//! PVFS implements the full `FsVtable` interface:
//!
//! - **`lookup`**: Searches a directory for an entry by name, returning the
//!   corresponding `InodeId`.
//! - **`readdr`**: Reads a directory entry by index, returning `(name, inode_id)`.
//! - **`read`**: Reads data from a file at a given offset into a buffer.
//! - **`write`**: Writes data to a file at a given offset, truncating or extending
//!   the file as needed.
//! - **`trunc`**: Truncates a file to a new size (currently returns `Err` for
//!   directories; files are truncated by shrinking the `Vec<u8>`).
//! - **`unlink`**: Removes a file or directory from its parent, freeing its data.
//! - **`link`**: Adds a new directory entry in a parent directory.
//! - **`new`**: Creates a new inode (file or directory) and returns its `InodeId`.
//! - **`get` / `get_mut`**: Retrieves an inode by its number from the filesystem's
//!   `reg` map.
//!
//! ## Limitations
//!
//! - No persistence: data is lost on reboot.
//! - No hard link count or reference tracking (but the VFS layer provides
//!   directory entries).
//! - No support for symlinks or sockets (though the `Kind` enum includes them,
//!   they are not implemented).
//! - Truncation of directories is not supported (returns `NotADirectory`).
//!
//! ## Usage Example
//!
//! ```ignore
//! let mbinst = PvfsMb::new();
//! let mblock = new_mblock(0, &PVFS_VTABLE, &mut mbinst);
//! let mb_id = reg_mblk(mblock);
//!
//! let inode = Inode::new();
//! let id = vfs::new(&mblock, inode, Kind::File).unwrap();
//! vfs::write(&id, 0, b"Hello, world!").unwrap();
//! let mut buf = [0; 13];
//! vfs::read(&id, 0, &mut buf).unwrap();
//! assert_eq!(&buf, b"Hello, world!");
//! ```
//!
//! ## Safety
//!
//! - The global `DATA` map is protected by a `Litex` (interrupt‑disabling spinlock).
//!   All operations on the map are performed while holding the lock.
//! - The `unsafe` blocks in `get` and `get_mut` are used to obtain references with
//!   static lifetime from the `BTreeMap`; this is safe because the map is never
//!   deallocated and the references are used only within the filesystem's lifetime.

use core::cmp::min;

use alloc::{collections::btree_map::BTreeMap, string::String, vec::Vec};

use crate::{sync::Litex, vfs::{Error, FsVtable, Inode, InodeId, Kind, MetaBlock}};

// ============================================================================
// DATA TYPES
// ============================================================================

/// The actual content stored for an inode.
///
/// - `File`: A byte vector representing the file's data.
/// - `Dir`: A vector of directory entries (name, inode ID).
enum Data {
    File(Vec<u8>),
    Dir(Vec<(String, InodeId)>),
}

// ============================================================================
// GLOBAL DATA STORAGE
// ============================================================================

/// Global storage for all PVFS files and directories.
///
/// This map is keyed by the full `InodeId` and contains the `Data` variant.
/// It is protected by a `Litex` spinlock.
static DATA: Litex<BTreeMap<InodeId, Data>> = Litex::new(BTreeMap::new());

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Returns `true` if the inode identified by `id` is a file.
#[inline]
fn is_file(id: InodeId) -> bool {
    matches!(&DATA.lock()[&id], Data::File(_))
}

// ============================================================================
// VTABLE FUNCTIONS
// ============================================================================

/// Look up an entry by name in a directory.
///
/// # Arguments
/// * `inode` – The directory inode.
/// * `name` – The name to look up.
///
/// # Returns
/// `Some(InodeId)` if found, `None` otherwise.
fn lookup(inode: &Inode, name: String) -> Option<InodeId> {
    match &DATA.lock()[&inode.id] {
        Data::Dir(d) => {
            for e in d {
                if e.0 == name {
                    return Some(e.1);
                }
            }
            None
        }
        _ => None,
    }
}

/// Read a directory entry by index.
///
/// # Arguments
/// * `inode` – The directory inode.
/// * `offset` – The index of the entry to read.
///
/// # Returns
/// `Some((name, inode_id))` if the index is valid, `None` otherwise.
fn readdr(inode: &Inode, offset: usize) -> Option<(String, InodeId)> {
    match &DATA.lock()[&inode.id] {
        Data::Dir(d) => {
            for (i, e) in d.iter().enumerate() {
                if i == offset {
                    return Some(e.clone());
                }
            }
            None
        }
        _ => None,
    }
}

/// Read data from a file.
///
/// # Arguments
/// * `inode` – The file inode.
/// * `offset` – The byte offset to start reading from.
/// * `buf` – The buffer to fill with data.
///
/// # Returns
/// The number of bytes read, or an `Error`.
fn read(inode: &Inode, offset: usize, buf: &mut [u8]) -> Result<usize, Error> {
    let _ = DATA.lock();

    if is_file(inode.id) {
        let data = unsafe { DATA.inner() }.get(&inode.id).ok_or(Error::NoEntry)?;
        let dlen = match data {
            Data::File(d) => d.len(),
            _ => return Err(Error::NotAFile),
        };
        if offset >= dlen {
            return Err(Error::OutOfBounds);
        }
        let ulen = min(dlen - offset, buf.len());
        let data_vec = match data {
            Data::File(d) => d,
            _ => unreachable!(),
        };
        buf[..ulen].copy_from_slice(&data_vec[offset..(ulen + offset)]);
        Ok(ulen)
    } else {
        Err(Error::NotAFile)
    }
}

/// Write data to a file.
///
/// The file is extended if the offset is beyond the current end, filling
/// with zero bytes. The buffer is then written at the offset.
///
/// # Arguments
/// * `inode` – The file inode.
/// * `offset` – The byte offset to start writing at.
/// * `buf` – The data to write.
///
/// # Returns
/// The number of bytes written, or an `Error`.
fn write(inode: &Inode, offset: usize, buf: &[u8]) -> Result<usize, Error> {
    let _ = DATA.lock();

    if !is_file(inode.id) {
        return Err(Error::NotAFile);
    }

    // Get current data (must exist)
    let current_data = unsafe { DATA.inner() }.get_mut(&inode.id).ok_or(Error::NoEntry)?;
    let old_len = match current_data {
        Data::File(d) => d.len(),
        _ => return Err(Error::NotAFile),
    };

    // Determine new length
    let new_len = if offset + buf.len() > old_len {
        offset + buf.len()
    } else {
        old_len
    };

    // Create a new vector and copy old data + new data
    let mut new_data = vec![0u8; new_len];
    // Copy old data if any
    if old_len > 0 {
        let old = match current_data {
            Data::File(d) => d,
            _ => unreachable!(),
        };
        new_data[0..old_len].copy_from_slice(old);
    }
    // Write new data at offset
    new_data[offset..offset + buf.len()].copy_from_slice(buf);

    // Replace the data
    *current_data = Data::File(new_data);

    Ok(buf.len())
}

/// Truncate a file to a new size.
///
/// # Arguments
/// * `inode` – The file inode.
/// * `new_size` – The new size in bytes.
///
/// # Returns
/// `Ok(())` on success, or an `Error`.
/// For directories, returns `NotADirectory`.
fn trunc(inode: &Inode, new_size: usize) -> Result<(), Error> {
    let _ = DATA.lock();

    if !is_file(inode.id) {
        return Err(Error::NotADirectory);
    }

    let data = unsafe { DATA.inner() }.get_mut(&inode.id).ok_or(Error::NoEntry)?;
    match data {
        Data::File(d) => {
            if new_size > d.len() {
                d.resize(new_size, 0);
            } else {
                d.truncate(new_size);
            }
            Ok(())
        }
        _ => Err(Error::NotAFile),
    }
}

/// Remove a child entry from a directory.
///
/// This helper is called from `unlink` to remove the entry from the parent.
fn remove_child(inode: &mut Inode, child: InodeId) {
    if let Data::Dir(d) = unsafe { DATA.inner() }.get_mut(&inode.id).unwrap() {
        d.retain(|e| e.1 != child);
    }
}

/// Unlink (remove) an inode from its parent.
///
/// This removes the inode's data from the global `DATA` map and also removes
/// the entry from its parent directory.
fn unlink(inode: &mut Inode) -> Result<(), Error> {
    match DATA.lock().remove(&inode.id) {
        Some(_) => {
            if let Some(i) = inode.parent.get_mut() {
                remove_child(i, inode.parent);
            }
            Ok(())
        }
        _ => Err(Error::NoEntry),
    }
}

/// Add a new directory entry (link) in a parent directory.
///
/// # Arguments
/// * `inode` – The parent directory inode (must be a directory).
/// * `name` – The name of the new entry.
/// * `new` – The `InodeId` to link to.
///
/// # Returns
/// `Ok(())` on success, or an `Error`.
fn link(inode: &mut Inode, name: String, new: InodeId) -> Result<(), Error> {
    let _ = DATA.lock();

    if !is_file(inode.id) {
        // Must be a directory to add entries
        let data = unsafe { DATA.inner() }.get_mut(&inode.id).ok_or(Error::NoEntry)?;
        if let Data::Dir(d) = data {
            // Check if name already exists
            for (n, _) in d.iter() {
                if *n == name {
                    return Err(Error::Found);
                }
            }
            d.push((name, new));
            return Ok(());
        }
    }
    Err(Error::NotADirectory)
}

/// Create a new inode in the filesystem.
///
/// # Arguments
/// * `mb` – The `MetaBlock` representing this PVFS instance.
/// * `inode` – The inode to create (must have `kind` and `flags` set).
/// * `kind` – The kind of inode (only `File` and `Directory` are supported).
///
/// # Returns
/// The new `InodeId`, or an `Error`.
fn new(mb: &MetaBlock, mut inode: Inode, kind: Kind) -> Result<InodeId, Error> {
    let _ = mb.write();
    match kind {
        Kind::Directory => {
            DATA.lock().insert(inode.id, Data::Dir(vec![]));
        }
        Kind::File => {
            DATA.lock().insert(inode.id, Data::File(vec![]));
        }
        _ => return Err(Error::Unknown),
    }
    unsafe {
        let mbr = (
            core::ptr::addr_of!(
                *mb
                    .inner()
                    .data_mut::<PvfsMb>()
            )
            as *mut ()
            as *mut PvfsMb
        )
            .as_mut_unchecked();
        inode.id = InodeId(mbr.nxt, mb.inner().id);
        mbr.nxt += 1;
        Ok(inode.id)
    }
}

/// Get an immutable reference to an inode by its inode number.
///
/// # Arguments
/// * `mb` – The `MetaBlock` of the filesystem.
/// * `id` – The inode number.
///
/// # Returns
/// `Some(&Inode)` if the inode exists, `None` otherwise.
fn get(mb: &MetaBlock, id: u32) -> Option<&'static Inode> {
    let _ = mb.read();

    unsafe {
        (
            core::ptr::addr_of!(
                *mb
                    .inner()
                    .data_ref::<PvfsMb>()
            )
        )
            .as_ref_unchecked()
    }
        .reg
        .get(&id)
}

/// Get a mutable reference to an inode by its inode number.
///
/// # Arguments
/// * `mb` – The `MetaBlock` of the filesystem.
/// * `id` – The inode number.
///
/// # Returns
/// `Some(&mut Inode)` if the inode exists, `None` otherwise.
fn get_mut(mb: &MetaBlock, id: u32) -> Option<&'static mut Inode> {
    let _ = mb.read();

    unsafe {
        (
            core::ptr::addr_of!(
                *mb
                    .inner()
                    .data_mut::<PvfsMb>()
            )
            as *mut ()
            as *mut PvfsMb
        )
            .as_mut_unchecked()
    }
        .reg
        .get_mut(&id)
}

// ============================================================================
// FILESYSTEM INSTANCE STRUCTURE
// ============================================================================

/// Per‑instance state for a PVFS mount.
///
/// This structure is stored in the `MetaBlock`'s `data` field.
pub struct PvfsMb {
    /// Map from inode number to inode metadata.
    reg: BTreeMap<u32, Inode>,
    /// Next available inode number.
    nxt: u32,
}

impl PvfsMb {
    /// Creates a new, empty PVFS instance.
    pub fn new() -> Self {
        Self {
            reg: BTreeMap::new(),
            nxt: 0,
        }
    }
}

// ============================================================================
// VTABLE EXPORT
// ============================================================================

/// The virtual function table for PVFS.
///
/// This static table provides all the operation pointers required by the VFS
/// to interact with a PVFS instance.
pub static PVFS_VTABLE: FsVtable = FsVtable {
    lookup,
    readdr,
    read,
    write,
    trunc,
    unlink,
    link,
    new,
    get,
    get_mut,
};

```

### `src/vfs/err.rs`

```rs
//! # VFS Error Definitions
//!
//! This module defines the error types used throughout the Virtual File System
//! subsystem. All VFS operations return `Result<T, Error>`, where `Error` is
//! an enum describing the various failure conditions that can occur during
//! filesystem operations.
//!
//! ## Error Categories
//!
//! The errors are designed to cover the most common filesystem operation failures:
//!
//! - **Lookup failures**: `NoEntry` – the requested file or directory does not exist.
//! - **Type mismatches**: `NotAFile`, `NotADirectory` – operation attempted on
//!   the wrong kind of inode.
//! - **Bounds errors**: `OutOfBounds` – read/write offset is beyond the file size.
//! - **Name conflicts**: `Found` – attempting to create an entry that already exists.
//! - **Generic failures**: `Unknown` – catch‑all for unexpected errors.
//!
//! ## Usage
//!
//! All VFS functions in the `vfs` module return `Result` with this `Error` type.
//! Filesystem implementations (`FsVtable` functions) also use this error type
//! to report failures to the VFS layer.
//!
//! ## Safety
//!
//! The `Error` enum is `#[repr(usize)]` and can be safely cast to/from `usize`
//! for FFI purposes (e.g., returning error codes to userspace). The discriminant
//! values are stable and should not be changed without careful consideration.

// ============================================================================
// ERROR ENUM
// ============================================================================

/// VFS operation error codes.
///
/// These are returned by all VFS functions when an operation cannot be completed.
/// The `#[repr(usize)]` attribute ensures that the variants have stable integer
/// values, suitable for FFI and system call interfaces.
#[repr(usize)]
#[derive(Debug)]
pub enum Error {
    /// An unknown or unspecified error occurred.
    ///
    /// This is a catch‑all for unexpected conditions that do not fit into
    /// other categories. It should be used sparingly; prefer more specific
    /// error types when possible.
    Unknown,

    /// The operation was attempted on a file, but the inode is not a regular file.
    ///
    /// For example, attempting to `read` or `write` on a directory will return
    /// this error.
    NotAFile,

    /// A read or write operation attempted to access an offset beyond the
    /// end of the file.
    ///
    /// This can also occur when truncating a file to a size larger than the
    /// current allocation (some filesystems may extend the file instead).
    OutOfBounds,

    /// The requested entry was not found in the directory or filesystem.
    ///
    /// This is typically returned by `lookup` when a name does not exist,
    /// or by `unlink` when the inode to remove is not present.
    NoEntry,

    /// The operation was attempted on a directory, but the inode is not a directory.
    ///
    /// For example, attempting to `link` or `lookup` on a regular file will
    /// return this error.
    NotADirectory,

    /// An entry with the same name already exists in the directory.
    ///
    /// This is returned by `link` or `add_root` when attempting to create
    /// an entry with a name that is already taken.
    Found,
}

```

### `src/vfs/mb.rs`

```rs
//! # VFS MetaBlock and Function Table
//!
//! This module defines the core abstractions that bind a filesystem implementation
//! to the VFS layer: the **MetaBlock** and the **FsVtable**.
//!
//! ## Overview
//!
//! The VFS is designed to be filesystem‑agnostic. To achieve this, each filesystem
//! instance (mount point) is represented by a `MetaBlock`, which is essentially a
//! container that holds:
//!
//! - A unique identifier (`id`).
//! - A pointer to a static **vtable** (`FsVtable`) containing function pointers
//!   for all filesystem operations.
//! - An opaque `data` pointer that points to the filesystem‑specific state
//!   (e.g., `PvfsMb` for the purely virtual filesystem, or a block device cache
//!   for a disk‑based filesystem).
//!
//! ## MetaBlock
//!
//! `MetaBlock` is a type alias for `RwLock<MetaBlockInner>`. This provides:
//! - **Read‑write locking**: Allows concurrent reads and exclusive writes, safe
//!   for multi‑CPU access.
//! - **Interior mutability**: The vtable and data can be accessed via the lock.
//!
//! ### MetaBlockInner
//!
//! The inner structure contains:
//! - `id: u32` – A unique identifier assigned by the global registry (`MBLK_REG`).
//!   This ID is used in `InodeId` to locate the filesystem.
//! - `vtable: &'static FsVtable` – A reference to the filesystem's operation table.
//! - `data: usize` – An opaque pointer to the filesystem's private state. This
//!   is typically a `*mut` to a struct like `PvfsMb`.
//!
//! The `data` field can be accessed via `data_ref::<T>()` and `data_mut::<T>()`,
//! which are safe methods that cast the pointer to the appropriate type.
//!
//! ## FsVtable
//!
//! The `FsVtable` is a struct of ten function pointers that every filesystem must
//! implement:
//!
//! - **`lookup`**: Find a child inode by name in a directory.
//! - **`readdr`**: Read a directory entry by index (for listing).
//! - **`read`**: Read data from a file at a given offset.
//! - **`write`**: Write data to a file at a given offset.
//! - **`trunc`**: Truncate a file to a new size.
//! - **`unlink`**: Remove an inode from its parent directory and free its data.
//! - **`link`**: Add a new directory entry in a parent directory.
//! - **`new`**: Create a new inode (file or directory) in the filesystem.
//! - **`get`**: Retrieve an immutable reference to an inode by its number.
//! - **`get_mut`**: Retrieve a mutable reference to an inode by its number.
//!
//! Each function pointer has a specific signature that takes the `MetaBlock` (or
//! `Inode`) and appropriate arguments, returning `Option`, `Result`, or the
//! requested data.
//!
//! ## Creating a MetaBlock
//!
//! The `new_mblock(id, vtab, data)` function creates a new `MetaBlock` instance.
//! It is typically called by a filesystem's initialization routine after creating
//! the private state. The resulting `MetaBlock` is then registered with the global
//! registry via `vfs::reg_mblk()`.
//!
//! ## Safety
//!
//! The `data` pointer in `MetaBlockInner` is `usize` and is cast to a concrete
//! type in `data_ref` and `data_mut`. The caller must ensure:
//! - The pointer is valid and points to the correct type.
//! - The pointer is not used after the filesystem instance is destroyed.
//! - The `data` is properly synchronized (the `MetaBlock` lock protects access).
//!
//! The `FsVtable` is `static` and must be immutable. All function pointers
//! should be safe to call concurrently (they must handle their own locking).
//!
//! ## Locking Strategy
//!
//! `MetaBlock` is a `RwLock`. This means:
//! - Multiple readers can access the vtable and data simultaneously.
//! - A writer has exclusive access, typically used when modifying the filesystem's
//!   internal state (e.g., when creating or deleting inodes).
//! - The lock is held for the duration of the operation, ensuring consistency.

use core::ptr::addr_of_mut;

use alloc::string::String;

use crate::{sync::RwLock, vfs::{Error, Inode, InodeId, Kind}};

// ============================================================================
// FILESYSTEM VIRTUAL TABLE
// ============================================================================

/// The virtual function table for a filesystem.
///
/// This structure contains function pointers for all operations that a filesystem
/// must support. Each filesystem provides a static instance of this table with
/// its own implementations.
///
/// # Operation Signatures
///
/// - **lookup**: `fn(&Inode, String) -> Option<InodeId>`
///   Searches for an entry by name in a directory inode.
///
/// - **readdr**: `fn(&Inode, usize) -> Option<(String, InodeId)>`
///   Reads a directory entry by index (0‑based).
///
/// - **read**: `fn(&Inode, usize, &mut [u8]) -> Result<usize, Error>`
///   Reads data from a file at a given offset into a buffer.
///
/// - **write**: `fn(&Inode, usize, &[u8]) -> Result<usize, Error>`
///   Writes data to a file at a given offset from a buffer.
///
/// - **trunc**: `fn(&Inode, usize) -> Result<(), Error>`
///   Truncates a file to a new size (or extends with zeros).
///
/// - **unlink**: `fn(&mut Inode) -> Result<(), Error>`
///   Removes an inode from its parent directory and frees resources.
///
/// - **link**: `fn(&mut Inode, String, InodeId) -> Result<(), Error>`
///   Adds a new entry in a directory inode.
///
/// - **new**: `fn(&MetaBlock, Inode, Kind) -> Result<InodeId, Error>`
///   Creates a new inode in the filesystem.
///
/// - **get**: `fn(&MetaBlock, u32) -> Option<&'static Inode>`
///   Returns an immutable reference to an inode by its number.
///
/// - **get_mut**: `fn(&MetaBlock, u32) -> Option<&'static mut Inode>`
///   Returns a mutable reference to an inode by its number.
#[repr(C)]
pub struct FsVtable {
    pub lookup  : fn (inode: &    Inode, name    : String                ) -> Option<InodeId>,
    pub readdr  : fn (inode: &    Inode, offset  : usize                 ) -> Option<(String, InodeId)>,
    pub read    : fn (inode: &    Inode, offset  : usize , buf: &mut [u8]) -> Result<usize  , Error>,
    pub write   : fn (inode: &    Inode, offset  : usize , buf: &    [u8]) -> Result<usize  , Error>,
    pub trunc   : fn (inode: &    Inode, new_size: usize                 ) -> Result<()     , Error>,
    pub unlink  : fn (inode: &mut Inode,                                 ) -> Result<()     , Error>,
    pub link    : fn (inode: &mut Inode, name    : String, new: InodeId  ) -> Result<()     , Error>,
    pub new     : fn (mb: &MetaBlock, inode: Inode, kind    : Kind      ) -> Result<InodeId, Error>,
    pub get     : fn (mb: &MetaBlock, id   :      u32                    ) -> Option<&'static Inode>,
    pub get_mut : fn (mb: &MetaBlock, id   :      u32                    ) -> Option<&'static mut Inode>,
}

// ============================================================================
// META BLOCK INNER
// ============================================================================

/// The inner data of a `MetaBlock`.
///
/// This structure is stored inside the `RwLock` and holds the filesystem's
/// identifier, vtable, and opaque state pointer.
#[derive(Clone)]
pub struct MetaBlockInner {
    /// The unique identifier of this filesystem instance.
    /// This ID is used in `InodeId` to reference this filesystem.
    pub id: u32,

    /// The virtual function table that implements all filesystem operations.
    vtable: &'static FsVtable,

    /// Opaque pointer to the filesystem‑specific private state.
    /// This is typically a `*mut` to a struct like `PvfsMb`.
    data: usize,
}

impl MetaBlockInner {
    /// Returns an immutable reference to the filesystem's private data.
    ///
    /// # Type Parameters
    /// * `T` – The type of the private data structure.
    ///
    /// # Safety
    /// The caller must ensure that the `data` pointer is valid and points to
    /// an instance of `T`. This method performs a cast from `usize` to `*const T`
    /// and dereferences it.
    pub fn data_ref<T>(&self) -> &T {
        unsafe {
            (self.data as *const T).as_ref_unchecked()
        }
    }

    /// Returns a mutable reference to the filesystem's private data.
    ///
    /// # Type Parameters
    /// * `T` – The type of the private data structure.
    ///
    /// # Safety
    /// The caller must ensure that the `data` pointer is valid and points to
    /// an instance of `T`. This method performs a cast from `usize` to `*mut T`
    /// and dereferences it.
    pub fn data_mut<T>(&mut self) -> &mut T {
        unsafe {
            (self.data as *mut T).as_mut_unchecked()
        }
    }

    /// Returns a reference to the vtable.
    #[inline]
    pub fn vtable(&self) -> &'static FsVtable {
        self.vtable
    }
}

// ============================================================================
// META BLOCK TYPE ALIAS
// ============================================================================

/// A filesystem instance (mount point), protected by a read‑write lock.
///
/// `MetaBlock` is a type alias for `RwLock<MetaBlockInner>`. It provides safe
/// concurrent access to the filesystem's state and operations.
pub type MetaBlock = RwLock<MetaBlockInner>;

// ============================================================================
// META BLOCK CONSTRUCTOR
// ============================================================================

/// Creates a new `MetaBlock` instance.
///
/// # Arguments
/// * `id` – The initial ID (will be overwritten by the registry).
/// * `vtab` – A static reference to the filesystem's vtable.
/// * `data` – A mutable reference to the filesystem's private state.
///   This is typically a `&mut` to a struct like `PvfsMb`.
///
/// # Returns
/// A new `MetaBlock` (i.e., a `RwLock<MetaBlockInner>`).
///
/// # Example
/// ```ignore
/// let mut pvfs = PvfsMb::new();
/// let mblock = new_mblock(0, &PVFS_VTABLE, &mut pvfs);
/// let id = reg_mblk(mblock);
/// ```
///
/// # Safety
/// The `data` reference is converted to a raw pointer. The caller must ensure
/// that the data remains valid for the lifetime of the `MetaBlock`.
pub fn new_mblock(id: u32, vtab: &'static FsVtable, data: &mut ()) -> MetaBlock {
    MetaBlock::new(MetaBlockInner {
        id,
        vtable: vtab,
        data: addr_of_mut!(*data) as usize,
    })
}

```

### `drafts/fb.rs`

```rs
use crate::driverkit::*;
use alloc::boxed::Box;

const FB_GET_INFO: MethodId = interface!(b"fb.get_info");
const FB_CLEAR   : MethodId = interface!(b"fb.clear");
const FB_PLOT    : MethodId = interface!(b"fb.plot");

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FbInfo {
    pub address: usize,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PlotArgs {
    pub x: u32,
    pub y: u32,
    pub color: u32,
}

pub struct FbState {
    pub info: FbInfo,
}

driver_method! {
    fb_get_info_method(dev):

    match crate::dev::get_driver_data(dev) {
        Some(x) => Ok(x as usize),
        None => Err(DeviceStatus::NotFound),
    }
}

driver_method! {
    fb_clear_method(dev, color: &usize):

    let state_ptr = fb_get_info_method(dev, 0).as_result()? as *const FbState;

    let state = unsafe { &*state_ptr };

    if state.info.bpp != 32 { return Err(DeviceStatus::Unsupported); }

    let pixels = (state.info.width * state.info.height) as usize;
    let ptr = state.info.address as *mut u32;

    for i in 0..pixels {
        unsafe { core::ptr::write_volatile(ptr.add(i), *color as u32); }
    }

    Ok(pixels)
}

driver_method! {
    fb_plot_method(dev, args: &PlotArgs):

    let state_ptr = fb_get_info_method(dev, 0).as_result()? as *const FbState;

    let state = unsafe { &*state_ptr };

    if args.x >= state.info.width || args.y >= state.info.height {
        return Err(DeviceStatus::InvalidArg);
    }
    if state.info.bpp != 32 { return Err(DeviceStatus::Unsupported); }

    let offset = ((args.y * state.info.pitch) + (args.x * 4)) as usize;
    let ptr = (state.info.address + offset) as *mut u32;
    unsafe { core::ptr::write_volatile(ptr, args.color); }

    Ok(0usize)
}

limine! { FBR <= FramebufferRequest }

pub fn probe() -> Option<DeviceId> {
    info!("Probing Limine Framebuffer...");
    let response = FBR.response()?;
    let fb = response.framebuffers().get(0)?;

    info!("Found {}x{} @ {:#X}", fb.width, fb.height, fb.address() as usize);

    let mut dev = Device::new("fb0");
    
    dev.add_method(FB_GET_INFO, fb_get_info_method);
    dev.add_method(FB_CLEAR, fb_clear_method);
    dev.add_method(FB_PLOT, fb_plot_method);
    
    let state = Box::new(FbState {
        info: FbInfo {
            address: fb.address() as usize,
            width: fb.width as u32,
            height: fb.height as u32,
            pitch: fb.pitch as u32,
            bpp: fb.bpp as u8,
        },
    });
    
    dev.driver_data = Box::into_raw(state) as usize;
    
    let dev_id = crate::dev::register_device(dev)?;
    
    info!("Registered with ID {:?}", dev_id);
    Some(dev_id)
}

```

### `drafts/driverkit.rs`

```rs
pub use crate::dev::*;

#[macro_export]
macro_rules! driver_method {
    (
        $(#[$meta:meta])*
        $name:ident($dev:ident, $args:ident: &$arg_ty:ty): $($body:tt)*
    ) => {
        $(#[$meta])*
        extern "C" fn $name(__raw_dev: $crate::dev::DeviceId, __raw_arg: usize) -> crate::dev::DeviceResult {
            fn ___($dev: $crate::dev::DeviceId, $args: &$arg_ty) -> Result<usize, crate::dev::DeviceStatus> {
                $($body)*
            }
            crate::dev::DeviceResult::from_result(___(__raw_dev, unsafe { &*(__raw_arg as *const $arg_ty) }))
        }
    };
    (
        $(#[$meta:meta])*
        $name:ident($dev:ident): $($body:tt)*
    ) => {
        $(#[$meta])*
        extern "C" fn $name(__raw_dev: $crate::dev::DeviceId, __raw_arg: usize) -> crate::dev::DeviceResult {
            fn ___($dev: $crate::dev::DeviceId) -> Result<usize, crate::dev::DeviceStatus> {
                $($body)*
            }
            crate::dev::DeviceResult::from_result(___(__raw_dev))
        }
    };
}

#[macro_export]
macro_rules! call_dev_method {
    ($dev_id:expr, $iface:expr, $args:expr) => {
        {
            let __args_ref = &($args);
            $crate::dev::call_method($dev_id, $iface, __args_ref as *const _ as usize)
        }
    };
}

#[macro_export]
macro_rules! interface {
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

```
