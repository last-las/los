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
}

impl TaskStruct {
    pub fn new_user_task(pc: usize) -> Self {
        let user_sp = alloc_a_user_stack();
        let pid_handle = alloc_pid().unwrap();

        let user_context = TrapContext::new(pc, user_sp, true);
        let mut kernel_stack = KernelStack::new();
        kernel_stack.push(user_context);

        let task_context = TaskContext::new(kernel_stack.sp);
        kernel_stack.push(task_context);

        let inner = TaskStructInner {
            kernel_stack,
            wait_queue: Vec::new(),
            flag: RuntimeFlags::READY,

        };

        Self {
            pid_handle,
            inner: Mutex::new(inner),
        }
    }

    pub fn acquire_inner_lock(&self) -> MutexGuard<TaskStructInner> {
        self.inner.lock()
    }

    #[allow(unused)]
    pub fn new_kernel_task() -> Self {
        unimplemented!();
    }

    pub fn pid(&self) -> usize {
        self.pid_handle.0
    }

    pub fn is_receiving_from(&self, another_task: Arc<TaskStruct>) -> bool {
        let this_task_inner = self.acquire_inner_lock();
        let another_pid = another_task.pid();

        let mut ans = false;

        if let RuntimeFlags::RECEIVING(receive_proc) = this_task_inner.flag {
            if let ReceiveProc::SPECIFIC(target_pid) = receive_proc {
                if target_pid == another_pid {
                    ans = true;
                }
            } else { // dst_task is waiting for ANY tasks thus always return true.
                ans = true;
            }
        }

        ans
    }

    pub fn is_sending_to(&self, another_task: Arc<TaskStruct>) -> bool {
        let this_task_inner = self.acquire_inner_lock();
        let mut ans = false;

        if let RuntimeFlags::SENDING(target_pid) = this_task_inner.flag {
            if target_pid == another_task.pid() {
                ans = true;
            }
        }

        ans
    }
}

impl TaskStructInner {
    // pub fn is_waiting_for()
}

/*bitflags! {
    pub struct RuntimeFlags: u8 {
        const RECEIVING = 1 << 0;
        const SENDING = 1 << 1;
        const ZOMBIE = 1 << 2;
        const RUNNING = 1 << 3;
    }
}*/

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