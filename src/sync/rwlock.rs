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
//! The lock state is stored in an `AtomicU64`:
//! - **Writer bit**: The most significant bit (`1 << (u64::BITS - 1)`).
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

pub use ketypes::{
    KeRwLock as RwLock,
    KeRwLockReadGuard as RwLockReadGuard,
    KeRwLockWriteGuard as RwLockWriteGuard,
};
