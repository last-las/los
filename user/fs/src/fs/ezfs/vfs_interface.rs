use alloc::rc::Rc;
use core::cell::RefCell;
use crate::vfs::super_block::SuperBlock;
use crate::vfs::inode::{InodeOperations, VfsInode, Rdev};
use crate::vfs::dentry::VfsDentry;
use share::file::FileTypeFlag;
use crate::vfs::file::{FileOperations, File};
use alloc::vec::Vec;
use super::ROOT_INODE;
use crate::fs::ezfs::EZFS_MAJOR_DEV;
use easy_fs::Inode;
use easy_fs::DiskInodeType;
use alloc::sync::Arc;
use alloc::boxed::Box;

pub fn read_ezfs_super_block(minor_dev: u32) -> Rc<RefCell<SuperBlock>> {
    assert_eq!(minor_dev, 0);
    let rdev = Rdev::new(minor_dev, EZFS_MAJOR_DEV);
    let sp = SuperBlock::new(rdev);
    let root_dentry =
        create_dentry_from_ezfs_inode("/", ROOT_INODE.clone(), sp.clone());
    sp.borrow_mut().root = Some(root_dentry);

    sp
}

pub struct EzFsInodeOperations;
pub struct EzFsFileOperations;

impl InodeOperations for EzFsInodeOperations {
    fn lookup(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Option<Rc<RefCell<VfsDentry>>> {
        assert_eq!(parent.borrow().ino, 0);
        let result = ROOT_INODE.find(name);
        if result.is_none() {
            return None;
        }
        let ezfs_inode = result.unwrap();
        let sp = parent.borrow().super_block.clone();

        Some(create_dentry_from_ezfs_inode(name, ezfs_inode, sp))
    }

    fn create(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Option<Rc<RefCell<VfsDentry>>> {
        assert_eq!(parent.borrow().ino, 0);
        let result = ROOT_INODE.create(name);
        if result.is_none() {
            return None;
        }
        let ezfs_inode = result.unwrap();
        let sp = parent.borrow().super_block.clone();

        Some(create_dentry_from_ezfs_inode(name, ezfs_inode, sp))
    }

    fn mkdir(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Option<Rc<RefCell<VfsDentry>>> {
        return None;
    }

    fn mknod(&self, name: &str, file_type: FileTypeFlag, rdev: Rdev, parent: Rc<RefCell<VfsInode>>) -> Option<Rc<RefCell<VfsDentry>>> {
        return None;
    }
}

impl FileOperations for EzFsFileOperations {
    fn read(&self, file: Rc<RefCell<File>>, size: usize) -> Vec<u8> {
        let name = file.borrow().dentry.borrow().name.clone();
        let ino = file.borrow().dentry.borrow().inode.borrow().ino;
        let pos = file.borrow().pos;

        let ezfs_inode = ROOT_INODE.find(name.as_str()).unwrap();
        let write_ino = get_inode_id_from(ezfs_inode.clone());
        assert_eq!(ino, write_ino);
        let mut content = vec![0; size];
        ezfs_inode.read_at(pos, content.as_mut_slice());
        content
    }

    fn write(&self, file: Rc<RefCell<File>>, content: &[u8]) {
        let name = file.borrow().dentry.borrow().name.clone();
        let ino = file.borrow().dentry.borrow().inode.borrow().ino;
        let pos = file.borrow().pos;

        let ezfs_inode = ROOT_INODE.find(name.as_str()).unwrap();
        let write_ino = get_inode_id_from(ezfs_inode.clone());
        assert_eq!(ino, write_ino);
        ezfs_inode.write_at(pos, content);
    }

    fn readdir(&self, file: Rc<RefCell<File>>) -> Vec<Rc<RefCell<VfsDentry>>> {
        let ino = file.borrow().dentry.borrow().inode.borrow().ino;
        let sp = file.borrow().dentry.borrow().inode.borrow().super_block.clone();

        let names = ROOT_INODE.ls();
        let mut results = Vec::new();
        for name in names {
            let ezfs_inode = ROOT_INODE.find(name.as_str()).unwrap();
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
    let iop = Rc::new(EzFsInodeOperations);
    let fop = Rc::new(EzFsFileOperations);
    let vfs_inode = VfsInode::new(ino, size, None, file_type, super_block, iop, fop);
    VfsDentry::new(name, vfs_inode)
}

fn get_inode_id_from(ezfs_inode: Arc<Inode>) -> usize {
    (ezfs_inode.block_id << 32) | ezfs_inode.block_offset
}