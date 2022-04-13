use core::str::from_utf8;
use share::syscall::error::{EBADF, SysError};
use crate::sbi::sbi_console_getchar;
use crate::task::stop_current_and_run_next_task;
use crate::syscall::ipc::{sys_send, sys_receive};
use share::ipc::{Msg, DEVICE, PROC_NR, BUFFER, LENGTH, REPLY_STATUS, READ, WRITE};
use crate::processor::clone_cur_task_in_this_hart;

pub fn do_write(fd: usize, buf_ptr: *const u8, length: usize) -> Result<usize, SysError>{
    if fd != 1 {
        return Err(SysError::new(EBADF));
    }
    let buffer = unsafe {
        core::slice::from_raw_parts(buf_ptr, length)
    };
    print!("{}", from_utf8(buffer).unwrap());
    Ok(0)
}

pub fn _do_write(fd: usize, buf_ptr: usize, length: usize) -> Result<usize, SysError> {
        let mut message = Msg::empty();
        let cur_pid = clone_cur_task_in_this_hart().pid();
        message.src_pid = cur_pid;
        message.mtype = WRITE;
        message.args[DEVICE] = 0;
        message.args[PROC_NR] = cur_pid;
        message.args[BUFFER] = buf_ptr;
        message.args[LENGTH] = length;
        sys_send(1, &message as *const _ as usize).unwrap();
        sys_receive(1, &mut message as *mut _ as usize).unwrap();

        Ok(message.args[REPLY_STATUS])
}


pub fn do_read(fd: usize, buf_ptr: *mut u8, length: usize) -> Result<usize, SysError> {
    if fd != 0 {
        return Err(SysError::new(EBADF));
    }

    let buffer = unsafe {
        core::slice::from_raw_parts_mut(buf_ptr, length)
    };
    let mut cnt = 0;
    for i in 0..length {
        let mut result = 0;
        loop {
            result = sbi_console_getchar();
            // info!("result is: {:#x}", result);
            if result == -1 {
                stop_current_and_run_next_task();
                continue;
            }
            break;
        }
        buffer[i] = result as usize as u8;
        cnt += 1;
    }

    Ok(cnt)
}

pub fn _do_read(fd: usize, buf_ptr: usize, length: usize) -> Result<usize, SysError> {
    let mut message = Msg::empty();
    let cur_pid = clone_cur_task_in_this_hart().pid();
    message.src_pid = cur_pid;
    message.mtype = READ;
    message.args[DEVICE] = 0;
    message.args[PROC_NR] = cur_pid;
    message.args[BUFFER] = buf_ptr;
    message.args[LENGTH] = length;
    sys_send(1, &message as *const _ as usize).unwrap();
    sys_receive(1, &mut message as *mut _ as usize).unwrap();

    Ok(message.args[REPLY_STATUS])
}