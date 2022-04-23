mod vfs_interface;

use spin::Mutex;
use alloc::collections::{BinaryHeap, VecDeque};
use alloc::rc::Rc;
use core::cell::RefCell;
use alloc::vec::Vec;
use alloc::string::String;
use crate::vfs::filesystem::{register_filesystem, FileSystem};
use crate::vfs::inode::Inode;
use crate::vfs::super_block::SuperBlock;
use crate::fs::ramfs::vfs_interface::{RamFsInodeOperations, RamFsFileOperations};
use alloc::boxed::Box;

pub fn register_ramfs() {
    let mut filesystem = FileSystem::new("ramfs", vfs_interface::alloc_ramfs_super_block);
    register_filesystem(filesystem);
}

pub static mut RAM_FILE_SYSTEMS: Vec<RamFileSystem> = Vec::new();

pub struct RamFileSystem {
    ino_allocator: InoAllocator,
    inode_queue: VecDeque<Rc<RefCell<RamFsInode>>>,
}

impl RamFileSystem {
    pub fn new() -> Self {
        // create root node, which is a directory and it's name is "/".
        let mut ino_allocator = InoAllocator::new();
        let ino = ino_allocator.alloc();
        assert_eq!(ino, 0);
        let mut root_ramfs_inode = RamFsInode::empty(ino);
        root_ramfs_inode.borrow_mut().mark_as_dir();
        root_ramfs_inode.borrow_mut().set_name("/");

        let mut inode_queue = VecDeque::new();
        inode_queue.push_back(root_ramfs_inode);

        Self {
            ino_allocator,
            inode_queue,
        }
    }

    pub fn alloc_ramfs_inode(&mut self,) -> Rc<RefCell<RamFsInode>> {
        let ino = self.ino_allocator.alloc();
        let new_ramfs_inode =  RamFsInode::empty(ino);
        self.inode_queue.push_back(Rc::clone(&new_ramfs_inode));
        new_ramfs_inode
    }

    pub fn search_ramfs_inode(&self, ino: usize) -> Option<Rc<RefCell<RamFsInode>>> {
        for ramfs_inode in self.inode_queue.iter() {
            if ramfs_inode.borrow().ino == ino {
                return Some(Rc::clone(ramfs_inode));
            }
        }

        None
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
    pub fn empty(ino: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Self {
                ino,
                name: String::new(),
                file_type: FileType::UNKNOWN,
                sub_nodes: Vec::new(),
                content: Vec::new(),
            }
        ))
    }

    pub fn mark_as_file(&mut self) {
        self.file_type = FileType::NORMAL;
    }

    pub fn mark_as_dir(&mut self) {
        self.file_type = FileType::DIRECTORY;
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn lookup(&self, name: &str) -> Option<Rc<RefCell<RamFsInode>>> {
        assert_eq!(self.file_type, FileType::DIRECTORY);
        for sub_node in self.sub_nodes.iter() {
            if sub_node.borrow().name.as_str() == name {
                return Some(Rc::clone(sub_node));
            }
        }

        None
    }

    pub fn read(&self, pos: usize, size: usize) -> Vec<u8> {
        let length = self.content.len();
        if pos >= length {
            return Vec::new();
        }
        let slice: &[u8] = &self.content.as_slice()[pos..usize::min(length, size + pos)];
        Vec::from(slice)
    }

    pub fn write(&mut self, pos: usize, content: Vec<u8>) {
        let length = self.content.len();
        if pos > length - 1 {
            for _ in 0..pos - length {
                self.content.push(0);
            }
        } else if pos < length - 1 {
            self.content.drain(pos..);
        }

        self.content.extend(content);
    }

    pub fn read_dir(&self) -> Vec<Rc<RefCell<RamFsInode>>> {
        assert_eq!(self.file_type, FileType::DIRECTORY);
        self.sub_nodes.clone()
    }

    pub fn get_vfs_inode(&self, super_block: Rc<RefCell<SuperBlock>>) -> Rc<RefCell<Inode>> {
        Inode::new(
            self.ino,
            super_block,
            Rc::new(RamFsInodeOperations),
            Rc::new(RamFsFileOperations),
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum FileType {
    NORMAL,
    DIRECTORY,
    UNKNOWN,
}
