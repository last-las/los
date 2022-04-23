use alloc::rc::Rc;
use crate::proc::fs_struct::FsStruct;
use core::cell::RefCell;
use crate::vfs::dentry::{Dentry, VfsMount};
use share::syscall::error::{SysError, ENOENT, EBADF};
use crate::vfs::file::File;
use alloc::sync::Arc;
use user_lib::syscall::{virt_copy, getpid};

pub struct NameIdata {
    dentry: Rc<RefCell<Dentry>>,
    mnt: Rc<VfsMount>,
}

impl NameIdata {
    pub fn new(dentry: Rc<RefCell<Dentry>>, mnt: Rc<VfsMount>) -> NameIdata {
        NameIdata {
            dentry,
            mnt,
        }
    }
}

pub fn do_getcwd() {
    unimplemented!()
}

pub fn do_pipe2() {
    unimplemented!();
}

pub fn do_dup() {
    unimplemented!()
}

pub fn do_dup3() {
    unimplemented!()
}

pub fn do_chdir(path: &str) {
    unimplemented!()
}

pub fn do_open(path: &str, flag: usize, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let nameidata = path_lookup(path, Rc::clone(&cur_fs))?;
    let mut cur_fs_refmut = cur_fs.borrow_mut();

    let fop = nameidata.dentry.borrow().inode.borrow().fop.clone();
    let dentry = nameidata.dentry.clone();
    let file = File::new(fop, dentry);

    let new_fd = cur_fs_refmut.alloc_fd()?;
    cur_fs_refmut.fd_table[new_fd] = Some(Rc::new(RefCell::new(file)));

    Ok(0)
}

pub fn do_close() {
    unimplemented!()
}

pub fn do_read(fs: usize, cur_fs: Rc<RefCell<FsStruct>>, buf: usize, count: usize, proc_nr: usize) -> Result<usize, SysError> {
    let cur_fs_ref = cur_fs.borrow();
    if fs >= cur_fs_ref.fd_table.len() {
        return Err(SysError::new(EBADF));
    }
    let result = cur_fs_ref.fd_table[fs].as_ref();
    if result.is_none() {
        return Err(SysError::new(EBADF));
    }

    let file = result.unwrap().clone();
    let content = file.borrow().fop.read(file.clone(), count);
    let length = content.len();
    virt_copy(getpid(), (*content).as_ptr() as usize, proc_nr, buf, length).unwrap();

    Ok(length)
}

pub fn do_write() {
    unimplemented!()
}

pub fn do_mkdir_at(path: &str, mode: usize) -> Result<usize, SysError> {
    unimplemented!()
}

fn path_lookup(path: &str, current: Rc<RefCell<FsStruct>>) -> Result<NameIdata, SysError> {
    let path = path.as_bytes();
    let mut index = 0;
    let mut dentry;
    let mut mnt;

    while index < path.len() && path[index] == '/' as u8 { // skip prefix '/'
        index += 1;
    }

    if index == 0 {
        dentry = current.borrow().pwd.clone();
        mnt = current.borrow().pwd_mnt.clone();
    } else {
        dentry = current.borrow().root.clone();
        mnt = current.borrow().root_mnt.clone();
    }

    while index < path.len() {
        // find next dentry
        let start = index;
        while index < path.len() && path[index] != '/' as u8 {
            index += 1;
        }

        if path[start] == '.' as u8 { // '..' and '.' situation
            if start + 1 < path.len() && path[start + 1] == '.' as u8 { // ..
                unimplemented!()
            }
        } else {
            let name = core::str::from_utf8(&path[start..index]).unwrap();
            let result = dentry.borrow().cached_lookup(name);
            if result.is_some() {
                dentry = result.unwrap();
            } else {
                dentry = real_lookup(dentry.clone(), name).ok_or(SysError::new(ENOENT))?;
            }

            // check mountpoint
            if dentry.borrow().mnt.is_some() {
                mnt = dentry.borrow().mnt.as_ref().unwrap().clone();
                dentry = mnt.mountpoint.clone();
            }
        }

        // skip trailing '/'
        while index < path.len() && path[index] != '/' as u8 {
            index += 1;
        }
    } // end the main while.

    Ok(NameIdata::new(dentry, mnt))
}

/// go to the low-level filesystem and lookup.
fn real_lookup(dentry: Rc<RefCell<Dentry>>, name: &str) -> Option<Rc<RefCell<Dentry>>> {
    let inode =dentry.borrow().inode.clone();
    let result = inode.borrow().iop.lookup(name, inode.clone());

    if result.is_some() {
        let child_dentry = result.unwrap();
        dentry.borrow_mut().children.push(child_dentry.clone());
        child_dentry.borrow_mut().parent = Some(dentry.clone());
        Some(child_dentry)
    } else {
        None
    }
}
