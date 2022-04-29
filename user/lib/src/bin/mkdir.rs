#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::env::get_args;
use user_lib::syscall::{exit, mkdir_at, getcwd, open, close};
use share::file::OpenFlag;

#[no_mangle]
fn main() {
    let args = get_args();
    if args.len() == 1 {
        println!("{}: missing operand", args[0]);
        exit(0);
    }

    let cwd = getcwd().unwrap();
    let fd = open(cwd.as_str(), OpenFlag::DIRECTORY, 0).unwrap();
    for i in 1..args.len() {
        let path = args[i].as_str();
        mkdir_at(fd, path, 0).unwrap();
    }

    close(fd);
}