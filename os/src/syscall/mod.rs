mod ipc;
mod proc;
mod mm;
mod file;
mod time;

use crate::syscall::proc::*;
use crate::syscall::mm::do_brk;
use crate::syscall::file::do_write;
use crate::syscall::time::do_get_time;
use crate::syscall::ipc::{sys_receive, sys_send};
use crate::task::stop_current_and_run_next_task;

const SYSCALL_SEND: usize = 1;
const SYSCALL_RECEIVE: usize = 2;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_PRIORITY: usize = 140;
const SYSCALL_SET_PRIORITY: usize = 141;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_GETPPID: usize = 173;
const SYSCALL_BRK: usize = 214;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 260;
const SYSCALL_TEST: usize = 1234;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_SEND => sys_send(args[0], args[1]),
        SYSCALL_RECEIVE => sys_receive(args[0], args[1]),

        SYSCALL_WRITE => do_write(args[0], args[1], args[2]),
        SYSCALL_EXIT => do_exit(args[0] as isize),
        SYSCALL_YIELD => do_yield(),
        SYSCALL_GET_TIME => do_get_time(),
        SYSCALL_BRK => do_brk(args[0]),

        SYSCALL_TEST =>  do_test(),
        _ => {
            panic!("unknown syscall_id {}", syscall_id);
        }
    }
}

pub fn do_test() -> isize {
    let value = 1234;
    stop_current_and_run_next_task();
    value
}
