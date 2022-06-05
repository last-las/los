#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use user_lib::syscall::{getcwd, open, get_dents, close, exit};
use share::file::OpenFlag;
use user_lib::env::get_args;
use alloc::string::String;

#[no_mangle]
fn main() {
    let args = get_args();
    let mut path = String::new();
    if args.len() == 1 {
        path = getcwd().unwrap();
    } else if args.len() == 2 {
        path = args[1].clone();
    } else { // This can be solved actually..
        println!("Too many arguments");
        exit(0);
    }
    let fd = open(path.as_str(), OpenFlag::DIRECTORY | OpenFlag::RDONLY, 0).unwrap();
    let dirents = get_dents(fd).unwrap();
    for dirent in dirents {
        println!("{}", dirent.name);
    }
    close(fd).unwrap();
}