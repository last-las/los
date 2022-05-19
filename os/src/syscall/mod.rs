mod ipc;
mod mm;
mod file;
mod time;
mod proc;
mod device;

use crate::syscall::mm::do_brk;
use crate::syscall::file::*;
use crate::syscall::time::do_get_time;
use crate::syscall::ipc::{sys_receive, sys_send};
use crate::syscall::proc::*;
use crate::task::stop_current_and_run_next_task;
use share::syscall::error::{SysError, EUNKOWN};
use share::syscall::sys_const::*;
use share::ffi::c_char;
use crate::mm::available_frame;
use crate::syscall::device::{kcall_read_dev, kcall_write_dev, kcall_virt_copy, kcall_continuous_alloc, kcall_virt_to_phys};

pub use ipc::notify;


pub fn syscall(syscall_id: usize, args: [usize; 5]) -> usize {
    let result: Result<usize, SysError> = match syscall_id {
        KCALL_SEND => sys_send(args[0], args[1]),
        KCALL_RECEIVE => sys_receive(args[0] as isize, args[1]),
        KCALL_READ_DEV => kcall_read_dev(args[0], args[1]),
        KCALL_WRITE_DEV => kcall_write_dev(args[0], args[1], args[2]),
        KCALL_VIRT_COPY => kcall_virt_copy(args[0], args[1], args[2], args[3], args[4]),
        KCALL_CONTINUOUS_ALLOC => kcall_continuous_alloc(args[0]),
        KCALL_VIRT_TO_PHYS => kcall_virt_to_phys(args[0]),

        SYSCALL_READ => do_read(args[0], args[1] as *mut u8, args[2]),
        _SYSCALL_READ => _do_read(args[0], args[1], args[2]),
        SYSCALL_WRITE => do_write(args[0], args[1] as *const u8, args[2]),
        _SYSCALL_WRITE => _do_write(args[0], args[1], args[2]),
        SYSCALL_EXIT => do_exit(args[0] as isize),
        SYSCALL_YIELD => do_yield(),
        SYSCALL_GET_PRIORITY => do_get_priority(args[0], args[1]),
        SYSCALL_SET_PRIORITY => do_set_priority(args[0], args[1], args[2] as isize),
        SYSCALL_GET_TIME => do_get_time(),
        SYSCALL_GETPID => do_get_pid(),
        SYSCALL_GETPPID => do_get_ppid(),
        SYSCALL_BRK => do_brk(args[0]),
        SYSCALL_FORK => do_fork(args[0] as u32, args[1], args[2], args[3], args[4]),
        SYSCALL_EXEC => do_exec(args[0], args[1] as *const *const c_char, args[2] as *const *const c_char),
        SYSCALL_WAITPID => do_waitpid(args[0] as isize, args[1], args[2]),

        SYSCALL_TEST =>  do_test(),

        DEBUG_FRAME_USAGE => debug_frame_usage(),

        _ => Err(SysError::new(EUNKOWN))
    };

    SysError::mux(result)


}

pub fn do_test() -> Result<usize, SysError>{
    unimplemented!();
}

pub fn debug_frame_usage() -> Result<usize, SysError> {
    Ok(available_frame())
}