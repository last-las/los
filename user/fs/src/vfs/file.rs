use alloc::rc::Rc;
use core::cell::RefCell;
use crate::vfs::dentry::Dentry;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct File {
    pub fop: Rc<dyn FileOperations>,
    pub dentry: Rc<RefCell<Dentry>>,

    /// Current read/write offset of the opened file.
    pub pos: usize,
}

impl File {
    pub fn new(fop: Rc<dyn FileOperations>, dentry: Rc<RefCell<Dentry>>) -> Self {
        Self {
            fop,
            dentry,
            pos: 0,
        }
    }

    pub fn read(&self) {
        unimplemented!()
    }
}

pub trait FileOperations {
    fn read(&self, file: Rc<RefCell<File>>, size: usize) -> Vec<u8>;
    fn write(&self, file: Rc<RefCell<File>>, content: Vec<u8>);
    fn readdir(&self, file: Rc<RefCell<File>>) -> Vec<Rc<RefCell<Dentry>>>;
}
