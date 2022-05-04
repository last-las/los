mod do_fork;
mod do_exec;
mod do_waitpid;
mod priority;

use crate::task::{exit_current_and_run_next_task, stop_current_and_run_next_task};
pub use do_fork::do_fork;
pub use do_exec::do_exec;
pub use do_waitpid::do_waitpid;
pub use priority::*;
use share::syscall::error::SysError;
use crate::processor::get_cur_task_in_this_hart;
use share::ipc::{Msg, EXIT, EXIT_PID, FS_PID};
use crate::syscall::ipc::kcall_send;

pub fn do_exit(exit_code: isize) -> Result<usize, SysError> {
    // get cur pid
    let cur_task = get_cur_task_in_this_hart();
    let pid = cur_task.pid();
    drop(cur_task);

    // send EXIT message to fs server
    let mut message = Msg::empty();
    message.mtype = EXIT;
    message.args[EXIT_PID] = pid;
    kcall_send(FS_PID, &message as *const _ as usize)?;

    // final step
    exit_current_and_run_next_task(exit_code as usize);
    Ok(0)
}

pub fn do_yield() -> Result<usize, SysError> {
    stop_current_and_run_next_task();
    Ok(0)
}

pub fn do_get_pid() -> Result<usize, SysError> {
    Ok(get_cur_task_in_this_hart().pid())
}

/// Except init process, this function is always successful.
pub fn do_get_ppid() -> Result<usize, SysError> {
    let cur_task = get_cur_task_in_this_hart();
    let inner = cur_task.acquire_inner_lock();
    Ok(inner.parent.as_ref().unwrap().upgrade().unwrap().pid())
}