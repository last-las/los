mod vfs_interface;

use alloc::collections::{VecDeque, BTreeMap};
use alloc::rc::Rc;
use core::cell::RefCell;
use alloc::vec::Vec;
use alloc::string::String;
use crate::vfs::filesystem::{register_filesystem, FileSystem};
use crate::vfs::inode::{VfsInode, Rdev};
use crate::vfs::super_block::SuperBlock;
use crate::fs::ramfs::vfs_interface::{RamFsInodeOperations, RamFsFileOperations};
use share::file::FileTypeFlag;


pub fn register_ramfs() {
    let filesystem = FileSystem::new("ramfs", vfs_interface::create_ramfs_super_block);
    assert!(register_filesystem(filesystem));
}

fn add_ram_fs_instance(rdev: u64, ram_fs_instance: RamFileSystem) {
    unsafe {
        assert!(RAM_FILE_SYSTEMS.get(&rdev).is_none());
        RAM_FILE_SYSTEMS.insert(rdev, ram_fs_instance);
    }
}

fn get_ram_fs_instance(rdev: u64) -> Option<&'static mut RamFileSystem> {
    unsafe {
        RAM_FILE_SYSTEMS.get_mut(&rdev)
    }
}


pub static mut RAM_FILE_SYSTEMS: BTreeMap<u64, RamFileSystem> = BTreeMap::new();

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
        let root_ramfs_inode = RamFsInode::empty(ino);
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
    pub const fn new() -> Self {
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

    #[allow(unused)]
    pub fn dealloc(&mut self, ino: usize) {
        self.recycled.push(ino);
    }
}

pub struct RamFsInode {
    ino: usize,
    name: String,
    file_type: FileTypeFlag,
    sub_nodes: Vec<Rc<RefCell<RamFsInode>>>,
    content: Vec<u8>,
}

impl RamFsInode {
    pub fn empty(ino: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Self {
                ino,
                name: String::new(),
                file_type: FileTypeFlag::DT_UNKNOWN,
                sub_nodes: Vec::new(),
                content: Vec::new(),
            }
        ))
    }

    pub fn mark_as_file(&mut self) {
        self.file_type = FileTypeFlag::DT_REG;
    }

    pub fn mark_as_dir(&mut self) {
        self.file_type = FileTypeFlag::DT_DIR;
    }

    pub fn set_file_type(&mut self, file_type: FileTypeFlag) {
        self.file_type = file_type;
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn lookup(&self, name: &str) -> Option<Rc<RefCell<RamFsInode>>> {
        assert_eq!(self.file_type, FileTypeFlag::DT_DIR);
        for sub_node in self.sub_nodes.iter() {
            if sub_node.borrow().name.as_str() == name {
                return Some(Rc::clone(sub_node));
            }
        }

        None
    }

    pub fn set_rdev(&mut self, rdev: Rdev) {
        assert!(self.file_type.is_device());
        self.content.clear();

        let rdev: u64 =  rdev.into();

        for i in 0..core::mem::size_of::<usize>() {
            let byte = ((rdev >> (8 * i)) & 0xFF) as u8;
            self.content.push(byte);
        }
    }

    pub fn read_rdev(&self) -> Rdev {
        assert!(self.file_type.is_device());

        let mut rdev = 0;
        for i in 0..core::mem::size_of::<usize>() {
            rdev |= (self.content[i] as u64) << (8 * i);
        }
        rdev.into()
    }

    pub fn read(&self, pos: usize, size: usize) -> Vec<u8> {
        let length = self.content.len();
        if pos >= length {
            return Vec::new();
        }
        let slice: &[u8] = &self.content.as_slice()[pos..usize::min(length, size + pos)];
        Vec::from(slice)
    }

    pub fn write(&mut self, pos: usize, content: &[u8]) {
        let length = self.content.len();
        for _ in length..pos + content.len() {
            self.content.push(0);
        }

        for i in pos..pos + content.len() {
            self.content[i] = content[i - pos];
        }
    }

    pub fn read_dir(&self) -> Vec<Rc<RefCell<RamFsInode>>> {
        assert_eq!(self.file_type, FileTypeFlag::DT_DIR);
        self.sub_nodes.clone()
    }

    pub fn get_vfs_inode(&self, super_block: Rc<RefCell<SuperBlock>>) -> Rc<RefCell<VfsInode>> {
        let mut rdev = None;
        if self.file_type.is_device() {
            rdev = Some(self.read_rdev());
        }
        VfsInode::new(
            self.ino,
            self.content.len(),
            rdev,
            self.file_type,
            super_block,
            Rc::new(RamFsInodeOperations),
            Rc::new(RamFsFileOperations),
        )
    }
}
