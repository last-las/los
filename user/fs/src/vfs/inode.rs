use alloc::rc::Rc;
use crate::vfs::dentry::VfsDentry;
use crate::vfs::file::FileOperations;
use core::cell::RefCell;
use crate::vfs::super_block::SuperBlock;
use share::file::FileTypeFlag;

pub struct VfsInode {
    pub ino: usize,
    pub size: usize,
    pub rdev: Option<Rdev>,
    pub file_type: FileTypeFlag,
    pub super_block: Rc<RefCell<SuperBlock>>,
    pub iop: Rc<dyn InodeOperations>,
    pub fop: Rc<dyn FileOperations>,
}

impl VfsInode {
    pub fn new(ino: usize, size: usize, rdev: Option<Rdev>, file_type: FileTypeFlag, super_block: Rc<RefCell<SuperBlock>>,
               iop: Rc<dyn InodeOperations>, fop: Rc<dyn FileOperations>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            VfsInode {
                ino,
                size,
                rdev,
                file_type,
                super_block,
                iop,
                fop,
            }
        ))
    }

    pub fn is_dir(&self) -> bool {
        self.file_type.contains(FileTypeFlag::DT_DIR)
    }

    pub fn is_blk(&self) -> bool {
        self.file_type.contains(FileTypeFlag::DT_BLK)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    fn lookup(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Option<Rc<RefCell<VfsDentry>>>;
    /// Create a normal file.
    fn create(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Option<Rc<RefCell<VfsDentry>>>;
    /// Create a directory.
    fn mkdir(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Option<Rc<RefCell<VfsDentry>>>;
    /// Create a node for special device.
    fn mknod(&self, name: &str, file_type: FileTypeFlag, rdev: Rdev, parent: Rc<RefCell<VfsInode>>) -> Option<Rc<RefCell<VfsDentry>>>;
}
