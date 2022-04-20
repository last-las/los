#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use core::arch::asm;

#[no_mangle]
fn main() {
    unsafe {
        asm! {
            "li a7, 8",
            "ecall",
        }
    }
}