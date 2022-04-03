#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use ipc::{Msg, MsgContent};
use user_lib::syscall::{sys_receive, sys_yield};

const PRODUCER_NUMBER: usize = 3;

const MESSAGES: [&str; PRODUCER_NUMBER] =
    [
        "Hi, this is message from ipc2",
        "Hi, this is message from ipc3",
        "Hi, this is message from ipc4"
    ];

#[no_mangle]
fn main() {
    let src_pid = 0;
    let dst_pid = usize::MAX;

    let mut msg = Msg::empty(src_pid);
    for i in 0..PRODUCER_NUMBER {
        sys_receive(dst_pid, &mut msg);
        if let MsgContent::TestMsg(message) = msg.content {
            assert_eq!(message, MESSAGES[i]);
            println!("ipc1 received message: {}", message);
        } else {
            panic!("didn't receive proper message");
        }
    }
}