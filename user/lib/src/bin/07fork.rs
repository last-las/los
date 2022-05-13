#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::{fork, sleep};

const LEN: usize = 100;

#[no_mangle]
fn main() {
    let pid = fork().expect("fork error!");
    if pid == 0 {
        println!("This is father");
    }else {
        println!("This is son, my pid is:{}", pid);
    }
}
