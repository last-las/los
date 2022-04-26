use crate::vfs::inode::{InodeOperations, Inode, Rdev};
use alloc::rc::Rc;
use core::cell::RefCell;
use crate::vfs::dentry::{Dentry, VfsMount};
use crate::vfs::super_block::SuperBlock;
use super::RAM_FILE_SYSTEMS;
use crate::fs::ramfs::{RamFileSystem, RamFsInode, RAMFS_MAJOR_DEV};
use crate::vfs::file::{FileOperations, File};
use alloc::boxed::Box;
use alloc::vec::Vec;
use share::file::FileTypeFlag;

/// Though it's name includes "read", it actually creates a new ram filesystem, append it to global
/// vector `RAM_FILE_SYSTEMS`, and return a `SuperBlock` structure.
pub fn read_ramfs_super_block(minor_dev: u32) -> Rc<RefCell<SuperBlock>> {
    unsafe {
        while minor_dev as usize + 1 > RAM_FILE_SYSTEMS.len() {
            RAM_FILE_SYSTEMS.push(None);
        }
        assert!(RAM_FILE_SYSTEMS[minor_dev as usize].is_none());
        RAM_FILE_SYSTEMS[minor_dev as usize] = Some(RamFileSystem::new());
    }
    // find out root ramfs inode and init related dir entry.
    let new_ramfs_sb = SuperBlock::new( Rdev::new(minor_dev, RAMFS_MAJOR_DEV));
    let root_ramfs_inode = get_ramfs_inode_from_related_ramfs(minor_dev, 0).unwrap();
    let root_dir_entry = create_dentry_from_ramfs_inode(root_ramfs_inode, new_ramfs_sb.clone());
    new_ramfs_sb.borrow_mut().root = Some(root_dir_entry.clone());
    let mnt = VfsMount::new(new_ramfs_sb.clone());
    root_dir_entry.borrow_mut().mnt = Some(mnt);

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
    fn lookup(&self, name: &str, parent: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Dentry>>> {
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

    fn create(&self, name: &str, parent: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Dentry>>> {
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

    fn mkdir(&self, name: &str, parent: Rc<RefCell<Inode>>) -> Option<Rc<RefCell<Dentry>>> {
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

    fn mknod(&self, name: &str, file_type: FileTypeFlag, rdev: Rdev, parent: Rc<RefCell<Inode>>)
        -> Option<Rc<RefCell<Dentry>>> {
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
        println!("read ino:{}", ino);
        let ram_fs_inode = get_ramfs_inode_from_related_ramfs(minor_dev, ino).unwrap();

        // read content
        let content = ram_fs_inode.borrow().read(file_ref.pos, size);

        content
    }

    fn write(&self, file: Rc<RefCell<File>>, content: &[u8]) {
        let mut file_ref = file.borrow();

        let minor_dev = file_ref.dentry.borrow().inode.borrow().super_block.borrow().rdev.minor;
        let ino = file_ref.dentry.borrow().inode.borrow().ino;
        println!("write ino:{}", ino);
        let ram_fs_inode = get_ramfs_inode_from_related_ramfs(minor_dev, ino).unwrap();

        let content = Vec::from(content);
        ram_fs_inode.borrow_mut().write(file_ref.pos, content);
    }

    fn readdir(&self, file: Rc<RefCell<File>>) -> Vec<Rc<RefCell<Dentry>>> {
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

fn create_dentry_from_ramfs_inode(new_ramfs_inode: Rc<RefCell<RamFsInode>>, super_block: Rc<RefCell<SuperBlock>>) -> Rc<RefCell<Dentry>> {
    let name = new_ramfs_inode.borrow().name.clone();
    let target_inode = new_ramfs_inode.borrow().get_vfs_inode(super_block);
    let target_dentry = Dentry::new(name.as_str(), target_inode);

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