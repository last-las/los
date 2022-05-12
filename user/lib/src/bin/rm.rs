#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::env::get_args;
use user_lib::syscall::{exit, unlink};

#[no_mangle]
fn main() {
    let args = get_args();
    if args.len() != 2 {
        println!("{}: bad usage.", args[0]);
        exit(1);
    }

    unlink(args[1].as_str()).unwrap();
}