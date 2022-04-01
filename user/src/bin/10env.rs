#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use user_lib::env::{setenv, getenv, cvt_c_like, EnvironVariable};
use user_lib::syscall::{fork, exec, waitpid};
use alloc::vec::Vec;

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
        let envs = cvt_c_like();
        exec("10env_child", Vec::new(),envs.as_slice()).unwrap();
    } else {
        let mut status = 0;
        let ret = waitpid(pid as isize, &mut status, 0).unwrap();
        assert_eq!(ret, pid);
        assert_eq!(status, 23);
    }
}