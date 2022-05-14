#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::brk;


#[no_mangle]
fn main() {
    let base = brk(None).unwrap();
    brk(Some(base + 0x2000)).unwrap();
    let addr = base + 0x1234;
    let val = 0xdeadbeef;
    println!("write {:#x} to addr {:#x}", val, addr);
    unsafe {
        (addr as * mut u32).write(val);
    }
    let read_val = unsafe {
        (addr as * const u32).read()
    };
    println!("read from addr {:#x}: {:#x}", addr, read_val);
}