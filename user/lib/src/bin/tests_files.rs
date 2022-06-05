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
    files.push("/bin/contest/brk");
    files.push("/bin/contest/chdir");
    files.push("/bin/contest/clone");
    files.push("/bin/contest/close");
    files.push("/bin/contest/dup");
    files.push("/bin/contest/dup2");
    files.push("/bin/contest/execve");
    files.push("/bin/contest/exit");
    files.push("/bin/contest/fork");
    files.push("/bin/contest/fstat");
    files.push("/bin/contest/getcwd");
    files.push("/bin/contest/getdents");
    files.push("/bin/contest/getpid");
    files.push("/bin/contest/getppid");
    files.push("/bin/contest/gettimeofday");
    files.push("/bin/contest/mkdir_");
    //test.push("/bin/contest/mmap");
    files.push("/bin/contest/mount");
    //test.push("/bin/contest/munmap");
    files.push("/bin/contest/open");
    files.push("/bin/contest/openat");
    //files.push("/bin/contest/pipe");
    files.push("/bin/contest/read");
    files.push("/bin/contest/sleep");
    //files.push("/bin/contest/times");
    files.push("/bin/contest/umount");
    files.push("/bin/contest/uname");
    //files.push("/bin/contest/unlink");
    files.push("/bin/contest/wait");
    files.push("/bin/contest/waitpid");
    files.push("/bin/contest/write");
    files.push("/bin/contest/yield");
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
                waitpid(pid as isize, None,0).unwrap();
            }
    }
}