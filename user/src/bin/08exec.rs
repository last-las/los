#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use user_lib::syscall::{exec, fork};
use alloc::vec::Vec;


#[no_mangle]
fn main() {
    let path = "00hello_world";
    println!("executing {}", path);
    exec(path, Vec::new(), Vec::new()).unwrap();
}