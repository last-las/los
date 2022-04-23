use crate::vfs::inode::Inode;
use crate::vfs::dentry::Dentry;
use alloc::rc::Rc;
use core::cell::RefCell;

pub struct SuperBlock {
    /// To distinguish from other `SuperBlock` in the same filesystem type.
    ///
    /// There might be more than one filesystem instance in the same filesystem type, so this field
    /// is used to select the right one.
    pub dev: usize,
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
    pub fn new(dev: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Self {
                dev,
                root: None,
            }
        ))
    }
}