use crate::task::trap_context::TrapContext;
use crate::task::kernel_stack::KernelStack;
use crate::task::user_stack_allocator::alloc_a_user_stack;
use crate::task::pid::{PidHandle, alloc_pid};
use alloc::vec::Vec;
use alloc::sync::Arc;
use spin::{Mutex, MutexGuard};
use crate::task::task_context::TaskContext;

pub struct TaskStruct {
    pub pid_handle: PidHandle,
    inner: Mutex<TaskStructInner>
}

pub struct TaskStructInner {
    pub kernel_stack: KernelStack,
    pub wait_queue: Vec<Arc<TaskStruct>>,
    pub flag: RuntimeFlags,
    pub task_context: TaskContext,
    pub msg_ptr: Option<usize>,
}

impl TaskStruct {
    pub fn new(pc: usize) -> Self {
        let user_sp = alloc_a_user_stack();
        let pid_handle = alloc_pid().unwrap();

        let user_context = TrapContext::new(pc, user_sp);
        let mut kernel_stack = KernelStack::new();
        kernel_stack.push(user_context);

        info!("kernel_stack sp: {:#x}", kernel_stack.sp);
        let task_context = TaskContext::new(kernel_stack.sp);

        let inner = TaskStructInner {
            kernel_stack,
            wait_queue: Vec::new(),
            flag: RuntimeFlags::READY,
            task_context,
            msg_ptr: None,
        };

        Self {
            pid_handle,
            inner: Mutex::new(inner),
        }
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
        unsafe {
            &self.task_context as *const _ as usize
        }
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
    ZOMBIE,
    RUNNING,
}

#[derive(Copy, Clone)]
pub enum ReceiveProc {
    ANY,
    SPECIFIC(usize),
}