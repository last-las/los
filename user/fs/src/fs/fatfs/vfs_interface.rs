use alloc::rc::Rc;
use core::cell::RefCell;
use crate::vfs::super_block::SuperBlock;
use crate::vfs::inode::{InodeOperations, VfsInode, Rdev};
use crate::vfs::dentry::VfsDentry;
use share::file::FileTypeFlag;
use crate::vfs::file::{FileOperations, File};
use crate::fs::fatfs::{get_fatfs_root_inode, add_fatfs_instance,get_fatfs_instance_by};
use alloc::vec::Vec;
use alloc::string::String;
use simple_fat32::{FAT32Manager,VFile,ATTRIBUTE_DIRECTORY, ATTRIBUTE_ARCHIVE};
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

pub struct FATFsInodeOperations;
pub struct FATFsFileOperations;

impl InodeOperations for FATFsInodeOperations {
    //match
    fn lookup(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Result<Rc<RefCell<VfsDentry>>, SysError> {
        let rdev = parent.borrow().super_block.borrow().rdev.into();
        let s_sector= parent.borrow().ino >> 32;
        let s_offset= parent.borrow().ino & (0x00000000ffffffff as usize);
        let fat=get_fatfs_instance_by(rdev).unwrap();
        let block = fat.try_read().unwrap().block_device.clone();
        let vfile=VFile::new(
            String::from(""),
            s_sector,
            s_offset,
            vec![],
            ATTRIBUTE_DIRECTORY,
            Arc::clone(fat),
            block);
        let result = vfile.find_vfile_byname(name);//Option<VFile>
        if result.is_none() {
            return Err(SysError::new(ENOENT));
        }
        let fat_inode = result.unwrap();
        let sp = parent.borrow().super_block.clone();

        Ok(create_dentry_from_fatfs_inode(name, Arc::new(fat_inode), sp))
    }

    //match
    fn create(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Result<Rc<RefCell<VfsDentry>>, SysError> {
        let rdev = parent.borrow().super_block.borrow().rdev.into();
        let s_sector= parent.borrow().ino >> 32;
        let s_offset= parent.borrow().ino & (0x00000000ffffffff as usize);
        let fat=get_fatfs_instance_by(rdev).unwrap();
        let block = fat.try_read().unwrap().block_device.clone();
        let vfile=VFile::new(
            String::from(""),
            s_sector,
            s_offset,
            vec![],
            ATTRIBUTE_DIRECTORY,
            Arc::clone(fat),
            block);
        let result = vfile.create(name,ATTRIBUTE_ARCHIVE);
        if result.is_none() {
            return Err(SysError::new(EEXIST));
        }
        let fatfs_inode = result.unwrap();
        let sp = parent.borrow().super_block.clone();

        Ok(create_dentry_from_fatfs_inode(name, fatfs_inode, sp))
        
    }

    //match
    fn mkdir(&self, _name: &str, _parent: Rc<RefCell<VfsInode>>) -> Result<Rc<RefCell<VfsDentry>>, SysError> {
        let sp = _parent.borrow().super_block.clone();
        let rdev = sp.borrow().rdev.into();
        let s_sector=_parent.borrow().ino >> 32;
        let s_offset=_parent.borrow().ino & (0x00000000ffffffff as usize);
        let fat=get_fatfs_instance_by(rdev).unwrap();
        let block = fat.try_read().unwrap().block_device.clone();
        let vfile=VFile::new(
            String::from(""),
            s_sector,
            s_offset,
            vec![],
            ATTRIBUTE_DIRECTORY,
            Arc::clone(fat),
            block);
        let new_dir = vfile.create(_name,ATTRIBUTE_DIRECTORY);
        Ok(create_dentry_from_fatfs_inode(_name, new_dir.unwrap(),sp))
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
    //s
    fn read(&self, file: Rc<RefCell<File>>, buf_ptr: usize, cnt: usize, proc_nr: usize) -> Result<usize, SysError> {
        /*let rdev = file.borrow().dentry.borrow().inode.borrow().super_block.borrow().rdev.into();
        let name = file.borrow().dentry.borrow().name.clone();
        let inode =file.borrow().dentry.borrow().parent.as_ref().unwrap().borrow().inode.clone();
        let p_name=file.borrow().dentry.borrow().parent.as_ref().unwrap().borrow().name.clone();
        let s_sector=inode.borrow().ino >> 32;
        let s_offset=inode.borrow().ino & (0x00000000ffffffff as usize);
        let fat=get_fatfs_instance_by(rdev).unwrap();
        let block = fat.try_read().unwrap().block_device.clone(); 
        //这个是file所在dentry的
        let vfile = VFile::new(
            p_name,
            s_sector,
            s_offset,
            vec![],
            ATTRIBUTE_DIRECTORY,
            Arc::clone(fat),
            block,
        );   
        let ino = file.borrow().dentry.borrow().inode.borrow().ino;
        let pos = file.borrow().pos;
        let fatfs_inode = vfile.find_vfile_byname(name.as_str()).unwrap();
        let write_ino = get_inode_id_from(Arc::new(fatfs_inode.clone()));
        assert_eq!(ino, write_ino);
        let mut content = vec![0; cnt];
        fatfs_inode.read_at(pos, content.as_mut_slice());
        let length = content.len();
        if length > 0 {
            virt_copy(getpid(), content.as_ptr() as usize, proc_nr, buf_ptr, length)?;
        }
        Ok(length)*/
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

    //unmatch
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

    //match
    fn readdir(&self, file: Rc<RefCell<File>>) -> Vec<Rc<RefCell<VfsDentry>>> {
        let sp = file.borrow().dentry.borrow().inode.borrow().super_block.clone();
        let rdev = sp.borrow().rdev.into();
        let inode = file.borrow().dentry.borrow().inode.clone();

        let dentry_name=file.borrow().dentry.borrow().name.clone();
        let s_sector=inode.borrow().ino >> 32;
        let s_offset=inode.borrow().ino & (0x00000000ffffffff as usize);
        let fat=get_fatfs_instance_by(rdev).unwrap();
        let block = fat.try_read().unwrap().block_device.clone();
        let vfile=VFile::new(
            dentry_name,
            s_sector,
            s_offset,
            vec![],
            ATTRIBUTE_DIRECTORY,
            Arc::clone(fat),
            block
            );      
        //下一步，shell根据目录/文件不同显示不同的颜色
        let names = vfile.ls_lite().unwrap();
        let mut results = Vec::new();
        for name in names {
            //根目录ls会出错
            let fatfs_inode = vfile.find_vfile_byname(name.0.as_str()).unwrap();
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
    let size = fatfs_inode.get_size() as usize;
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