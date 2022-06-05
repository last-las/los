use core::arch::asm;
use share::ipc::Msg;
use share::syscall::sys_const::*;
use share::file::Stat;

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

#[inline(always)]
fn syscall6(id: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize) -> isize {
    let ret;
    unsafe {
        asm!(
        "ecall",
        inout("a0") arg1 => ret,
        in("a1") arg2,
        in("a2") arg3,
        in("a3") arg4,
        in("a4") arg5,
        in("a5") arg6,
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

pub fn sys_lseek(fd: usize, offset: usize, whence: usize) -> isize {
    syscall3(SYSCALL_LSEEK, fd, offset, whence)
}

pub fn sys_getcwd(buf: &mut [u8]) -> isize {
    syscall2(SYSCALL_GETCWD, buf.as_ptr() as usize, buf.len())
}

pub fn sys_dup(old_fd: usize) -> isize {
    syscall1(SYSCALL_DUP, old_fd)
}

pub fn sys_dup3(old_fd: usize, new_fd: usize) -> isize {
    syscall2(SYSCALL_DUP3, old_fd, new_fd)
}

pub fn sys_unmount(target: usize, flags: usize) -> isize {
    syscall2(SYSCALL_UNMOUNT, target, flags)
}

pub fn sys_mount(source: usize, target: usize, fs_type: usize, flags: usize, data: usize) -> isize {
    syscall5(SYSCALL_MOUNT, source, target, fs_type, flags, data)
}

pub fn sys_chdir(path_ptr: usize) -> isize {
    syscall1(SYSCALL_CHDIR, path_ptr)
}

pub fn sys_open(fd: usize, path_ptr: usize, flags: u32, mode: u32) -> isize {
    syscall4(SYSCALL_OPEN, fd, path_ptr, flags as usize, mode as usize)
}

pub fn sys_close(fd: usize) -> isize {
    syscall1(SYSCALL_CLOSE, fd)
}

pub fn sys_get_dents(fd: usize, buf: usize, length: usize) -> isize {
    syscall3(SYSCALL_GETDENTS, fd, buf, length)
}

pub fn sys_read(fd: usize, buf: &mut [u8]) -> isize {
    syscall3(SYSCALL_READ, fd, buf.as_ptr() as usize, buf.len())
}

pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall3(SYSCALL_WRITE, fd, buf.as_ptr() as usize, buf.len())
}

pub fn sys_mkdir_at(dir_fd: usize, path_ptr: usize, mode: u32) -> isize {
    syscall3(SYSCALL_MKDIRAT, dir_fd, path_ptr, mode as usize)
}

pub fn sys_fstat(fd: usize, stat: &mut Stat) -> isize {
    syscall2(SYSCALL_FSTAT, fd, stat as *const _ as usize)
}

pub fn sys_unlink(path_ptr: usize) -> isize {
    syscall1(SYSCALL_UNLINK, path_ptr)
}

pub fn sys_rmdir(path_ptr: usize) -> isize {
    syscall1(SYSCALL_RMDIR, path_ptr)
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

pub fn sys_uname(which: usize) -> isize {
    syscall1(SYSCALL_UNAME, which)
}

pub fn sys_get_time(ptr: usize) -> isize {
    syscall1(SYSCALL_GET_TIME, ptr)
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

pub fn sys_munmap(start: usize, len: usize) -> isize {
    syscall2(SYSCALL_MUNMAP, start, len)
}

pub fn sys_fork(flags: u32, stack: usize, ptid_ptr: usize, tls_ptr: usize, ctid_ptr: usize) -> isize {
    syscall5(SYSCALL_FORK, flags as usize, stack, ptid_ptr, tls_ptr, ctid_ptr)

}

pub fn sys_exec(path_ptr: usize, argv_ptr: usize, envp_ptr: usize) -> isize {
    syscall3(SYSCALL_EXEC, path_ptr, argv_ptr, envp_ptr)
}

pub fn sys_mmap(start: usize, len: usize, prot: u32, flags: u32, fd: usize, offset: usize) -> isize {
    syscall6(SYSCALL_MMAP, start, len, prot as usize, flags as usize, fd, offset)
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

pub fn k_copy_c_path(proc: usize, path_ptr: usize, buf_ptr: usize, size: usize) -> isize {
    syscall4(KCALL_COPY_C_PATH, proc, path_ptr, buf_ptr, size)
}

pub fn k_sbi_read(fd: usize, buf: &mut [u8]) -> isize {
    syscall3(KCALL_SBI_READ, fd, buf.as_ptr() as usize, buf.len())
}

pub fn k_sbi_write(fd: usize, buf: &[u8]) -> isize {
    syscall3(KCALL_SBI_WRITE, fd, buf.as_ptr() as usize, buf.len())
}

pub fn k_terminal_read(fd: usize, buf: &mut [u8]) -> isize {
    syscall3(KCALL_TERMINAL_READ, fd, buf.as_ptr() as usize, buf.len())
}

pub fn k_terminal_write(fd: usize, buf: &[u8]) -> isize {
    syscall3(KCALL_TERMINAL_WRITE, fd, buf.as_ptr() as usize, buf.len())
}

pub fn k_sdcard_read(block_id: usize, buf_ptr: usize, size: usize) -> isize {
    syscall3(KCALL_SDCARD_READ, block_id, buf_ptr, size)
}

pub fn k_sdcard_write(block_id: usize, buf_ptr: usize, size: usize) -> isize {
    syscall3(KCALL_SDCARD_WRITE, block_id, buf_ptr, size)
}
