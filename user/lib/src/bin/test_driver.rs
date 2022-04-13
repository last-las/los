#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::{_read, fork, _write};

#[no_mangle]
fn main() {
    const LENGTH: usize = 5;

    let mut buf = [0; LENGTH];
    let cnt = _read(0, &mut buf).unwrap();
    _write(1, &buf[..cnt]).unwrap();
}