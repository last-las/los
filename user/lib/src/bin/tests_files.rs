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
    files.push("/bin/brk");
    files.push("/bin/chdir");
    files.push("/bin/clone");
    files.push("/bin/close");
    files.push("/bin/dup");
    files.push("/bin/dup2");
    files.push("/bin/execve");
    files.push("/bin/exit");
    files.push("/bin/fork");
    files.push("/bin/fstat");
    files.push("/bin/getcwd");
    files.push("/bin/getdents");
    files.push("/bin/getpid");
    files.push("/bin/getppid");
    files.push("/bin/gettimeofday");
    files.push("/bin/mkdir_");
    //test.push("/bin/contest/mmap");
    files.push("/bin/mount");
    //test.push("/bin/contest/munmap");
    files.push("/bin/open");
    files.push("/bin/openat");
    //files.push("/bin/contest/pipe");
    files.push("/bin/read");
    files.push("/bin/sleep");
    //files.push("/bin/contest/times");
    files.push("/bin/umount");
    files.push("/bin/uname");
    //files.push("/bin/contest/unlink");
    files.push("/bin/wait");
    files.push("/bin/waitpid");
    files.push("/bin/write");
    files.push("/bin/yield");
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