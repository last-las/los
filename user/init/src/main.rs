#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

use user_lib::io::read_line;
use user_lib::syscall::{fork, exec, waitpid, sys_yield, exit, getpid};

#[no_mangle]
fn main() {
    fork_and_exec("terminal"); // pid = 1
    fork_and_exec("idle"); // pid = 2
    fork_and_exec("shell"); // pid = 3
    // fork_and_exec("virtio-blk"); // pid = 4

    loop {
        match waitpid(-1, None, 0) {
            Ok(_) => continue,
            Err(_) => sys_yield(),
        };
    }
}

fn fork_and_exec(path: &str) {
    let ret = fork().unwrap();
    if ret == 0 {
        exec(path, vec![path]).unwrap();
        exit(0); // never reach here.
    }
}