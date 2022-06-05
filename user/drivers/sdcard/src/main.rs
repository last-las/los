#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use sdcard::SDCardWrapper;

use crate::sdcard::BLOCK_DEVICE;
use share::ipc::{Msg, READ, WRITE, DEVICE, REPLY_PROC_NR, REPLY_STATUS, REPLY, PROC_NR, BUFFER, LENGTH, POSITION};
use user_lib::syscall::{receive, send, virt_copy, getpid, sdcard_read, sdcard_write};

#[macro_use]
extern crate user_lib;

mod sdcard;

const BLOCK_SZ: usize = 512;

#[no_mangle]
fn main() {
     let mut message = Msg::empty();

     loop {
         receive(-1, &mut message).unwrap();

         let nr = message.args[DEVICE];
         assert_eq!(nr, 0);

         match message.mtype {
             READ => do_read(message),
             WRITE => do_write(message),
             _ => {
                 panic!("Unknown message type:{}", message.mtype);
             }
         }
     }
}

pub fn do_read(message: Msg) {
    let proc_nr = message.args[PROC_NR];
    let dst_ptr = message.args[BUFFER];
    let block_id = message.args[POSITION];
    let mut buffer = [0u8; BLOCK_SZ];

    sdcard_read(block_id, buffer.as_mut_slice()).unwrap();
    virt_copy(getpid(), buffer.as_ptr() as usize, proc_nr, dst_ptr, BLOCK_SZ).unwrap();
    reply(message.src_pid, REPLY, proc_nr, BLOCK_SZ as isize);
}

pub fn do_write(message: Msg) {
    let proc_nr = message.args[PROC_NR];
    let src_ptr = message.args[BUFFER];
    let block_id = message.args[POSITION];
    let mut buffer = [0; BLOCK_SZ];

    virt_copy(proc_nr, src_ptr, getpid(), buffer.as_mut_ptr() as usize, BLOCK_SZ).unwrap();
    sdcard_write(block_id, buffer.as_slice()).unwrap();
    reply(message.src_pid, REPLY, proc_nr, BLOCK_SZ as isize);
}

fn reply(caller: usize, mtype: usize, proc_nr: usize, status: isize) {
    let mut message = Msg::empty();
    message.mtype = mtype;
    message.args[REPLY_PROC_NR] = proc_nr;
    message.args[REPLY_STATUS] = status as usize;

    send(caller, &message).unwrap();
}