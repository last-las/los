// TODO: IPC
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;

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


#[allow(unused)]
pub fn sys_receive() -> isize {
    unimplemented!();
}

#[allow(unused)]
pub fn sys_read() -> isize {
    unimplemented!();
}

pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    //TODO: modify with process IPC by talking with terminal controller.
    syscall(SYSCALL_WRITE, [fd, buf.as_ptr() as usize, buf.len()])
}

pub fn sys_exit(exit_code: usize) -> isize{
    //TODO: modify with process IPC by talking with process manager.
    syscall(SYSCALL_EXIT, [exit_code, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

pub fn sys_get_time() -> isize {
    // TODO: this syscall right now only get the value of mtime, it should be replace with RTC.
    syscall(SYSCALL_GET_TIME, [0, 0, 0])
}