use core::str::from_utf8;
use share::syscall::error::{EBADF, SysError};
use crate::sbi::sbi_console_getchar;
use crate::task::stop_current_and_run_next_task;
use crate::syscall::ipc::{sys_send, sys_receive};
use share::ipc::{Msg, DEVICE, PROC_NR, BUFFER, LENGTH, REPLY_STATUS, READ, WRITE, FSYSCALL, SYSCALL_TYPE, FS_SYSCALL_ARG0, FS_SYSCALL_ARG1, FS_SYSCALL_ARG2, FS_SYSCALL_ARG3, FS_SYSCALL_ARG4};
use crate::processor::clone_cur_task_in_this_hart;
use share::syscall::sys_const::{SYSCALL_GETCWD, SYSCALL_DUP, SYSCALL_DUP3, SYSCALL_CHDIR, SYSCALL_OPEN, SYSCALL_CLOSE, SYSCALL_WRITE, __SYSCALL_WRITE, SYSCALL_MKDIRAT, __SYSCALL_READ, SYSCALL_GETDENTS, SYSCALL_MOUNT, SYSCALL_UNMOUNT};

const FS_PID: usize = 5;

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

pub fn do_open(path_ptr: usize, flags: usize, mode: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_OPEN, [path_ptr, flags, mode, 0, 0])
}

pub fn do_close(fd: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_CLOSE, [fd, 0, 0, 0, 0])
}

pub fn do_get_dents(fd: usize, buf: usize, length: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_GETDENTS, [fd, buf, length, 0, 0])
}

pub fn do_write(fd: usize, buf_ptr: *const u8, length: usize) -> Result<usize, SysError>{
    if fd != 1 {
        return Err(SysError::new(EBADF));
    }
    let buffer = unsafe {
        core::slice::from_raw_parts(buf_ptr, length)
    };
    print!("{}", from_utf8(buffer).unwrap());
    Ok(0)
}

pub fn _do_write(fd: usize, buf_ptr: usize, length: usize) -> Result<usize, SysError> {
        let mut message = Msg::empty();
        let cur_pid = clone_cur_task_in_this_hart().pid();
        message.src_pid = cur_pid;
        message.mtype = WRITE;
        message.args[DEVICE] = 0;
        message.args[PROC_NR] = cur_pid;
        message.args[BUFFER] = buf_ptr;
        message.args[LENGTH] = length;
        sys_send(1, &message as *const _ as usize).unwrap();
        sys_receive(1, &mut message as *mut _ as usize).unwrap();

        Ok(message.args[REPLY_STATUS])
}

///TODO-FUTURE: current write behaviour is not the same as MINIX..
pub fn __do_write(fd: usize, buf_ptr: usize, length: usize) -> Result<usize, SysError> {
    send_receive_fs(__SYSCALL_WRITE, [fd, buf_ptr, length, 0, 0])
}

pub fn do_read(fd: usize, buf_ptr: *mut u8, length: usize) -> Result<usize, SysError> {
    if fd != 0 {
        return Err(SysError::new(EBADF));
    }

    let buffer = unsafe {
        core::slice::from_raw_parts_mut(buf_ptr, length)
    };
    let mut cnt = 0;
    for i in 0..length {
        let mut result = 0;
        loop {
            result = sbi_console_getchar();
            // info!("result is: {:#x}", result);
            if result == -1 {
                stop_current_and_run_next_task();
                continue;
            }
            break;
        }
        buffer[i] = result as usize as u8;
        cnt += 1;
    }

    Ok(cnt)
}

pub fn _do_read(fd: usize, buf_ptr: usize, length: usize) -> Result<usize, SysError> {
    let mut message = Msg::empty();
    let cur_pid = clone_cur_task_in_this_hart().pid();
    message.src_pid = cur_pid;
    message.mtype = READ;
    message.args[DEVICE] = 0;
    message.args[PROC_NR] = cur_pid;
    message.args[BUFFER] = buf_ptr;
    message.args[LENGTH] = length;
    sys_send(1, &message as *const _ as usize).unwrap();
    sys_receive(1, &mut message as *mut _ as usize).unwrap();

    Ok(message.args[REPLY_STATUS])
}

///TODO-FUTURE: current read behaviour is not the same as MINIX..
pub fn __do_read(fd: usize, buf_ptr: usize, length: usize) -> Result<usize, SysError> {
    send_receive_fs(__SYSCALL_READ, [fd, buf_ptr, length, 0, 0])
}

pub fn do_mkdir_at(dir_fd: usize, path_ptr: usize, mode: usize) -> Result<usize, SysError> {
    send_receive_fs(SYSCALL_MKDIRAT, [dir_fd, path_ptr, mode, 0, 0])
}

fn send_receive_fs(syscall_id: usize, args: [usize; 5]) -> Result<usize, SysError> {
    let mut message = Msg::empty();
    let cur_pid = clone_cur_task_in_this_hart().pid();
    message.src_pid = cur_pid;
    message.mtype = FSYSCALL;
    message.args[SYSCALL_TYPE] = syscall_id;
    message.args[FS_SYSCALL_ARG0] = args[0];
    message.args[FS_SYSCALL_ARG1] = args[1];
    message.args[FS_SYSCALL_ARG2] = args[2];
    message.args[FS_SYSCALL_ARG3] = args[3];
    message.args[FS_SYSCALL_ARG4] = args[4];
    sys_send(FS_PID, &message as *const _ as usize).unwrap();
    sys_receive(FS_PID as isize, &mut message as *mut _ as usize).unwrap();

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
