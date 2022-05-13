#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

use user_lib::env::{setenv, getenv, get_envp_copy, EnvironVariable};
use user_lib::syscall::{fork, exec, waitpid};
use alloc::vec::Vec;
use alloc::string::String;

const SIZE: usize = 3;
const NAME: [&str; SIZE] = [ "PATH", "USER", "SHELL"];
const VALUE: [&str; SIZE] = [ "/user/bin", "ROOT", "/bin/bash"];

#[no_mangle]
fn main() {
    for i in 0..SIZE {
        setenv(NAME[i], VALUE[i], true);
    }

    let pid = fork().unwrap();
    if pid == 0 {
        exec("10env_child", vec!["-hello", "me"]).unwrap();
    } else {
        let mut status = 0;
        let ret = waitpid(pid as isize, Some(&mut status), 0).unwrap();
        assert_eq!(ret, pid);
        assert_eq!(status, 0);
    }
}