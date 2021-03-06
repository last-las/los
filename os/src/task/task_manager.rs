use crate::task::task_struct::TaskStruct;
use spin::Mutex;
use alloc::sync::{Arc, Weak};
use alloc::collections::VecDeque;
use crate::config::MAX_TASK_NUMBER;
use alloc::vec::Vec;
use crate::syscall::MAX_PRIORITY;

pub fn fetch_a_task_from_manager() -> Option<Arc<TaskStruct>> {
    TASK_MANAGER.lock().dequeue()
}

pub fn rm_task_from_manager(task_struct: Arc<TaskStruct>) {
    let pid = task_struct.pid_handle.0;
    assert_eq!(TASK_MANAGER.lock().rm_task_by_pid(pid), true);
}

pub fn add_a_task_to_manager(task_struct: Arc<TaskStruct>) {
    TASK_MANAGER.lock().add(task_struct);
}

pub fn return_task_to_manager(task_struct: Arc<TaskStruct>) {
    TASK_MANAGER.lock().enqueue(task_struct);
}

pub fn get_task_by_pid(pid: usize) -> Option<Arc<TaskStruct>> {
    TASK_MANAGER.lock().get_task_by_pid(pid)
}

lazy_static!{
    pub static ref TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());
}

const QUEUE_NUM: usize = MAX_PRIORITY as usize + 1;

/// a simple multi queue scheduler.
pub struct TaskManager {
    pid_2_task: Vec<Option<Arc<TaskStruct>>>,
    queues: [VecDeque<Arc<TaskStruct>>; QUEUE_NUM],
    last_task: Option<Weak<TaskStruct>>,
}

impl TaskManager {
    pub fn new() -> Self {
        let mut pid_2_task = Vec::new();
        (0..MAX_TASK_NUMBER).into_iter().for_each(|_| {
            pid_2_task.push(None);
        });

        let queues = [
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
        ];

        Self {
            pid_2_task,
            queues,
            last_task: None,
        }
    }

    pub fn add(&mut self, task: Arc<TaskStruct>)  {
        let pid = task.pid_handle.0;
        assert!(self.pid_2_task[pid].is_none());
        self.pid_2_task[pid] = Some(Arc::clone(&task));

        self.enqueue(task);
    }

    /// difference between `add()` and `enqueue()` is that `add()` is used when new task is created,
    /// while `enqueue()` is used when an old task is temporarily stopped.
    pub fn enqueue(&mut self, task: Arc<TaskStruct>) {
        let pid = task.pid_handle.0;
        assert!(self.pid_2_task[pid].is_some());
        if self.is_last_running_task(&task) {
            task.increase_priority();
        } else {
            task.decrease_priority();
        }

        let priority = task.acquire_inner_lock().priority;
        assert!(priority >= 0);
        self.queues[priority as usize].push_back(task);
    }

    pub fn dequeue(&mut self) -> Option<Arc<TaskStruct>> {
        let result = self.queues.iter().enumerate().find(|(_, queue)| {
            queue.len() != 0
        });
        if result.is_none() {
            return None;
        }

        let index = result.unwrap().0;
        let task: Arc<TaskStruct> = self.queues[index].pop_front().unwrap();
        self.last_task = Some(Arc::downgrade(&task));

        Some(task)
    }


    pub fn get_task_by_pid(&self, pid: usize) -> Option<Arc<TaskStruct>> {
        if pid >= MAX_TASK_NUMBER {
            None
        } else {
            self.pid_2_task[pid].clone()
        }
    }

    /// This function should be removed when the signal module is implemented.
    pub fn rm_task_by_pid(&mut self, pid: usize) -> bool {
        if pid >= MAX_TASK_NUMBER {
            return false;
        }
        self.pid_2_task[pid].take();
        true
    }

    fn is_last_running_task(&self, task: &Arc<TaskStruct>) -> bool {
        if self.last_task.is_none() {
            return false;
        }
        let result = self.last_task.as_ref().unwrap().upgrade();
        if result.is_none() { // last running task has already been removed.
            return false;
        }
        let last_task = result.unwrap();

        return Arc::ptr_eq(&last_task, task);
    }
}