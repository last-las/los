use crate::vfs::inode::Inode;
use crate::vfs::dentry::Dentry;
use alloc::rc::Rc;
use core::cell::RefCell;

pub struct SuperBlock {
    root: Rc<RefCell<Dentry>>,
}

pub trait SuperBlockOperations {
    fn read_inode(&self, ino: usize) -> Option<Rc<RefCell<Inode>>>;
    fn alloc_inode(&mut self) -> Option<Rc<RefCell<Inode>>>;
    fn write_inode(&self, inode: Rc<RefCell<Inode>>);

    fn sync(&self);
}