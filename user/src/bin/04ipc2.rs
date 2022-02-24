#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use ipc::{Msg, MsgContent};
use user_lib::syscall::{sys_receive, sys_yield, sys_send};

const INFO1: &str = "Hello, this is ipc1";
const INFO2: &str = "Hello, this is ipc2";

#[no_mangle]
fn main() {
    let src_pid = 1;
    let dst_pid = 0;

    // 1. test receive but not block.
    let mut msg = Msg::empty(src_pid);
    sys_yield();
    sys_receive(dst_pid, &mut msg);
    if let MsgContent::TestMsg(some_str) = msg.content {
        assert_eq!(INFO1, some_str);
        println!("ipc2 received message: {}", some_str);
    } else {
        panic!("didn't receive proper message");
    }

    // 2. test send but not block.
    sys_yield();
    let msg = Msg::new(src_pid, MsgContent::TestMsg(INFO2));
    sys_send(dst_pid, &msg);
}