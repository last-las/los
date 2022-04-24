#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::{yield_, getppid, send};
use share::ipc::{Msg, FORK, FS_SYSCALL_ARG0, FORK_PARENT};

#[no_mangle]
fn main() {
    send_fork_message();
}

fn send_fork_message() {
    let mut message = Msg::empty();
    message.mtype = FORK;
    message.args[FORK_PARENT] = getppid();
    send(5, &message).unwrap();
}
