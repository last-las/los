use crate::task::task_struct::TaskStruct;
use spin::Mutex;
use alloc::sync::Arc;
use alloc::collections::VecDeque;
use crate::config::MAX_TASK_NUMBER;
use alloc::vec::Vec;

pub fn fetch_a_task_from_manager() -> Option<Arc<TaskStruct>> {
    TASK_MANAGER.lock().fetch()
}

pub fn rm_task_from_manager(task_struct: Arc<TaskStruct>) {
    let pid = task_struct.pid_handle.0;
    TASK_MANAGER.lock().rm_task_by_pid(pid);
}

pub fn add_a_task_to_manager(task_struct: Arc<TaskStruct>) {
    TASK_MANAGER.lock().add(task_struct);
}

pub fn return_task_to_manager(task_struct: Arc<TaskStruct>) {
    TASK_MANAGER.lock().return_(task_struct);
}

pub fn get_task_by_pid(pid: usize) -> Option<Arc<TaskStruct>> {
    TASK_MANAGER.lock().get_task_by_pid(pid)
}

lazy_static!{
    pub static ref TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());
}

pub struct TaskManager {
    pid_2_task: Vec<Option<Arc<TaskStruct>>>,
    task_queue: VecDeque<Arc<TaskStruct>>,
}

impl TaskManager {
    pub fn new() -> Self {
        let mut pid_2_task = Vec::new();
        (0..MAX_TASK_NUMBER).into_iter().for_each(|_| {
            pid_2_task.push(None);
        });
        Self {
            pid_2_task,
            task_queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, task: Arc<TaskStruct>)  {
        self.task_queue.push_back(task.clone());
        let pid = task.pid_handle.0;
        assert!(self.pid_2_task[pid].is_none());
        self.pid_2_task[pid] = Some(task);
    }

    /// difference between add() and return_() is that add() is used when new task is created,
    /// while return_ is used when an old task is temporarily stopped thus it is returned to the manager.
    pub fn return_(&mut self, task: Arc<TaskStruct>) {
        self.task_queue.push_back(task.clone());
        let pid =task.pid_handle.0;
        assert!(self.pid_2_task[pid].is_some());
    }

    pub fn fetch(&mut self) -> Option<Arc<TaskStruct>> {
        self.task_queue.pop_front()
    }


    pub fn get_task_by_pid(&self, pid: usize) -> Option<Arc<TaskStruct>> {
        self.pid_2_task[pid].clone()
    }

    /// This function should be removed when the signal module is implemented.
    pub fn rm_task_by_pid(&mut self, pid: usize) -> bool {
        if let Some((index, _)) = self.task_queue
            .iter()
            .enumerate()
            .find(|(_, task)| {
                task.pid_handle.0 == pid
        }) {
            self.task_queue.remove(index);
            self.pid_2_task[pid].take();
            true
        } else {
            false
        }
    }
}

/// test case for task_manager.
pub fn test_task_manager() {
    info!("starting task_manager.rs test cases.");

    let mut task_manager = TaskManager::new();
    // 1. test add() and fetch()
    task_manager.add(Arc::new(TaskStruct::new(0)));
    assert!(task_manager.fetch().is_some());
    assert!(task_manager.fetch().is_none());

    // 2. test get_task_by_pid()
    task_manager.add(Arc::new(TaskStruct::new(0)));
    assert!(task_manager.get_task_by_pid(0).is_some());
    assert!(task_manager.get_task_by_pid(1).is_some());
    assert!(task_manager.get_task_by_pid(2).is_none());



    info!("end of task_manager.rs test.\n");
}