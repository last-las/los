#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

use user_lib::syscall::{fork, exec, waitpid};

#[no_mangle]
fn main() {
    let ret = fork().unwrap();
    if ret == 0 { // child
        exec("00hello_world", vec![]);
    } else { // father
        let val = waitpid(ret as isize, None, 0).unwrap();
        assert_eq!(val, ret);
        println!("child process has exited.");
    }
}