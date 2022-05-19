use alloc::rc::Rc;
use crate::vfs::{Dentry, VFSmount, File};
use alloc::vec::Vec;

pub struct FsStruct {
    pub pwd: Rc<Dentry>,
    pub pwd_mnt: Rc<VFSmount>,
    pub root: Rc<Dentry>,
    pub root_mnt: Rc<VFSmount>,

    pub fd_table: Vec<Option<Rc<File>>>,
}