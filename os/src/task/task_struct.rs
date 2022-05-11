use crate::task::trap_context::TrapContext;
use crate::task::kernel_stack::KernelStack;
use crate::task::pid::{PidHandle, alloc_pid};
use alloc::vec::Vec;
use alloc::sync::{Arc, Weak};
use spin::{Mutex, MutexGuard};
use crate::task::task_context::TaskContext;
use crate::mm::memory_manager::MemoryManager;
use share::syscall::error::SysError;
use share::ipc::Msg;
use crate::syscall::{MAX_PRIORITY, MIN_PRIORITY};
use alloc::string::String;

pub struct TaskStruct {
    pub pid_handle: PidHandle,
    pub inner: Mutex<TaskStructInner>
}

pub struct TaskStructInner {
    pub name: String,

    pub kernel_stack: KernelStack,
    pub flag: RuntimeFlags,
    pub task_context: TaskContext,
    // ipc
    pub message_holder: Option<Msg>,
    pub interrupt_flag: bool,
    pub wait_queue: Vec<Arc<TaskStruct>>,

    pub mem_manager: MemoryManager,

    pub priority: isize,
    pub min_priority: isize,

    pub children:Vec<Arc<TaskStruct>>,
    pub parent: Option<Weak<TaskStruct>>,
}

impl TaskStruct {
    pub fn new(data: &[u8], name: String) -> Result<Self, SysError> {
        let (mem_manager, pc,mut user_sp) = MemoryManager::new(data)?;
        let pid_handle = alloc_pid().unwrap();
        user_sp -= core::mem::size_of::<usize>() * 3; // push argc, NULL and NULL onto stack.

        let kernel_stack = KernelStack::new()?;
        let task_context = TaskContext::new(kernel_stack.sp() - core::mem::size_of::<TrapContext>());

        let mut inner = TaskStructInner {
            name,
            kernel_stack,
            wait_queue: Vec::new(),
            flag: RuntimeFlags::READY,
            task_context,
            message_holder: None,
            interrupt_flag: false,
            mem_manager,
            priority: 0,
            min_priority: 0,
            children: Vec::new(),
            parent: None,
        };
        // push `trap_context` onto `kernel_stack`
        let trap_context_ref = inner.trap_context_ref();
         *trap_context_ref = TrapContext::new(pc, user_sp);

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

    pub fn increase_priority(&self) {
        let mut inner = self.acquire_inner_lock();
        if inner.priority < MAX_PRIORITY {
            inner.priority += 1;
        }
    }

    pub fn decrease_priority(&self) {
        let mut inner = self.acquire_inner_lock();
        let min_priority = isize::max(MIN_PRIORITY, inner.min_priority);
        if inner.priority > min_priority {
            inner.priority -= 1;
        }
    }
}

impl TaskStructInner {
    pub fn task_context_ptr(&self) -> usize {
        &self.task_context as *const _ as usize
    }

    pub fn trap_context_ref(&mut self) -> &'static mut TrapContext {
        let trap_context_pa = self.kernel_stack.sp.sub(core::mem::size_of::<TrapContext>());
        trap_context_pa.as_mut()
    }

    pub fn is_receiving_from(&self, another_task: &Arc<TaskStruct>) -> bool {
        match self.flag {
            RuntimeFlags::RECEIVING(target_pid) =>
                target_pid < 0 || target_pid as usize == another_task.pid(),
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
    RECEIVING(isize),
    SENDING(usize),
    READY,
    ZOMBIE(isize),
    RUNNING,
}