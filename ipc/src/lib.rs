#![no_std]

#[repr(C)]
pub struct Msg<'a> {
    src_pid: usize,
    pub content: MsgContent<'a>,
}

impl<'a> Msg<'a> {
    pub fn new(src_pid: usize, content: MsgContent<'a>) -> Self {
        Self {
            src_pid,
            content
        }
    }

    pub fn empty(src_pid: usize) -> Self {
        Self {
            src_pid,
            content: MsgContent::EMPTY,
        }
    }
}

pub enum MsgContent<'a> {
    TestMsg(&'a str),
    EMPTY,
}