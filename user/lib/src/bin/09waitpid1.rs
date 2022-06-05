#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

use user_lib::syscall::{exec, fork, waitpid};

#[no_mangle]
fn main() {
    let pid = fork().unwrap();
    if pid == 0 {
        let path = "00hello_world";
        exec(path, vec![]).unwrap();
    } else {
        let mut status= 0;
        let ret = waitpid(pid as isize, Some(&mut status), 0).unwrap();
        assert_eq!(ret, pid);
        println!("Child exit with {}", status);
    }
}
