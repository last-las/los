#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::*;
use share::file::OpenFlag;
use share::mmap::{Prot, MMAPFlags};

const STR: &str = "  Hello, mmap successfully!";

#[no_mangle]
fn main() {
    let fd = open("test_mmap.txt", OpenFlag::RDWR | OpenFlag::CREAT, 0).unwrap();
    write(fd, STR.as_bytes()).unwrap();
    let stat = fstat(fd).unwrap();
    let ptr =
        mmap(None, stat.size, Prot::WRITE | Prot::READ, MMAPFlags::SHARED, fd, 0).unwrap();

    let mmap_bytes = unsafe {
        core::slice::from_raw_parts(ptr as *const u8, stat.size)
    };
    let mmap_str = core::str::from_utf8(mmap_bytes).unwrap();

    println!("mmap content: {}", mmap_str);
    munmap(ptr, stat.size).unwrap();
    println!("should trigger load page fault");
    println!("mmap content: {}", mmap_str);
}