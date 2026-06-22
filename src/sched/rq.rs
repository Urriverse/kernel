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
