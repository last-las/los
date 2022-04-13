#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::yield_;

#[no_mangle]
fn main() {
    loop {
        yield_();
    }
}