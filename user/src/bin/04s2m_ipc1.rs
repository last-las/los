#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use ipc::{Msg, MsgContent};
use user_lib::syscall::{sys_receive, sys_yield, sys_send};

const CONSUMER_NUMBER: usize = 3;

const MESSAGES: [&str; CONSUMER_NUMBER] =
    [
        "Hi ipc2, this is a message",
        "Hi ipc3, this is a message",
        "Hi ipc4, this is a message"
    ];

#[no_mangle]
fn main() {
    let src_pid = 0;

    for i in 0..CONSUMER_NUMBER {
        let mut msg = Msg::new(src_pid, MsgContent::TestMsg(MESSAGES[i]));
        sys_send(i + 1, &msg);
    }
}