#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

use user_lib::env::get_args;
use user_lib::syscall::{exit, fork, exec, debug_schedule_record_enable, debug_schedule_record_print, waitpid};
use alloc::string::String;

#[no_mangle]
fn main() {
    let apps = get_args();
    if apps.len() == 1 {
        println!("{}: usage: {} app1 app2 app3...", apps[0], apps[0]);
        exit(1);
    }

    debug_schedule_record_enable(1);
    for i in 1..apps.len() {
        fork_and_exec(&apps[i]);
    }
    for _ in 1..apps.len() {
        waitpid(-1, None, 0).unwrap();
    }
    debug_schedule_record_print();
    debug_schedule_record_enable(0);
}

fn fork_and_exec(name: &String) {
    let pid = fork().unwrap();
    if pid == 0 {
        exec(name, vec![]).unwrap();
    }
}