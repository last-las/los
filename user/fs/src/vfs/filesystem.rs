use crate::vfs::super_block::SuperBlock;
use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;
use alloc::rc::Rc;
use core::cell::RefCell;
use crate::fs::ramfs::RAM_FILE_SYSTEMS;

lazy_static! {
    static ref FILE_SYSTEMS: Mutex<Vec<Option<FileSystem>>> = Mutex::new(Vec::new());
}

pub fn register_filesystem(filesystem: FileSystem, major_dev: u32) -> bool {
    let mut filesystems_inner = FILE_SYSTEMS.lock();
    for exist_filesystem in filesystems_inner.iter() {
        if exist_filesystem.is_some() && exist_filesystem.as_ref().unwrap().name == filesystem.name {
            // filesystem name already exists!
            return false;
        }
    }

    let major_dev = major_dev as usize;
    while major_dev + 1 > filesystems_inner.len() {
        filesystems_inner.push(None);
    }

    if filesystems_inner[major_dev].is_some() { // major_dev already in used!
        return false;
    }

    filesystems_inner[major_dev] = Some(filesystem);

    true
}

pub fn read_super_block(fs_name: &str, minor_dev: u32) -> Option<Rc<RefCell<SuperBlock>>> {
    let mut filesystems_inner = FILE_SYSTEMS.lock();
    for exist_filesystem_wrapper in filesystems_inner.iter() {
        if exist_filesystem_wrapper.is_some() {
            let exist_filesystem = exist_filesystem_wrapper.as_ref().unwrap();
            if exist_filesystem.name.as_str() == fs_name {
                return  Some((exist_filesystem.read_sb)(minor_dev));
            }
        }
    }

    None
}

pub struct FileSystem {
    name: String,
    read_sb: fn(u32) -> Rc<RefCell<SuperBlock>>,
}

impl FileSystem {
    pub fn new(name: &str, get_sb: fn(u32) -> Rc<RefCell<SuperBlock>>)-> Self {
        Self {
            name: String::from(name),
            read_sb: get_sb,
        }
    }
}