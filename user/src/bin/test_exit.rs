#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::sys_exit;

#[no_mangle]
fn main() {
    sys_exit(0);
}