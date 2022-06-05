use alloc::rc::Rc;
use core::cell::RefCell;
use crate::vfs::dentry::{VfsDentry, VfsMount};
use alloc::vec::Vec;
use share::file::{OpenFlag, Stat, FileTypeFlag};
use share::syscall::error::SysError;

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
        self.dentry.borrow().inode.borrow().file_type.contains(FileTypeFlag::DT_DIR)
    }

    pub fn fstat(&self) -> Stat {
        let mut stat = Stat::empty();
        let size = self.dentry.borrow().inode.borrow().size;
        stat.size = size as u64;

        stat
    }
}

pub trait FileOperations {
    fn read(&self, file: Rc<RefCell<File>>, buf_ptr: usize, cnt: usize, proc_nr: usize) -> Result<usize, SysError>;
    fn write(&self, file: Rc<RefCell<File>>, buf_ptr: usize, cnt: usize, proc_nr: usize) -> Result<(), SysError>;
    fn readdir(&self, file: Rc<RefCell<File>>) -> Vec<Rc<RefCell<VfsDentry>>>;
}
