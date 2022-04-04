use crate::task::trap_context::TrapContext;
use crate::task::kernel_stack::KernelStack;
use crate::task::pid::{PidHandle, alloc_pid};
use alloc::vec::Vec;
use alloc::sync::{Arc, Weak};
use spin::{Mutex, MutexGuard};
use crate::task::task_context::TaskContext;
use crate::mm::memory_manager::MemoryManager;
use share::syscall::error::{SysError, EAGAIN};

pub struct TaskStruct {
    pub pid_handle: PidHandle,
    pub inner: Mutex<TaskStructInner>
}

pub struct TaskStructInner {
    pub kernel_stack: KernelStack,
    pub wait_queue: Vec<Arc<TaskStruct>>,
    pub flag: RuntimeFlags,
    pub task_context: TaskContext,
    pub msg_ptr: Option<usize>,
    pub mem_manager: MemoryManager,
    pub priority: usize,

    pub children:Vec<Arc<TaskStruct>>,
    pub parent: Option<Weak<TaskStruct>>,
}

impl TaskStruct {
    pub fn new(data: &[u8]) -> Result<Self, SysError> {
        let (mem_manager, pc,mut user_sp) = MemoryManager::new(data)?;
        let pid_handle = alloc_pid().unwrap();
        user_sp -= core::mem::size_of::<usize>() * 3; // push argc, NULL and NULL onto stack.

        let trap_context = TrapContext::new(pc, user_sp);
        let mut kernel_stack = KernelStack::new();
        kernel_stack.push(trap_context);

        let task_context = TaskContext::new(kernel_stack.sp);

        let inner = TaskStructInner {
            kernel_stack,
            wait_queue: Vec::new(),
            flag: RuntimeFlags::READY,
            task_context,
            msg_ptr: None,
            mem_manager,
            priority: 7,
            children: Vec::new(),
            parent: None,
        };

        Ok(Self {
            pid_handle,
            inner: Mutex::new(inner),
        })
    }

    pub fn acquire_inner_lock(&self) -> MutexGuard<TaskStructInner> {
        self.inner.lock()
    }

    pub fn pid(&self) -> usize {
        self.pid_handle.0
    }
}

impl TaskStructInner {
    pub fn task_context_ptr(&self) -> usize {
        &self.task_context as *const _ as usize
    }


    pub fn is_receiving_from(&self, another_task: &Arc<TaskStruct>) -> bool {
        match self.flag {
            RuntimeFlags::RECEIVING(ReceiveProc::SPECIFIC(target_pid)) =>
                target_pid == another_task.pid(),
            RuntimeFlags::RECEIVING(ReceiveProc::ANY) => true,
            _ => false
        }
    }

    pub fn is_sending_to(&self, another_task: &Arc<TaskStruct>) -> bool {
        match self.flag {
            RuntimeFlags::SENDING(target_pid) => target_pid == another_task.pid(),
            _ => false
        }
    }
}

#[derive(Copy, Clone)]
pub enum RuntimeFlags {
    RECEIVING(ReceiveProc),
    SENDING(usize),
    READY,
    ZOMBIE(usize),
    RUNNING,
}

#[derive(Copy, Clone)]
pub enum ReceiveProc {
    ANY,
    SPECIFIC(usize),
}