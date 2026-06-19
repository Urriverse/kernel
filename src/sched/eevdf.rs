// src/sched/eevdf.rs
use alloc::collections::{BTreeMap, BTreeSet};
use super::task::{Task, TaskId};

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

pub struct Runqueue {
    map: BTreeMap<TaskId, Task>,
    set: BTreeSet<TaskKey>,
    min_vruntime: u64,
    current: Option<TaskId>,
}

impl Runqueue {
    pub const fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            set: BTreeSet::new(),
            min_vruntime: 0,
            current: None,
        }
    }

    pub fn insert_task(&mut self, mut task: Task) {
        // Нормализация vruntime, чтобы новые задачи не получали нечестное преимущество
        if task.vruntime < self.min_vruntime {
            task.vruntime = self.min_vruntime;
        }
        
        let key = TaskKey { deadline: task.deadline, vruntime: task.vruntime, id: task.id };
        self.set.insert(key);
        self.map.insert(task.id, task);
    }

    pub fn remove_task(&mut self, id: TaskId) -> Option<Task> {
        if let Some(task) = self.map.remove(&id) {
            let key = TaskKey { deadline: task.deadline, vruntime: task.vruntime, id: task.id };
            self.set.remove(&key);
            Some(task)
        } else {
            None
        }
    }

    pub fn current_task_id(&self) -> Option<TaskId> { self.current }
    pub fn set_current_task(&mut self, task: Task) { self.current = Some(task.id); }

    pub fn update_vruntime(&mut self, delta: u64) {
        if let Some(curr_id) = self.current {
            if let Some(mut task) = self.remove_task(curr_id) {
                let delta_vruntime = (delta * NICE_0_WEIGHT) / task.weight;
                task.vruntime += delta_vruntime;
                self.insert_task(task);
            }
        }
        self.advance_min_vruntime();
    }

    fn advance_min_vruntime(&mut self) {
        if let Some(key) = self.set.iter().next() {
            // min_vruntime должен монотонно расти, но не опережать самую "отстающую" задачу
            let min_vr = self.set.iter().map(|k| k.vruntime).min().unwrap_or(self.min_vruntime);
            if min_vr > self.min_vruntime {
                self.min_vruntime = min_vr;
            }
        }
    }

    pub fn pick_next_task(&mut self) -> Option<Task> {
        if self.set.is_empty() { return None; }

        // Ищем самую раннюю eligible задачу (vruntime <= min_vruntime)
        for key in self.set.iter() {
            if key.vruntime <= self.min_vruntime {
                return self.map.get(&key.id).copied();
            }
        }

        // Fallback: если все задачи "переели" (vruntime > min_vruntime), 
        // выбираем ту, у которой vruntime минимальный, чтобы восстановить баланс.
        let mut min_vr_key = self.set.iter().next().unwrap();
        for key in self.set.iter() {
            if key.vruntime < min_vr_key.vruntime {
                min_vr_key = key;
            }
        }
        self.map.get(&min_vr_key.id).copied()
    }
}
