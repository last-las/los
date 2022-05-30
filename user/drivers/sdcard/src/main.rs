#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use sdcard::SDCardWrapper;

use crate::sdcard::BLOCK_DEVICE;
#[macro_use]
extern crate user_lib;

mod sdcard;

fn main() {
    let sdcard = SDCardWrapper::new();
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
