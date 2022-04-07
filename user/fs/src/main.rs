#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod vfs;
mod fs;
mod proc;
mod syscall;

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

#[no_mangle]
fn main() {
    println!("Hello, world!");
}