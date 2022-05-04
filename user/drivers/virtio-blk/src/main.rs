#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod virtio_driver;

extern crate user_lib;
extern crate alloc;
extern crate bitflags;
extern crate log;
extern crate volatile;

use user_lib::syscall::{receive, virt_copy, getpid, send};
use crate::virtio_driver::{VirtIOBlk, VirtIOHeader};
use share::ipc::{Msg, READ, WRITE, POSITION, PROC_NR, BUFFER, REPLY_PROC_NR, REPLY_STATUS, REPLY};
use share::syscall::error::EINVAL;

/*
    Module virtio_driver is an userspace version of https://github.com/rcore-os/virtio-drivers,
    so it's also non-blocking.

    The difference between module virtio_driver and the github repository:
        There are two "volatile" in this module:
            virtio_driver::volatile, and the external crate volatile.

            [`virtio_driver::volatile::Volatile`] uses `dev_write` and `dev_read` to write and read a register.
             virtio_driver::volatile is only used in [`virtio_driver::header::VirtIOHeader`].

        The kernel provides `continuous_alloc` and `virt_to_phys` for [`virtio_driver::hal::DMA`]
        to alloc continuous physical memory and convert virtual address to physical address.
*/

const VIRTIO0: usize = 0x10001000;
const BLOCK_SZ: usize = 512;

#[no_mangle]
fn main() {
    let mut virtio_blk = unsafe {
        VirtIOBlk::new(&mut *(VIRTIO0 as *mut VirtIOHeader)).unwrap()
    };

    let mut message = Msg::empty();

    loop {
        receive(-1, &mut message).unwrap();

        let ret = match message.mtype {
            READ => do_read(&mut virtio_blk, message),
            WRITE => do_write(&mut virtio_blk, message),
            _ => {
                panic!("Unknown message type:{}", message.mtype);
            }
        };

        reply(message.src_pid, REPLY, message.args[PROC_NR], ret);
    }
}

pub fn do_read(virtio_blk: &mut VirtIOBlk, message: Msg) -> isize {
    let proc_nr = message.args[PROC_NR];
    let dst_ptr = message.args[BUFFER];
    let block_id = message.args[POSITION];
    let mut buffer = [0; BLOCK_SZ];
    if virtio_blk.read_block(block_id, &mut buffer).is_err() {
        return -EINVAL as isize;
    }
    virt_copy(getpid(), buffer.as_ptr() as usize, proc_nr, dst_ptr, BLOCK_SZ).unwrap();

    BLOCK_SZ as isize
}

pub fn do_write(virtio_blk: &mut VirtIOBlk, message: Msg) -> isize {
    let proc_nr = message.args[PROC_NR];
    let src_ptr = message.args[BUFFER];
    let block_id = message.args[POSITION];
    let mut buffer = [0; BLOCK_SZ];
    virt_copy(proc_nr, src_ptr, getpid(), buffer.as_mut_ptr() as usize, BLOCK_SZ).unwrap();
    if virtio_blk.write_block(block_id, &buffer).is_err() {
        return -EINVAL as isize;
    }

    BLOCK_SZ as isize
}

fn reply(caller: usize, mtype: usize, proc_nr: usize, status: isize) {
    let mut message = Msg::empty();
    message.mtype = mtype;
    message.args[REPLY_PROC_NR] = proc_nr;
    message.args[REPLY_STATUS] = status as usize;

    send(caller, &message).unwrap();
}
