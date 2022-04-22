mod vfs_interface;

use spin::Mutex;
use alloc::collections::{BinaryHeap, VecDeque};
use alloc::rc::Rc;
use core::cell::RefCell;
use alloc::vec::Vec;
use alloc::string::String;
use crate::vfs::filesystem::{register_filesystem, FileSystem};

pub fn register_ramfs() {
    let mut filesystem = FileSystem::new("ramfs", vfs_interface::alloc_ramfs_super_block);
    register_filesystem(filesystem);
}

pub static mut RAM_FILE_SYSTEMS: Vec<RamFileSystem> = Vec::new();

pub struct RamFileSystem {
    ino_allocator: InoAllocator,
    inode_searcher: VecDeque<Rc<RefCell<RamFsInode>>>,
}

impl RamFileSystem {
    pub fn new() -> Self {
        Self {
            ino_allocator: InoAllocator::new(),
            inode_searcher: VecDeque::new(),
        }
    }

    pub fn alloc_ramfs_inode(&mut self) -> Rc<RefCell<RamFsInode>> {
        unimplemented!()
    }

    pub fn search_ramfs_inode(&self, ino: usize) -> Option<Rc<RefCell<RamFsInode>>> {
        unimplemented!()
    }
}

/// A simple Inode number allocator, each RamFileSystem will keep one.
pub struct InoAllocator {
    cur: usize,
    recycled: Vec<usize>,
}

impl InoAllocator {
    pub fn new() -> Self {
        Self {
            cur: 0,
            recycled: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> usize {
        if self.recycled.is_empty() {
            self.cur += 1;
            self.cur - 1
        } else {
            self.recycled.pop().unwrap()
        }
    }

    pub fn dealloc(&mut self, ino: usize) {
        self.recycled.push(ino);
    }
}

pub struct RamFsInode {
    ino: usize,
    name: String,
    file_type: FileType,
    sub_nodes: Vec<Rc<RefCell<RamFsInode>>>,
    content: Vec<u8>,
}

impl RamFsInode {
    pub fn new_dir(ino: usize, name: &str) -> Self {
        Self {
            ino,
            name: String::from(name),
            file_type: FileType::DIRECTORY,
            sub_nodes: Vec::new(),
            content: Vec::new(),
        }
    }

    pub fn new_file(ino: usize, name: &str) -> Self {
        Self {
            ino,
            name: String::from(name),
            file_type: FileType::NORMAL,
            sub_nodes: Vec::new(),
            content: Vec::new(),
        }
    }
}

pub enum FileType {
    NORMAL,
    DIRECTORY,
}
