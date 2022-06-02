use share::syscall::sys_const::{SYSCALL_BRK, SYSCALL_WRITE, KCALL_SBI_WRITE, SYSCALL_GETPID};
use core::arch::asm;

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
fn syscall(id: usize, arg: usize) -> isize {
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

pub fn sys_brk(new_brk: usize) -> isize {
    syscall(SYSCALL_BRK, new_brk)
}

pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall3(SYSCALL_WRITE, fd, buf.as_ptr() as usize, buf.len())
}

pub fn k_sbi_write(fd: usize, buf: &[u8]) -> isize {
    syscall3(KCALL_SBI_WRITE, fd, buf.as_ptr() as usize, buf.len())
}

pub fn sys_get_pid() -> isize {
    syscall0(SYSCALL_GETPID)
}