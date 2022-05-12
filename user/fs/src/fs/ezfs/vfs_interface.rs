use alloc::rc::Rc;
use core::cell::RefCell;
use crate::vfs::super_block::SuperBlock;
use crate::vfs::inode::{InodeOperations, VfsInode, Rdev};
use crate::vfs::dentry::VfsDentry;
use share::file::FileTypeFlag;
use crate::vfs::file::{FileOperations, File};
use alloc::vec::Vec;
use crate::fs::ezfs::{add_ez_fs_instance, get_ez_fs_root_inode};
use easy_fs::{Inode, EasyFileSystem};
use easy_fs::DiskInodeType;
use alloc::sync::Arc;
use crate::device::block::Block;
use share::syscall::error::{SysError, EEXIST, EPERM};

pub fn create_ezfs_super_block(rdev: Rdev) -> Option<Rc<RefCell<SuperBlock>>> {
    // create a new easy filesystem instance.
    let block = Block::new(rdev);
    let easy_fs = EasyFileSystem::open(block);
    add_ez_fs_instance(rdev.into(), easy_fs.clone());

    // create a new super block
    let sp = SuperBlock::new(rdev);
    let root_inode = EasyFileSystem::root_inode(&easy_fs);
    let root_dentry =
        create_dentry_from_ezfs_inode("/",root_inode, sp.clone());
    sp.borrow_mut().root = Some(root_dentry);

    Some(sp)
}

pub struct EzFsInodeOperations;
pub struct EzFsFileOperations;

impl InodeOperations for EzFsInodeOperations {
    fn lookup(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Result<Rc<RefCell<VfsDentry>>, SysError> {
        let rdev = parent.borrow().super_block.borrow().rdev.into();
        let root_inode = get_ez_fs_root_inode(rdev).unwrap();
        let result = root_inode.find(name);
        if result.is_none() {
            return Err(SysError::new(EEXIST));
        }
        let ezfs_inode = result.unwrap();
        let sp = parent.borrow().super_block.clone();

        Ok(create_dentry_from_ezfs_inode(name, ezfs_inode, sp))
    }

    fn create(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Result<Rc<RefCell<VfsDentry>>, SysError> {
        let rdev = parent.borrow().super_block.borrow().rdev.into();
        let root_inode = get_ez_fs_root_inode(rdev).unwrap();
        let result = root_inode.create(name);
        if result.is_none() {
            return Err(SysError::new(EEXIST));
        }
        let ezfs_inode = result.unwrap();
        let sp = parent.borrow().super_block.clone();

        Ok(create_dentry_from_ezfs_inode(name, ezfs_inode, sp))
    }

    fn mkdir(&self, _name: &str, _parent: Rc<RefCell<VfsInode>>) -> Result<Rc<RefCell<VfsDentry>>, SysError> {
        return Err(SysError::new(EPERM));
    }

    fn mknod(&self, _name: &str, _file_type: FileTypeFlag, _rdev: Rdev, _parent: Rc<RefCell<VfsInode>>)
        -> Result<Rc<RefCell<VfsDentry>>, SysError> {
        return Err(SysError::new(EPERM));
    }

    fn unlink(&self, _name: &str, _parent: Rc<RefCell<VfsInode>>) -> Result<(), SysError> {
        return Err(SysError::new(EPERM));
    }
}

impl FileOperations for EzFsFileOperations {
    fn read(&self, file: Rc<RefCell<File>>, size: usize) -> Vec<u8> {
        let rdev = file.borrow().dentry.borrow().inode.borrow().super_block.borrow().rdev.into();
        let root_inode = get_ez_fs_root_inode(rdev).unwrap();

        let name = file.borrow().dentry.borrow().name.clone();
        let ino = file.borrow().dentry.borrow().inode.borrow().ino;
        let pos = file.borrow().pos;

        let ezfs_inode = root_inode.find(name.as_str()).unwrap();
        let write_ino = get_inode_id_from(ezfs_inode.clone());
        assert_eq!(ino, write_ino);
        let mut content = vec![0; size];
        ezfs_inode.read_at(pos, content.as_mut_slice());
        content
    }

    fn write(&self, file: Rc<RefCell<File>>, content: &[u8]) {
        let rdev = file.borrow().dentry.borrow().inode.borrow().super_block.borrow().rdev.into();
        let root_inode = get_ez_fs_root_inode(rdev).unwrap();

        let name = file.borrow().dentry.borrow().name.clone();
        let ino = file.borrow().dentry.borrow().inode.borrow().ino;
        let pos = file.borrow().pos;

        let ezfs_inode = root_inode.find(name.as_str()).unwrap();
        let write_ino = get_inode_id_from(ezfs_inode.clone());
        assert_eq!(ino, write_ino);
        ezfs_inode.write_at(pos, content);
    }

    fn readdir(&self, file: Rc<RefCell<File>>) -> Vec<Rc<RefCell<VfsDentry>>> {
        let sp = file.borrow().dentry.borrow().inode.borrow().super_block.clone();
        let rdev = sp.borrow().rdev.into();
        let root_inode = get_ez_fs_root_inode(rdev).unwrap();

        let names = root_inode.ls();
        let mut results = Vec::new();
        for name in names {
            let ezfs_inode = root_inode.find(name.as_str()).unwrap();
            let dentry = create_dentry_from_ezfs_inode(name.as_str(), ezfs_inode, sp.clone());
            results.push(dentry);
        }

        results
    }
}

fn create_dentry_from_ezfs_inode(name: &str, ezfs_inode: Arc<Inode>, super_block: Rc<RefCell<SuperBlock>>)
    -> Rc<RefCell<VfsDentry>> {
    let ino = get_inode_id_from(ezfs_inode.clone());
    let size = ezfs_inode.read_disk_inode(|disk_inode| {
        disk_inode.size
    }) as usize;
    let file_type = match ezfs_inode.read_disk_inode(|disk_inode| { disk_inode.type_ }) {
        DiskInodeType::File => FileTypeFlag::DT_REG,
        DiskInodeType::Directory => FileTypeFlag::DT_DIR,
    };
    let iop = Rc::new(EzFsInodeOperations); // No need to create a new one.. we can clone on the root inode, so does fop.
    let fop = Rc::new(EzFsFileOperations);
    let vfs_inode = VfsInode::new(ino, size, None, file_type, super_block, iop, fop);
    VfsDentry::new(name, vfs_inode)
}

fn get_inode_id_from(ezfs_inode: Arc<Inode>) -> usize {
    (ezfs_inode.block_id << 32) | ezfs_inode.block_offset
}