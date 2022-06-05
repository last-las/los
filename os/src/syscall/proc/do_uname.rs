use share::syscall::error::SysError;
use crate::config::{DOMAINNAME, MACHINE, NODENAME, RELEASE, SYSNAME, VERSION};

#[repr(C)]
#[derive(Debug)]
pub struct Utsname {
    sysname: [u8; 65],
    nodename: [u8; 65],
    release: [u8; 65],
    version: [u8; 65],
    machine: [u8; 65],
    domainname: [u8; 65],
}

pub fn do_uname(pointer: usize) -> Result<usize, SysError> {
    // println!("Hello Uname!");
    let utsname_ptr = pointer as *mut Utsname;
    unsafe {
        (*utsname_ptr).sysname[..SYSNAME.len()].copy_from_slice(SYSNAME.as_bytes());
        (*utsname_ptr).nodename[..NODENAME.len()].copy_from_slice(NODENAME.as_bytes());
        (*utsname_ptr).release[..RELEASE.len()].copy_from_slice(RELEASE.as_bytes());
        (*utsname_ptr).version[..VERSION.len()].copy_from_slice(VERSION.as_bytes());
        (*utsname_ptr).machine[..MACHINE.len()].copy_from_slice(MACHINE.as_bytes());
        (*utsname_ptr).domainname[..DOMAINNAME.len()].copy_from_slice(DOMAINNAME.as_bytes());
    }
    Ok(0)
}