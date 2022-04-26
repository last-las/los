use alloc::rc::Rc;
use crate::vfs::dentry::Dentry;
use crate::vfs::file::FileOperations;
use core::cell::RefCell;
use crate::vfs::super_block::SuperBlock;
use share::file::FileTypeFlag;

pub struct Inode {
    pub ino: usize,

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
}

pub trait InodeOperations {
    fn lookup(&self, name: &str, inode: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Dentry>>>;
    /// Create a normal file.
    fn create(&self, name: &str, inode: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Dentry>>>;
    /// Create a directory.
    fn mkdir(&self, name: &str, inode: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Dentry>>>;
}
