mod raw;
mod error;

pub use raw::*;
pub use error::SysError;
use crate::syscall::error::ENOMEM;

pub fn exit(exit_code: usize) -> isize {
    sys_exit(exit_code)
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

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

pub fn brk(new_brk: Option<usize>) -> Result<usize, SysError> {
    let new_brk = if new_brk.is_some() {new_brk.unwrap()} else { 0 };
    let ret = sys_brk(new_brk);

    if ret == -1 {
        Result::Err(SysError::new(ENOMEM))
    } else {
        Result::Ok(ret as usize)
    }
}