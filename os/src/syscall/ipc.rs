use crate::task::{get_task_by_pid, RuntimeFlags, TaskStruct, block_current_and_run_next_task, return_task_to_manager, TaskStructInner};
use crate::processor::clone_cur_task_in_this_hart;
use alloc::sync::Arc;
use share::ipc::Msg;
use share::syscall::error::{EINVAL, SysError, EDLOCK};
use spin::MutexGuard;

// TODO-FUTURE: using registers to pass the message could improve performance. L4 stuff.

/// Send a message, which `msg_ptr` points to, from current task to `dst_pid` task.
///
/// Before any real work, caller task checks whether there is a deadlock situation.
/// Caller task reads the message from `msg_ptr`, and If `dst_pid` task is receiving,
/// it moves the message to dst task's [`TaskStruct`] and wakes it up. Otherwise it stores the message
/// inside caller task's [`TaskStruct`] and blocks itself.
pub fn sys_send(dst_pid: usize, msg_ptr: usize) -> Result<usize, SysError> {
    let dst_task = get_dst_task_or_err(dst_pid)?;
    let caller_task = clone_cur_task_in_this_hart();
    check_deadlock(caller_task.clone(), dst_task.clone())?;

    let mut message = unsafe { (msg_ptr as *const Msg).read() };
    message.src_pid = caller_task.pid();
    let mut dst_task_inner =
        dst_task.acquire_inner_lock(); // acquire lock to avoid race condition

    if dst_task_inner.is_receiving_from(&caller_task) {
        assert!(dst_task_inner.message_holder.is_none());
        dst_task_inner.message_holder = Some(message);
        dst_task_inner.flag = RuntimeFlags::READY;
        drop(dst_task_inner);

        return_task_to_manager(dst_task.clone());
    } else {
        let mut src_task_inner = caller_task.acquire_inner_lock();
        src_task_inner.flag = RuntimeFlags::SENDING(dst_pid);
        src_task_inner.message_holder = Some(message);
        dst_task_inner.wait_queue.push(caller_task.clone());
        drop(src_task_inner);
        drop(dst_task_inner);

        block_current_and_run_next_task();
    }

    Ok(0)
}

/// Receive a [`Msg`] `dst_pid` task, and save it to `msg_ptr` address.
///
/// If `interrupt_flag` is set for caller task, it return with an interrupt message immediately.
/// Caller task finds out possible sending tasks, if there is someone sending, it moves the [`Msg`]
/// from sending task to address where `msg_ptr` points to and wakes the sending task up. Otherwise
/// it blocks itself, and after it is waked up, it moves message to that address.
pub fn sys_receive(dst_pid: isize, msg_ptr: usize) -> Result<usize, SysError>{
    let src_task = clone_cur_task_in_this_hart();
    let mut src_task_inner = src_task.acquire_inner_lock();
    if dst_pid == -1 && src_task_inner.interrupt_flag {
        src_task_inner.interrupt_flag = false;
        build_and_move_interrupt_message_to(msg_ptr);
        return Ok(0);
    }
    let idx = find_possible_sending_task_index(&src_task_inner, dst_pid);

    if idx.is_some() {
        let idx = idx.unwrap();
        let dst_task = src_task_inner.wait_queue[idx].clone();
        let mut dst_task_inner = dst_task.acquire_inner_lock();
        assert!(dst_task_inner.is_sending_to(&src_task));
        let message = dst_task_inner.message_holder.take().unwrap();
        unsafe {
            (msg_ptr as *mut Msg).write(message);
        }
        dst_task_inner.flag = RuntimeFlags::READY;
        src_task_inner.wait_queue.remove(idx);

        drop(dst_task_inner);
        return_task_to_manager(dst_task.clone());
        return Ok(0);
    }

    src_task_inner.flag = RuntimeFlags::RECEIVING(dst_pid);
    drop(src_task_inner);
    drop(src_task);
    block_current_and_run_next_task();

    // After the task is waked up the message has been received.
    let src_task = clone_cur_task_in_this_hart();
    let mut src_task_inner = src_task.acquire_inner_lock();
    unsafe {
        (msg_ptr as *mut Msg).write(src_task_inner.message_holder.take().unwrap());
    }
    Ok(0)
}

/// This function is only used by kernel to notify `dst_pid` task that there is an interrupt for it.
pub fn notify(dst_pid: usize) -> Result<(), SysError> {
    let dst_task = get_dst_task_or_err(dst_pid)?;
    let mut dst_task_inner = dst_task.acquire_inner_lock();
    match dst_task_inner.flag {
        RuntimeFlags::RECEIVING(-1) => {
            let mut message = Msg::empty();
            message.mtype = 1;
            dst_task_inner.message_holder = Some(message);
            dst_task_inner.flag = RuntimeFlags::READY;
            drop(dst_task_inner);
            return_task_to_manager(dst_task);
        },
        _ => {
            dst_task_inner.interrupt_flag = true;
        },
    }
    Ok(())
}

fn get_dst_task_or_err(dst_pid: usize) -> Result<Arc<TaskStruct>, SysError> {
    let wrapped_dst_task = get_task_by_pid(dst_pid);
    match wrapped_dst_task {
        Some(task) => Ok(task),
        None => Err(SysError::new(EINVAL)),
    }
}

// If task_a -sending-> task_b -sending-> task_c -sending-> task_a, then there is a dead lock.
fn check_deadlock(src_task: Arc<TaskStruct>, mut dst_task: Arc<TaskStruct>) -> Result<(), SysError> {
    let src_pid = src_task.pid();
    loop {
        let dst_task_inner = dst_task.acquire_inner_lock();
        match dst_task_inner.flag {
            RuntimeFlags::SENDING(target_pid) => {
                if target_pid == src_pid {
                    return Err(SysError::new(EDLOCK));
                }

                drop(dst_task_inner);
                let wrapped_task = get_task_by_pid(target_pid);
                if wrapped_task.is_none() {
                    break;
                }
                dst_task = get_task_by_pid(target_pid).unwrap();
            },

            _ => {
                break;
            }
        }
    }

    Ok(())
}

fn build_and_move_interrupt_message_to(msg_ptr: usize) {
    let mut message = Msg::empty();
    message.mtype = 1;
    unsafe {
        (msg_ptr as *mut Msg).write(message);
    }
}

fn find_possible_sending_task_index(src_task_inner:& MutexGuard<TaskStructInner>, dst_pid: isize) -> Option<usize> {
    let mut idx = None;
    for i in 0..src_task_inner.wait_queue.len() {
        if dst_pid < 0 || src_task_inner.wait_queue[i].pid() == dst_pid as usize {
            idx = Some(i);
            break;
        }
    }

    idx
}