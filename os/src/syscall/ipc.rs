use alloc::string::String;
use crate::task::{get_task_by_pid, RuntimeFlags, ReceiveProc, TaskStruct, stop_current_and_run_next_task};
use crate::processor::get_cur_task_in_this_hart;
use alloc::sync::Arc;
use riscv::register::fcsr::RoundingMode::RoundUp;

#[repr(C)]
struct Msg {
    src_pid: usize,
    content: MsgContent,
}

enum MsgContent {
    TestMsg(String),
}

pub fn sys_send(dst_pid: usize, msg_ptr: usize) -> isize {
    let wrapped_dst_task = get_task_by_pid(dst_pid);
    if wrapped_dst_task.is_none() {
        return -1;
    }
    let dst_task = wrapped_dst_task.unwrap();
    let caller_task = get_cur_task_in_this_hart();

    if dst_task.is_receiving_from(caller_task.clone()) {

    } else {
        let mut src_task_inner = caller_task.acquire_inner_lock();
        src_task_inner.flag = RuntimeFlags::SENDING(dst_pid);
        unimplemented!();
        stop_current_and_run_next_task();
    }
    unimplemented!()
}

pub fn sys_receive(dst_pid: usize, msg_ptr: usize) -> isize {
    let wrapped_dst_task = get_task_by_pid(dst_pid);
    if wrapped_dst_task.is_none() {
        return -1;
    }
    let dst_task = wrapped_dst_task.unwrap();
    let caller_task = get_cur_task_in_this_hart();

    if dst_task.is_sending_to(caller_task.clone()) {
    } else {
    }

    unimplemented!();
}