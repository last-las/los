use alloc::rc::Rc;
use core::cell::RefCell;
use crate::vfs::super_block::SuperBlock;
use crate::vfs::inode::{InodeOperations, VfsInode, Rdev};
use crate::vfs::dentry::VfsDentry;
use share::file::FileTypeFlag;
use crate::vfs::file::{FileOperations, File};
use crate::fs::fatfs::{get_fatfs_root_inode, add_fatfs_instance};
use alloc::vec::Vec;
use simple_fat32::{FAT32Manager,VFile,ATTRIBUTE_DIRECTORY};
//use easy_fs::DiskInodeType;
use alloc::sync::Arc;
use crate::device::block::Block;
use share::syscall::error::{SysError, EEXIST, EPERM, ENOENT};
use user_lib::syscall::{virt_copy, getpid};

pub fn create_fatfs_super_block(rdev: Rdev) -> Option<Rc<RefCell<SuperBlock>>> {
    // create a new easy filesystem instance.
    let block = Block::new(rdev);
    let fat_fs = FAT32Manager::open(block);
    //let manager_reader = fat_fs.read();
    add_fatfs_instance(rdev.into(), fat_fs.clone());

    // create a new super block，dentry always /
    let sp = SuperBlock::new(rdev);
    let fat_read=fat_fs.read();
    let root_inode = fat_read.get_root_vfile(&fat_fs);
    let root_dentry =
        create_dentry_from_fatfs_inode("/",Arc::new(root_inode), sp.clone());
    sp.borrow_mut().root = Some(root_dentry);

    Some(sp)
}

//pub create_root_dentry()->Rc<RefCell<VfsDentry>>

pub struct FATFsInodeOperations;
pub struct FATFsFileOperations;

impl InodeOperations for FATFsInodeOperations {
    fn lookup(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Result<Rc<RefCell<VfsDentry>>, SysError> {
        let rdev = parent.borrow().super_block.borrow().rdev.into();
        let root_inode = get_fatfs_root_inode(rdev).unwrap();
        let result = root_inode.find_vfile_byname(name);//Option<VFile>
        if result.is_none() {
            return Err(SysError::new(ENOENT));
        }
        let fat_inode = result.unwrap();
        let sp = parent.borrow().super_block.clone();

        Ok(create_dentry_from_fatfs_inode(name, Arc::new(fat_inode), sp))
    }

    fn create(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Result<Rc<RefCell<VfsDentry>>, SysError> {
        let rdev = parent.borrow().super_block.borrow().rdev.into();
        let root_inode = get_fatfs_root_inode(rdev).unwrap();
        let result = root_inode.create(name,ATTRIBUTE_DIRECTORY);
        if result.is_none() {
            return Err(SysError::new(EEXIST));
        }
        let fatfs_inode = result.unwrap();
        let sp = parent.borrow().super_block.clone();

        Ok(create_dentry_from_fatfs_inode(name, fatfs_inode, sp))
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

    fn rmdir(&self, _name: &str, _parent: Rc<RefCell<VfsInode>>) -> Result<(), SysError> {
        return Err(SysError::new(EPERM));
    }
}

impl FileOperations for FATFsFileOperations {
    fn read(&self, file: Rc<RefCell<File>>, buf_ptr: usize, cnt: usize, proc_nr: usize) -> Result<usize, SysError> {
        let rdev = file.borrow().dentry.borrow().inode.borrow().super_block.borrow().rdev.into();
        let root_inode = get_fatfs_root_inode(rdev).unwrap();

        let name = file.borrow().dentry.borrow().name.clone();
        let ino = file.borrow().dentry.borrow().inode.borrow().ino;
        let pos = file.borrow().pos;

        let fatfs_inode = root_inode.find_vfile_byname(name.as_str()).unwrap();
        let write_ino = get_inode_id_from(Arc::new(fatfs_inode.clone()));
        assert_eq!(ino, write_ino);
        let mut content = vec![0; cnt];
        fatfs_inode.read_at(pos, content.as_mut_slice());
        let length = content.len();
        if length > 0 {
            virt_copy(getpid(), content.as_ptr() as usize, proc_nr, buf_ptr, length)?;
        }

        Ok(length)
    }

    fn write(&self, file: Rc<RefCell<File>>, buf_ptr: usize, cnt: usize, proc_nr: usize) -> Result<(), SysError> {
        let rdev = file.borrow().dentry.borrow().inode.borrow().super_block.borrow().rdev.into();
        let root_inode = get_fatfs_root_inode(rdev).unwrap();

        let name = file.borrow().dentry.borrow().name.clone();
        let ino = file.borrow().dentry.borrow().inode.borrow().ino;
        let pos = file.borrow().pos;

        let fatfs_inode = root_inode.find_vfile_byname(name.as_str()).unwrap();
        let write_ino = get_inode_id_from(Arc::new(fatfs_inode.clone()));
        assert_eq!(ino, write_ino);

        let content = vec![0; cnt];
        virt_copy(proc_nr, buf_ptr, getpid(), content.as_ptr() as usize, cnt)?;
        fatfs_inode.write_at(pos, content.as_slice());

        Ok(())
    }

    
    fn readdir(&self, file: Rc<RefCell<File>>) -> Vec<Rc<RefCell<VfsDentry>>> {
        let sp = file.borrow().dentry.borrow().inode.borrow().super_block.clone();
        let rdev = sp.borrow().rdev.into();
        let root_inode = get_fatfs_root_inode(rdev).unwrap();

        let names = root_inode.ls().unwrap();
        let mut results = Vec::new();
        for name in names {
            let fatfs_inode = root_inode.find_vfile_byname(name.0.as_str()).unwrap();
            let dentry = create_dentry_from_fatfs_inode(name.0.as_str(), 
            Arc::new(fatfs_inode), sp.clone());
            results.push(dentry);
        }

        results
    }
}

fn create_dentry_from_fatfs_inode(name: &str, fatfs_inode: Arc<VFile>, super_block: Rc<RefCell<SuperBlock>>)
    -> Rc<RefCell<VfsDentry>> {
    let ino = get_inode_id_from(fatfs_inode.clone());
    let size = fatfs_inode.size as usize;
    let file_type = match fatfs_inode.is_dir(){
        false => FileTypeFlag::DT_REG,
        true => FileTypeFlag::DT_DIR,
    };
    let iop = Rc::new(FATFsInodeOperations);//clone?
    let fop = Rc::new(FATFsFileOperations);
    let vfs_inode = VfsInode::new(ino, size, None, file_type, super_block, iop, fop);
    VfsDentry::new(name, vfs_inode)
}

fn get_inode_id_from(fatfs_inode: Arc<VFile>) -> usize {
    (fatfs_inode.short_sector << 32) | fatfs_inode.short_offset
}