#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::syscall::{yield_, set_priority};

#[no_mangle]
fn main() {
    set_priority(0, 0, 7).unwrap();
    loop {
        yield_();
    }
}