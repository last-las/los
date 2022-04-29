#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::{getcwd, open, exit, close};
use share::file::OpenFlag;
use user_lib::env::get_args;

#[no_mangle]
fn main() {
    let args = get_args();
    if args.len() == 1 {
        println!("{}: missing file operand", args[0]);
        exit(1);
    }

    for i in 1.. args.len() {
        let fd = open(args[i].as_str(), OpenFlag::CREAT, 0).unwrap();
        close(fd).unwrap();
    }
}