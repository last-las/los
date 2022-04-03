#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::{fork, getpid, getppid, waitpid};


#[no_mangle]
fn main() {
    let pid = fork().unwrap();
    if pid == 0 {
        println!("child output: child pid is {}", getpid());
        println!("child output: parent pid is {}", getppid());
    } else {
        println!("parent output: child pid is {}", pid);
        println!("parent output: parent pid is {}", getpid());
        waitpid(pid as isize, None, 0).unwrap();
    }
}
