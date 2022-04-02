#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::env::{setenv, getenv, get_args};
use user_lib::syscall::{fork, exit};

const SIZE: usize = 3;
const NAME: [&str; SIZE] = [ "PATH", "USER", "SHELL"];
const VALUE: [&str; SIZE] = [ "/user/bin", "ROOT", "/bin/bash"];

#[no_mangle]
fn main() {
    println!("print child task env and arg:");
    println!("env:");
    for i in 0..SIZE {
        println!("{}={}", NAME[i], getenv(NAME[i]).unwrap());
    }

    println!("\nargs: {:#?}", get_args());
    exit(23);
}