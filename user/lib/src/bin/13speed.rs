#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::get_time;

const SENTENCE: &str = "The big brother is watching you!";
// const SENTENCE: &str = "B";
const TIMES: usize = 32;

#[no_mangle]
fn main() {
    let start = get_time();
    for _ in 0..TIMES {
        println!("{}", SENTENCE);
    }
    let end = get_time();
    println!("Gap after print {} times: {}", TIMES, end - start);
}