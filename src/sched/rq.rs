use alloc::{boxed::Box, collections::{BTreeMap, BTreeSet}};
use super::task::{Task, TaskId};
use crate::{sched::task::TaskState, sync::Nitex};

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
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Runqueue {
    tasks: BTreeMap<TaskId, Box<Task>>,
    by_deadline: BTreeSet<TaskKey>,
    min_vruntime: u64,
    current: Option<TaskId>,
    load: u64,
}

impl Runqueue {
    pub const fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            by_deadline: BTreeSet::new(),
            min_vruntime: 0,
            current: None,
            load: 0,
        }
    }

    pub fn tasks(&self) -> &BTreeMap<TaskId, Box<Task>> {
        &self.tasks
    }

    // ИСПРАВЛЕНО: Убран transmute, теперь честный &mut self
    pub fn tasks_mut(&mut self) -> &mut BTreeMap<TaskId, Box<Task>> {
        &mut self.tasks
    }

    pub fn current_task(&self) -> Option<&Task> {
        self.current.and_then(|id| self.tasks.get(&id)).map(|b| b.as_ref())
    }

    pub fn current_task_mut(&mut self) -> Option<&mut Task> {
        self.current.and_then(|id| self.tasks.get_mut(&id)).map(|b| b.as_mut())
    }

    pub fn set_current(&mut self, id: TaskId) {
        self.current = Some(id);
    }

    pub fn update_vruntime(&mut self, delta_ms: u64) {
        if let Some(curr_id) = self.current {
            if let Some(mut task) = self.remove(curr_id) {
                // delta_ms - это реальное время в миллисекундах (обычно 10 мс)
                // vruntime += delta_ms * (NICE_0_WEIGHT / weight)
                let delta_vruntime = (delta_ms as u128 * NICE_0_WEIGHT as u128 / task.weight as u128) as u64;
                task.vruntime += delta_vruntime;
                
                // Update deadline if slice exhausted
                if task.vruntime >= task.deadline {
                    task.deadline = task.vruntime + task.slice;
                }
                self.insert(task);
            }
        }
        self.advance_min_vruntime();
    }

    fn advance_min_vruntime(&mut self) {
        // ИСПРАВЛЕНО: min_vruntime должен быть минимумом по всем vruntime в очереди
        if let Some(min_key) = self.by_deadline.iter().min_by_key(|k| k.vruntime) {
            if min_key.vruntime > self.min_vruntime {
                self.min_vruntime = min_key.vruntime;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn load(&self) -> u64 {
        self.load
    }

    pub fn current_task_id(&self) -> Option<TaskId> {
        self.current
    }

    pub fn insert(&mut self, task: Box<Task>) {
        let mut t = task;
        if t.vruntime < self.min_vruntime { t.vruntime = self.min_vruntime; }
        if t.deadline == 0 { t.deadline = t.vruntime + t.slice; }
        let key = TaskKey { deadline: t.deadline, vruntime: t.vruntime, id: t.id };
        self.load += t.weight;
        self.by_deadline.insert(key);
        self.tasks.insert(t.id, t);
    }

    pub fn remove(&mut self, id: TaskId) -> Option<Box<Task>> {
        if let Some(task) = self.tasks.remove(&id) {
            let key = TaskKey { deadline: task.deadline, vruntime: task.vruntime, id: task.id };
            self.by_deadline.remove(&key);
            self.load -= task.weight;
            Some(task)
        } else { None }
    }

    pub fn pick_next(&mut self) -> Option<TaskId> {
        if self.by_deadline.is_empty() { return None; }
        
        for key in self.by_deadline.iter() {
            if let Some(task) = self.tasks.get(&key.id) {
                if task.state == TaskState::Runnable && key.vruntime <= self.min_vruntime {
                    return Some(key.id);
                }
            }
        }
        
        let mut best_id = None;
        let mut min_vr = u64::MAX;
        for key in self.by_deadline.iter() {
            if let Some(task) = self.tasks.get(&key.id) {
                if task.state == TaskState::Runnable && key.vruntime < min_vr {
                    min_vr = key.vruntime;
                    best_id = Some(key.id);
                }
            }
        }
        best_id
    }

    pub fn clear_current(&mut self) {
        self.current = None;
    }
}

// Per-CPU runqueues
pub static RUNQUEUES: [Nitex<Runqueue>; crate::arch::MAX_CPUS] =
    [const { Nitex::new(Runqueue::new()) }; crate::arch::MAX_CPUS];
