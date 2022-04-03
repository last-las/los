#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::{set_priority, sleep};

#[no_mangle]
fn main() {
    set_priority(0, 0, 0).unwrap();
    println!("Other tasks should not be running until this function is finished/blocked!");
    sleep(5);
}