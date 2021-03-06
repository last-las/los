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
            EPERM => "EPERM: Operation not permitted",
            ENOENT => "ENOENT: No such file or directory",
            ESRCH => "ESRCH: No such process",
            EIO => "EIO: input/output error",
            ENOEXEC => "ENOEXEC: Exec format error",
            EBADF => "EBADF: fd is not a valid file descriptor or is not open for reading/writing",
            ECHILD => "ECHILD: No child processes",
            EAGAIN => "EAGAIN: Try again",
            ENOMEM => "ENOMEM: Out of memory",
            EACCES => "EACCES: Permission denied",
            EFAULT => "EFAULT: Bad address",
            ENOTBLK => "ENOTBLK: Block device required",
            EBUSY => "EBUSY: Device or resource busy",
            EEXIST => "EEXIST: File exists",
            ENODEV => "ENODEV: No such device",
            ENOTDIR => "ENOTDIR: Not a directory",
            EISDIR => "EISDIR: Is a directory",
            EINVAL => "EINVAL: Invalid argument",
            ENFILE => "ENFILE: File table overflow",
            ERANGE => "ERANGE: The argument is less than the length of the absolute pathname",
            ENAMETOOLONG => "ENAMETOOLONG: File name too long",
            ENOTEMPTY => "ENOTEMPTY: Directory is not empty",

            EUNKOWN => "Unknown error nnn.",
            EDLOCK => "EDLOCK: Ipc dead lock",
            _ => {
                return f.write_fmt(format_args!("Unknown errno: {}", self.errno));
            },
        };
        f.write_fmt(format_args!("{}", info))
    }
}

pub const EPERM: i32 = 1;
pub const ENOENT: i32 = 2;
pub const ESRCH: i32 = 3;
pub const EIO: i32 = 5;
pub const ENOEXEC: i32 = 8;
pub const EBADF: i32 = 9;
pub const ECHILD: i32 = 10;
pub const EAGAIN: i32 = 11;
pub const ENOMEM: i32 = 12;
pub const EACCES: i32 = 13;
pub const EFAULT: i32 = 14;
pub const ENOTBLK: i32 = 15;
pub const EBUSY: i32 = 16;
pub const EEXIST: i32 = 17;
pub const ENODEV: i32 = 19;
pub const ENOTDIR: i32 = 20;
pub const EISDIR: i32 = 21;
pub const EINVAL: i32 = 22;
pub const ENFILE: i32 = 23;
pub const ERANGE: i32 = 34;
pub const ENAMETOOLONG: i32 = 36;
pub const ENOTEMPTY: i32 = 39;

// Self designed error numbers..
pub const EUNKOWN: i32 = 400;
pub const EDLOCK: i32 = 401;