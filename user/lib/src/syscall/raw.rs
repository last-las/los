use core::arch::asm;
use share::ipc::Msg;
use share::syscall::sys_const::*;

#[inline(always)]
fn syscall0(id: usize) -> isize {
    let ret;
    unsafe {
        asm!(
        "ecall",
        out("a0")  ret,
        in("a7") id,
        );
    }
    ret
}

#[inline(always)]
fn syscall1(id: usize, arg: usize) -> isize {
    let ret;
    unsafe {
        asm!(
        "ecall",
        inout("a0") arg => ret,
        in("a7") id,
        );
    }
    ret
}

#[inline(always)]
fn syscall2(id: usize, arg1: usize, arg2: usize) -> isize {
    let ret;
    unsafe {
        asm!(
        "ecall",
        inout("a0") arg1 => ret,
        in("a1") arg2,
        in("a7") id,
        );
    }
    ret
}

#[inline(always)]
fn syscall3(id: usize, arg1: usize, arg2: usize, arg3: usize) -> isize {
    let ret;
    unsafe {
        asm!(
        "ecall",
        inout("a0") arg1 => ret,
        in("a1") arg2,
        in("a2") arg3,
        in("a7") id,
        );
    }
    ret
}

fn syscall4(id: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> isize {
    let ret;
    unsafe {
        asm!(
        "ecall",
        inout("a0") arg1 => ret,
        in("a1") arg2,
        in("a2") arg3,
        in("a3") arg4,
        in("a7") id,
        );
    }
    ret
}

#[inline(always)]
fn syscall5(id: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize) -> isize {
    let ret;
    unsafe {
        asm!(
        "ecall",
        inout("a0") arg1 => ret,
        in("a1") arg2,
        in("a2") arg3,
        in("a3") arg4,
        in("a4") arg5,
        in("a7") id,
        );
    }
    ret
}

pub fn sys_send(dst_pid: usize, msg: &Msg) -> isize {
    let msg_ptr = msg as *const _ as usize;
    syscall2(KCALL_SEND, dst_pid, msg_ptr)
}

pub fn sys_receive(dst_pid: isize, msg: &mut Msg) -> isize {
    let msg_ptr = msg as *mut _ as usize;
    syscall2(KCALL_RECEIVE, dst_pid as usize, msg_ptr)
}

pub fn sys_read(fd: usize, buf: &mut [u8]) -> isize {
    syscall3(SYSCALL_READ, fd, buf.as_ptr() as usize, buf.len())
}

pub fn _sys_read(fd: usize, buf: &mut [u8]) -> isize {
    syscall3(_SYSCALL_READ, fd, buf.as_ptr() as usize, buf.len())
}

pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall3(SYSCALL_WRITE, fd, buf.as_ptr() as usize, buf.len())
}

pub fn _sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall3(_SYSCALL_WRITE, fd, buf.as_ptr() as usize, buf.len())
}

pub fn sys_exit(exit_code: usize) -> isize {
    syscall1(SYSCALL_EXIT, exit_code)
}

pub fn sys_yield() -> isize {
    syscall0(SYSCALL_YIELD)
}

pub fn sys_get_priority(which: usize, who: usize) -> isize {
    syscall2(SYSCALL_GET_PRIORITY, which, who)
}

pub fn sys_set_priority(which: usize, who: usize, prio: isize) -> isize {
    syscall3(SYSCALL_SET_PRIORITY, which, who, prio as usize)
}

pub fn sys_get_time() -> isize {
    syscall0(SYSCALL_GET_TIME)
}

pub fn sys_get_pid() -> isize {
    syscall0(SYSCALL_GETPID)
}

pub fn sys_get_ppid() -> isize {
    syscall0(SYSCALL_GETPPID)
}

pub fn sys_brk(new_brk: usize) -> isize {
    syscall1(SYSCALL_BRK, new_brk)
}

pub fn sys_fork(
    flags: u32,
    stack: usize,
    ptid_ptr: usize,
    tls_ptr: usize,
    ctid_ptr: usize,
) -> isize {
    syscall5(
        SYSCALL_FORK,
        flags as usize,
        stack,
        ptid_ptr,
        tls_ptr,
        ctid_ptr,
    )
}

pub fn sys_exec(path_ptr: usize, argv_ptr: usize, envp_ptr: usize) -> isize {
    syscall3(SYSCALL_EXEC, path_ptr, argv_ptr, envp_ptr)
}

pub fn sys_waitpid(pid: usize, status_ptr: usize, options: usize) -> isize {
    syscall3(SYSCALL_WAITPID, pid, status_ptr, options)
}

pub fn sys_test() -> isize {
    syscall0(SYSCALL_TEST)
}

pub fn debug_frame_usage() -> usize {
    syscall0(DEBUG_FRAME_USAGE) as usize
}

pub fn k_read_dev(dev_phys_addr: usize, byte_size: usize) -> isize {
    syscall2(KCALL_READ_DEV, dev_phys_addr, byte_size)
}

pub fn k_write_dev(dev_phys_addr: usize, val: usize, byte_size: usize) -> isize {
    syscall3(KCALL_WRITE_DEV, dev_phys_addr, val, byte_size)
}

pub fn k_virt_copy(
    src_proc: usize,
    src_ptr: usize,
    dst_proc: usize,
    dst_ptr: usize,
    length: usize,
) -> isize {
    syscall5(
        KCALL_VIRT_COPY,
        src_proc,
        src_ptr,
        dst_proc,
        dst_ptr,
        length,
    )
}

pub fn k_continuous_alloc(size: usize) -> isize {
    syscall1(KCALL_CONTINUOUS_ALLOC, size)
}

pub fn k_virt_to_phys(virt_addr: usize) -> isize {
    syscall1(KCALL_VIRT_TO_PHYS, virt_addr)
}

pub fn k_sdcard_read(block_id: usize, buf_ptr: usize, size: usize) -> isize {
    syscall3(KCALL_SDCARD_READ, block_id, buf_ptr, size)
}
pub fn k_sdcard_write(block_id: usize, buf_ptr: usize, size: usize) -> isize {
    syscall3(KCALL_SDCARD_WRITE, block_id, buf_ptr, size)
}
