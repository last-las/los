#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

use user_lib::syscall::{yield_, getppid, send, open, __write, __read, close, get_dents, mkdir_at, getcwd, chdir};
use share::ipc::{Msg, FORK, FS_SYSCALL_ARG0, FORK_PARENT};
use share::file::{OpenFlag, AT_FD_CWD};
use share::syscall::error::SysError;
use alloc::string::{String};
use alloc::string::ToString;

const BUF_SIZE: usize = 64;

#[no_mangle]
fn main() {
    send_fork_message();
    mk_tmp_dir();

    test_read_write();
    test_get_dirent_at();
    test_mkdir_at();
    test_getcwd_and_chdir();
}

fn send_fork_message() {
    let mut message = Msg::empty();
    message.mtype = FORK;
    message.args[FORK_PARENT] = 5;
    send(5, &message).unwrap();
}

fn test_read_write() {
    println!("[ test_read_write] start");
    // write content
    let fd = open("/test1.txt",OpenFlag::CREAT | OpenFlag::WRONLY, 0).unwrap();
    let mut w_buf = [8; BUF_SIZE];
    __write(fd, &w_buf).unwrap();
    assert!(__read(fd, &mut w_buf).is_err());
    close(fd).unwrap();
    // println!("write content success");

    // read content
    let fd = open("/test1.txt",OpenFlag::CREAT | OpenFlag::RDONLY, 0).unwrap();
    let mut r_buf = [0; BUF_SIZE];
     __read(fd, &mut r_buf).unwrap();
    assert!(__write(fd, &r_buf).is_err());
    close(fd).unwrap();
    // println!("read content success");

    assert_eq!(w_buf, r_buf);
    println!("[ test_read_write] end\n");
}

fn test_get_dirent_at() {
    println!("[ test_get_dirent_at] start");
    for i in 0..10 {
        let path = format!("/x{}.txt", i);
        let fd = open(path.as_str(), OpenFlag::CREAT, 0).unwrap();
        close(fd).unwrap();
    }

    let fd = open("/", OpenFlag::DIRECTORY, 0).unwrap();
    let dirents = get_dents(fd).unwrap();
    for dirent in dirents {
        println!("{}", dirent.name);
    }
    close(fd).unwrap();

    println!("[ test_get_dirent_at] end\n");
}

fn test_mkdir_at() {
    println!("[test_mkdir_at] start");
    let tmp_fd = open("/tmp", OpenFlag::DIRECTORY | OpenFlag::RDWR, 0).unwrap();
    // 1. relative path
    mkdir_at(tmp_fd, "relative_dir1", 0).unwrap();
    // 2. relative path but AT_FDCWD
    mkdir_at(AT_FD_CWD as usize, "relative_dir2", 0).unwrap();
    // 3. absolute path
    mkdir_at(tmp_fd, "/tmp/absolute_dir", 0).unwrap();
    close(tmp_fd).unwrap();

    println!("list /tmp dir:");
    list_dir("/tmp");
    println!("list / dir:");
    list_dir("/");
    println!("[test_mkdir_at] end\n");
}

fn test_getcwd_and_chdir() {
    println!("[test_getcwd_and_chdir] start");
    assert_eq!(getcwd().unwrap().as_str(), "/");
    chdir("/tmp/").unwrap();
    assert_eq!(getcwd().unwrap().as_str(), "/tmp");
    chdir("./relative_dir1").unwrap();
    assert_eq!(getcwd().unwrap().as_str(), "/tmp/relative_dir1");
    chdir("../absolute_dir").unwrap();
    assert_eq!(getcwd().unwrap().as_str(), "/tmp/absolute_dir");
    chdir("../").unwrap();
    assert_eq!(getcwd().unwrap().as_str(), "/tmp");
    chdir("../").unwrap();
    assert_eq!(getcwd().unwrap().as_str(), "/");
    chdir("/relative_dir2").unwrap();
    assert_eq!(getcwd().unwrap().as_str(), "/relative_dir2");
    // test complicated path name
    chdir("////tmp/////").unwrap();
    assert_eq!(getcwd().unwrap().as_str(), "/tmp");
    chdir("/../../../../../tmp").unwrap();
    assert_eq!(getcwd().unwrap().as_str(), "/tmp");
    chdir("././././././././").unwrap();
    assert_eq!(getcwd().unwrap().as_str(), "/tmp");

    println!("[test_getcwd_and_chdir] end");
}

fn list_dir(path: &str) {
    let fd = open(path, OpenFlag::DIRECTORY | OpenFlag::RDONLY, 0).unwrap();
    let dirents = get_dents(fd).unwrap();
    for dirent in dirents {
        println!("{}", dirent.name);
    }
    close(fd).unwrap();
}

fn mk_tmp_dir() {
    let fd = open("/", OpenFlag::DIRECTORY | OpenFlag::WRONLY, 0).unwrap();
    mkdir_at(fd, "tmp", 0).unwrap();
    close(fd).unwrap();
}
