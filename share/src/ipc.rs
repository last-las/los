use core::fmt::{Debug, Formatter};

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
pub struct Msg {
    pub src_pid: usize,
    pub args: [usize; 6]
}

impl Msg {
    pub const fn empty() -> Self {
        Self {
            src_pid: 0,
            args: [0; 6],
        }
    }
}

impl Debug for Msg {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("args: {:#?}", self.args))
    }
}