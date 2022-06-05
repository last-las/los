use crate::timer::{get_time_ms, get_time_s, get_time_us};
use share::syscall::error::SysError;

use super::proc::do_yield;

pub fn do_get_time() -> Result<usize, SysError> {
    Ok(get_time_ms())
}

pub fn do_get_time_of_day(time: *mut Timespec) -> Result<usize, SysError> {
    unsafe {
        (*time).tv_sec = get_time_s() as u64;
        (*time).tv_usec = get_time_us() as u64;
    }
    do_get_time()
}

pub fn do_nanosleep(req: *mut Timespec, rem: *mut Timespec) -> Result<usize, SysError> {
    unsafe {
        let end_sec = get_time_s() + (*req).tv_sec as usize;
        let end_usec = get_time_us() + (*req).tv_usec as usize;
        loop {
            let cur_sec = get_time_s();
            let cur_usec = get_time_us();
            if cur_sec >= end_sec && cur_usec >= end_usec {
                return Ok(0);
            } else {
                do_yield();
            }
        }
    }
}

pub struct Timespec {
    pub tv_sec: u64,
    pub tv_usec: u64,
}
