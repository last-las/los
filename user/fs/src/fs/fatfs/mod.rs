mod vfs_interface;
use simple_fat32::{FAT32Manager,VFile};
use alloc::sync::Arc;
use crate::vfs::filesystem::{FileSystem, register_filesystem};
//use spin::Mutex;
use spin::RwLock;
use alloc::collections::BTreeMap;

// Global easy file system instances
pub static mut FAT32SYSTEMS: BTreeMap<u64, Arc<RwLock<FAT32Manager>>> = BTreeMap::new();


//FileSystem is "fat32"
pub fn register_fatfs() {
    let filesystem = FileSystem::new("fat32", vfs_interface::create_fatfs_super_block);
    assert!(register_filesystem(filesystem));
}

//增加fatfs实例
pub fn add_fatfs_instance(rdev: u64,fat_fs_instance: Arc<RwLock<FAT32Manager>>) {
    unsafe {
        assert!(get_fatfs_instance_by(rdev).is_none());
        FAT32SYSTEMS.insert(rdev, fat_fs_instance);
    }
}

//找到指定设备号的根Vfile
pub fn get_fatfs_root_inode(rdev: u64) -> Option<Arc<VFile>> {
    let result = get_fatfs_instance_by(rdev);
    if result.is_none() {
        return None;
    }

    let instance = result.unwrap();
    let reader=instance.read();
    let root=reader.get_root_vfile(instance);
    return Some(Arc::new(root));
}

//通过设备号寻找具体文件系统实例
pub fn get_fatfs_instance_by(rdev: u64) -> Option<&'static Arc<RwLock<FAT32Manager>>> {
    unsafe {
        FAT32SYSTEMS.get(&rdev)
    }
}