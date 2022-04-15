use core::fmt::{Debug, Formatter};
use crate::syscall::error::SysError;

/* Message Type */
pub const INTERRUPT: usize = 1;
pub const OPEN: usize = 2;
pub const READ: usize = 3;
pub const WRITE: usize = 4;
pub const IOCTL: usize = 5;
pub const CLOSE: usize = 6;
pub const REPLY: usize = 7;

/* Args position constant */
pub const MSG_ARGS_0: usize = 0;
pub const MSG_ARGS_1: usize = 1;
pub const MSG_ARGS_2: usize = 2;
pub const MSG_ARGS_3: usize = 3;
pub const MSG_ARGS_4: usize = 4;
pub const MSG_ARGS_5: usize = 5;

/* read write message FOR block and character device drivers */
pub const DEVICE: usize = MSG_ARGS_0;
pub const PROC_NR: usize = MSG_ARGS_1;
pub const BUFFER: usize = MSG_ARGS_2;
pub const LENGTH: usize = MSG_ARGS_3;
pub const POSITION: usize =MSG_ARGS_4;
/* ioctl message */
pub const IOCTL_TYPE: usize = MSG_ARGS_2;
pub const ADDRESS: usize = MSG_ARGS_3;
/* Reply message */
pub const REPLY_PROC_NR: usize = MSG_ARGS_0;
pub const REPLY_STATUS: usize = MSG_ARGS_1;
pub const STATUS_OK: usize = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Msg {
    pub src_pid: usize,
    pub mtype: usize,
    pub args: [usize; 6]
}

impl PartialEq for Msg {
    fn eq(&self, other: &Self) -> bool {
        self.mtype == other.mtype && self.args == other.args
    }
}

impl Msg {
    pub const fn empty() -> Self {
        Self {
            src_pid: 0,
            mtype: 0,
            args: [0; 6],
        }
    }

    pub fn cvt_reply_message_to_result(&self) -> Result<usize, SysError> {
        assert_eq!(self.mtype, REPLY);
        let status = self.args[REPLY_STATUS] as isize;
        if status >= 0 {
            Ok(status as usize)
        } else {
            Err(SysError::new(status as i32))
        }
    }
}

impl Debug for Msg {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("src_pid:{} mtype: {} args: {:#?}", self.src_pid,self.mtype, self.args))
    }
}