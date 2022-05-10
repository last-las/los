use crate::processor::get_cur_task_in_this_hart;
use share::syscall::error::SysError;
use crate::task::{schedule, RuntimeFlags};

pub const MIN_PRIORITY: isize = 0;
pub const MAX_PRIORITY: isize = 7;

// TODO-FUTURE: get/set_priority should be updated after implementing process group and user privilege
pub fn do_get_priority(_: usize, _: usize) -> Result<usize, SysError> {
    let cur_task = get_cur_task_in_this_hart();
    let inner = cur_task.acquire_inner_lock();
    Ok(inner.priority as usize)
}

pub fn do_set_priority(_: usize, _: usize, mut prio: isize) -> Result<usize, SysError> {
    let cur_task = get_cur_task_in_this_hart();
    let mut inner = cur_task.acquire_inner_lock();
    let min_priority = isize::max(MIN_PRIORITY, inner.min_priority);
    if prio < min_priority{
        prio = min_priority;
    } else if prio > MAX_PRIORITY {
        prio = MAX_PRIORITY;
    }
    inner.priority = prio;
    drop(inner);
    drop(cur_task);
    schedule(RuntimeFlags::READY);

    Ok(0)
}
