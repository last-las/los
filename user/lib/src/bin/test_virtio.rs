#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::{_read, fork, _write, getpid, send, receive};
use user_lib::termios::{tc_get_attr, tc_set_attr};
use share::terminal::Clflag;
use share::ipc::{Msg, READ, PROC_NR, BUFFER, POSITION, WRITE, REPLY_STATUS};

const BLK_SZ: usize = 512;
const BLK_COUNT: usize = 1024;

#[no_mangle]
fn main() {
    let mut buffer: [u8; BLK_SZ] = [0; BLK_SZ];
    let buffer_ptr = buffer.as_ptr() as usize;

    for i in 0..BLK_COUNT {
        buffer.fill((i % 0xff) as u8);
        write_block(i, buffer_ptr);
    }

    let mut compared_block: [u8; BLK_SZ] = [0; BLK_SZ];
    for i in 0..BLK_COUNT {
        read_block(i, buffer_ptr);
        compared_block.fill((i % 0xff) as u8);
        assert_eq!(buffer, compared_block);
    }
}

fn read_block(block_id: usize, ptr: usize) {
    let mut message = Msg::empty();

    message.mtype = READ;
    message.args[PROC_NR] = getpid();
    message.args[BUFFER] = ptr;
    message.args[POSITION] = block_id;

    send(4, &message).unwrap();
    receive(4, &mut message).unwrap();
    assert_eq!(message.args[REPLY_STATUS] as isize, BLK_SZ as isize);
}

fn write_block(block_id: usize, ptr: usize) {
    let mut message = Msg::empty();

    message.mtype = WRITE;
    message.args[PROC_NR] = getpid();
    message.args[BUFFER] = ptr;
    message.args[POSITION] = block_id;

    send(4, &message).unwrap();
    receive(4, &mut message).unwrap();
    assert_eq!(message.args[REPLY_STATUS] as isize, BLK_SZ as isize);
}
