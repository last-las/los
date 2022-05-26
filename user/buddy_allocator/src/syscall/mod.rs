mod raw;
use raw::*;
use share::syscall::error::{SysError};

fn isize2result(ret: isize) -> Result<usize, SysError> {
    if ret < 0 {
        Result::Err(SysError::new(-ret as i32))
    } else {
        Result::Ok(ret as usize)
    }
}

pub fn brk(new_brk: Option<usize>) -> Result<usize, SysError> {
    let new_brk = if new_brk.is_some() {new_brk.unwrap()} else { 0 };

    isize2result(sys_brk(new_brk))
}

pub fn write(fd: usize, buf: &[u8]) -> Result<usize, SysError>{
    isize2result(sys_write(fd, buf))
}

pub fn sbi_write(fd: usize, buf: &[u8]) -> isize {
    k_sbi_write(fd, buf)
}

pub fn getpid() -> usize {
    sys_get_pid() as usize
}