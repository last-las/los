#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

use user_lib::io::read_line;
use user_lib::syscall::{fork, exec, exit, waitpid, debug_frame_usage};
use share::terminal::{Termios, Ciflag, Clflag};
use user_lib::termios::tc_set_attr;

#[no_mangle]
fn main() {
    let mut shell_termios = Termios::default();
    shell_termios.c_lflag.remove(Clflag::ECHO);
    tc_set_attr(1, shell_termios).unwrap();

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
            tc_set_attr(1, Termios::default()).unwrap();
            if exec(line.as_str(), vec![line.as_str()]).is_err() {
                println!("{}: no such file", line);
                exit(127);
            }
        } else {
            let pid = waitpid(ret as isize, None, 0).unwrap();
            assert_eq!(pid, ret);
            tc_set_attr(1, shell_termios).unwrap(); // reset default shell termios.
        }
    }
}
