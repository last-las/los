// TODO: wrapper
pub use crate::syscall::syscall::{sys_exit, sys_get_time, sys_yield, sys_test};

mod syscall;

pub use syscall::sys_write;

pub fn exit(exit_code: usize) -> isize {
    sys_exit(exit_code)
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

// TODO: implement this function with signal.
pub fn sleep(seconds: usize) {
    let start_time = get_time();
    let mseconds = seconds * 1000;
    loop {
        let current_time = get_time();
        if current_time < mseconds + start_time {
            sys_yield();
        } else {
            break;
        }
    }
}

pub fn get_time() -> usize {
    sys_get_time() as usize
}
