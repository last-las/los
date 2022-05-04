use crate::vfs::inode::{InodeOperations, VfsInode, Rdev};
use alloc::rc::Rc;
use core::cell::RefCell;
use crate::vfs::dentry::VfsDentry;
use crate::vfs::super_block::SuperBlock;
use crate::fs::ramfs::{RamFileSystem, RamFsInode, add_ram_fs_instance, get_ram_fs_instance};
use crate::vfs::file::{FileOperations, File};
use alloc::vec::Vec;
use share::file::FileTypeFlag;

/// The function will create a new ramfs.
pub fn create_ramfs_super_block(rdev: Rdev) -> Option<Rc<RefCell<SuperBlock>>> {
    let new_ram_fs_instance = RamFileSystem::new();
    add_ram_fs_instance(rdev.into(), new_ram_fs_instance);
    // new super block
    let new_ramfs_sb = SuperBlock::new(rdev);
    // create root inode and dentry
    let root_ramfs_inode = get_ramfs_inode_from_related_ramfs(rdev.into(), 0).unwrap();
    let root_ramfs_dentry = create_dentry_from_ramfs_inode(root_ramfs_inode, new_ramfs_sb.clone());
    // set root dentry on super block
    new_ramfs_sb.borrow_mut().root = Some(root_ramfs_dentry.clone());

    Some(new_ramfs_sb)
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
        let minor_dev = super_block.borrow().rdev.into();
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
        let rdev = super_block.borrow().rdev.into();
        let ino = parent.borrow().ino;
        let ramfs_inode = get_ramfs_inode_from_related_ramfs(rdev, ino).unwrap();

        // lookup on `ramfs_inode`, if `name` already exists return None.
        if ramfs_inode.borrow().lookup(name).is_some() {
            return None;
        }
        let new_ramfs_inode = alloc_ramfs_inode_on_related_ramfs(rdev);
        new_ramfs_inode.borrow_mut().mark_as_file();
        new_ramfs_inode.borrow_mut().set_name(name);

        Some(create_dentry_from_ramfs_inode(new_ramfs_inode, super_block))
    }

    fn mkdir(&self, name: &str, parent: Rc<RefCell<VfsInode>>) -> Option<Rc<RefCell<VfsDentry>>> {
        let super_block = parent.borrow().super_block.clone();
        let rdev = super_block.borrow().rdev.into();
        let ino = parent.borrow().ino;
        let ramfs_inode = get_ramfs_inode_from_related_ramfs(rdev, ino).unwrap();

        // lookup on `ramfs_inode`, if `name` already exists then return None.
        if ramfs_inode.borrow().lookup(name).is_some() {
            return None;
        }
        let new_ramfs_inode = alloc_ramfs_inode_on_related_ramfs(rdev);
        new_ramfs_inode.borrow_mut().mark_as_dir();
        new_ramfs_inode.borrow_mut().set_name(name);
        ramfs_inode.borrow_mut().sub_nodes.push(new_ramfs_inode.clone());

        Some(create_dentry_from_ramfs_inode(new_ramfs_inode, super_block))
    }

    fn mknod(&self, name: &str, file_type: FileTypeFlag, rdev: Rdev, parent: Rc<RefCell<VfsInode>>)
        -> Option<Rc<RefCell<VfsDentry>>> {
        let super_block = parent.borrow().super_block.clone();
        let parent_rdev = super_block.borrow().rdev.into();
        let ino = parent.borrow().ino;
        let ramfs_inode = get_ramfs_inode_from_related_ramfs(parent_rdev, ino).unwrap();

        if ramfs_inode.borrow().lookup(name).is_some() {
            return None;
        }
        let new_ramfs_inode = alloc_ramfs_inode_on_related_ramfs(parent_rdev);
        new_ramfs_inode.borrow_mut().set_file_type(file_type);
        new_ramfs_inode.borrow_mut().set_rdev(rdev);
        new_ramfs_inode.borrow_mut().set_name(name);

        Some(create_dentry_from_ramfs_inode(new_ramfs_inode, super_block))
    }
}

impl FileOperations for RamFsFileOperations {
    fn read(&self, file: Rc<RefCell<File>>, size: usize) -> Vec<u8> {
        let file_ref = file.borrow();

        let rdev = file_ref.dentry.borrow().inode.borrow().super_block.borrow().rdev.into();
        let ino = file_ref.dentry.borrow().inode.borrow().ino;
        let ram_fs_inode = get_ramfs_inode_from_related_ramfs(rdev, ino).unwrap();

        // read content
        let content = ram_fs_inode.borrow().read(file_ref.pos, size);

        content
    }

    fn write(&self, file: Rc<RefCell<File>>, content: &[u8]) {
        let file_ref = file.borrow();

        let rdev = file_ref.dentry.borrow().inode.borrow().super_block.borrow().rdev.into();
        let ino = file_ref.dentry.borrow().inode.borrow().ino;
        let ram_fs_inode = get_ramfs_inode_from_related_ramfs(rdev, ino).unwrap();

        ram_fs_inode.borrow_mut().write(file_ref.pos, content);
    }

    fn readdir(&self, file: Rc<RefCell<File>>) -> Vec<Rc<RefCell<VfsDentry>>> {
        let file_ref = file.borrow();

        let super_block = file_ref.dentry.borrow().inode.borrow().super_block.clone();
        let rdev = super_block.borrow().rdev.into();
        let ino = file_ref.dentry.borrow().inode.borrow().ino;
        let ram_fs_inode = get_ramfs_inode_from_related_ramfs(rdev, ino).unwrap();

        // read the sub directories
        let sub_ramfs_inodes = ram_fs_inode.borrow_mut().read_dir();

        // convert `RamFsInode` to `Dentry`
        let mut ans = Vec::new();
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

fn get_ramfs_inode_from_related_ramfs(rdev: u64, ino: usize) -> Option<Rc<RefCell<RamFsInode>>> {
    let target_ramfs = get_ram_fs_instance(rdev).unwrap();

    target_ramfs.search_ramfs_inode(ino)
}

fn alloc_ramfs_inode_on_related_ramfs(rdev: u64) -> Rc<RefCell<RamFsInode>> {
    let target_ramfs = get_ram_fs_instance(rdev).unwrap();

    target_ramfs.alloc_ramfs_inode()
}