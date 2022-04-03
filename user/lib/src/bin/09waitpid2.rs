#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::{fork, exit, waitpid, sleep};
use share::syscall::error::ECHILD;

const CHILD_NUM: usize = 5;

#[no_mangle]
fn main() {
    for i in 0..CHILD_NUM {
        let pid = fork().unwrap();
        if pid == 0 {
            exit(i);
        }
    }

    let mut status = 0;
    loop {
        match waitpid(-1, Some(&mut status), 0) {
            Ok(ret) => {
                println!("child pid {} exit with:{}", ret, status);
            },
            Err(e) => {
                assert_eq!(e.errno, ECHILD);
                break;
            }
        }
    }
}