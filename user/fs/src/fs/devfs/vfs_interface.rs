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
use share::syscall::error::{SysError, EPERM};
use user_lib::syscall::{virt_copy, getpid};

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
    fn lookup(&self, _name: &str, _parent: Rc<RefCell<VfsInode>>) -> Result<Rc<RefCell<VfsDentry>>, SysError> {
        return Err(SysError::new(EPERM));
    }

    fn create(&self, _name: &str, _parent: Rc<RefCell<VfsInode>>) -> Result<Rc<RefCell<VfsDentry>>, SysError> {
        return Err(SysError::new(EPERM));
    }

    fn mkdir(&self, _name: &str, _parent: Rc<RefCell<VfsInode>>) -> Result<Rc<RefCell<VfsDentry>>, SysError> {
        return Err(SysError::new(EPERM));
    }


    fn mknod(&self, name: &str, file_type: FileTypeFlag, rdev: Rdev, parent: Rc<RefCell<VfsInode>>)
        -> Result<Rc<RefCell<VfsDentry>>, SysError> {
        let ino = alloc_devfs_ino();
        let sp = parent.borrow().super_block.clone();
        let iop = Rc::new(DevFsInodeOperations);
        let fop = Rc::new(DevFsFileOperations);

        let inode =
            VfsInode::new(ino, 0, Some(rdev),file_type,sp, iop, fop);
        let dentry = VfsDentry::new(name, inode);

        Ok(dentry)
    }

    fn unlink(&self, _name: &str, _parent: Rc<RefCell<VfsInode>>) -> Result<(), SysError> {
        return Err(SysError::new(EPERM));
    }

    fn rmdir(&self, _name: &str, _parent: Rc<RefCell<VfsInode>>) -> Result<(), SysError> {
        return Err(SysError::new(EPERM));
    }
}

impl FileOperations for DevFsFileOperations {
    fn read(&self, file: Rc<RefCell<File>>, buf_ptr: usize, cnt: usize, proc_nr: usize) -> Result<usize, SysError> {
        let inode = file.borrow().dentry.borrow().inode.clone();
        let rdev = inode.borrow().rdev.unwrap();
        let mut content = vec![0; cnt];

        match inode.borrow().file_type {
            FileTypeFlag::DT_BLK => {
                let block_device = Block::new(rdev);
                let pos = file.borrow().pos;
                let mut content_start = 0;

                let mut block_buffer = vec![0; BLOCK_SIZE];
                let start_block_id = pos / BLOCK_SIZE;
                let last_block_id = (pos + cnt - 1) / BLOCK_SIZE;

                for block_id in start_block_id..last_block_id + 1 {
                    block_device.read_block(block_id, block_buffer.as_mut_slice());
                    let mut start = 0;
                    let mut end = BLOCK_SIZE;
                    if block_id == start_block_id {
                        start = pos % BLOCK_SIZE;
                    }
                    if block_id == last_block_id {
                        end = (pos + cnt) % BLOCK_SIZE;
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

        let length = content.len();
        if length > 0 {
            virt_copy(getpid(), content.as_ptr() as usize, proc_nr, buf_ptr, length)?;
        }

        Ok(length)
    }

    fn write(&self, file: Rc<RefCell<File>>, buf_ptr: usize, cnt: usize, proc_nr: usize) -> Result<(), SysError> {
        let inode = file.borrow().dentry.borrow().inode.clone();
        let rdev = inode.borrow().rdev.unwrap();
        let content = vec![0; cnt];
        virt_copy(proc_nr, buf_ptr, getpid(), content.as_ptr() as usize, cnt)?;

        match inode.borrow().file_type {
            FileTypeFlag::DT_BLK => {
                return Err(SysError::new(EPERM));
            },
            FileTypeFlag::DT_CHR => {
                let chr_device = Character::new(rdev);
                chr_device.write(content.as_slice());
            },
            _ => panic!("devfs cannot write on file type except block or chr!")
        };

        Ok(())
    }

    fn readdir(&self, _file: Rc<RefCell<File>>) -> Vec<Rc<RefCell<VfsDentry>>> {
        return Vec::new();
    }
}

fn alloc_devfs_ino() -> usize {
    unsafe {
        INO_ALLOCATOR.alloc()
    }
}

static mut INO_ALLOCATOR: InoAllocator = InoAllocator::new();