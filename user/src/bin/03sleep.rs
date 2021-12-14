#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::{sleep, get_time};

const SECONDS: usize = 5;

#[no_mangle]
fn main() {
    println!("sleep for {} seconds", SECONDS);
    let start_time = get_time();
    sleep(SECONDS);
    println!("start time: {}", start_time);
    println!("end time: {}", get_time());
    println!("test 03sleep ok!");
}