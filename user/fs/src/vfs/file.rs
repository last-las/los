use alloc::rc::Rc;
use core::cell::RefCell;
use crate::vfs::dentry::{VfsDentry, VfsMount};
use alloc::boxed::Box;
use alloc::vec::Vec;
use share::file::{OpenFlag, Stat};

pub struct File {
    pub fop: Rc<dyn FileOperations>,
    pub dentry: Rc<RefCell<VfsDentry>>,
    pub open_flags: OpenFlag,
    pub mnt: Rc<RefCell<VfsMount>>,

    /// Current read/write offset of the opened file.
    pub pos: usize,
}

impl File {
    pub fn new(fop: Rc<dyn FileOperations>, dentry: Rc<RefCell<VfsDentry>>,
               open_flags: OpenFlag, mnt: Rc<RefCell<VfsMount>>) -> Self {
        Self {
            fop,
            dentry,
            open_flags,
            mnt,
            pos: 0,
        }
    }

    pub fn readable(&self) -> bool {
        !self.open_flags.contains(OpenFlag::WRONLY) | self.open_flags.contains(OpenFlag::RDWR)
    }

    pub fn writable(&self) -> bool {
        self.open_flags.contains(OpenFlag::WRONLY) | self.open_flags.contains(OpenFlag::RDWR)
    }

    pub fn is_directory(&self) -> bool {
        self.open_flags.contains(OpenFlag::DIRECTORY)
    }

    pub fn fstat(&self) -> Stat {
        let mut stat = Stat::empty();
        let size = self.dentry.borrow().inode.borrow().size;
        stat.size = size;

        stat
    }
}

pub trait FileOperations {
    fn read(&self, file: Rc<RefCell<File>>, size: usize) -> Vec<u8>;
    fn write(&self, file: Rc<RefCell<File>>, content: &[u8]);
    fn readdir(&self, file: Rc<RefCell<File>>) -> Vec<Rc<RefCell<VfsDentry>>>;
}
