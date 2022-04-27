mod vfs_interface;
use easy_fs::{EasyFileSystem, Inode, BlockDevice, BLOCK_SZ};
use lazy_static::*;
use alloc::sync::Arc;
use share::ipc::{Msg, PROC_NR, BUFFER, POSITION, LENGTH, DEVICE, VIRTIO_BLK_PID, WRITE, READ, REPLY_STATUS};
use user_lib::syscall::{getpid, send, receive};
use crate::vfs::filesystem::{FileSystem, register_filesystem};

pub const EZFS_MAJOR_DEV: u32 = 1;

pub fn register_ezfs() {
    let mut filesystem = FileSystem::new("ezfs", vfs_interface::read_ezfs_super_block);
    assert!(register_filesystem(filesystem, EZFS_MAJOR_DEV));
}

lazy_static! {
    pub static ref ROOT_INODE: Arc<Inode> = {
        let efs = EasyFileSystem::open(Arc::new(VirtioBlock));
        Arc::new(EasyFileSystem::root_inode(&efs))
    };
}

pub struct VirtioBlock;

impl BlockDevice for VirtioBlock {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        // Virtio-blk driver only supports 512 bytes at a time.
        assert_eq!(buf.len(), BLOCK_SZ);
        let mut message = Msg::empty();
        message.mtype = READ;
        message.args[DEVICE] = 0;
        message.args[PROC_NR] = getpid();
        message.args[BUFFER] = buf.as_ptr() as usize;
        message.args[LENGTH] = buf.len();
        message.args[POSITION] = block_id;
        send(VIRTIO_BLK_PID, &message).unwrap();
        receive(VIRTIO_BLK_PID as isize, &mut message).unwrap();
        assert_eq!(message.args[REPLY_STATUS], BLOCK_SZ);
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) {
        // Virtio-blk driver only supports 512 bytes at a time.
        assert_eq!(buf.len(), BLOCK_SZ);
        let mut message = Msg::empty();
        message.mtype = WRITE;
        message.args[PROC_NR] = getpid();
        message.args[BUFFER] = buf.as_ptr() as usize;
        message.args[LENGTH] = buf.len();
        message.args[POSITION] = block_id;
        send(VIRTIO_BLK_PID, &message).unwrap();
        receive(VIRTIO_BLK_PID as isize, &mut message).unwrap();
        assert_eq!(message.args[REPLY_STATUS], BLOCK_SZ);
    }
}