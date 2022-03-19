use ipc::Msg;
use core::arch::asm;

const SYSCALL_SEND: usize = 1;
const SYSCALL_RECEIVE: usize = 2;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_TEST: usize = 1234;

#[inline(always)]
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let ret;
    unsafe {
        asm!(
        "ecall",
        inout("a0") args[0] => ret,
        in("a1") args[1],
        in("a2") args[2],
        in("a7") id,
        );
    }
    ret
}

pub fn sys_test() -> isize {
    syscall(SYSCALL_TEST, [0, 0, 0])
}

pub fn sys_send(dst_pid: usize, msg: &Msg) -> isize {
    let msg_ptr = msg as *const _ as usize;
    syscall(SYSCALL_SEND, [dst_pid, msg_ptr, 0])
}

pub fn sys_receive(dst_pid: usize, msg: &mut Msg) -> isize {
    let msg_ptr = msg as *mut _ as usize;
    syscall(SYSCALL_RECEIVE, [dst_pid, msg_ptr, 0])
}

#[allow(unused)]
pub fn sys_read() -> isize {
    unimplemented!();
}

pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buf.as_ptr() as usize, buf.len()])
}

pub fn sys_exit(exit_code: usize) -> isize{
    syscall(SYSCALL_EXIT, [exit_code, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, [0, 0, 0])
}