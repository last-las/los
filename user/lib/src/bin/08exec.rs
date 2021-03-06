#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

use user_lib::syscall::exec;


#[no_mangle]
fn main() {
    let path = "00hello_world";
    println!("executing {}", path);
    exec(path,vec![]).unwrap();
}