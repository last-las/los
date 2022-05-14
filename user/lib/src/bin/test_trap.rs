#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() {
    let addr = 0x0;
    println!("read from addr:{:#x}", addr);
    let val = unsafe {
        (addr as *const u8).read()
    };
    println!("read value is: {}", val);
}