#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::{_read, fork, _write};
use user_lib::termios::{tc_get_attr, tc_set_attr};
use share::terminal::Clflag;

#[no_mangle]
fn main() {
    read_write();

    let mut termios = tc_get_attr(1).unwrap();
    termios.c_lflag.remove(Clflag::ECHO);
    tc_set_attr(1, termios).unwrap();

    read_write();
}

fn read_write() {
    const LENGTH: usize = 50;

    let mut buf = [0; LENGTH];
    let cnt = _read(0, &mut buf).unwrap();
    _write(1, &buf[..cnt]).unwrap();
}