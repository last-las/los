mod ipc;
mod mm;
mod file;
mod time;
mod proc;

use crate::syscall::mm::do_brk;
use crate::syscall::file::do_write;
use crate::syscall::time::do_get_time;
use crate::syscall::ipc::{sys_receive, sys_send};
use crate::syscall::proc::{do_exit, do_yield, do_fork, do_exec, do_waitpid};
use crate::task::stop_current_and_run_next_task;
use share::syscall::error::SysError;
use share::syscall::sys_const::*;


pub fn syscall(syscall_id: usize, args: [usize; 5]) -> usize {
    let result: Result<usize, SysError> = match syscall_id {
        SYSCALL_SEND => sys_send(args[0], args[1]),
        SYSCALL_RECEIVE => sys_receive(args[0], args[1]),

        SYSCALL_WRITE => do_write(args[0], args[1], args[2]),
        SYSCALL_EXIT => do_exit(args[0] as isize),
        SYSCALL_YIELD => do_yield(),
        SYSCALL_GET_TIME => do_get_time(),
        SYSCALL_BRK => do_brk(args[0]),
        SYSCALL_FORK => do_fork(args[0] as u32, args[1], args[2], args[3], args[4]),
        SYSCALL_EXEC => do_exec(args[0], args[1], args[2]),
        SYSCALL_WAITPID => do_waitpid(args[0] as isize, args[1], args[2]),

        SYSCALL_TEST =>  do_test(),
        _ => {
            panic!("unknown syscall_id {}", syscall_id);
        }
    };

    SysError::mux(result)


}

pub fn do_test() -> Result<usize, SysError>{
    unimplemented!();
}
