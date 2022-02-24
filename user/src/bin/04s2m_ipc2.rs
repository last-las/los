#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use ipc::{Msg, MsgContent};
use user_lib::syscall::{sys_receive, sys_yield, sys_send};

const MESSAGE: &str = "Hi ipc2, this is a message";
#[no_mangle]
fn main() {
    let src_pid = 1;
    let dst_pid = 0;

    let mut msg = Msg::empty(src_pid);
    sys_receive(dst_pid, &mut msg);
    match msg.content {
        MsgContent::TestMsg(message) => {
            assert_eq!(message, MESSAGE);
            println!("ipc1 received message: {}", message);
        },
        _ => {
            panic!("didn't receive proper message");
        }
    }
}