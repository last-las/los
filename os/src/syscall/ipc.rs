use alloc::string::String;
use crate::task::{get_task_by_pid, RuntimeFlags, ReceiveProc, TaskStruct, stop_current_and_run_next_task, block_current_and_run_next_task, return_task_to_manager};
use crate::processor::get_cur_task_in_this_hart;
use alloc::sync::Arc;
use riscv::register::fcsr::RoundingMode::RoundUp;
use ipc::{Msg, MsgContent};
use crate::config::MAX_TASK_NUMBER;

pub fn sys_send(dst_pid: usize, msg_ptr: usize) -> isize {
    let wrapped_dst_task = get_task_by_pid(dst_pid);
    if wrapped_dst_task.is_none() {
        return -1;
    }
    let dst_task = wrapped_dst_task.unwrap();
    let caller_task = get_cur_task_in_this_hart();
    check_deadlock(caller_task.clone(), dst_task.clone());


    let mut dst_task_inner = dst_task.acquire_inner_lock();
    if dst_task_inner.is_receiving_from(&caller_task) {
        // copy current message to dst_task's msg_ptr
        unsafe {
            let dst_msg_ptr = dst_task_inner.msg_ptr.take().unwrap() as * mut Msg;
            let src_msg = (msg_ptr as *const Msg).read();
            dst_msg_ptr.write(src_msg);
        }

        return_task_to_manager(dst_task.clone());
        dst_task_inner.flag = RuntimeFlags::READY;
    } else {
        let mut src_task_inner = caller_task.acquire_inner_lock();
        src_task_inner.flag = RuntimeFlags::SENDING(dst_pid);
        src_task_inner.msg_ptr = Some(msg_ptr);

        dst_task_inner.wait_queue.push(caller_task.clone());

        drop(src_task_inner);
        drop(dst_task_inner);

        block_current_and_run_next_task();
    }
    0
}

pub fn sys_receive(dst_pid: usize, msg_ptr: usize) -> isize {
    let src_task = get_cur_task_in_this_hart();
    let mut src_task_inner = src_task.acquire_inner_lock();
    let mut exist_possible_task = false;
    let mut idx = None;

    for i in 0..src_task_inner.wait_queue.len() {
        if dst_pid == usize::MAX || src_task_inner.wait_queue[i].pid() == dst_pid {
            idx = Some(i);
            exist_possible_task = true;
            break;
        }
    }

    if exist_possible_task {
        let idx = idx.unwrap();
        let dst_task = src_task_inner.wait_queue[idx].clone();
        let mut dst_task_inner = dst_task.acquire_inner_lock();
        assert!(dst_task_inner.is_sending_to(&src_task));
        unsafe {
            let dst_msg = (dst_task_inner.msg_ptr.take().unwrap() as *const Msg).read();
            let src_msg_ptr = msg_ptr as *mut Msg;
            src_msg_ptr.write(dst_msg);
        }

        return_task_to_manager(dst_task.clone());
        dst_task_inner.flag = RuntimeFlags::READY;

        src_task_inner.wait_queue.remove(idx);
        debug!("wait_queue length: {}",src_task_inner.wait_queue.len());
        return 0;
    }

    if dst_pid == usize::MAX {
        src_task_inner.flag = RuntimeFlags::RECEIVING(ReceiveProc::ANY);
    } else {
        src_task_inner.flag = RuntimeFlags::RECEIVING(ReceiveProc::SPECIFIC(dst_pid));
    }
    src_task_inner.msg_ptr = Some(msg_ptr);
    drop(src_task_inner);

    block_current_and_run_next_task();
    0
}

fn check_deadlock(src_task: Arc<TaskStruct>, mut dst_task: Arc<TaskStruct>) {
    let src_pid = src_task.pid();
    loop {
        let mut dst_task_inner = dst_task.acquire_inner_lock();
        match dst_task_inner.flag {
            RuntimeFlags::SENDING(target_pid) => {
                assert_ne!(target_pid, src_pid);
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
}
