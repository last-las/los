use crate::vfs::inode::{InodeOperations, Inode};
use alloc::rc::Rc;
use core::cell::RefCell;
use crate::vfs::dentry::Dentry;
use crate::vfs::super_block::{SuperBlock, SuperBlockOperations};
use super::RAM_FILE_SYSTEMS;
use crate::fs::ramfs::RamFileSystem;

/// Create a new ram filesystem,
/// append it to global vector `RAM_FILE_SYSTEMS`, and return a `SuperBlock` structure.
pub fn alloc_ramfs_super_block() -> Rc<RefCell<SuperBlock>> {
    let dev;
    unsafe {
        dev = RAM_FILE_SYSTEMS.len();
        RAM_FILE_SYSTEMS.push(RamFileSystem::new());
    }
    let new_ramfs_sb = SuperBlock::new(dev,Rc::new(RootFsSuperBlockOperations));

    new_ramfs_sb
}

pub struct RootFsSuperBlockOperations;
pub struct RootFsInodeOperations;
pub struct RootFsFileOperations;

impl SuperBlockOperations for RootFsSuperBlockOperations {
    fn read_inode(&self, ino: usize, dev: usize) -> Option<Rc<RefCell<Inode>>> {
        todo!()
    }

    fn alloc_inode(&self, dev: usize) -> Option<Rc<RefCell<Inode>>> {
        todo!()
    }

    fn write_inode(&self, inode: Rc<RefCell<Inode>>) {
        todo!()
    }

    fn sync(&self) {
        todo!()
    }
}

impl InodeOperations for RootFsInodeOperations {
    fn lookup(&self, name: &str, inode: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Dentry>>> {
        todo!()
    }

    fn create(&self, name: &str, inode: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Dentry>>> {
        todo!()
    }

    fn mkdir(&self, name: &str, inode: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Dentry>>> {
        todo!()
    }
}
