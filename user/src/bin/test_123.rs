#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::{sys_test, sys_exit};

#[no_mangle]
fn main() {
    assert_eq!(sys_test(), 1234);
    println!("{}", sys_test());
    sys_exit(1234);
}