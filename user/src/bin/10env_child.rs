#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::env::{setenv, getenv, cvt_c_like, EnvironVariable};
use user_lib::syscall::{fork, exit};

const SIZE: usize = 3;
const NAME: [&str; SIZE] = [ "PATH", "USER", "SHELL"];
const VALUE: [&str; SIZE] = [ "/user/bin", "ROOT", "/bin/bash"];

#[no_mangle]
fn main() {
    println!("env_child:");
    for i in 0..SIZE {
        println!("{}={}", NAME[i], getenv(NAME[i]).unwrap());
    }
    exit(23);
}