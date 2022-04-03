#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use ipc::{Msg, MsgContent};
use user_lib::syscall::{sys_send, sys_yield, sys_receive};

const INFO1: &str = "Hello, this is ipc1";
const INFO2: &str = "Hello, this is ipc2";

#[no_mangle]
fn main() {
    let src_pid = 0;
    let dst_pid = 1;

    // 1. test send and block.
    let msg = Msg::new(src_pid, MsgContent::TestMsg(INFO1));
    sys_yield();
    sys_send(dst_pid, &msg);

    // 2. test receive and block.
    let mut msg = Msg::empty(src_pid);
    sys_receive(dst_pid, &mut msg);
    if let MsgContent::TestMsg(some_str) = msg.content {
        assert_eq!(INFO2, some_str);
        println!("ipc1 received message: {}", some_str);
    } else {
        panic!("didn't receive proper message");
    }
}