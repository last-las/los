mod do_fork;

use crate::task::{exit_current_and_run_next_task, stop_current_and_run_next_task};
pub use do_fork::do_fork;
use share::syscall::error::SysError;

pub fn do_exit(exit_code: isize) -> Result<usize, SysError> {
    info!("task exit with exit_code:{}", exit_code);
    exit_current_and_run_next_task();
    Ok(0)
}

pub fn do_yield() -> Result<usize, SysError> {
    stop_current_and_run_next_task();
    Ok(0)
}

pub fn do_get_priority(which: usize, who: usize) -> isize {
    unimplemented!();
}

pub fn do_set_priority(which: usize, who: usize, prio: usize) -> isize {
    unimplemented!();
}

pub fn do_get_pid() -> isize {
    unimplemented!();
}

pub fn do_get_ppid() -> isize {
    unimplemented!();
}

pub fn do_exec(path_ptr: usize, argv_ptr: usize, envp_ptr: usize) -> isize {
    unimplemented!();
}

pub fn do_waitpid(pid: usize, status_ptr: usize, options: usize) -> isize {
    unimplemented!();
}