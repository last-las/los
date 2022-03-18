use super::task_manager;
use super::kernel_stack;
use super::pid;

#[cfg(feature = "test")]
pub fn test_task_mod() {
    // task_manager::test_task_manager();
    kernel_stack::test_kernel_stack();
    pid::test_pid_allocation();
}