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
    start_terminal_driver();

    let ret = fork().unwrap();
    if ret == 0 {
        let path = "shell";
        exec(path, vec![path]).unwrap();
    } else {
        loop {
            match waitpid(-1, None, 0) {
                Ok(_) => continue,
                Err(_) => sys_yield(),
            };
        }
    }
}

fn start_terminal_driver() {
    let ret = fork().unwrap();
    if ret == 0 {
        assert_eq!(
            getpid(),
            1
        );
        let path = "terminal";
        exec(path, vec![path]).unwrap();
        exit(0); // never reach here.
    }
}
