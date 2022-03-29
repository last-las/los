use crate::timer::get_time_ms;
use share::syscall::error::SysError;

pub fn do_get_time() -> Result<usize, SysError>{
    Ok(get_time_ms())
}


