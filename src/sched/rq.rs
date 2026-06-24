//! # EEVDF Runqueue with NVDL (Non-Virtual Deadline) Support
//!
//! This module implements the per‑CPU runqueue for the EEVDF (Earliest Eligible
//! Virtual Deadline First) scheduler, extended with a strict Earliest Deadline First 
//! (EDF) "Green Corridor" for real-time tasks.

use alloc::{boxed::Box, collections::{BTreeMap, BTreeSet}};
use super::task::{Task, TaskId};
use crate::{sched::task::TaskState, sync::Nitex};

// ============================================================================
// CONSTANTS
// ============================================================================
/// Weight for a task with niceness 0 (baseline).
const NICE_0_WEIGHT: u64 = 1024;

// ============================================================================
// TASK KEY (for ordering normal tasks by virtual deadline)
// ============================================================================
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
// NVDL: REAL-TIME TASK KEY (EDF Ordering)
// ============================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RtTaskKey {
    deadline: u64,
    id: TaskId,
}

impl Ord for RtTaskKey {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.deadline
            .cmp(&other.deadline)
            .then_with(|| self.id.0.cmp(&other.id.0))
    }
}

impl PartialOrd for RtTaskKey {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// ============================================================================
// RUNQUEUE STRUCTURE
// ============================================================================
pub struct Runqueue {
    tasks: BTreeMap<TaskId, Box<Task>>,
    by_deadline: BTreeSet<TaskKey>,
    rt_tasks: BTreeSet<RtTaskKey>, 
    min_vruntime: u64,
    current: Option<TaskId>,
    load: u64,
}

impl Runqueue {
    pub const fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            by_deadline: BTreeSet::new(),
            rt_tasks: BTreeSet::new(),
            min_vruntime: 0,
            current: None,
            load: 0,
        }
    }

    // ========================================================================
    // ACCESSORS
    // ========================================================================
    pub fn tasks(&self) -> &BTreeMap<TaskId, Box<Task>> { &self.tasks }
    pub fn tasks_mut(&mut self) -> &mut BTreeMap<TaskId, Box<Task>> { &mut self.tasks }

    #[allow(dead_code)]
    pub fn current_task(&self) -> Option<&Task> {
        self.current.and_then(|id| self.tasks.get(&id)).map(|b| b.as_ref())
    }

    #[allow(dead_code)]
    pub fn current_task_mut(&mut self) -> Option<&mut Task> {
        self.current.and_then(|id| self.tasks.get_mut(&id)).map(|b| b.as_mut())
    }

    pub fn set_current(&mut self, id: TaskId) { self.current = Some(id); }
    pub fn current_task_id(&self) -> Option<TaskId> { self.current }
    pub fn clear_current(&mut self) { self.current = None; }

    #[allow(dead_code)] pub fn load(&self) -> u64 { self.load }
    #[allow(dead_code)] pub fn len(&self) -> usize { self.tasks.len() }
    #[allow(dead_code)] pub fn is_empty(&self) -> bool { self.tasks.is_empty() }

    // ========================================================================
    // TASK INSERTION AND REMOVAL
    // ========================================================================
    /// Internal insert. Assumes vruntime and deadline are already correctly set.
    fn insert(&mut self, task: Box<Task>) {
        let t = task;
        
        if t.rt_deadline > 0 {
            self.rt_tasks.insert(RtTaskKey { deadline: t.rt_deadline, id: t.id });
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

    pub fn remove(&mut self, id: TaskId) -> Option<Box<Task>> {
        if let Some(task) = self.tasks.remove(&id) {
            if task.rt_deadline > 0 {
                self.rt_tasks.remove(&RtTaskKey { deadline: task.rt_deadline, id: task.id });
            }

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
    // SPAWN, SLEEP, AND WAKEUP (ECO-FRIENDLY LAG DECAY)
    // ========================================================================
    /// Spawns a new task. Sets its vruntime to the current min_vruntime so it 
    /// doesn't get a massive, system-stalling boost upon creation.
    pub fn spawn_task(&mut self, mut task: Box<Task>) {
        task.vruntime = self.min_vruntime;
        task.deadline = task.vruntime + task.slice;
        self.insert(task);
    }

    /// Moves a task to the Sleeping state.
    /// Crucially, it removes the task from `by_deadline` so it doesn't freeze `min_vruntime`.
    pub fn sleep_task(&mut self, id: TaskId) {
        if let Some(mut task) = self.tasks.remove(&id) {
            let key = TaskKey {
                deadline: task.deadline,
                vruntime: task.vruntime,
                id: task.id,
            };
            self.by_deadline.remove(&key);
            self.load -= task.weight;
            
            if task.rt_deadline > 0 {
                self.rt_tasks.remove(&RtTaskKey { deadline: task.rt_deadline, id: task.id });
            }
            
            task.state = TaskState::Sleeping;
            // Keep in `tasks` map for lookups, but out of scheduling trees
            self.tasks.insert(id, task);
        }
    }

    /// Wakes up a task and applies Eco-Friendly Lag Decay.
    pub fn wakeup_task(&mut self, id: TaskId) {
        if let Some(mut task) = self.tasks.remove(&id) {
            task.state = TaskState::Runnable;
            
            // LAG DECAY:
            // If the task slept for a long time, it accumulated massive lag (credit).
            // Giving it all that credit causes long, power-hungry execution bursts.
            // We halve the lag and cap it at a single time slice to be "eco-friendly".
            if task.rt_deadline == 0 && task.vruntime < self.min_vruntime {
                let lag = self.min_vruntime - task.vruntime;
                let decayed_lag = (lag >> 1).min(task.slice);
                task.vruntime = self.min_vruntime - decayed_lag;
            }
            
            if task.deadline == 0 || task.vruntime >= task.deadline {
                task.deadline = task.vruntime + task.slice;
            }

            self.insert(task);
        }
    }

    // ========================================================================
    // VIRTUAL RUNTIME UPDATE (GREEN CORRIDOR)
    // ========================================================================
    pub fn update_vruntime(&mut self, delta_ms: u64) {
        if let Some(curr_id) = self.current {
            if let Some(mut task) = self.remove(curr_id) {
                if task.rt_deadline == 0 {
                    let delta_vruntime = (delta_ms as u128 * NICE_0_WEIGHT as u128 / task.weight as u128) as u64;
                    task.vruntime += delta_vruntime;
                    if task.vruntime >= task.deadline {
                        task.deadline = task.vruntime + task.slice;
                    }
                }
                self.insert(task);
            }
        }
        self.advance_min_vruntime();
    }

    fn advance_min_vruntime(&mut self) {
        // Because sleeping tasks are no longer in `by_deadline`, this correctly
        // advances the virtual clock based ONLY on runnable tasks.
        if let Some(min_key) = self.by_deadline.iter().min_by_key(|k| k.vruntime)
            && min_key.vruntime > self.min_vruntime {
            self.min_vruntime = min_key.vruntime;
        }
    }

    // ========================================================================
    // TASK SELECTION (NVDL + EEVDF)
    // ========================================================================
    pub fn pick_next(&mut self) -> Option<TaskId> {
        // 1. NVDL: Check Real-Time tasks (Earliest Deadline First)
        for key in self.rt_tasks.iter() {
            if let Some(task) = self.tasks.get(&key.id) {
                if task.state == TaskState::Runnable {
                    let now = crate::arch::get_time_from_boot();
                    if now > key.deadline && key.deadline > 0 {
                        crate::warn!("NVDL: RT task {} missed deadline! (now: {}, deadline: {})", key.id.0, now, key.deadline);
                    }
                    return Some(key.id);
                }
            }
        }

        // 2. EEVDF: Normal tasks
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

    // ========================================================================
    // NVDL API
    // ========================================================================
    pub fn set_rt_deadline(&mut self, id: TaskId, new_deadline: u64) {
        if let Some(task) = self.tasks.get_mut(&id) {
            let old_deadline = task.rt_deadline;
            if old_deadline > 0 {
                self.rt_tasks.remove(&RtTaskKey { deadline: old_deadline, id });
            }
            task.rt_deadline = new_deadline;
            if new_deadline > 0 {
                self.rt_tasks.insert(RtTaskKey { deadline: new_deadline, id });
            }
        }
    }
}

// ============================================================================
// PER‑CPU RUNQUEUES
// ============================================================================
pub static RUNQUEUES: [Nitex<Runqueue>; crate::arch::MAX_CPUS]
=   [const { Nitex::new(Runqueue::new()) }; crate::arch::MAX_CPUS];
