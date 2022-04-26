use crate::vfs::inode::{Inode, Rdev};
use crate::vfs::dentry::Dentry;
use alloc::rc::Rc;
use core::cell::RefCell;

pub struct SuperBlock {
    pub rdev: Rdev,
    /// Root Directory Entry, it's name is always "/".
    pub root: Option<Rc<RefCell<Dentry>>>,

    // pub sop: Rc<dyn SuperBlockOperations>,
}

/*pub trait SuperBlockOperations {
    fn read_inode(&self, ino: usize, dev: usize) -> Option<Rc<RefCell<Inode>>>;
    fn alloc_inode(&self, dev: usize) -> Option<Rc<RefCell<Inode>>>;
    fn write_inode(&self, inode: Rc<RefCell<Inode>>);

    fn sync(&self);
}
*/
impl SuperBlock {
    pub fn new(rdev: Rdev) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Self {
                rdev,
                root: None,
            }
        ))
    }
}