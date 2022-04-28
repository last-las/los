#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(arbitrary_self_types)]
#![feature(const_btree_new)]

mod vfs;
mod fs;
mod proc;
mod syscall;
mod device;

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;

use crate::fs::ramfs::register_ramfs;
use crate::vfs::filesystem::read_super_block;
use crate::proc::fs_struct::FsStruct;
use crate::proc::fs_manager::*;
use crate::syscall::*;
use user_lib::syscall::{getpid, receive, copy_path_from, send};
use share::ipc::{Msg, FORK, EXIT, MSG_ARGS_0, PROC_NR, MSG_ARGS_1, FS_SYSCALL_ARG0, FS_SYSCALL_ARG1, SYSCALL_TYPE, FS_SYSCALL_ARG2, FS_SYSCALL_ARG3, REPLY_PROC_NR, REPLY_STATUS, REPLY, FORK_PARENT, FSYSCALL, FS_SYSCALL_ARG4};
use share::syscall::sys_const::*;
use core::cell::RefCell;
use alloc::rc::Rc;
use share::syscall::error::SysError;
use crate::vfs::dentry::{VfsDentry, VfsMount};
use share::file::{FileTypeFlag, VIRT_BLK_MAJOR, CONSOLE_MAJOR, RAM_MAJOR};
use crate::vfs::inode::Rdev;
use crate::fs::ezfs::register_ezfs;
use crate::fs::devfs::register_devfs;

#[no_mangle]
fn main() {
    let cur_pid = getpid();

    register_ezfs();
    register_ramfs();
    register_devfs();
    let rdev = Rdev::new(0, RAM_MAJOR);
    let sp = read_super_block("ramfs", rdev).unwrap();
    let root = sp.borrow().root.clone().unwrap();
    let mnt = VfsMount::new(sp.clone());
    init_dev_directory(root.clone(), mnt.clone());

    let fs_struct = FsStruct::new(root.clone(), mnt.clone(), root.clone(), mnt.clone());
    init_fs_struct_of_proc(fs_struct, cur_pid);

    let mut message = Msg::empty();
    loop {
        receive(-1, &mut message).unwrap();

        if message.mtype == FORK {
            let parent_pid = message.args[FORK_PARENT];
            let child_pid = message.src_pid;
            let parent_fs = get_fs_struct_by_pid(parent_pid);
            let child_fs = parent_fs.borrow().clone_fs_struct();
            init_fs_struct_of_proc(child_fs, child_pid);
        }else if message.mtype == EXIT {
            todo!()
        } else { // FSYSCALL
            let result = handle_syscall(&mut message);
            let reply_status = SysError::mux(result);
            reply(message.src_pid, REPLY, reply_status as isize);
        }
    }
}

fn init_dev_directory(root_dentry: Rc<RefCell<VfsDentry>>, mnt: Rc<RefCell<VfsMount>>) {
    let root_inode = root_dentry.borrow().inode.clone();
    // create dev directory
    let dev_dentry = root_inode.borrow().iop.mkdir("dev", root_inode.clone()).unwrap();
    root_dentry.borrow_mut().children.push(dev_dentry.clone());
    dev_dentry.borrow_mut().parent = Some(root_dentry.clone());
    // mount devfs on dev directory.
    let dev_dentry =
        real_mount(dev_dentry, mnt, "devfs", 0.into()).unwrap();

    // create sda inode.
    let rdev = Rdev::new(0, VIRT_BLK_MAJOR);
    let file_type = FileTypeFlag::DT_BLK;
    attach_device_to(dev_dentry.clone(), "sda2", file_type, rdev);

    // create console inode.
    let rdev = Rdev::new(0, CONSOLE_MAJOR);
    let file_type = FileTypeFlag::DT_CHR;
    attach_device_to(dev_dentry.clone(), "console", file_type, rdev);

    // create ram inode.
    for i in 1..4 {
        let rdev = Rdev::new(i, RAM_MAJOR);
        let file_type = FileTypeFlag::DT_BLK;
        attach_device_to(dev_dentry.clone(), format!("ram{}", i).as_str(), file_type, rdev);
    }
}

fn attach_device_to(dev_dentry: Rc<RefCell<VfsDentry>>, name: &str, file_type: FileTypeFlag, rdev: Rdev) {
    let dev_inode = dev_dentry.borrow().inode.clone();
    let device_dentry = dev_inode.borrow().iop.mknod(name, file_type, rdev, dev_inode.clone()).unwrap();
    dev_dentry.borrow_mut().children.push(device_dentry.clone());
    device_dentry.borrow_mut().parent = Some(dev_dentry.clone());
}

fn handle_syscall(message: &mut Msg) -> Result<usize, SysError> {
    assert_eq!(message.mtype, FSYSCALL);

    let src_pid = message.src_pid;
    let cur_fs = get_fs_struct_by_pid(src_pid);
    let result = match message.args[SYSCALL_TYPE] {
        SYSCALL_LSEEK => do_lseek(message.args[FS_SYSCALL_ARG0], message.args[FS_SYSCALL_ARG1], message.args[FS_SYSCALL_ARG2], cur_fs),
        SYSCALL_GETCWD => do_getcwd(message.args[FS_SYSCALL_ARG0], message.args[FS_SYSCALL_ARG1], src_pid, cur_fs),
        SYSCALL_DUP => do_dup(message.args[FS_SYSCALL_ARG0], cur_fs),
        SYSCALL_DUP3 => do_dup3(message.args[FS_SYSCALL_ARG0], message.args[FS_SYSCALL_ARG1], cur_fs),
        SYSCALL_UNMOUNT => {
            let target = copy_path_from(src_pid, message.args[FS_SYSCALL_ARG0])?;
            do_unmount(target.as_str(), message.args[FS_SYSCALL_ARG1], cur_fs)
        },
        SYSCALL_MOUNT => {
            let source = copy_path_from(src_pid, message.args[FS_SYSCALL_ARG0])?;
            let target = copy_path_from(src_pid, message.args[FS_SYSCALL_ARG1])?;
            let fs_type = copy_path_from(src_pid, message.args[FS_SYSCALL_ARG2])?;
            let mount_flags = message.args[FS_SYSCALL_ARG3];
            let data = message.args[FS_SYSCALL_ARG4];
            do_mount(source.as_str(), target.as_str(), fs_type.as_str(), mount_flags, data, cur_fs)
        },
        SYSCALL_CHDIR => {
            let path = copy_path_from(src_pid, message.args[FS_SYSCALL_ARG0])?;
            do_chdir(path.as_str(),cur_fs)
        },
        SYSCALL_OPEN => {
            let path = copy_path_from(src_pid, message.args[FS_SYSCALL_ARG0])?;
            do_open(path.as_str(), message.args[FS_SYSCALL_ARG1] as u32, message.args[FS_SYSCALL_ARG2] as u32, cur_fs)
        },
        SYSCALL_CLOSE => do_close(message.args[FS_SYSCALL_ARG0], cur_fs),
        SYSCALL_GETDENTS => do_get_dents(message.args[FS_SYSCALL_ARG0], message.args[FS_SYSCALL_ARG1], message.args[FS_SYSCALL_ARG2], src_pid, cur_fs),
        __SYSCALL_READ => do_read(message.args[FS_SYSCALL_ARG0], message.args[FS_SYSCALL_ARG1], message.args[FS_SYSCALL_ARG2],src_pid, cur_fs),
        __SYSCALL_WRITE => do_write(message.args[FS_SYSCALL_ARG0], message.args[FS_SYSCALL_ARG1], message.args[FS_SYSCALL_ARG2],src_pid, cur_fs),
        SYSCALL_MKDIRAT => {
            let path = copy_path_from(src_pid, message.args[FS_SYSCALL_ARG1])?;
            do_mkdir_at(message.args[FS_SYSCALL_ARG0],path.as_str(), message.args[FS_SYSCALL_ARG2], cur_fs)
        },
        _ => {
            panic!("Unknown FSYSCALL id: {}", message.args[SYSCALL_TYPE]);
        }
    };

    result
}

fn reply(caller: usize, mtype: usize, status: isize) {
    let mut message = Msg::empty();
    message.mtype = mtype;
    message.args[REPLY_PROC_NR] = caller;
    message.args[REPLY_STATUS] = status as usize;

    send(caller, &message).unwrap();
}
