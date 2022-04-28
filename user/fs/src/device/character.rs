use crate::vfs::inode::Rdev;
use share::ipc::{Msg, READ, DEVICE, PROC_NR, BUFFER, LENGTH, POSITION, VIRTIO_BLK_PID, TERMINAL_PID, WRITE, REPLY_STATUS};
use user_lib::syscall::{getpid, send, receive};

pub struct Character {
    rdev: Rdev,
}

impl Character {
    pub fn new(rdev: Rdev) -> Self {
        Self {
            rdev,
        }
    }

    pub fn read(&self, buf: &mut [u8]) {
        let mut message = Msg::empty();
        message.mtype = READ;
        message.args[DEVICE] = 0;
        message.args[PROC_NR] = getpid();
        message.args[BUFFER] = buf.as_ptr() as usize;
        message.args[LENGTH] = buf.len();
        // message.args[POSITION] = ???;
        send(TERMINAL_PID, &message).unwrap();
        receive(TERMINAL_PID as isize, &mut message).unwrap();
    }

    pub fn write(&self, buf: &[u8]) {
        let mut message = Msg::empty();
        message.mtype = WRITE;
        message.args[DEVICE] = 0;
        message.args[PROC_NR] = getpid();
        message.args[BUFFER] = buf.as_ptr() as usize;
        message.args[LENGTH] = buf.len();
        // message.args[POSITION] = ???;
        send(TERMINAL_PID, &message).unwrap();
        receive(TERMINAL_PID as isize, &mut message).unwrap();
    }
}