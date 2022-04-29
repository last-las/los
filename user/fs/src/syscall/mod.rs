use alloc::rc::Rc;
use crate::proc::fs_struct::FsStruct;
use core::cell::RefCell;
use crate::vfs::dentry::{VfsDentry, VfsMount};
use share::syscall::error::{SysError, ENOENT, EBADF, ENOTDIR, EEXIST, EINVAL, ERANGE, ENOTBLK, ENODEV};
use crate::vfs::file::File;
use alloc::sync::Arc;
use user_lib::syscall::{virt_copy, getpid};
use share::file::{OpenFlag, FileTypeFlag, Dirent, AT_FD_CWD, DIRENT_BUFFER_SZ, SEEKFlag, Stat};
use alloc::vec::Vec;
use alloc::string::String;
use share::ffi::CString;
use alloc::collections::{VecDeque, BinaryHeap};
use crate::vfs::filesystem::read_super_block;
use crate::vfs::inode::Rdev;

/// The return value of `path_lookup` function.
pub struct NameIdata {
    /// The target file Dentry.
    dentry: Rc<RefCell<VfsDentry>>,
    /// Mount information of the device where the target file is located
    mnt: Rc<RefCell<VfsMount>>,
    ///  The remaining path name.
    ///
    /// When `LookupFlags::PARENT` is set, `path_lookup` will only find the target file's parent directory.
    /// The target file name will be kept in `left_path_name`. In other cases, `left_path_name` is always an empty string.
    ///
    /// For example, if the path name is "/foo/bar.txt" and `LookupFlags::PARENT` is set, `left_path_name` is "bar.txt".
    left_path_name: String,
}

impl NameIdata {
    pub fn new(dentry: Rc<RefCell<VfsDentry>>, mnt: Rc<RefCell<VfsMount>>, left_path_name: String) -> NameIdata {
        NameIdata {
            dentry,
            mnt,
            left_path_name,
        }
    }
}

pub fn do_lseek(fd: usize, offset: usize, whence: usize, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let whence = SEEKFlag::from_bits(whence as u32).unwrap();
    let file = cur_fs.borrow().get_file(fd)?;

    if whence.contains(SEEKFlag::CUR) { // current location plus `offset`
        file.borrow_mut().pos += offset;
    } else if whence.contains(SEEKFlag::END) { // size of the file plus `offset`
        let size = file.borrow().dentry.borrow().inode.borrow().size;
        file.borrow_mut().pos = size + offset;
    } else if whence.is_empty() { // set to `offset`
        file.borrow_mut().pos = offset;
    } else {
        return Err(SysError::new(EINVAL));
    }

    let result = file.borrow().pos;
    Ok(result)
}

pub fn do_getcwd(buf: usize, size: usize, proc_nr: usize, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let fs_ref = cur_fs.borrow();
    let path = get_path_name(fs_ref.pwd.clone(), fs_ref.pwd_mnt.clone(),
                             fs_ref.root.clone(), fs_ref.root_mnt.clone());
    let length = path.len();
    if length > size {
        return Err(SysError::new(ERANGE));
    }

    virt_copy(getpid(), path.as_ptr() as usize, proc_nr, buf, length).unwrap();

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
    let nameidata = path_lookup(path, Rc::clone(&cur_fs), LookupFlags::DIRECTORY, None)?;
    cur_fs.borrow_mut().pwd = nameidata.dentry;
    cur_fs.borrow_mut().pwd_mnt = nameidata.mnt;

    Ok(0)
}

pub fn do_unmount(target: &str, _: usize, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let target_nameidata = path_lookup(target, cur_fs.clone(), LookupFlags::empty(), None)?;
    let vfs_mount = target_nameidata.mnt;
    let mount_point = vfs_mount.borrow().mount_point.clone().unwrap();
    mount_point.borrow_mut().mnt.take();

    Ok(0)
}

pub fn do_mount(source: &str, target: &str, fs_type: &str, _: usize, _: usize, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    // Use `source` to find device inode.
    let dev_nameidata = path_lookup(source, cur_fs.clone(), LookupFlags::empty(), None)?;
    let dev_dentry = dev_nameidata.dentry;
    let dev_inode = dev_dentry.borrow().inode.clone();
    if ! dev_inode.borrow().is_blk() {
        return Err(SysError::new(ENOTBLK));
    }

    // Get Super block with minor device number.
    let rdev = dev_inode.borrow().rdev.clone().unwrap();


    // Use `target` to find mountpoint. `target` has to be a directory.
    let target_nameidata = path_lookup(target, cur_fs.clone(), LookupFlags::DIRECTORY, None)?;
    let target_dentry = target_nameidata.dentry;
    if target_dentry.borrow().mnt.is_some() { //TODO-FUTURE: Vfs only support mount once on a directory for now.
        return Err(SysError::new(EINVAL))
    }

    real_mount(target_dentry, target_nameidata.mnt, fs_type, rdev)?;
    Ok(0)
}

pub fn real_mount(target_dentry: Rc<RefCell<VfsDentry>>, target_mnt: Rc<RefCell<VfsMount>>, fs_type: &str, rdev: Rdev)
                  -> Result<Rc<RefCell<VfsDentry>>, SysError> {
    // read super block
    let result = read_super_block(fs_type, rdev);
    if result.is_none() {
        return Err(SysError::new(ENODEV));
    }
    let super_block = result.unwrap();
    let root_dentry = super_block.borrow().root.clone().unwrap();

    let vfs_mount = VfsMount::new(super_block);
    vfs_mount.borrow_mut().set_mnt_point(target_dentry.clone());
    vfs_mount.borrow_mut().set_mnt_parent(target_mnt);
    target_dentry.borrow_mut().mnt = Some(vfs_mount);

    Ok(root_dentry)
}

pub fn do_open(path: &str, flag: u32, mode: u32, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let open_flag = OpenFlag::from_bits(flag).unwrap();
    let nameidata =
        path_lookup(path, Rc::clone(&cur_fs), LookupFlags::PARENT | LookupFlags::DIRECTORY, None)?;

    let (target_dentry, target_mnt) = get_target_dentry_and_mnt_by_parent(nameidata, open_flag)?;
    let fop = target_dentry.borrow().inode.borrow().fop.clone();
    let file = File::new(fop, target_dentry, open_flag, target_mnt);
    // println!("open ino:{}", file.dentry.borrow().inode.borrow().ino);

    let mut cur_fs_refmut = cur_fs.borrow_mut();
    let new_fd = cur_fs_refmut.alloc_fd()?;
    assert!(cur_fs_refmut.fd_table[new_fd].is_none());
    cur_fs_refmut.fd_table[new_fd] = Some(Rc::new(RefCell::new(file)));

    Ok(new_fd)
}

pub fn do_close(fd: usize, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    assert!(cur_fs.borrow_mut().fd_table[fd].is_some());
    cur_fs.borrow_mut().fd_table[fd].take().ok_or(SysError::new(EBADF))?;
    Ok(0)
}

pub fn do_get_dents(fd: usize, buf: usize, length: usize, proc_nr: usize, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let file = cur_fs.borrow().get_file(fd)?;
    if !file.borrow().is_directory() {
        return Err(SysError::new(ENOTDIR));
    }

    let dentry = file.borrow().dentry.clone();
    if !dentry.borrow().read_dir_flag { // if true,we will search on the real filesystem.
        // because `do_mkdir_at` may be invoked before `do_get_dents`, so we have to compare contents
        // on the cache and the real filesystem to make sure it's not duplicated.
        let filesystem_dentries = file.borrow().fop.readdir(file.clone());
        let uncached_dentries = find_uncached_dentries(dentry.clone(), filesystem_dentries);
        dentry.borrow_mut().children.extend(uncached_dentries);
        dentry.borrow_mut().read_dir_flag = true;
    }

    // construct dirent array buffer.
    let mut buffer: [u8; DIRENT_BUFFER_SZ] = [0; DIRENT_BUFFER_SZ];
    let mut offset = 0;
    let mut dirent_size = core::mem::size_of::<Dirent>();
    for child_dentry in dentry.borrow().children.iter() {
        let child_dentry_ref = child_dentry.borrow();
        let cstring_name = CString::new(child_dentry_ref.name.clone());
        let mut reclen = dirent_size + cstring_name.as_bytes_with_nul().len();
        if buffer.len() - offset < reclen {
            return Err(SysError::new(EINVAL));
        }

        let dirent = Dirent {
            d_ino: child_dentry_ref.inode.borrow().ino as u64,
            d_offset: 0,
            d_reclen: reclen as u16,
            d_type: child_dentry_ref.inode.borrow().file_type,
            d_name: unsafe {
                (buf + offset + dirent_size) as *const u8
            },
        };
        unsafe {
            ((buffer.as_mut_ptr() as usize + offset) as *mut Dirent).write(dirent);
            let dst_slice: &mut [u8] = &mut buffer[offset + dirent_size..offset + reclen];
            dst_slice.copy_from_slice(cstring_name.as_bytes_with_nul());
        }

        offset += reclen;
    }

    if offset > length {
        return Err(SysError::new(EINVAL));
    }

    // Copy the buffer to destination
    virt_copy(getpid(), buffer.as_ptr() as usize, proc_nr, buf, offset).unwrap();

    Ok(offset)
}

pub fn do_read(fd: usize, buf: usize, count: usize, proc_nr: usize, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let file = cur_fs.borrow().get_file(fd)?;
    if !file.borrow().readable() {
        return Err(SysError::new(EBADF));
    }
    let content = file.borrow().fop.read(file.clone(), count);
    let length = content.len();
    if length > 0 {
        virt_copy(getpid(), content.as_ptr() as usize, proc_nr, buf, length).unwrap();
    }
    file.borrow_mut().pos += length;

    Ok(length)
}

const BUFFER_SIZE: usize = 512;

pub fn do_write(fd: usize, buf: usize, count: usize, proc_nr: usize, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let file = cur_fs.borrow().get_file(fd)?;
    if !file.borrow().writable() {
        return Err(SysError::new(EBADF));
    }

    let buffer = [0; BUFFER_SIZE];
    for offset in (0..count).step_by(BUFFER_SIZE) {
        let length = usize::min(BUFFER_SIZE, count - offset);
        virt_copy(proc_nr, buf, getpid(), buffer.as_ptr() as usize, length).unwrap();
        file.borrow().fop.write(file.clone(), &buffer[0..length]);
        file.borrow_mut().pos += length;
    }

    // update size.
    let inode = file.borrow().dentry.borrow().inode.clone();
    let size = inode.borrow().size;
    let pos = file.borrow().pos;
    inode.borrow_mut().size = usize::max(size, pos);

    Ok(count)
}

pub fn do_mkdir_at(dir_fd: usize, path: &str, mode: usize, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let mut dir_fs = None;
    if path.as_bytes()[0] != '/' as u8 && dir_fd != AT_FD_CWD as usize {
        let file = cur_fs.borrow().get_file(dir_fd)?;
        if !file.borrow().is_directory() {
            return Err(SysError::new(ENOTDIR));
        }
        dir_fs = Some(file);
    }
    let nameidata = path_lookup(path, cur_fs, LookupFlags::PARENT | LookupFlags::DIRECTORY, dir_fs)?;
    let parent = nameidata.dentry;
    let parent_inode = parent.borrow().inode.clone();
    let dir_entry =
        parent_inode.borrow()
            .iop.mkdir(nameidata.left_path_name.as_str(), parent_inode.clone())
            .ok_or(SysError::new(EEXIST))?;

    parent.borrow_mut().children.push(dir_entry.clone());
    dir_entry.borrow_mut().parent = Some(parent.clone());

    Ok(0)
}

pub fn do_fstat(fd: usize, stat_ptr: usize, proc_nr: usize, cur_fs: Rc<RefCell<FsStruct>>) -> Result<usize, SysError> {
    let file = cur_fs.borrow().get_file(fd)?;
    let stat = file.borrow().fstat();
    virt_copy(getpid(), &stat as *const _ as usize,
              proc_nr, stat_ptr, core::mem::size_of::<Stat>()).unwrap();

    Ok(0)
}

bitflags! {
    pub struct LookupFlags: u8 {
        const FOLLOW = 0x1;     // follow links
        const DIRECTORY = 0x2;  // require a directory
        const PARENT = 0x4;     // find file's parent directory
    }
}

/// Lookup `NameIdata` corresponding to `path` and `flag`.
///
/// The `dir_fs` is only available when `do_mkdir_at` invokes this function. In other cases it's always `None`.
fn path_lookup(path: &str, current: Rc<RefCell<FsStruct>>, flag: LookupFlags, dir_fs: Option<Rc<RefCell<File>>>) -> Result<NameIdata, SysError> {
    let path = path.as_bytes();
    let mut index = 0;
    let mut dentry;
    let mut mnt;

    while index < path.len() && path[index] == '/' as u8 { // skip prefix '/'
        index += 1;
    }

    if index == 0 { // relative pathname
        if dir_fs.is_some() {
            let file = dir_fs.unwrap();
            dentry = file.borrow().dentry.clone();
            mnt = file.borrow().mnt.clone();
        } else {
            dentry = current.borrow().pwd.clone();
            mnt = current.borrow().pwd_mnt.clone();
        }
    } else { // absolute pathname
        dentry = current.borrow().root.clone();
        mnt = current.borrow().root_mnt.clone();
    }

    let mut start = index;
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
                let result = follow_dotdot(dentry, mnt, current.clone());
                dentry = result.0;
                mnt = result.1;
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
            while dentry.borrow().mnt.is_some() {
                mnt = dentry.borrow().mnt.as_ref().unwrap().clone();
                dentry = mnt.borrow().mount_root.clone();
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

fn get_target_dentry_and_mnt_by_parent(nameidata: NameIdata, open_flag: OpenFlag)
    -> Result<(Rc<RefCell<VfsDentry>>, Rc<RefCell<VfsMount>>), SysError> {
    let last_name = nameidata.left_path_name.as_str();

    if last_name.len() == 0 { // The open target path is root directory. Now `parent_dentry` is also root directory, so simply return `parent_dentry`
        Ok((nameidata.dentry.clone(), nameidata.mnt.clone()))
    } else { // The open target path is not root directory,so we will search the target on `parent_dentry`.
        let parent_dentry = nameidata.dentry;
        let parent_inode = parent_dentry.borrow().inode.clone();

        let mut child_dentry;
        // Find on the cache.
        let result = parent_dentry.borrow().cached_lookup(last_name);
        if result.is_some() {
            child_dentry = result.unwrap();
        } else {  // Find on the real filesystem, if target doesn't exist and `OpenFlag::CREAT` is set, create on the real filesystem.
            let result = parent_inode.borrow().iop.lookup(last_name, parent_inode.clone());
            if result.is_some() {
                child_dentry = result.unwrap();
            } else if open_flag.contains(OpenFlag::CREAT) {
                child_dentry = parent_inode.borrow().iop.create(last_name, parent_inode.clone()).ok_or(SysError::new(EEXIST))?;
            } else {
                return Err(SysError::new(ENOENT));
            }

            // successfully find, set the cache
            parent_dentry.borrow_mut().children.push(child_dentry.clone());
            child_dentry.borrow_mut().parent = Some(parent_dentry.clone());
        }

        // ENOTDIR situation
        if open_flag.contains(OpenFlag::DIRECTORY) && !child_dentry.borrow().inode.borrow().is_dir() {
            return Err(SysError::new(ENOTDIR));
        }

        // child dentry might be a mountpoint
        let mut child_mnt = nameidata.mnt;
        while child_dentry.borrow().mnt.is_some() {
            child_mnt = child_dentry.borrow().mnt.as_ref().unwrap().clone();
            child_dentry = child_mnt.borrow().mount_root.clone();
        }

        Ok((child_dentry, child_mnt))
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
fn real_lookup(dentry: Rc<RefCell<VfsDentry>>, name: &str) -> Option<Rc<RefCell<VfsDentry>>> {
    let inode = dentry.borrow().inode.clone();
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

fn find_uncached_dentries(parent: Rc<RefCell<VfsDentry>>, filesystem_dentries: Vec<Rc<RefCell<VfsDentry>>>) -> Vec<Rc<RefCell<VfsDentry>>> {
    let mut uncached_dentries = Vec::new();
    for filesystem_dentry in filesystem_dentries {
        let mut is_the_same_name = false;
        for cached_dentry in parent.borrow().children.iter() {
            if cached_dentry.borrow().name == filesystem_dentry.borrow().name {
                is_the_same_name = true;
                break;
            }
        }

        if !is_the_same_name {
            uncached_dentries.push(filesystem_dentry);
        }
    }

    uncached_dentries
}

/// Return the path(end with null) of `cur_dentry`
fn get_path_name(mut cur_dentry: Rc<RefCell<VfsDentry>>, mut cur_mnt: Rc<RefCell<VfsMount>>,
                 root_dentry: Rc<RefCell<VfsDentry>>, root_mnt: Rc<RefCell<VfsMount>>) -> String {
    let mut path_names = VecDeque::new();
    loop {
        if Rc::ptr_eq(&cur_dentry, &root_dentry) && Rc::ptr_eq(&cur_mnt, &root_mnt) {
            break;
        } else if Rc::ptr_eq(&cur_dentry, &cur_mnt.borrow().mount_root) {
            cur_dentry = cur_mnt.borrow().mount_point.clone().unwrap();
            let parent_mnt = cur_mnt.borrow().mount_parent.clone().unwrap();
            cur_mnt = parent_mnt;
            continue;
        }

        path_names.push_front(cur_dentry.borrow().name.clone());
        let parent = cur_dentry.borrow().parent.clone().unwrap();
        cur_dentry = parent;
    }

    // concat together
    let mut path = String::from('/');
    let length = path_names.len();
    for i in 0..length {
        path.push_str(path_names[i].as_str());
        if i != length - 1 {
            path.push('/');
        }
    }
    path.push('\0');

    return path;
}

fn follow_dotdot(mut dentry: Rc<RefCell<VfsDentry>>, mut mnt: Rc<RefCell<VfsMount>>, current: Rc<RefCell<FsStruct>>)
                 -> (Rc<RefCell<VfsDentry>>, Rc<RefCell<VfsMount>>) {
    loop {
        if Rc::ptr_eq(&mnt, &current.borrow().root_mnt)
            && Rc::ptr_eq(&dentry, &current.borrow().root) {
            // reach root mountpoint, can't follow anymore.
            break;
        }else if !Rc::ptr_eq(&dentry, &mnt.borrow().mount_root) {
            // `dentry` is not the root dentry of current device,
            // which means `dentry` and it's parent are on the same device.
            let parent_dentry = dentry.borrow().parent.clone().unwrap();
            dentry = parent_dentry;
            break;
        } else if mnt.borrow().mount_parent.is_none(){
            // current mountpoint doesn't have parent. This only happens on the root dentry of ramfs.
            break;
        }
        // `dentry` is the root dentry of current device, and current device is mounted on another one, so we will follow it's parent.
        // Parent may mount on other as well, so we continue the loop.
        dentry = mnt.borrow().mount_point.clone().unwrap();
        let mnt_parent = mnt.borrow().mount_parent.clone().unwrap();
        mnt = mnt_parent;
    }

    (dentry, mnt)
}