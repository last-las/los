use crate::task::task_context::TaskContext;
use crate::task::kernel_stack::KernelStack;
use crate::task::user_stack_allocator::alloc_a_user_stack;
use crate::task::pid::{Pid, alloc_pid};

pub struct TaskStruct {
    pub kernel_stack: KernelStack,
    pub pid: Pid,
}

impl TaskStruct {
    pub fn new_user_task(pc: usize) -> Self {
        let user_sp = alloc_a_user_stack();
        let pid = alloc_pid().unwrap();

        let user_context = TaskContext::new(pc, user_sp, true);
        let mut kernel_stack = KernelStack::new();
        kernel_stack.push(user_context);
        Self {
            kernel_stack,
            pid
        }
    }

    #[allow(unused)]
    pub fn new_kernel_task() -> Self {
        unimplemented!();
    }
}