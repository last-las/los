use alloc::rc::{Rc, Weak};
use alloc::string::String;
use crate::vfs::inode::Inode;
use core::cell::{RefCell, Ref};
use alloc::vec::Vec;
use crate::vfs::super_block::SuperBlock;

pub struct Dentry {
    pub name: String,
    pub inode: Rc<RefCell<Inode>>,
    pub parent: Option<Rc<RefCell<Dentry>>>,
    pub children: Vec<Rc<RefCell<Dentry>>>,

    pub mnt: Option<Rc<VfsMount>>,
}

pub struct VfsMount {
    pub mountpoint: Rc<RefCell<Dentry>>,
    pub mnt_sb: Rc<RefCell<SuperBlock>>,
}

impl Dentry {
    pub fn new(name: &str, inode: Rc<RefCell<Inode>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Self {
                name: String::from(name),
                inode,
                parent: None,
                children: Vec::new(),

                mnt: None,
            }
        ))
    }

    /// find out if the target dentry is already exists.
    pub fn cached_lookup(&self, name: &str) -> Option<Rc<RefCell<Dentry>>> {
        if let Some((index, _)) = self.children
            .iter()
            .enumerate()
            .find(|(_, child_dentry)| { child_dentry.borrow().name.as_str() == name })
        {
            Some(self.children[index].clone())
        } else {
            None
        }
    }
}

impl VfsMount {
    pub fn new(mountpoint: Rc<RefCell<Dentry>>, mnt_sb: Rc<RefCell<SuperBlock>>) -> Rc<Self> {
        Rc::new( Self {
            mountpoint,
            mnt_sb,
        })
    }
}