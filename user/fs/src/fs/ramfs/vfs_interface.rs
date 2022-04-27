use crate::vfs::inode::{InodeOperations, VfsInode, Rdev};
use alloc::rc::Rc;
use core::cell::RefCell;
use crate::vfs::dentry::{VfsDentry, VfsMount};
use crate::vfs::super_block::SuperBlock;
use super::RAM_FILE_SYSTEMS;
use crate::fs::ramfs::{RamFileSystem, RamFsInode, RAMFS_MAJOR_DEV};
use crate::vfs::file::{FileOperations, File};
use alloc::boxed::Box;
use alloc::vec::Vec;
use share::file::FileTypeFlag;

/// If there is a ramfs matched to `minor_dev`, it clones its super block and return.
/// The function will create a new ramfs when it doesn't find one.
pub fn read_ramfs_super_block(minor_dev: u32) -> Rc<RefCell<SuperBlock>> {
    unsafe {
        while minor_dev as usize + 1 > RAM_FILE_SYSTEMS.len() {
            RAM_FILE_SYSTEMS.push(None);
        }
        // There is a ramfs matched to `minor_dev`, clone super block and return.
        if RAM_FILE_SYSTEMS[minor_dev as usize].is_some() {
            return RAM_FILE_SYSTEMS[minor_dev as usize].as_ref().unwrap().super_block.clone().unwrap();
        }
        // Create a new ramfs.
        RAM_FILE_SYSTEMS[minor_dev as usize] = Some(RamFileSystem::new());
    }

    // new super block
    let new_ramfs_sb = SuperBlock::new( Rdev::new(minor_dev, RAMFS_MAJOR_DEV));
    // create root inode and dentry
    let root_ramfs_inode = get_ramfs_inode_from_related_ramfs(minor_dev, 0).unwrap();
    let root_ramfs_dentry = create_dentry_from_ramfs_inode(root_ramfs_inode, new_ramfs_sb.clone());
    // set root dentry on super block
    new_ramfs_sb.borrow_mut().root = Some(root_ramfs_dentry.clone());

    // set super block on the ram filesystem
    unsafe {
        RAM_FILE_SYSTEMS[minor_dev as usize].as_mut().unwrap().super_block = Some(new_ramfs_sb.clone());
    }

    new_ramfs_sb
}

// pub struct RamFsSuperBlockOperations;
pub struct RamFsInodeOperations;
pub struct RamFsFileOperations;

/*impl SuperBlockOperations for RamFsSuperBlockOperations {
    fn read_inode(&self, ino: usize, dev: usize) -> Option<Rc<RefCell<Inode>>> {
        todo!()
    }

    fn alloc_inode(&self, dev: usize) -> Option<Rc<RefCell<Inode>>> {
        todo!()
    }

    fn write_inode(&self, inode: Rc<RefCell<Inode>>) {
        todo!()
    }

    fn sync(&self) {
        todo!()
    }
}*/

impl InodeOperations for RamFsInodeOperations {
    fn lookup(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Option<Rc<RefCell<VfsDentry>>> {
        let super_block = parent.borrow().super_block.clone();
        let minor_dev = super_block.borrow().rdev.minor;
        let ino = parent.borrow().ino;
        let ramfs_inode = get_ramfs_inode_from_related_ramfs(minor_dev, ino).unwrap();

        // lookup on the `ramfs_inode`
        let result = ramfs_inode.borrow().lookup(name);
        if result.is_none() {
            return None;
        }
        let target_ramfs_inode = result.unwrap();

        Some(create_dentry_from_ramfs_inode(target_ramfs_inode, super_block))
    }

    fn create(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Option<Rc<RefCell<VfsDentry>>> {
        let super_block = parent.borrow().super_block.clone();
        let minor_dev = super_block.borrow().rdev.minor;
        let ino = parent.borrow().ino;
        let ramfs_inode = get_ramfs_inode_from_related_ramfs(minor_dev, ino).unwrap();

        // lookup on `ramfs_inode`, if `name` already exists return None.
        if ramfs_inode.borrow().lookup(name).is_some() {
            return None;
        }
        let new_ramfs_inode = alloc_ramfs_inode_on_related_ramfs(minor_dev);
        new_ramfs_inode.borrow_mut().mark_as_file();
        new_ramfs_inode.borrow_mut().set_name(name);

        Some(create_dentry_from_ramfs_inode(new_ramfs_inode, super_block))
    }

    fn mkdir(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Option<Rc<RefCell<VfsDentry>>> {
        let super_block = parent.borrow().super_block.clone();
        let minor_dev = super_block.borrow().rdev.minor;
        let ino = parent.borrow().ino;
        let ramfs_inode = get_ramfs_inode_from_related_ramfs(minor_dev, ino).unwrap();

        // lookup on `ramfs_inode`, if `name` already exists then return None.
        if ramfs_inode.borrow().lookup(name).is_some() {
            return None;
        }
        let new_ramfs_inode = alloc_ramfs_inode_on_related_ramfs(minor_dev);
        new_ramfs_inode.borrow_mut().mark_as_dir();
        new_ramfs_inode.borrow_mut().set_name(name);
        ramfs_inode.borrow_mut().sub_nodes.push(new_ramfs_inode.clone());

        Some(create_dentry_from_ramfs_inode(new_ramfs_inode, super_block))
    }

    fn mknod(&self, name: &str, file_type: FileTypeFlag, rdev: Rdev, parent: Rc<RefCell<VfsInode>>)
        -> Option<Rc<RefCell<VfsDentry>>> {
        let super_block = parent.borrow().super_block.clone();
        let minor_dev = super_block.borrow().rdev.minor;
        let ino = parent.borrow().ino;
        let ramfs_inode = get_ramfs_inode_from_related_ramfs(minor_dev, ino).unwrap();

        if ramfs_inode.borrow().lookup(name).is_some() {
            return None;
        }
        let new_ramfs_inode = alloc_ramfs_inode_on_related_ramfs(minor_dev);
        new_ramfs_inode.borrow_mut().set_file_type(file_type);
        new_ramfs_inode.borrow_mut().set_rdev(rdev);
        new_ramfs_inode.borrow_mut().set_name(name);

        Some(create_dentry_from_ramfs_inode(new_ramfs_inode, super_block))
    }
}

impl FileOperations for RamFsFileOperations {
    fn read(&self, file: Rc<RefCell<File>>, size: usize) -> Vec<u8> {
        let mut file_ref = file.borrow();

        let minor_dev = file_ref.dentry.borrow().inode.borrow().super_block.borrow().rdev.minor;
        let ino = file_ref.dentry.borrow().inode.borrow().ino;
        let ram_fs_inode = get_ramfs_inode_from_related_ramfs(minor_dev, ino).unwrap();

        // read content
        let content = ram_fs_inode.borrow().read(file_ref.pos, size);

        content
    }

    fn write(&self, file: Rc<RefCell<File>>, content: &[u8]) {
        let mut file_ref = file.borrow();

        let minor_dev = file_ref.dentry.borrow().inode.borrow().super_block.borrow().rdev.minor;
        let ino = file_ref.dentry.borrow().inode.borrow().ino;
        let ram_fs_inode = get_ramfs_inode_from_related_ramfs(minor_dev, ino).unwrap();

        ram_fs_inode.borrow_mut().write(file_ref.pos, content);
    }

    fn readdir(&self, file: Rc<RefCell<File>>) -> Vec<Rc<RefCell<VfsDentry>>> {
        let mut file_ref = file.borrow();

        let super_block = file_ref.dentry.borrow().inode.borrow().super_block.clone();
        let minor_dev = super_block.borrow().rdev.minor;
        let ino = file_ref.dentry.borrow().inode.borrow().ino;
        let ram_fs_inode = get_ramfs_inode_from_related_ramfs(minor_dev, ino).unwrap();

        // read the sub directories
        let sub_ramfs_inodes = ram_fs_inode.borrow_mut().read_dir();

        // convert `RamFsInode` to `Dentry`
        let mut ans = Vec::new();
        let inode = file_ref.dentry.borrow().inode.clone();
        for sub_ramfs_inode in sub_ramfs_inodes {
            ans.push(create_dentry_from_ramfs_inode(sub_ramfs_inode,super_block.clone()));
        }

        ans
    }
}

fn create_dentry_from_ramfs_inode(new_ramfs_inode: Rc<RefCell<RamFsInode>>, super_block: Rc<RefCell<SuperBlock>>) -> Rc<RefCell<VfsDentry>> {
    let name = new_ramfs_inode.borrow().name.clone();
    let target_inode = new_ramfs_inode.borrow().get_vfs_inode(super_block);
    let target_dentry = VfsDentry::new(name.as_str(), target_inode);

    target_dentry
}

fn get_ramfs_inode_from_related_ramfs(minor_dev: u32, ino: usize) -> Option<Rc<RefCell<RamFsInode>>> {
    let target_ramfs = unsafe {
        RAM_FILE_SYSTEMS[minor_dev as usize].as_ref().unwrap()
    };

    target_ramfs.search_ramfs_inode(ino)
}

fn alloc_ramfs_inode_on_related_ramfs(minor_dev: u32) -> Rc<RefCell<RamFsInode>> {
    let target_ramfs = unsafe {
        RAM_FILE_SYSTEMS[minor_dev as usize].as_mut().unwrap()
    };

    target_ramfs.alloc_ramfs_inode()
}