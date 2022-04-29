#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

use user_lib::io::read_line;
use user_lib::syscall::{fork, exec, waitpid, sys_yield, exit, getpid, sleep, mkdir_at, mount, open, get_dents, close};
use share::file::OpenFlag;
use user_lib::env::setenv;

#[no_mangle]
fn main() {
    setenv("PATH", "/bin:/usr/bin:", true);
    sleep(1); // wait for fs server init.
    open_standard_fd();
    mount_ezfs_on("/bin");
    fork_and_exec("/bin/idle");
    fork_and_exec("/bin/shell");

    loop {
        match waitpid(-1, None, 0) {
            Ok(_) => continue,
            Err(_) => sys_yield(),
        };
    }
}

fn open_standard_fd() {
    let fd = open("/dev/console", OpenFlag::RDONLY, 0).unwrap();
    assert_eq!(fd, 0);
    let fd = open("/dev/console", OpenFlag::WRONLY, 0).unwrap();
    assert_eq!(fd, 1);
    let fd = open("/dev/console", OpenFlag::WRONLY, 0).unwrap();
    assert_eq!(fd, 2);
}

fn mount_ezfs_on(path: &str) {
    mkdir_at(0, path, 0).unwrap();
    mount("/dev/sda2", path, "ezfs", 0, 0).unwrap();
}

fn fork_and_exec(path: &str) {
    let ret = fork().unwrap();
    if ret == 0 {
        exec(path, vec![path]).unwrap();
        exit(0); // never reach here.
    }
}