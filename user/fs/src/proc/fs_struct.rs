use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use crate::vfs::dentry::{Dentry, VfsMount};
use crate::vfs::file::File;
use share::syscall::error::{SysError, ENFILE, EBADF};

const FD_LIMIT: usize = 0xFF;

#[derive(Clone)]
pub struct FsStruct {
    pub pwd: Rc<RefCell<Dentry>>,
    pub pwd_mnt: Rc<RefCell<VfsMount>>,
    pub root: Rc<RefCell<Dentry>>,
    pub root_mnt: Rc<RefCell<VfsMount>>,

    pub fd_table: Vec<Option<Rc<RefCell<File>>>>,
}

impl FsStruct {
    pub fn new(pwd: Rc<RefCell<Dentry>>, pwd_mnt: Rc<RefCell<VfsMount>>,
               root: Rc<RefCell<Dentry>>, root_mnt: Rc<RefCell<VfsMount>>) -> Rc<RefCell<FsStruct>> {
        Rc::new(RefCell::new(FsStruct {
            pwd,
            pwd_mnt,
            root,
            root_mnt,
            fd_table: Vec::new(),
        }))
    }

    pub fn clone_fs_struct(&self) -> Rc<RefCell<FsStruct>> {
        Rc::new(RefCell::new(FsStruct {
            pwd: self.pwd.clone(),
            pwd_mnt: self.pwd_mnt.clone(),
            root: self.root.clone(),
            root_mnt: self.root_mnt.clone(),
            fd_table: self.fd_table.clone(),
        }))
    }

    pub fn alloc_fd(&mut self) -> Result<usize, SysError> {
        for i in 0..self.fd_table.len() {
            if self.fd_table[i].is_none() {
                return Ok(i);
            }
        }

        if self.fd_table.len() < FD_LIMIT {
            self.fd_table.push(None);
            Ok(self.fd_table.len() - 1)
        } else {
            Err(SysError::new(ENFILE))
        }
    }

    pub fn get_file(&self, fd: usize) -> Result<Rc<RefCell<File>>, SysError> {
        if fd >= self.fd_table.len() || self.fd_table[fd].is_none() {
            Err(SysError::new(EBADF))
        } else {
            Ok(self.fd_table[fd].clone().unwrap())
        }
    }

    pub fn add_file(&mut self, fd: usize, file: Rc<RefCell<File>>) -> Result<(), SysError> {
        if fd >= FD_LIMIT {
            return Err(SysError::new(ENFILE));
        }
        for _ in self.fd_table.len()..fd + 1 {
            self.fd_table.push(None);
        }
        assert!(self.fd_table[fd].is_none());
        self.fd_table[fd] = Some(file);

        Ok(())
    }
}