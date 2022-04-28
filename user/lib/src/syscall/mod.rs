mod raw;

pub use raw::*;
use share::syscall::error::{SysError, ENOMEM};
use alloc::vec::Vec;
use alloc::string::String;
use crate::env::get_envp_copy;
use share::ipc::Msg;
use share::file::{MAX_PATH_LENGTH, OpenFlag, RDirent, Dirent, DIRENT_BUFFER_SZ, SEEKFlag, Stat};
use share::ffi::{CString, CStr};

fn isize2result(ret: isize) -> Result<usize, SysError> {
    if ret < 0 {
        Result::Err(SysError::new(-ret as i32))
    } else {
        Result::Ok(ret as usize)
    }
}

pub fn lseek(fd: usize, offset: usize, whence: SEEKFlag) -> Result<usize, SysError> {
    isize2result(sys_lseek(fd, offset, whence.bits() as usize))
}

pub fn getcwd() -> Result<String, SysError> {
    let mut buf: [u8; MAX_PATH_LENGTH] = [0; MAX_PATH_LENGTH];
    isize2result(sys_getcwd(&mut buf))?;
    let cstr = CStr::from_ptr(buf.as_ptr());

    Ok(String::from(cstr.as_str()))
}

pub fn dup(old_fd : usize) -> Result<usize, SysError> {
    isize2result(sys_dup(old_fd))
}

pub fn dup3(old_fd: usize, new_fd: usize) -> Result<usize, SysError> {
    isize2result(sys_dup3(old_fd, new_fd))
}

pub fn unmount(source: &str, flags: usize) -> Result<usize, SysError> {
    let c_source = CString::from(source);
    isize2result(sys_unmount(c_source.as_ptr() as usize, flags))
}

pub fn mount(source: &str, target: &str, fs_type: &str, flags: usize, data: usize) -> Result<usize, SysError> {
    let c_source = CString::from(source);
    let c_target = CString::from(target);
    let c_fs_type = CString::from(fs_type);
    isize2result(
        sys_mount(c_source.as_ptr() as usize, c_target.as_ptr() as usize, c_fs_type.as_ptr() as usize, flags, data)
    )
}

pub fn chdir(path: &str) -> Result<(), SysError> {
    let cstring = CString::from(path);
    isize2result(sys_chdir(cstring.as_ptr() as usize))?;
    Ok(())
}

pub fn open(path: &str, flags: OpenFlag, mode: u32) -> Result<usize, SysError> {
    let cstring = CString::from(path);

    isize2result(sys_open(cstring.as_ptr() as usize, flags.bits(), mode))
}

pub fn close(fd: usize) -> Result<usize, SysError> {
    isize2result(sys_close(fd))
}

static mut DENTS_BUFFER: [u8; DIRENT_BUFFER_SZ] = [0; DIRENT_BUFFER_SZ];
pub fn get_dents(fd: usize) -> Result<Vec<RDirent>, SysError> {
    let buffer_ptr = unsafe { DENTS_BUFFER.as_ptr() as usize };
    let buffer_length = unsafe { DENTS_BUFFER.len() };
    let nbytes = isize2result(sys_get_dents(fd,buffer_ptr,buffer_length))?;
    let mut rdirents = Vec::new();

    let mut pos = 0;
    while pos < nbytes {
        let dirent = unsafe {
            ((buffer_ptr + pos) as *const Dirent).read()
        };

        pos += dirent.d_reclen as usize;
        rdirents.push(dirent.into());
    }

    Ok(rdirents)
}

pub fn exit(exit_code: usize) -> isize {
    sys_exit(exit_code)
}

pub fn yield_() {
    sys_yield();
}

pub fn read(fd: usize, buf: &mut [u8]) -> Result<usize, SysError> {
    isize2result(sys_read(fd, buf))
}

pub fn _read(fd: usize, buf: &mut [u8]) -> Result<usize, SysError> {
    isize2result(_sys_read(fd, buf))
}

pub fn __read(fd: usize, buf: &mut [u8]) -> Result<usize, SysError> {
    isize2result(__sys_read(fd, buf))
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub fn _write(fd: usize, buf: &[u8]) -> Result<usize, SysError>{
    isize2result(_sys_write(fd, buf))
}

pub fn __write(fd: usize, buf: &[u8]) -> Result<usize, SysError>{
    isize2result(__sys_write(fd, buf))
}

pub fn mkdir_at(dir_fd: usize, path: &str, mode: u32) -> Result<usize, SysError> {
    let cstring = CString::from(path);
    isize2result(sys_mkdir_at(dir_fd, cstring.as_ptr() as usize, mode))
}

pub fn fstat(fd: usize) -> Result<Stat, SysError> {
    let mut stat = Stat::empty();
    isize2result(sys_fstat(fd, &mut stat))?;
    Ok(stat)
}

pub fn sleep(seconds: usize) {
    let start_time = get_time();
    let mseconds = seconds * 1000;
    loop {
        let current_time = get_time();
        if current_time < mseconds + start_time {
            sys_yield();
        } else {
            break;
        }
    }
}

pub fn get_priority(which: usize, who: usize) -> Result<usize, SysError> {
    isize2result(sys_get_priority(which, who))
}

pub fn set_priority(which: usize, who: usize, prio: isize) -> Result<usize, SysError> {
    isize2result(sys_set_priority(which, who, prio))
}

pub fn get_time() -> usize {
    sys_get_time() as usize
}

pub fn getpid() -> usize {
    sys_get_pid() as usize
}

pub fn getppid() -> usize {
    sys_get_ppid() as usize
}

pub fn brk(new_brk: Option<usize>) -> Result<usize, SysError> {
    let new_brk = if new_brk.is_some() {new_brk.unwrap()} else { 0 };

    isize2result(sys_brk(new_brk))
}

pub fn fork() -> Result<usize, SysError> {
    isize2result(sys_fork(0, 0, 0, 0, 0))
}

#[allow(unused_variables)]
pub fn exec(path: &str, mut args: Vec<&str>) -> Result<usize, SysError> {
    let mut s = String::from(path);
    s.push('\0');
    let path_ptr = s.as_ptr() as usize;

    let mut args_end_with_zero = Vec::new();
    let mut argv = Vec::new();
    for arg in args {
        let mut s = String::from(arg);
        s.push('\0');
        argv.push(s.as_ptr() as usize);
        args_end_with_zero.push(s);
    }
    argv.push(0);
    let argv_ptr = argv.as_ptr() as usize;

    let envp = get_envp_copy();
    let envp_ptr = envp.as_ptr() as usize;
    isize2result(sys_exec(path_ptr, argv_ptr,envp_ptr))
}

pub fn waitpid(pid: isize, status: Option<&mut usize>, options: usize) -> Result<usize, SysError> {
    let status_ptr = match status {
        Some(status) => status as *mut usize as usize,
        None => 0,
    };
    isize2result(sys_waitpid(pid as usize, status_ptr, options))
}

pub fn send(dst_pid: usize, msg: &Msg) -> Result<usize, SysError> {
    isize2result(sys_send(dst_pid, msg))
}

pub fn receive(dst_pid: isize, msg: &mut Msg) -> Result<usize, SysError> {
    isize2result(sys_receive(dst_pid, msg))
}

pub fn dev_read(dev_phys_addr: usize, byte_size: usize) -> Result<usize, SysError> {
    isize2result(k_read_dev(dev_phys_addr, byte_size))
}

pub fn dev_write(dev_phys_addr: usize, val: usize, byte_size: usize) -> Result<usize, SysError> {
    isize2result(k_write_dev(dev_phys_addr, val, byte_size))
}

pub fn dev_read_u8(dev_phys_addr: usize) -> Result<usize, SysError> {
    isize2result(k_read_dev(dev_phys_addr, 1))
}

pub fn dev_write_u8(dev_phys_addr: usize, val: u8) -> Result<usize, SysError> {
    isize2result(k_write_dev(dev_phys_addr, val as usize, 1))
}

pub fn dev_write_u32(dev_phys_addr: usize, val: u32) -> Result<usize, SysError> {
    isize2result(k_write_dev(dev_phys_addr, val as usize, 4))
}

pub fn virt_copy(src_proc: usize, src_ptr: usize, dst_proc: usize, dst_ptr: usize, length: usize) -> Result<usize, SysError> {
    isize2result(k_virt_copy(src_proc, src_ptr, dst_proc, dst_ptr, length))
}

pub fn continuous_alloc(size: usize) -> Result<usize, SysError> {
    isize2result(k_continuous_alloc(size))
}

pub fn virt_to_phys(virt_addr: usize) -> Result<usize, SysError> {
    isize2result(k_virt_to_phys(virt_addr))
}

pub fn copy_path_from(proc: usize, path_ptr: usize) -> Result<String, SysError> {
    let buffer: [u8; MAX_PATH_LENGTH] = [0; MAX_PATH_LENGTH];
    let length = isize2result(k_copy_c_path(proc, path_ptr, buffer.as_ptr() as usize, MAX_PATH_LENGTH))?;
    let str = core::str::from_utf8(&buffer[..length]).unwrap();
    Ok(String::from(str))
}