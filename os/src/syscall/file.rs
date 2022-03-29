use core::str::from_utf8;
use share::syscall::error::{EBADF, SysError};

pub fn do_write(fd: usize, buf_ptr: usize, length: usize) -> Result<usize, SysError>{
    if fd != 1 {
        return Err(SysError::new(EBADF));
    }
    let buf_ptr = buf_ptr as *const u8;
    let buffer = unsafe {
        core::slice::from_raw_parts(buf_ptr, length)
    };
    print!("{}", from_utf8(buffer).unwrap());
    Ok(0)
}
