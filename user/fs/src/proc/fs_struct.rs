use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use crate::vfs::dentry::{Dentry, VfsMount};
use crate::vfs::file::File;
use share::syscall::error::{SysError, ENFILE};

const FD_LIMIT: usize = 0xFF;

pub struct FsStruct {
    pub pwd: Rc<RefCell<Dentry>>,
    pub pwd_mnt: Rc<VfsMount>,
    pub root: Rc<RefCell<Dentry>>,
    pub root_mnt: Rc<VfsMount>,

    pub fd_table: Vec<Option<Rc<RefCell<File>>>>,
}

impl FsStruct {
    pub fn new(pwd: Rc<RefCell<Dentry>>, pwd_mnt: Rc<VfsMount>, root: Rc<RefCell<Dentry>>, root_mnt: Rc<VfsMount>) -> Rc<RefCell<FsStruct>> {
        Rc::new(RefCell::new(FsStruct {
            pwd,
            pwd_mnt,
            root,
            root_mnt,
            fd_table: Vec::new(),
        }))
    }

    pub fn alloc_fd(&mut self) -> Result<usize, SysError>{
        let result = self.fd_table.iter().enumerate().find(|(_, opt)| {
            opt.is_none()
        });

        if result.is_some() {
            Ok(result.unwrap().0)
        } else if self.fd_table.len() < FD_LIMIT {
            self.fd_table.push(None);
            Ok(self.fd_table.len())
        } else {
            Err(SysError::new(ENFILE))
        }
    }
}