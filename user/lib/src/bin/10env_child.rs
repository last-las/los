#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::env::{getenv, get_args};

const SIZE: usize = 3;
const NAME: [&str; SIZE] = [ "PATH", "USER", "SHELL"];

#[no_mangle]
fn main() {
    println!("print child task env and arg:");
    println!("env:");
    for i in 0..SIZE {
        println!("{}={}", NAME[i], getenv(NAME[i]).unwrap());
    }

    println!("\nargs: {:#?}", get_args());
}