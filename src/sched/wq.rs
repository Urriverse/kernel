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
    #[allow(dead_code)]
    pub fn wakeup_all(&mut self) -> alloc::vec::Vec<TaskId> {
        self.waiters.drain(..).collect()
    }

    /// Returns `true` if the wait queue is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.waiters.is_empty()
    }
}
