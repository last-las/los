use crate::vfs::super_block::SuperBlock;
use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;
use alloc::rc::Rc;
use core::cell::RefCell;

lazy_static! {
    static ref FILE_SYSTEMS: Mutex<Vec<FileSystem>> = Mutex::new(Vec::new());
}

pub fn register_filesystem(filesystem: FileSystem) -> bool {
    let mut filesystems_inner = FILE_SYSTEMS.lock();
    for exist_filesystem in filesystems_inner.iter() {
        if exist_filesystem.name == filesystem.name {
            return false;
        }
    }

    filesystems_inner.push(filesystem);
    true
}

pub fn alloc_super_block(fs_name: &str) -> Option<Rc<RefCell<SuperBlock>>> {
    unimplemented!()
}

pub struct FileSystem {
    name: String,
    get_sb: fn() -> Rc<RefCell<SuperBlock>>,
}