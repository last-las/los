#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use alloc::vec::Vec;
use alloc::boxed::Box;
use user_lib::syscall::{fork, getppid, receive, waitpid, send, yield_, exit, sleep};
use share::ipc::Msg;

static mut MESSAGE: Msg = Msg::empty();
const NUM: usize = 3;

#[no_mangle]
fn main() {
    init_message();
    test_send_before_receive();
    test_receive_before_send();
    test_receive_after_multiple_send();
    test_receive_before_multiple_send();
}

fn init_message() {
    unsafe {
        for i in 0..MESSAGE.args.len() {
            MESSAGE.args[i] = i * i + 1;
        }
    }
}

fn test_send_before_receive() {
    let ret = fork().unwrap();
    if ret == 0 {
        yield_();
        let ppid = getppid() as isize;
        let mut msg = Msg::empty();
        receive(ppid, &mut msg).unwrap();
        unsafe {
            assert_eq!(MESSAGE, msg);
        }
        exit(0);
    } else {
        let msg = unsafe { MESSAGE.clone() };
        send(ret, &msg);
        waitpid(ret as isize, None, 0).unwrap();
        println!("test_send_before_receive success!");
    }
}

fn test_receive_before_send() {
    let ret = fork().unwrap();
    if ret == 0 {
        let ppid = getppid() as isize;
        let mut msg = Msg::empty();
        receive(ppid, &mut msg).unwrap();
        unsafe {
            assert_eq!(msg, MESSAGE);
        }
        exit(0);
    } else {
        yield_();
        let msg = unsafe { MESSAGE.clone() };
        send(ret, &msg);
        waitpid(ret as isize, None, 0).unwrap();
        println!("test_receive_before_send success!");
    }
}

fn test_receive_after_multiple_send() {
    let mut ret = 0;
    for _ in 0..NUM {
        ret = fork().unwrap();
        if ret == 0 {
            let ppid = getppid();
            let msg = unsafe {
                MESSAGE.clone()
            };
            send(ppid, &msg).unwrap();
            exit(0);
        }
    }

    sleep(1);
    for _ in 0..NUM {
        let mut msg = Msg::empty();
        receive(-1, &mut msg).unwrap();
        unsafe {
            assert_eq!(msg, MESSAGE);
        }
        waitpid(-1, None, 0);
    }

    println!("test_receive_after_multiple_send success!");
}

fn test_receive_before_multiple_send() {
    let mut ret = 0;
    for _ in 0..NUM {
        ret = fork().unwrap();
        if ret == 0 {
            sleep(1);
            let ppid = getppid();
            let msg = unsafe {
                MESSAGE.clone()
            };
            send(ppid, &msg).unwrap();
            exit(0);
        }
    }

    for _ in 0..NUM {
        let mut msg = Msg::empty();
        receive(-1, &mut msg).unwrap();
        unsafe {
            assert_eq!(msg, MESSAGE);
        }
        waitpid(-1, None, 0);
    }

    println!("test_receive_before_multiple_send success!");
}