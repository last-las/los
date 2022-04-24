use alloc::rc::Rc;
use crate::proc::fs_struct::FsStruct;
use core::cell::RefCell;
use crate::vfs::dentry::{Dentry, VfsMount};
use share::syscall::error::{SysError, ENOENT, EBADF, ENOTDIR};
use crate::vfs::file::File;
use alloc::sync::Arc;
use user_lib::syscall::{virt_copy, getpid};
use share::file::{OpenFlag, FileTypeFlag};
use alloc::vec::Vec;
use alloc::string::String;

/// The return value of `path_lookup` function.
pub struct NameIdata {
    /// The target file Dentry.
    dentry: Rc<RefCell<Dentry>>,
    /// The target file's mount point.
    mnt: Rc<VfsMount>,
    ///  The remaining path name.
    ///
    /// When `LookupFlags::PARENT` is set, `path_lookup` will only find the target file's parent directory.
    /// The target file name will be kept in `left_path_name`. In other cases, `left_path_name`'s value is always an empty string.
    ///
    /// For example, if the path name is "/foo/bar.txt", `left_path_name` is "bar.txt".
    left_path_name: String,
}

impl NameIdata {
    pub fn new(dentry: Rc<RefCell<Dentry>>, mnt: Rc<VfsMount>, left_path_name: String) -> NameIdata {
        NameIdata {
            dentry,
            mnt,
            left_path_name,
        }
    }
}

pub fn do_getcwd(buf: usize, size: usize, proc_nr: usize, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let path = cur_fs.borrow().pwd.borrow().name.clone();
    let slice = path.as_bytes();
    virt_copy(getpid(), slice.as_ptr() as usize, proc_nr, buf, size).unwrap();

    Ok(0)
}

pub fn do_dup(old_fd: usize, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let file = cur_fs.borrow().get_file(old_fd)?;
    let new_fd = cur_fs.borrow_mut().alloc_fd()?;
    cur_fs.borrow_mut().add_file(new_fd, file)?;

    Ok(new_fd)
}

pub fn do_dup3(old_fd: usize, new_fd: usize, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let file = cur_fs.borrow().get_file(old_fd)?;
    cur_fs.borrow_mut().add_file(new_fd, file)?;

    Ok(new_fd)
}

pub fn do_chdir(path: &str, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let lookup_flag = LookupFlags::DIRECTORY;
    let nameidata = path_lookup(path,Rc::clone(&cur_fs), lookup_flag)?;
    cur_fs.borrow_mut().pwd = nameidata.dentry;
    cur_fs.borrow_mut().pwd_mnt = nameidata.mnt;

    Ok(0)
}

pub fn do_open(path: &str, flag: u32, mode: u32, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let open_flag = OpenFlag::from_bits(flag).unwrap();
    let nameidata =
        path_lookup(path, Rc::clone(&cur_fs),LookupFlags::PARENT | LookupFlags::DIRECTORY)?;

    let target_dentry = get_target_dentry_by_parent(nameidata, open_flag)?;
    let fop = target_dentry.borrow().inode.borrow().fop.clone();
    let file = File::new(fop, target_dentry, open_flag);

    let mut cur_fs_refmut = cur_fs.borrow_mut();
    let new_fd = cur_fs_refmut.alloc_fd()?;
    cur_fs_refmut.fd_table[new_fd] = Some(Rc::new(RefCell::new(file)));

    Ok(0)
}

pub fn do_close(fd: usize, cur_fs: Rc<RefCell<FsStruct>>) ->Result<usize, SysError> {
    cur_fs.borrow_mut().fd_table[fd].take().ok_or(SysError::new(EBADF))?;
    Ok(0)
}

pub fn do_read(fd: usize, cur_fs: Rc<RefCell<FsStruct>>, buf: usize, count: usize, proc_nr: usize) -> Result<usize, SysError> {
    let file = cur_fs.borrow().get_file(fd)?;
    let content = file.borrow().fop.read(file.clone(), count);
    let length = content.len();
    virt_copy(getpid(), (*content).as_ptr() as usize, proc_nr, buf, length).unwrap();
    file.borrow_mut().pos += length;

    Ok(length)
}

const BUFFER_SIZE: usize = 512;
pub fn do_write(fd: usize, buf: usize, count: usize, proc_nr: usize, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let file = cur_fs.borrow().get_file(fd)?;
    let buffer = [0; BUFFER_SIZE];
    for offset in (0..count).step_by(BUFFER_SIZE) {
        let length = usize::max(BUFFER_SIZE, count - offset);
        virt_copy(proc_nr, buf, getpid(), buffer.as_ptr() as usize, length).unwrap();
        file.borrow().fop.write(file.clone(), &buffer[0..length]);
        file.borrow_mut().pos += length;
    }

    Ok(count)
}

pub fn do_mkdir_at(path: &str, mode: usize, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let nameidata = path_lookup(path, cur_fs, LookupFlags::PARENT | LookupFlags::DIRECTORY)?;
    let parent = nameidata.dentry;
    let parent_inode = parent.borrow().inode.clone();
    let dir_entry = parent_inode.borrow().iop.mkdir(path, parent_inode.clone()).unwrap();
    parent.borrow_mut().children.push(dir_entry);

    Ok(0)
}

bitflags! {
    pub struct LookupFlags: u8 {
        const FOLLOW = 0x1;     // follow links
        const DIRECTORY = 0x2;  // require a directory
        const PARENT = 0x4;     // find file's parent directory
    }
}

fn path_lookup(path: &str, current: Rc<RefCell<FsStruct>>, flag: LookupFlags) -> Result<NameIdata, SysError> {
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

    let mut start= index;
    while index < path.len() {
        // find next dentry
        while index < path.len() && path[index] != '/' as u8 {
            index += 1;
        }

        if flag.contains(LookupFlags::PARENT) && reach_the_end(path, index) {
            break;
        }

        if path[start] == '.' as u8 { // '..' and '.'
            if start + 1 < path.len() && path[start + 1] == '.' as u8 { // ..
                unimplemented!()
            }
        } else { // other file names
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
        while index < path.len() && path[index] == '/' as u8 {
            index += 1;
        }
        start = index;
    } // end the main while.

    // If LookupFlags::DIRECTORY is set and `dentry` is not a directory, return `ENOTDIR` errno.
    if flag.contains(LookupFlags::DIRECTORY) {
        let inode = dentry.borrow().inode.clone();
        if !inode.borrow().file_type.contains(FileTypeFlag::DT_DIR) {
            return Err(SysError::new(ENOTDIR));
        }
    }

    let last_filename = String::from(core::str::from_utf8(&path[start..index]).unwrap());
    Ok(NameIdata::new(dentry, mnt, last_filename))
}

fn get_target_dentry_by_parent(nameidata: NameIdata, open_flag: OpenFlag) -> Result<Rc<RefCell<Dentry>>, SysError> {
    let parent_dentry = nameidata.dentry;
    let parent_inode = parent_dentry.borrow().inode.clone();
    let last_name = nameidata.left_path_name.as_str();

    if last_name.len() == 0 { // The open target path is root directory. Now `parent_dentry` is also root directory, so simply return `parent_dentry`
        Ok(parent_dentry)
    } else { // The open target path is not root directory,so we will search the target on `parent_dentry`.

        // Find on the cache.
        let result = parent_dentry.borrow().cached_lookup(last_name);
        if result.is_some() {
            return Ok(result.unwrap());
        }

        // Find on the real filesystem, if target doesn't exist and `OpenFlag::CREAT` is set, create on the real filesystem.
        let result = parent_inode.borrow().iop.lookup(last_name, parent_inode.clone());
        let mut child_dentry;
        if result.is_some() {
            child_dentry = result.unwrap();
        } else if open_flag.contains(OpenFlag::CREAT) {
            child_dentry = parent_inode.borrow().iop.create(last_name, parent_inode.clone()).unwrap();
        } else {
            return Err(SysError::new(ENOENT));
        }
        parent_dentry.borrow_mut().children.push(child_dentry.clone());
        child_dentry.borrow_mut().parent = Some(parent_dentry.clone());

        Ok(child_dentry)
    }
}

fn reach_the_end(path: &[u8], mut index: usize) -> bool {
    while index < path.len() && path[index] == '/' as u8 {
        index += 1;
    }

    if index == path.len() {
        true
    } else {
        false
    }
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
