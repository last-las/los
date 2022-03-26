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
}

impl Debug for SysError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let info = match self.errno {
            ENOMEM => "ENOMEM: Out of memory",
            _ => "Unknown errno",
        };
        f.write_fmt(format_args!("{}", info))
    }
}

pub const ENOMEM: i32 = 12;
