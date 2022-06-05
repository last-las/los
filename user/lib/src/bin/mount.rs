#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::env::get_args;
use user_lib::syscall::{exit, mount};

#[no_mangle]
fn main() {
    let args = get_args();
    if args.len() != 4 {
        println!("{}: bad usage.", args[0]);
        exit(1);
    }

    mount(args[1].as_str(), args[2].as_str(), args[3].as_str(), 0, 0).unwrap();
}