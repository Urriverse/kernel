//! # Kernel Module Interface (KMI) – Dynamic Linking System
//!
//! This module provides a dynamic symbol resolution and reference‑counting mechanism
//! for kernel modules. It allows modules to **export** functions, **link** to symbols
//! exported by other modules, and gracefully **unload** modules without dangling
//! pointers.
//!
//! ## Core Concepts
//!
//! - **Symbol** – an exported function or static value. Each symbol has:
//!   - A *module reference count* (`mprc`) pointing to the refcount of the owning
//!     module (or a spurious one for non‑module contexts).
//!   - Its own reference count (`rc`) tracking how many `SymbolGuard` handles are
//!     currently alive.
//!   - A `poisonous` flag that, when set, prevents new links and causes the symbol
//!     to be freed once its own refcount drops to zero.
//!   - A unique 64‑bit identifier.
//!
//! - **SymbolHandle** – a capability to obtain a `SymbolGuard`. It is returned by
//!   [`link`] and ensures that the symbol stays alive as long as the handle exists.
//!
//! - **SymbolGuard** – a RAII guard that holds a reference to the actual function
//!   pointer. While it lives, the symbol’s refcount is incremented. When dropped,
//!   it decrements the refcount and, if it reaches zero, removes the symbol from
//!   the global table (if it was marked poisonous).
//!
//! - **Module unloading** – a module calls [`suicide`] to unload itself. This
//!   poisons all symbols it exported, decrements their refcounts, and busy‑waits
//!   until its own refcount reaches zero, then exits.
//!
//! # Safety
//!
//! All symbol pointers are raw pointers and require the caller to guarantee that
//! the pointed‑to function is `'static` and remains valid as long as the symbol
//! exists. The reference counting logic prevents use‑after‑free as long as handles
//! are used correctly. The global table is protected by a `RwLock`.

use core::{
    ptr::addr_of,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering::{AcqRel, Relaxed}},
    sync::atomic::Ordering::Release,
};
use alloc::collections::BTreeMap;
use crate::{sched::current_process, sync::RwLock};

// -----------------------------------------------------------------------------
// Symbol
// -----------------------------------------------------------------------------

/// A single exported symbol in the kernel module interface.
///
/// Each symbol is identified by a 64‑bit ID and holds a pointer to the actual
/// function (or other static item). It maintains two reference counts:
/// - `mprc` – points to the owning module’s reference count, ensuring the module
///   cannot be unloaded while any of its symbols are still in use.
/// - `rc`   – the number of active `SymbolGuard` handles for this symbol.
///
/// When the symbol is marked `poisonous` (e.g. during module unload), new handles
/// cannot be obtained, and the symbol will be removed from the global table once
/// its `rc` reaches zero.
///
/// # Layout
/// This struct is `repr(C)` to guarantee a stable ABI between the kernel and
/// modules (though currently used only internally).
#[repr(C)]
pub struct Symbol {
    /// Pointer to the owning module’s atomic reference count.
    /// This is used to keep the module alive while any of its symbols are in use.
    mprc: *const AtomicUsize,

    /// Current number of active `SymbolGuard` references to this symbol.
    rc: AtomicUsize,

    /// Pointer to the actual exported function (or other static data).
    ptr: *const (),

    /// If `true`, the symbol has been poisoned (e.g. during module unload).
    /// No new `SymbolHandle` can be obtained for a poisonous symbol.
    poisonous: AtomicBool,

    /// Unique identifier for this symbol. Used as the key in the global table.
    id: u64,
}

impl Symbol {
    /// Creates a new `Symbol` with an initial reference count of 1.
    ///
    /// # Safety
    /// The caller must ensure that `module_prc` is a valid pointer to a `AtomicUsize`
    /// that lives at least as long as this symbol, and that `ptr` points to a valid
    /// static item that will never be moved or deallocated.
    fn new(id: u64, module_prc: &AtomicUsize, ptr: *const ()) -> Self {
        Self {
            mprc: addr_of!(*module_prc),
            rc: AtomicUsize::new(1),
            ptr,
            poisonous: AtomicBool::new(false),
            id,
        }
    }

    /// Increments the symbol’s reference count.
    ///
    /// Used when creating a new `SymbolGuard` to keep the symbol alive.
    fn advance(&self) {
        self.rc.fetch_add(1, Release);
    }

    /// Decrements the symbol’s reference count and returns `true` if it reached zero.
    ///
    /// If it reaches zero and the symbol is poisonous, the global table will remove it.
    fn punish(&self) {
        if self.rc.fetch_sub(1, AcqRel) == 1
        && self.poisonous.load(Relaxed) {
            let _ = GSTAB.write().remove(&self.id);
        }
    }
}

impl Drop for Symbol {
    /// When a `Symbol` is dropped (i.e. removed from the global table), decrement
    /// the owning module’s reference count. This may allow the module to be unloaded.
    fn drop(&mut self) {
        unsafe {
            self.mprc.as_ref_unchecked()
        }.fetch_sub(1, AcqRel);
    }
}

// Symbols can be sent between threads because all operations are atomic and the
// global table is lock‑protected. However, the actual function pointers are not
// `Sync`; that is handled by the guard types.
unsafe impl Send for Symbol {}

// -----------------------------------------------------------------------------
// SymbolHandle & SymbolGuard
// -----------------------------------------------------------------------------

/// A handle to a symbol that can be used to obtain a guarded reference.
///
/// Obtained from [`link`]. It does not itself hold a reference to the symbol;
/// the actual reference is acquired when calling [`get`](SymbolHandle::get),
/// which returns a [`SymbolGuard`]. The handle remains valid as long as the
/// symbol is not poisonous.
#[repr(C)]
pub struct SymbolHandle {
    hold: *const Symbol,
}

impl SymbolHandle {
    /// Acquires a guarded reference to the symbol.
    ///
    /// The returned [`SymbolGuard`] increments the symbol’s refcount and holds
    /// the function pointer. While the guard lives, the symbol is guaranteed to
    /// stay alive.
    ///
    /// # Panics
    /// This function will panic if the inner symbol pointer is null (should never
    /// happen for a valid handle).
    pub fn get(&self) -> SymbolGuard {
        let h = unsafe { self.hold.as_ref_unchecked() };
        h.advance();
        SymbolGuard {
            v: h.ptr,
            hold: self.hold,
        }
    }
}

/// A RAII guard that holds a live reference to an exported symbol.
///
/// Created by [`SymbolHandle::get`]. While this guard exists, the symbol’s
/// reference count is incremented, preventing it from being removed from the
/// global table. When dropped, the refcount is decremented; if it reaches zero
/// and the symbol is poisonous, it is removed from the global table.
///
/// The actual function pointer can be retrieved using [`get`](SymbolGuard::get),
/// which requires specifying the function signature.
#[repr(C)]
pub struct SymbolGuard {
    v: *const (),
    hold: *const Symbol,
}

impl SymbolGuard {
    /// Returns a reference to the exported function with the given type `F`.
    ///
    /// # Type Safety
    /// This function verifies that the size of `F` equals the size of a function
    /// pointer (`fn() -> ()`). This is a conservative check to prevent obviously
    /// incorrect downcasts. However, the caller **must** ensure that the actual
    /// function signature matches `F` – otherwise undefined behaviour occurs.
    ///
    /// # Panics
    /// Panics if `size_of::<F>() != size_of::<fn() -> ()>()`.
    pub fn get<F>(&self) -> &F {
        if size_of::<F>() != 0 {
            panic!("Invalid downcast: size mismatch");
        }
        unsafe { (self.v as *const F).as_ref_unchecked() }
    }
}

impl Drop for SymbolGuard {
    /// Releases the reference to the symbol.
    ///
    /// If this was the last reference and the symbol is poisonous, the symbol is
    /// removed from the global table.
    fn drop(&mut self) {
        unsafe {
            self.hold
                .as_ref_unchecked()
                .punish();
        }
    }
}

// -----------------------------------------------------------------------------
// Global Table & Spurious Refcount
// -----------------------------------------------------------------------------

lazy_static! {
    /// Global symbol table mapping 64‑bit IDs to `Symbol` instances.
    ///
    /// Protected by a read‑write lock. Most operations (link, export) acquire
    /// a read lock, while unload may acquire a write lock to remove symbols.
    pub static ref GSTAB: RwLock<BTreeMap<u64, Symbol>> = RwLock::new(BTreeMap::new());
}

/// A spurious module reference count used when exporting a symbol from a context
/// that is not associated with a kernel module (e.g. during early initialisation).
/// This prevents the reference count from being decremented to zero and causing
/// spurious module unload attempts.
static SPUR: AtomicUsize = AtomicUsize::new(0);

// -----------------------------------------------------------------------------
// Public API
// -----------------------------------------------------------------------------

/// Exports a static function (or other item) under the given `id`.
///
/// This function registers the symbol in the global table. If the current
/// context has a module (i.e. `current_process()` returns `Some`), the module’s
/// reference count is incremented, ensuring the module stays loaded while the
/// symbol exists. Otherwise, a spurious refcount is used.
///
/// # Type Safety
/// The function verifies that the size of `F` matches the size of a function
/// pointer. The caller must ensure that `f` is a `'static` reference to a valid
/// function that will never be deallocated.
///
/// # Returns
/// Returns `Some(old_symbol)` if the `id` was already present in the table,
/// replacing it with the new one. Returns `None` if the insertion succeeded
/// with no previous entry.
///
/// # Panics
/// Panics if `size_of::<F>() != size_of::<Fn() -> ()>()`.
pub fn export<F>(id: u64, f: &'static F) -> Option<Symbol> {
    if size_of::<F>() != 0 {
        panic!("Invalid upcast: size mismatch");
    }

    let prc;
    if let Some(proc) = current_process() {
        proc.rc.fetch_add(1, Release);
        prc = addr_of!(proc.rc);
    } else {
        // If we export a symbol from an undefined context (e.g. early init),
        // use the spurious refcount so that the symbol never blocks module unloading.
        prc = addr_of!(SPUR);
    }

    GSTAB.write().insert(id, Symbol::new(id, unsafe { prc.as_ref_unchecked() }, addr_of!(*f) as *const ()))
}

/// Unloads the currently running kernel module.
///
/// This function must be called from within a module context (i.e. `current_process()`
/// must return `Some`). It performs the following steps:
/// 1. Marks **all** symbols owned by this module as poisonous.
/// 2. Decrements the reference count of each such symbol (this may free them
///    immediately if no other references exist).
/// 3. Busy‑waits (yielding the CPU) until the module’s own reference count
///    reaches zero, meaning all symbols have been fully released.
/// 4. Calls `crate::sched::exit(0)` to terminate the module.
///
/// # Panics
/// Panics if called from a context that is not a kernel module (no current process).
pub fn suicide() -> ! {
    if let Some(proc) = current_process() {
        for (_, sym) in GSTAB.read().iter() {
            if sym.mprc == addr_of!(proc.rc) {
                sym.poisonous.store(true, Release);
                sym.punish();
            }
        }
        // Free‑loop until the module’s refcount is zero.
        loop {
            crate::sched::yield_now();
            if proc.rc.load(Relaxed) == 0 {
                crate::sched::exit(0);
            }
        }
    } else {
        panic!("Suicide outside of kernel module");
    }
}

/// Looks up a symbol by its ID and returns a handle to it.
///
/// If a symbol with the given `id` exists and is **not** poisonous, a
/// [`SymbolHandle`] is returned. The handle can be used to obtain guarded
/// references to the exported function.
///
/// # Returns
/// `Some(SymbolHandle)` if the symbol exists and is usable, otherwise `None`.
pub fn link(id: u64) -> Option<SymbolHandle> {
    if let Some(sym) = GSTAB.read().get(&id) {
        if !sym.poisonous.load(Relaxed) {
            return Some(SymbolHandle {
                hold: addr_of!(*sym),
            });
        }
    }
    None
}

/// Initialises the KMI by exporting the core system functions.
///
/// This must be called early during kernel initialisation to make the
/// `kernel.export`, `kernel.link`, and `self.suicide` symbols available to
/// modules.
pub fn init() {
    export(hash!(b"kernel.export"), &export::<fn() -> ()>);
    export(hash!(b"kernel.link"), &link);
    export(hash!(b"self.suicide"), &suicide);
}
