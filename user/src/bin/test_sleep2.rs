#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::{sys_yield, sleep};

#[no_mangle]
fn main() {
    sleep(1);
}