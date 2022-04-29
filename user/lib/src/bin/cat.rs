#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use user_lib::env::get_args;
use user_lib::syscall::{open, read, dup, fstat};
use share::file::OpenFlag;
use alloc::vec;

const TERMINAL_READ_SIZE: usize = 512;

#[no_mangle]
fn main() {
    let args = get_args();
    if args.len() == 1 {
        let fd = dup(0).unwrap();
        let mut buffer = [0; TERMINAL_READ_SIZE];
        let size = read(fd, buffer.as_mut()).unwrap();
        let content = core::str::from_utf8(&buffer[0..size]).unwrap();
        print!("{}", content);
    } else {
        for i in 1..args.len() {
            let fd = open(args[i].as_str(), OpenFlag::RDONLY, 0).unwrap();
            let stat = fstat(fd).unwrap();
            let mut content = vec![0; stat.size];
            let size = read(fd, content.as_mut_slice()).unwrap();
            assert_eq!(size, stat.size);
            let content = unsafe { core::str::from_utf8_unchecked(&content[..size]) };
            println!("{}", content);
        }
    }
}