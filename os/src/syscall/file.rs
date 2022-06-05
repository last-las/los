use share::syscall::error::SysError;
use crate::syscall::ipc::{kcall_send, kcall_receive};
use share::ipc::{Msg, REPLY_STATUS, FSYSCALL, SYSCALL_TYPE, FS_SYSCALL_ARG0, FS_SYSCALL_ARG1, FS_SYSCALL_ARG2, FS_SYSCALL_ARG3, FS_SYSCALL_ARG4, FS_PID};
use crate::processor::get_cur_task_in_this_hart;
use share::syscall::sys_const::{SYSCALL_GETCWD, SYSCALL_DUP, SYSCALL_DUP3, SYSCALL_CHDIR, SYSCALL_OPEN, SYSCALL_CLOSE, SYSCALL_WRITE, SYSCALL_MKDIRAT, SYSCALL_READ, SYSCALL_GETDENTS, SYSCALL_MOUNT, SYSCALL_UNMOUNT, SYSCALL_LSEEK, SYSCALL_FSTAT, SYSCALL_UNLINK, SYSCALL_RMDIR};

pub fn do_lseek(fd: usize, offset: usize, whence: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_LSEEK, [fd, offset, whence, 0, 0])
}

pub fn do_getcwd(buf: usize, length: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_GETCWD, [buf, length, 0, 0, 0])
}

pub fn do_dup(old_fd: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_DUP, [old_fd, 0, 0, 0, 0])
}

pub fn do_dup3(old_fd: usize, new_fd: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_DUP3, [old_fd, new_fd, 0, 0, 0])
}

pub fn do_unmount(target: usize, flags: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_UNMOUNT, [target, flags, 0, 0, 0])
}

pub fn do_mount(source: usize, target: usize, fs_type: usize, mount_flags: usize, data: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_MOUNT, [source, target, fs_type, mount_flags, data])
}

pub fn do_chdir(path_ptr: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_CHDIR, [path_ptr, 0, 0, 0, 0])
}

pub fn do_open(fd: usize, path_ptr: usize, flags: usize, mode: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_OPEN, [fd, path_ptr, flags, mode, 0])
}

pub fn do_close(fd: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_CLOSE, [fd, 0, 0, 0, 0])
}

pub fn do_get_dents(fd: usize, buf: usize, length: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_GETDENTS, [fd, buf, length, 0, 0])
}

///TODO-FUTURE: current write behaviour is not the same as MINIX..
pub fn do_write(fd: usize, buf_ptr: usize, length: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_WRITE, [fd, buf_ptr, length, 0, 0])
}

///TODO-FUTURE: current read behaviour is not the same as MINIX..
pub fn do_read(fd: usize, buf_ptr: usize, length: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_READ, [fd, buf_ptr, length, 0, 0])
}

pub fn do_mkdir_at(dir_fd: usize, path_ptr: usize, mode: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_MKDIRAT, [dir_fd, path_ptr, mode, 0, 0])
}

pub fn do_fstat(fd: usize, stat_ptr: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_FSTAT, [fd, stat_ptr, 0, 0, 0])
}

pub fn do_unlink(path_ptr: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_UNLINK, [path_ptr, 0, 0, 0, 0])
}

pub fn do_rmdir(path_ptr: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_RMDIR, [path_ptr, 0, 0, 0, 0])
}

fn send_receive_fs(syscall_id: usize, args: [usize; 5]) -> Result<usize, SysError> {
    let mut message = Msg::empty();
    let cur_pid = get_cur_task_in_this_hart().pid();
    message.src_pid = cur_pid;
    message.mtype = FSYSCALL;
    message.args[SYSCALL_TYPE] = syscall_id;
    message.args[FS_SYSCALL_ARG0] = args[0];
    message.args[FS_SYSCALL_ARG1] = args[1];
    message.args[FS_SYSCALL_ARG2] = args[2];
    message.args[FS_SYSCALL_ARG3] = args[3];
    message.args[FS_SYSCALL_ARG4] = args[4];
    kcall_send(FS_PID, &message as *const _ as usize).unwrap();
    kcall_receive(FS_PID as isize, &mut message as *mut _ as usize).unwrap();

    let status = message.args[REPLY_STATUS] as isize;
    isize2result(status)
}

fn isize2result(ret: isize) -> Result<usize, SysError> {
    if ret < 0 {
        Result::Err(SysError::new(-ret as i32))
    } else {
        Result::Ok(ret as usize)
    }
}
