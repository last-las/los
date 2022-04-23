#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(arbitrary_self_types)]

mod vfs;
mod fs;
mod proc;
mod syscall;

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate lazy_static;

use crate::fs::ramfs::register_ramfs;
use crate::vfs::filesystem::alloc_super_block;
use crate::proc::fs_struct::FsStruct;
use crate::proc::fs_manager::*;
use user_lib::syscall::getpid;

#[no_mangle]
fn main() {
    let cur_pid = getpid();

    register_ramfs();
    let sp = alloc_super_block("ramfs").unwrap();
    let root = sp.borrow().root.clone().unwrap();
    let mnt = root.borrow().mnt.clone().unwrap();
    let fs_struct = FsStruct::new(root.clone(), mnt.clone(), root.clone(), mnt.clone());
    init_fs_struct_of_proc(fs_struct, cur_pid);
    println!("Hello, world!");
}
