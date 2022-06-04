#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use sdcard::SDCardWrapper;

use crate::sdcard::BLOCK_DEVICE;
use share::ipc::{Msg, DEVICE, REPLY_PROC_NR, REPLY_STATUS, REPLY, PROC_NR, BUFFER, LENGTH, POSITION};
use user_lib::syscall::{receive, send, virt_copy, getpid};

#[macro_use]
extern crate user_lib;

mod sdcard;

#[no_mangle]
fn main() {
    //let sdcard = SDCardWrapper::new();

    println!("block test");
    write_test();

    read_test();
    //block_device_test();
    // let mut message = Msg::empty();
    //
    // loop {
    //     receive(-1, &mut message).unwrap();
    //
    //     let nr = message.args[DEVICE];
    //     assert_eq!(nr, 0);
    //
    //     match message.mtype {
    //         READ => do_read(message),
    //         WRITE => do_write(message),
    //         _ => {
    //             panic!("Unknown message type:{}", message.mtype);
    //         }
    //     }
    // }
}

pub fn read_test() {
    let block_device = BLOCK_DEVICE.clone();
    let mut buf = [0u8; 512];
     block_device.read_block(0, &mut buf);

    println!("read block success {:?}", buf);
}

pub fn write_test() {
    let block_device = BLOCK_DEVICE.clone();
    let mut buf = [0u8; 512];
    for val in buf.iter_mut() {
        *val = 2;
    }
    block_device.write_block(0, &buf);

    println!("write block success");
}

pub fn do_read(message: Msg) {
    let block_device = BLOCK_DEVICE.clone();
    let proc_nr = message.args[PROC_NR];
    let dst_ptr = message.args[BUFFER];
    let block_id = message.args[POSITION];

    let mut read_buffer = [0u8; 512];
    block_device.read_block(block_id, &mut read_buffer);
    // transfer to user
    transfer_to_usr(&read_buffer, &message);
}

pub fn do_write(message: Msg) {
    let block_device = BLOCK_DEVICE.clone();
    const BUFFER_SIZE: usize = 512;

    let proc_nr = message.args[PROC_NR];
    let src_ptr = message.args[BUFFER];
    let block_id = message.args[POSITION];
    let mut buffer = [0; BUFFER_SIZE];


    virt_copy(proc_nr, src_ptr, getpid(), buffer.as_mut_ptr() as usize, BUFFER_SIZE).unwrap();

    block_device.write_block(block_id as usize, &buffer);


    reply(message.src_pid, REPLY, proc_nr, BUFFER_SIZE as isize);
}

fn transfer_to_usr(buf: &[u8], msg: &Msg) {
    let buffer_ptr = buf.as_ptr() as usize;
    let length = buf.len();

    virt_copy(getpid(), buffer_ptr, msg.args[PROC_NR], msg.args[BUFFER], length).unwrap();

    reply(msg.src_pid, REPLY, msg.args[PROC_NR], length as isize);
}

fn reply(caller: usize, mtype: usize, proc_nr: usize, status: isize) {
    let mut message = Msg::empty();
    message.mtype = mtype;
    message.args[REPLY_PROC_NR] = proc_nr;
    message.args[REPLY_STATUS] = status as usize;

    send(caller, &message).unwrap();
}

#[allow(unused)]
pub fn block_device_test() {
    let block_device = BLOCK_DEVICE.clone();
    let mut write_buffer = [0u8; 512];
    let mut read_buffer = [0u8; 512];
    for i in 0..512 {
        for byte in write_buffer.iter_mut() {
            *byte = i as u8;
        }
        block_device.write_block(i as usize, &write_buffer);
        block_device.read_block(i as usize, &mut read_buffer);
        assert_eq!(write_buffer, read_buffer);
    }
    println!("block device test passed!");
}
