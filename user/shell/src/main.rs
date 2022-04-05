#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

use user_lib::io::read_line;
use user_lib::syscall::{fork, exec, exit, waitpid, debug_frame_usage};

#[no_mangle]
fn main() {
    loop {
        print!("root@los$ ");
        let line = read_line();
        if line.len() == 0 {
            continue;
        }
        if line.as_str() =="frame_usage"  {
            println!("available frames: {:#x}", debug_frame_usage());
            continue;
        }

        let ret = fork().unwrap();
        if ret == 0 {
            if exec(line.as_str(), vec![line.as_str()]).is_err() {
                println!("{}: no such file", line);
                exit(127);
            }
        } else {
            let pid = waitpid(ret as isize, None, 0).unwrap();
            assert_eq!(pid, ret);
        }
    }
}
