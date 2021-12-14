use crate::task::{fetch_a_task_from_task_manager, exit_current_and_run_next_task, stop_current_and_run_next_task};
use crate::processor::get_hart_id;
use crate::sbi::sbi_console_putchar;
use core::str::from_utf8;
use crate::timer::get_time_ms;

#[allow(unused)]
const SYSCALL_SEND: usize = 1;
#[allow(unused)]
const SYSCALL_RECEIVE: usize = 2;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1], args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as isize),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(),
        _ => {
            panic!("unknown syscall_id {}", syscall_id);
        }
    }
}

pub fn sys_write(fd: usize, buf_ptr: usize, length: usize) -> isize {
    if fd != 1 {
        return -1;
    }
    let buf_ptr = buf_ptr as *const u8;
     let buffer = unsafe {core::slice::from_raw_parts(buf_ptr, length) };
    print!("{}", from_utf8(buffer).unwrap());
    0
}

// TODO: this syscall should be deleted and managed by the SYSTEM_TASK in the future.
pub fn sys_exit(exit_code: isize) -> isize {
    info!("task exit with exit_code:{} on hart:{}", exit_code, get_hart_id());
    exit_current_and_run_next_task();
    0
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_yield() -> isize {
    stop_current_and_run_next_task();
    0
}