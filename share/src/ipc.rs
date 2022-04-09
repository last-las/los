use core::fmt::{Debug, Formatter};

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
}

impl Debug for Msg {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("src_pid:{} mtype: {} args: {:#?}", self.src_pid,self.mtype, self.args))
    }
}