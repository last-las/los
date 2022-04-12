#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::{_read, fork};

#[no_mangle]
fn main() {
    let mut buf = [0; 100];
    let cnt = _read(1, &mut buf).unwrap();
    let s = core::str::from_utf8(&buf[..cnt]).unwrap();
    println!("read result:{}", s);
}