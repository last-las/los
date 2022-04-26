use alloc::rc::Rc;
use crate::vfs::dentry::Dentry;
use crate::vfs::file::FileOperations;
use core::cell::RefCell;
use crate::vfs::super_block::SuperBlock;
use share::file::FileTypeFlag;

pub struct Inode {
    pub ino: usize,

    pub rdev: Option<Rdev>,
    pub file_type: FileTypeFlag,
    pub super_block: Rc<RefCell<SuperBlock>>,
    pub iop: Rc<dyn InodeOperations>,
    pub fop: Rc<dyn FileOperations>,
}

impl Inode {
    pub fn new(ino: usize, file_type: FileTypeFlag, super_block: Rc<RefCell<SuperBlock>>,
               iop: Rc<dyn InodeOperations>, fop: Rc<dyn FileOperations>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Inode {
                ino,
                rdev: None,
                file_type,
                super_block,
                iop,
                fop,
            }
        ))
    }

    pub fn set_rdev(&mut self, rdev: Rdev) {
        assert!(self.file_type.is_device());
        self.rdev = Some(rdev);
    }

    pub fn is_dir(&self) -> bool {
        self.file_type.contains(FileTypeFlag::DT_DIR)
    }
}

#[derive(Copy, Clone)]
pub struct Rdev {
    pub minor: u32,
    pub major: u32,
}

impl Rdev {
    pub fn new(minor: u32, major: u32) -> Rdev {
        Self {
            minor,
            major
        }
    }
}

impl From<u64> for Rdev {
    fn from(val: u64) -> Self {
        Self {
            minor: (val & 0xFFFFFFFF) as u32,
            major: ((val >> 32) & 0xFFFFFFFF) as u32,
        }
    }
}

impl Into<u64> for Rdev {
    fn into(self) -> u64 {
        self.minor as u64 | ((self.major as u64) << 32)
    }
}


pub trait InodeOperations {
    fn lookup(&self, name: &str, parent: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Dentry>>>;
    /// Create a normal file.
    fn create(&self, name: &str, parent: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Dentry>>>;
    /// Create a directory.
    fn mkdir(&self, name: &str, parent: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Dentry>>>;
    /// Create a node for special device.
    fn mknod(&self, name: &str, file_type: FileTypeFlag, rdev: Rdev, parent: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Dentry>>>;
}
