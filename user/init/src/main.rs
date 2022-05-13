#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

use user_lib::io::read_line;
use user_lib::syscall::{fork, exec, waitpid, sys_yield};

#[no_mangle]
fn main() {
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
