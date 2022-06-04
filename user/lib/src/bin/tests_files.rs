#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

use user_lib::syscall::{fork,exec,waitpid,exit};
use alloc::vec::Vec;

#[no_mangle]
fn main() {
    let mut files = Vec::new();
    files.push("brk\0");
    files.push("chdir\0");
    //test.push("clone\0");
    files.push("close\0");
    files.push("dup\0");
    files.push("dup2\0");
    files.push("execve\0");
    files.push("exit\0");
    files.push("fork\0");
    //files.push("fstat\0");
    files.push("getcwd\0");
    files.push("getdents\0");
    files.push("getpid\0");
    files.push("getppid\0");
    files.push("gettimeofday\0");
    files.push("mkdir_\0");
    //test.push("mmap\0");
    files.push("mount\0");
    //test.push("munmap\0");
    files.push("open\0");
    files.push("openat\0");
    //files.push("pipe\0");
    files.push("read\0");
    files.push("sleep\0");
    //files.push("times\0");
    files.push("umount\0");
    files.push("uname\0");
    //files.push("unlink\0");
    files.push("wait\0");
    files.push("waitpid\0");
    files.push("write\0");
    files.push("yield\0");
    println!("-- Test mode,Start test --");
    for file in files {
        let pid = fork().unwrap();
            if pid == 0 {
                if exec(file, vec![]).is_err() {
                    println!("Fail to exec {}",file);
                    exit(1);
                }
                exit(0);
            } else {
                waitpid(pid as isize, None,0);
            }
    }
}