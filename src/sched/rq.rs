//! # EEVDF Runqueue with NVDL & SMP Support
use alloc::{boxed::Box, collections::{BTreeMap, BTreeSet}};
use super::task::{Task, TaskId};
use crate::{sched::task::TaskState, sync::Nitex}; // Changed to Nutex

const NICE_0_WEIGHT: u64 = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TaskKey {
    deadline: u64,
    vruntime: u64,
    id: TaskId,
}

impl Ord for TaskKey {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.deadline.cmp(&other.deadline)
            .then_with(|| self.vruntime.cmp(&other.vruntime))
            .then_with(|| self.id.0.cmp(&other.id.0))
    }
}
impl PartialOrd for TaskKey {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> { Some(self.cmp(other)) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RtTaskKey {
    deadline: u64,
    id: TaskId,
}
impl Ord for RtTaskKey {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.deadline.cmp(&other.deadline).then_with(|| self.id.0.cmp(&other.id.0))
    }
}
impl PartialOrd for RtTaskKey {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> { Some(self.cmp(other)) }
}

pub struct Runqueue {
    tasks: BTreeMap<TaskId, Box<Task>>,
    by_deadline: BTreeSet<TaskKey>,
    rt_tasks: BTreeSet<RtTaskKey>, 
    min_vruntime: u64,
    current: Option<TaskId>,
    load: u64,
}

const impl Default for Runqueue {
    fn default() -> Self {
        Self {
            tasks: BTreeMap::new(),
            by_deadline: BTreeSet::new(),
            rt_tasks: BTreeSet::new(),
            min_vruntime: 0,
            current: None,
            load: 0,
        }
    }
}

impl Runqueue {
    pub const fn new() -> Self { Self::default() }

    pub fn tasks(&self) -> &BTreeMap<TaskId, Box<Task>> { &self.tasks }
    pub fn tasks_mut(&mut self) -> &mut BTreeMap<TaskId, Box<Task>> { &mut self.tasks }
    pub fn current_task(&self) -> Option<&Task> { self.current.and_then(|id| self.tasks.get(&id)).map(|b| b.as_ref()) }
    pub fn set_current(&mut self, id: TaskId) { self.current = Some(id); }
    pub fn current_task_id(&self) -> Option<TaskId> { self.current }
    pub fn clear_current(&mut self) { self.current = None; }
    pub fn load(&self) -> u64 { self.load }
    pub fn min_vruntime(&self) -> u64 { self.min_vruntime }
    
    pub fn runnable_count(&self) -> usize {
        self.tasks.values().filter(|t| t.state == TaskState::Runnable).count()
    }

    pub fn insert(&mut self, task: Box<Task>) {
        let t = task;
        if t.rt_deadline > 0 {
            self.rt_tasks.insert(RtTaskKey { deadline: t.rt_deadline, id: t.id });
        }
        let key = TaskKey { deadline: t.deadline, vruntime: t.vruntime, id: t.id };
        self.load += t.weight;
        self.by_deadline.insert(key);
        self.tasks.insert(t.id, t);
    }

    pub fn remove(&mut self, id: TaskId) -> Option<Box<Task>> {
        if let Some(task) = self.tasks.remove(&id) {
            if task.rt_deadline > 0 {
                self.rt_tasks.remove(&RtTaskKey { deadline: task.rt_deadline, id: task.id });
            }
            let key = TaskKey { deadline: task.deadline, vruntime: task.vruntime, id: task.id };
            self.by_deadline.remove(&key);
            if self.load > task.weight {
                self.load -= task.weight;
            } else {
                self.load = 0;
            }
            Some(task)
        } else { None }
    }

    pub fn spawn_task(&mut self, mut task: Box<Task>) {
        task.vruntime = self.min_vruntime;
        task.deadline = task.vruntime + task.slice;
        self.insert(task);
    }

    pub fn sleep_task(&mut self, id: TaskId) {
        if let Some(mut task) = self.tasks.remove(&id) {
            // 1. Remove from EEVDF tree
            let key = TaskKey { deadline: task.deadline, vruntime: task.vruntime, id: task.id };
            self.by_deadline.remove(&key);
            if task.weight < self.load {
                self.load -= task.weight;
            } else {
                self.load = 0;
            }
            
            // 2. Remove from RT tree if applicable
            if task.rt_deadline > 0 {
                self.rt_tasks.remove(&RtTaskKey { deadline: task.rt_deadline, id: task.id });
            }
            
            task.state = TaskState::Sleeping;
            
            // 3. Put it back in the map, but NOT in the scheduling trees
            self.tasks.insert(id, task);
        }
    }

    pub fn wakeup_task(&mut self, id: TaskId) {
        if let Some(mut task) = self.tasks.remove(&id) {
            task.state = TaskState::Runnable;
            
            // EEVDF Lag compensation
            if task.rt_deadline == 0 && task.vruntime < self.min_vruntime {
                let lag = self.min_vruntime - task.vruntime;
                let decayed_lag = (lag >> 1).min(task.slice);
                task.vruntime = self.min_vruntime - decayed_lag;
            }
            
            // Reset deadline if it expired or was uninitialized
            if task.deadline == 0 || task.vruntime >= task.deadline {
                task.deadline = task.vruntime + task.slice;
            }
            
            // Re-insert into scheduling trees via `insert()`
            self.insert(task);
        }
    }

    pub fn update_vruntime(&mut self, delta_ms: u64) {
        if let Some(curr_id) = self.current {
            // CRITICAL FIX: Check if the task is actually in the scheduling trees.
            // If a task called `sleep()`, it was already removed from `by_deadline` 
            // and `self.load` by `sleep_task()`, but it remains `self.current` until 
            // the context switch in `reschedule()` completes.
            let is_in_trees = self.tasks.get(&curr_id).is_some_and(|t| {
                t.state != TaskState::Sleeping && t.state != TaskState::Zombie
            });

            if is_in_trees && let Some(mut task) = self.remove(curr_id) {
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
        if let Some(min_key) = self.by_deadline.iter().min_by_key(|k| k.vruntime)
            && min_key.vruntime > self.min_vruntime {
            self.min_vruntime = min_key.vruntime;
        }
    }

    pub fn pick_next(&mut self) -> Option<TaskId> {
        for key in self.rt_tasks.iter() {
            if let Some(task) = self.tasks.get(&key.id)
            && task.state == TaskState::Runnable {
                let now = crate::arch::get_time_from_boot();
                if now > key.deadline && key.deadline > 0 {
                    crate::warn!("NVDL: RT task {} missed deadline!", key.id.0);
                }
                return Some(key.id);
            }
        }

        if self.by_deadline.is_empty() { return None; }
        
        for key in self.by_deadline.iter() {
            if let Some(task) = self.tasks.get(&key.id)
                && task.state == TaskState::Runnable && key.vruntime <= self.min_vruntime {
                return Some(key.id);
            }
        }
        
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

// Changed from Litex to Nutex to allow safe cross-CPU locking for SMP balancing!
pub static RUNQUEUES: [Nitex<Runqueue>; crate::arch::MAX_CPUS]
=   [const { Nitex::new(Runqueue::new()) }; crate::arch::MAX_CPUS];
