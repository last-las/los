#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

use user_lib::syscall::*;
use share::file::OpenFlag;
use user_lib::env::setenv;
use alloc::vec::Vec;

#[no_mangle]
fn main() {
    set_priority(0, 0, 3).unwrap();
    setenv("PATH", "/bin:/usr/bin:", true);
    sleep(1); // wait for fs server init.
    open_standard_fd();
    mount_fatfs_on("/bin");
    mkdir_at(0,"/bin/mnt",0).unwrap();
    chdir("/bin");

    test_all();

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

fn mount_fatfs_on(path: &str) {
    mkdir_at(0, path, 0).unwrap();
    //mkdir_at(0,"/bin/contest",0).unwrap();
    mount("/dev/sda2", path, "vfat", 0, 0).unwrap();
}
fn test_all() {
    let mut files = Vec::new();
    files.push("brk");
    files.push("chdir");
    files.push("clone");
    files.push("close");
    files.push("dup");
    files.push("dup2");
    files.push("execve");
    files.push("exit");
    files.push("fork");
    files.push("fstat");
    files.push("getcwd");
    // files.push("getdents"); file system panic
    files.push("getpid");
    files.push("getppid");
    files.push("gettimeofday");
    files.push("mkdir_");
    //test.push("/bin/mmap");
    files.push("mount");
    //test.push("/bin/munmap");
    files.push("open");
    files.push("openat");
    //files.push("/bin/pipe");
    files.push("read");
    files.push("sleep");
    //files.push("/bin/times");
    files.push("umount");
    files.push("uname");
    //files.push("unlink");
    files.push("wait");
    files.push("waitpid");
    files.push("write");
    files.push("yield");
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
