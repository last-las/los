use alloc::rc::Rc;
use alloc::string::String;
use crate::vfs::inode::VfsInode;
use core::cell::RefCell;
use alloc::vec::Vec;
use crate::vfs::super_block::SuperBlock;

pub struct VfsDentry {
    pub name: String,
    pub inode: Rc<RefCell<VfsInode>>,
    pub parent: Option<Rc<RefCell<VfsDentry>>>,
    pub children: Vec<Rc<RefCell<VfsDentry>>>,

    pub mnt: Option<Rc<RefCell<VfsMount>>>,
    /// `read_dir_flag` indicates whether `inode.borrow().iop.readdir()` has been invoked before, if so,
    /// there is no need to search on the real filesystem once again, because all the child directories
    /// can be found in `children` field.
    pub read_dir_flag: bool,
}

pub struct VfsMount {
    /// Root dentry of current fs.
    pub mount_root: Rc<RefCell<VfsDentry>>,
    /// dentry of mountpoint on parent fs.
    pub mount_point: Option<Rc<RefCell<VfsDentry>>>,
    /// Parent that current fs is mounted on.
    pub mount_parent: Option<Rc<RefCell<VfsMount>>>,
    /// Pointer to super block.
    pub mnt_sb: Rc<RefCell<SuperBlock>>,
}

impl VfsDentry {
    pub fn new(name: &str, inode: Rc<RefCell<VfsInode>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Self {
                name: String::from(name),
                inode,
                parent: None,
                children: Vec::new(),
                mnt: None,
                read_dir_flag: false,
            }
        ))
    }

    /// find out if the target dentry is already exists.
    pub fn cached_lookup(&self, name: &str) -> Option<Rc<RefCell<VfsDentry>>> {
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

    /// remove
    pub fn remove_cache(&mut self, name: &str) {
        for i in 0..self.children.len() {
            if self.children[i].borrow().name.as_str() == name {
                self.children.remove(i);
                return;
            }
        }
    }
}

impl VfsMount {
    pub fn new(mnt_sb: Rc<RefCell<SuperBlock>>) -> Rc<RefCell<Self>> {
        let mount_root = mnt_sb.borrow().root.as_ref().unwrap().clone();
        Rc::new(RefCell::new(
            Self {
                mount_root,
                mount_point: None,
                mount_parent: None,
                mnt_sb,
            }
        ))
    }

    pub fn set_mnt_point(&mut self, mount_point: Rc<RefCell<VfsDentry>>) {
        self.mount_point = Some(mount_point);
    }

    pub fn set_mnt_parent(&mut self, mount_parent: Rc<RefCell<VfsMount>>) {
        self.mount_parent = Some(mount_parent);
    }
}