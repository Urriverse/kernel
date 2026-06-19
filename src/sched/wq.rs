use alloc::collections::VecDeque;
use super::task::TaskId;

pub struct WaitQueue {
    waiters: VecDeque<TaskId>,
}

impl WaitQueue {
    pub const fn new() -> Self {
        Self {
            waiters: VecDeque::new(),
        }
    }

    pub fn sleep(&mut self, task_id: TaskId) {
        self.waiters.push_back(task_id);
    }

    pub fn wakeup_one(&mut self) -> Option<TaskId> {
        self.waiters.pop_front()
    }

    pub fn wakeup_all(&mut self) -> alloc::vec::Vec<TaskId> {
        self.waiters.drain(..).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.waiters.is_empty()
    }
}
