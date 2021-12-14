use crate::task::task_struct::TaskStruct;
use spin::Mutex;
use alloc::sync::Arc;
use crate::processor::get_hart_id;
use alloc::collections::VecDeque;


pub fn fetch_a_task_from_task_manager() -> Option<Arc<TaskStruct>> {
    TASK_MANAGER.lock().fetch()
}

pub fn add_a_task_to_task_manager(task_struct: Arc<TaskStruct>) {
    TASK_MANAGER.lock().add(task_struct);
}

lazy_static!{
    pub static ref TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());
}

pub struct TaskManager {
    task_queue: VecDeque<Arc<TaskStruct>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            task_queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, task: Arc<TaskStruct>)  {
        self.task_queue.push_back(task);
    }

    pub fn fetch(&mut self) -> Option<Arc<TaskStruct>> {
        self.task_queue.pop_front()
    }
}

/// test case for task_manager.
pub fn test_task_manager() {
    let mut task_manager = TaskManager::new();
    let task = Arc::new(TaskStruct::new_user_task(0, 0));
    task_manager.add(task);
    assert!(task_manager.fetch().is_some());
    assert!(task_manager.fetch().is_none());
}