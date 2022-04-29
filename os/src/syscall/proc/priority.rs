use crate::processor::get_cur_task_in_this_hart;
use share::syscall::error::SysError;
use crate::task::stop_current_and_run_next_task;

const MIN_PRIORITY: isize = 0;
const MAX_PRIORITY: isize = 7;

// TODO-FUTURE: get/set_priority should be updated after implementing process group and user privilege
pub fn do_get_priority(_: usize, _: usize) -> Result<usize, SysError> {
    let cur_task = get_cur_task_in_this_hart();
    let inner = cur_task.acquire_inner_lock();
    Ok(inner.priority)
}

pub fn do_set_priority(_: usize, _: usize, mut prio: isize) -> Result<usize, SysError> {
    if prio < MIN_PRIORITY {
        prio = MIN_PRIORITY;
    } else if prio > MAX_PRIORITY {
        prio = MAX_PRIORITY;
    }
    let cur_task = get_cur_task_in_this_hart();
    let mut inner = cur_task.acquire_inner_lock();
    inner.priority = prio as usize;
    drop(inner);
    drop(cur_task);
    stop_current_and_run_next_task(); // reschedule

    Ok(0)
}
