use share::syscall::error::{SysError, ECHILD};
use crate::processor::get_cur_task_in_this_hart;
use crate::task::{RuntimeFlags, stop_current_and_run_next_task};

// TODO-FUTURE: implement WNOHANG, WUNTRACED and WCONTINUED for waitpid
pub fn do_waitpid(pid: isize, status_ptr: usize, options: usize) -> Result<usize, SysError> {
    let cur_task = get_cur_task_in_this_hart();
    if cur_task.acquire_inner_lock().children.is_empty() {
        return Err(SysError::new(ECHILD));
    }

    if pid == -1 {
        wait_on_all_children(status_ptr, options)
    } else {
        wait_on_target_child(pid as usize, status_ptr, options)
    }
}

fn wait_on_all_children(status_ptr: usize, _: usize) -> Result<usize, SysError> {
    let cur_task = get_cur_task_in_this_hart();

    loop {
        let mut inner = cur_task.acquire_inner_lock();
        let mut exit_code = 0;
        let mut pid = 0;
        let result = inner.children.iter().enumerate().find(|(index, child)|{
            let child_inner = child.acquire_inner_lock();
            match child_inner.flag {
                RuntimeFlags::ZOMBIE(exit) => {
                    exit_code = exit;
                    pid = child.pid();
                    true
                },
                _ => false,
            }
        });
        if result.is_none() {
            drop(inner);
            stop_current_and_run_next_task();
            continue;
        }

        let (index,_) = result.unwrap();
        if status_ptr != 0 {
            unsafe {
                (status_ptr as *mut usize).write(exit_code);
            }
        }
        inner.children.remove(index);

        return Ok(pid);
    }
}

fn wait_on_target_child(pid: usize, status_ptr: usize, _: usize) -> Result<usize, SysError> {
    let cur_task = get_cur_task_in_this_hart();

    loop {
        let mut inner = cur_task.acquire_inner_lock();
        let result = inner.children.iter().enumerate().find(|(index, child)| {
            child.pid() == pid as usize
        });
        if result.is_none() {
            return Err(SysError::new(ECHILD));
        }

        let (index, child) = result.unwrap();
        let child_inner = child.acquire_inner_lock();
        match child_inner.flag {
            RuntimeFlags::ZOMBIE(exit_code) => {
                if status_ptr != 0 {
                    unsafe {
                        (status_ptr as *mut usize).write(exit_code);
                    }
                }
                drop(child_inner);
                inner.children.remove(index);
                return Ok(pid as usize);
            },
            _ => {
                drop(child_inner);
                drop(inner);
                stop_current_and_run_next_task();
            },
        }
    }
}