use share::ipc::{Msg, READ, DEVICE, PROC_NR, BUFFER, LENGTH, POSITION, VIRTIO_BLK_PID, REPLY_STATUS, WRITE};
use user_lib::syscall::{getpid, send, receive};
use crate::vfs::inode::Rdev;
use share::device::BlockDevice;
use alloc::sync::Arc;

pub const BLOCK_SIZE: usize = 512;

pub struct Block {
    rdev: Rdev,
}

impl Block {
    pub fn new(rdev: Rdev) -> Arc<Self> { // Because Block struct should be compatible with easy-fs,
                                            // the return value is "Arc<Self>".
        Arc::new(
            Self {
                rdev,
            }
        )
    }
}

impl BlockDevice for Block {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        // Virtio-blk driver only supports 512 bytes at a time.
        assert_eq!(buf.len(), BLOCK_SIZE);
        let mut message = Msg::empty();
        message.mtype = READ;
        message.args[DEVICE] = 0;
        message.args[PROC_NR] = getpid();
        message.args[BUFFER] = buf.as_ptr() as usize;
        message.args[LENGTH] = buf.len();
        message.args[POSITION] = block_id;
        send(VIRTIO_BLK_PID, &message).unwrap();
        receive(VIRTIO_BLK_PID as isize, &mut message).unwrap();
        assert_eq!(message.args[REPLY_STATUS], BLOCK_SIZE);
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) {
        // Virtio-blk driver only supports 512 bytes at a time.
        assert_eq!(buf.len(), BLOCK_SIZE);
        let mut message = Msg::empty();
        message.mtype = WRITE;
        message.args[PROC_NR] = getpid();
        message.args[BUFFER] = buf.as_ptr() as usize;
        message.args[LENGTH] = buf.len();
        message.args[POSITION] = block_id;
        send(VIRTIO_BLK_PID, &message).unwrap();
        receive(VIRTIO_BLK_PID as isize, &mut message).unwrap();
        assert_eq!(message.args[REPLY_STATUS], BLOCK_SIZE);
    }
}