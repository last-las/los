use core::str::from_utf8;
use share::syscall::error::{EBADF, SysError};
use crate::sbi::sbi_console_getchar;
use crate::task::stop_current_and_run_next_task;

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