use crate::vfs::inode::{InodeOperations, VfsInode, Rdev};
use alloc::rc::Rc;
use core::cell::RefCell;
use crate::vfs::dentry::VfsDentry;
use share::file::FileTypeFlag;
use crate::vfs::super_block::SuperBlock;
use crate::fs::ramfs::InoAllocator;
use crate::vfs::file::{FileOperations, File};
use alloc::vec::Vec;
use crate::device::block::{Block, BLOCK_SIZE};
use share::device::BlockDevice;
use crate::device::character::Character;

pub fn create_devfs_super_block(rdev: Rdev) -> Option<Rc<RefCell<SuperBlock>>> {
    let val: u64 = rdev.into();
    assert_eq!(val, 0 as u64); // We only allow one device filesystem
    let new_devfs_sb = SuperBlock::new(rdev);
    let ino =  alloc_devfs_ino();
    let iop = Rc::new(DevFsInodeOperations);
    let fop = Rc::new(DevFsFileOperations);
    let root_devfs_inode =
        VfsInode::new(ino, 0, None, FileTypeFlag::DT_DIR, new_devfs_sb.clone(), iop, fop);
    let root_devfs_dentry = VfsDentry::new("/", root_devfs_inode);
    new_devfs_sb.borrow_mut().root = Some(root_devfs_dentry);

    Some(new_devfs_sb)
}

pub struct DevFsInodeOperations;
pub struct DevFsFileOperations;

impl InodeOperations for  DevFsInodeOperations {
    fn lookup(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Option<Rc<RefCell<VfsDentry>>> {
        return None;
    }

    fn create(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Option<Rc<RefCell<VfsDentry>>> {
        return None;
    }

    fn mkdir(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Option<Rc<RefCell<VfsDentry>>> {
        return None;
    }

    fn mknod(&self, name: &str, file_type: FileTypeFlag, rdev: Rdev, parent: Rc<RefCell<VfsInode>>)
        -> Option<Rc<RefCell<VfsDentry>>> {
        let ino = alloc_devfs_ino();
        let sp = parent.borrow().super_block.clone();
        let iop = Rc::new(DevFsInodeOperations);
        let fop = Rc::new(DevFsFileOperations);

        let inode =
            VfsInode::new(ino, 0, Some(rdev),file_type,sp, iop, fop);
        let dentry = VfsDentry::new(name, inode);

        Some(dentry)
    }
}

impl FileOperations for DevFsFileOperations {
    fn read(&self, file: Rc<RefCell<File>>, size: usize) -> Vec<u8> {
        let inode = file.borrow().dentry.borrow().inode.clone();
        let rdev = inode.borrow().rdev.unwrap();
        let mut content = vec![0; size];

        match inode.borrow().file_type {
            FileTypeFlag::DT_BLK => {
                let block_device = Block::new(rdev);
                let pos = file.borrow().pos;
                let mut content_start = 0;

                let mut block_buffer = vec![0; BLOCK_SIZE];
                let start_block_id = pos / BLOCK_SIZE;
                let last_block_id = (pos + size - 1) / BLOCK_SIZE;

                for block_id in start_block_id..last_block_id + 1 {
                    block_device.read_block(block_id, block_buffer.as_mut_slice());
                    let mut start = 0;
                    let mut end = BLOCK_SIZE;
                    if block_id == start_block_id {
                        start = pos % BLOCK_SIZE;
                    }
                    if block_id == last_block_id {
                        end = (pos + size) % BLOCK_SIZE;
                    }
                    let src_slice: &[u8] = &block_buffer.as_slice()[start..end];
                    let dst_slice: &mut[u8] = &mut content.as_mut_slice()[content_start..content_start + src_slice.len()];
                    dst_slice.copy_from_slice(src_slice);

                    content_start += src_slice.len();
                }
            },
            FileTypeFlag::DT_CHR => {
                let chr_device = Character::new(rdev);
                chr_device.read(content.as_mut_slice());
            },
            _ => panic!("devfs cannot read on file type except block or chr!")
        };

        content
    }

    fn write(&self, file: Rc<RefCell<File>>, content: &[u8]) {
        let inode = file.borrow().dentry.borrow().inode.clone();
        let rdev = inode.borrow().rdev.unwrap();

        match inode.borrow().file_type {
            FileTypeFlag::DT_BLK => {
                panic!("System doesn't allow to write on block device now!!!");
            },
            FileTypeFlag::DT_CHR => {
                let chr_device = Character::new(rdev);
                chr_device.write(content);
            },
            _ => panic!("devfs cannot write on file type except block or chr!")
        };
    }

    fn readdir(&self, file: Rc<RefCell<File>>) -> Vec<Rc<RefCell<VfsDentry>>> {
        unimplemented!()
    }
}

fn alloc_devfs_ino() -> usize {
    unsafe {
        INO_ALLOCATOR.alloc()
    }
}

static mut INO_ALLOCATOR: InoAllocator = InoAllocator::new();