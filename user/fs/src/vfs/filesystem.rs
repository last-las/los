use crate::vfs::super_block::SuperBlock;
use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;
use alloc::rc::Rc;
use core::cell::RefCell;
use crate::fs::ramfs::RAM_FILE_SYSTEMS;
use crate::vfs::inode::Rdev;

static mut FILE_SYSTEMS: Vec<FileSystem> = Vec::new();

pub fn register_filesystem(new_filesystem: FileSystem) -> bool {
    unsafe {
        for exist_filesystem in FILE_SYSTEMS.iter() {
            if exist_filesystem.name == new_filesystem.name { // filesystem name already exists!
                return false;
            }
        }

        FILE_SYSTEMS.push(new_filesystem);
    }
    true
}

pub fn read_super_block(fs_name: &str, rdev: Rdev) -> Option<Rc<RefCell<SuperBlock>>> {
    unsafe {
        for exist_filesystem in FILE_SYSTEMS.iter_mut() {
            if exist_filesystem.name.as_str() == fs_name {
                return exist_filesystem.read_super_block(rdev);
            }
        }
    }

    None
}

pub struct FileSystem {
    name: String,
    read_sb: fn(Rdev) -> Option<Rc<RefCell<SuperBlock>>>,
    super_blocks: Vec<Rc<RefCell<SuperBlock>>>,
}

impl FileSystem {
    pub fn new(name: &str, get_sb: fn(Rdev) -> Option<Rc<RefCell<SuperBlock>>>) -> Self {
        Self {
            name: String::from(name),
            read_sb: get_sb,
            super_blocks: Vec::new(),
        }
    }

    pub fn read_super_block(&mut self, rdev: Rdev) -> Option<Rc<RefCell<SuperBlock>>> {
        for super_block in self.super_blocks.iter() {
            if super_block.borrow().rdev == rdev {
                return Some(super_block.clone());
            }
        }

        let result = (self.read_sb)(rdev);
        if result.is_some() {
            let new_super_block = result.unwrap();
            self.super_blocks.push(new_super_block.clone());
            Some(new_super_block)
        } else {
            None
        }
    }
}