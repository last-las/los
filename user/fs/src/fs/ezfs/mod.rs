mod vfs_interface;
use easy_fs::{EasyFileSystem, Inode};
use alloc::sync::Arc;
use crate::vfs::filesystem::{FileSystem, register_filesystem};
use spin::Mutex;
use alloc::collections::BTreeMap;

/*
    easy-fs `BlockDevice` trait is moved to share::device
*/
pub fn register_ezfs() {
    let filesystem = FileSystem::new("ezfs", vfs_interface::create_ezfs_super_block);
    assert!(register_filesystem(filesystem));
}

pub fn add_ez_fs_instance(rdev: u64,ez_fs_instance: Arc<Mutex<EasyFileSystem>>) {
    unsafe {
        assert!(get_ez_fs_instance_by(rdev).is_none());
        EASY_FILE_SYSTEMS.insert(rdev, ez_fs_instance);
    }
}

pub fn get_ez_fs_root_inode(rdev: u64) -> Option<Arc<Inode>> {
    let result = get_ez_fs_instance_by(rdev);
    if result.is_none() {
        return None;
    }

    let instance = result.unwrap();
    Some(EasyFileSystem::root_inode(instance))
}

pub fn get_ez_fs_instance_by(rdev: u64) -> Option<&'static Arc<Mutex<EasyFileSystem>>> {
    unsafe {
        EASY_FILE_SYSTEMS.get(&rdev)
    }
}

// Global easy file system instances
pub static mut EASY_FILE_SYSTEMS: BTreeMap<u64, Arc<Mutex<EasyFileSystem>>> = BTreeMap::new();