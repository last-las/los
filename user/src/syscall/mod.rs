mod raw;

pub use raw::*;
use share::syscall::error::{SysError, ENOMEM};
use alloc::vec::Vec;
use alloc::string::String;
use crate::env::get_envp_copy;

fn isize2result(ret: isize) -> Result<usize, SysError> {
    if ret < 0 {
        Result::Err(SysError::new(-ret as i32))
    } else {
        Result::Ok(ret as usize)
    }
}

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

    isize2result(sys_brk(new_brk))
}

pub fn fork() -> Result<usize, SysError> {
    isize2result(sys_fork(0, 0, 0, 0, 0))
}

#[allow(unused_variables)]
pub fn exec(path: &str, mut args: Vec<String>) -> Result<usize, SysError> {
    let mut s = String::from(path);
    s.push('\0');
    let path_ptr = s.as_ptr() as usize;

    let mut c_like_args = Vec::new();
    args.iter_mut().map(|string| {
        string.push('\0');
        c_like_args.push(string.as_ptr() as usize);
    });
    c_like_args.push(0);
    let argv_ptr = c_like_args.as_ptr() as usize;

    let envp = get_envp_copy();
    isize2result(sys_exec(path_ptr, argv_ptr, envp.as_ptr() as usize))
}

pub fn waitpid(pid: isize, status: &mut usize, options: usize) -> Result<usize, SysError> {
    let status_ptr = status as *mut usize as usize;
    isize2result(sys_waitpid(pid as usize, status_ptr, options))
}