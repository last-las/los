mod do_fork;
mod do_exec;
mod do_waitpid;
mod priority;

use crate::task::{exit_current_and_run_next_task, stop_current_and_run_next_task, RuntimeFlags};
pub use do_fork::do_fork;
pub use do_exec::do_exec;
pub use do_waitpid::do_waitpid;
pub use priority::*;
use share::syscall::error::{SysError, ECHILD};
use crate::processor::clone_cur_task_in_this_hart;

pub fn do_exit(exit_code: isize) -> Result<usize, SysError> {
    info!("task exit with exit_code:{}", exit_code);
    exit_current_and_run_next_task(exit_code as usize);
    Ok(0)
}

pub fn do_yield() -> Result<usize, SysError> {
    stop_current_and_run_next_task();
    Ok(0)
}

pub fn do_get_pid() -> Result<usize, SysError> {
    Ok(clone_cur_task_in_this_hart().pid())
}

/// Except init process, this function is always successful.
pub fn do_get_ppid() -> Result<usize, SysError> {
    let cur_task = clone_cur_task_in_this_hart();
    let inner = cur_task.acquire_inner_lock();
    Ok(inner.parent.as_ref().unwrap().upgrade().unwrap().pid())
}