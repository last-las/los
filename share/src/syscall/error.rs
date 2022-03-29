use core::fmt::{Debug, Formatter};

pub struct SysError {
    pub errno: i32,
}

impl SysError {
    pub fn new(errno: i32) -> Self {
        Self {
            errno
        }
    }

    pub fn mux(result: Result<usize, SysError>) -> usize{
        match result {
            Ok(value) => value,
            Err(error) => -error.errno as usize
        }
    }
}

impl Debug for SysError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let info = match self.errno {
            EBADF => "EBADF: Bad file number",
            EAGAIN => "EAGAIN: Try again",
            ENOMEM => "ENOMEM: Out of memory",
            EACCES => "EACCES: Permission denied",
            EINVAL => "EINVAL: Invalid argument",
            _ => "Unknown errno",
        };
        f.write_fmt(format_args!("{}", info))
    }
}

pub const EBADF: i32 = 9;
pub const EAGAIN: i32 = 11;
pub const ENOMEM: i32 = 12;
pub const EACCES: i32 = 13;
pub const EINVAL: i32 = 22;