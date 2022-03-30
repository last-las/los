#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use user_lib::syscall::{exec, fork, waitpid};
use alloc::vec::Vec;


#[no_mangle]
fn main() {
    let pid = fork().unwrap();
    if pid == 0 {
        let path = "00hello_world";
        exec(path, Vec::new(), Vec::new()).unwrap();
    } else {
        let mut status= 0;
        let ret = waitpid(pid as isize, &mut status, 0).unwrap();
        assert_eq!(ret, pid);
        println!("Child exit with {}", status);
    }
}
