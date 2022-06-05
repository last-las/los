#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use user_lib::io::read_line;
use user_lib::syscall::{fork, exec, exit, waitpid, debug_frame_usage, getcwd, chdir, open, write, close, dup};
use share::terminal::{Termios, Clflag};
use user_lib::termios::tc_set_attr;
use alloc::vec::Vec;
use user_lib::env::get_args;
use share::file::OpenFlag;

#[no_mangle]
fn main() {
    let mut shell_termios = Termios::default();
    shell_termios.c_lflag.remove(Clflag::ECHO);
    tc_set_attr(1, shell_termios).unwrap();

    loop {
        let cur_dir = getcwd().unwrap();
        print!("root@los:{}$ ", cur_dir);
        let line = read_line();
        let args: Vec<&str> = line.split_whitespace().collect();
        if args.len() == 0 {
            continue;
        }

        if handle_inner_command(&args) {
            continue;
        }

        let ret = fork().unwrap();
        if ret == 0 {
            tc_set_attr(1, Termios::default()).unwrap();
            if exec(args[0], args).is_err() {
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

fn handle_inner_command(args: &Vec<&str>) -> bool {
    if args[0] == "frame_usage" {
        println!("available frames: {:#x}", debug_frame_usage());
        return true;
    }

    if args[0] == "cd" {
        if args.len() != 2 {
            println!("{}: cd: wrong arguments", get_args()[0].as_str());
        } else if chdir(args[1]).is_err() {
            println!("{}: cd: {}: No such file or directory", get_args()[0].as_str(), args[1]);
        }
        return true;
    } else if args[0] == "echo" {
        let len = args.len();
        if len == 4 && args[2] == ">" { // No pipe syscall right now, use a stupid method instead.
            let fd = open(args[3], OpenFlag::CREAT | OpenFlag::WRONLY, 0).unwrap();
            write(fd, args[1].as_bytes()).unwrap();
            close(fd).unwrap();
        } else if len == 2 {
            let fd = dup(1).unwrap();
            write(fd, args[1].as_bytes()).unwrap();
            close(fd).unwrap();
        }
        return true;
    }

    return false;
}
