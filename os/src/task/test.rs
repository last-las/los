use super::user_stack_allocator;
use super::task_manager;
use super::kernel_stack;

pub fn test_task_mod() {
    task_manager::test_task_manager();
    user_stack_allocator::test_user_stack_allocator();
    kernel_stack::test_kernel_stack();
}