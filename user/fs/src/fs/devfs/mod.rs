use crate::vfs::filesystem::{FileSystem, register_filesystem};

mod vfs_interface;

pub fn register_devfs() {
    let filesystem = FileSystem::new("devfs", vfs_interface::create_devfs_super_block);
    assert!(register_filesystem(filesystem));
}