use alloc::rc::Rc;
use crate::vfs::dentry::Dentry;
use crate::vfs::file::FileOperations;
use core::cell::RefCell;
use crate::vfs::super_block::SuperBlock;

pub struct Inode {
    pub ino: usize,

    pub super_block: Rc<RefCell<SuperBlock>>,
    pub iop: Rc<dyn InodeOperations>,
    pub fop: Rc<dyn FileOperations>,
}

impl Inode {}

pub trait InodeOperations {
    fn lookup(&self, name: &str, inode: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Dentry>>>;
    /// Create a normal file.
    fn create(&self, name: &str, inode: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Dentry>>>;
    /// Create a directory.
    fn mkdir(&self, name: &str, inode: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Dentry>>>;
}
