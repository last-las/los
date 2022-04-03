#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use ipc::{Msg, MsgContent};
use user_lib::syscall::{sys_receive, sys_yield, sys_send};

const MESSAGE: &str = "Hi, this is message from ipc3";
#[no_mangle]
fn main() {
    let src_pid = 2;
    let dst_pid = 0;

    let msg = Msg::new(src_pid, MsgContent::TestMsg(MESSAGE));
    sys_send(dst_pid, &msg);
}