#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod virtio_driver;

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate bitflags;
extern crate log;
extern crate volatile;

use user_lib::syscall::{continuous_alloc, virt_to_phys};
use crate::virtio_driver::{VirtIOBlk, VirtIOHeader};

/*
    Module virtio_driver is an userspace version of https://github.com/rcore-os/virtio-drivers,
    so it's also non-blocking.

    How does it work in userspace:
        [`VirtIOHeader`] uses virtio_driver::volatile instead of the external crate volatile, while
        others in virtio_driver still use the external one.

        [`virtio_driver::volatile::Volatile`] uses `dev_write` and `dev_read` to write and read a register.

        The kernel provides `continuous_alloc` and `virt_to_phys` for [`virtio_driver::hal::DMA`]
        to alloc continuous physical memory and convert virtual address to physical address.
*/

const VIRTIO0: usize = 0x10001000;

#[no_mangle]
fn main() {
    let mut virtio_blk = unsafe {
        VirtIOBlk::new(&mut *(VIRTIO0 as *mut VirtIOHeader)).unwrap()
    };

    let mut buffer = [0; 512];
    for i in 0..1024 {
        virtio_blk.read_block(i, &mut buffer).unwrap();
        buffer.fill((i % 0xff) as u8);
        virtio_blk.write_block(i, &buffer).unwrap();
    }

    let mut compared_buffer = [0; 512];
    for i in 0..1024 {
        virtio_blk.read_block(i, &mut buffer).unwrap();
        compared_buffer.fill((i % 0xff) as u8);
        assert_eq!(buffer, compared_buffer);
    }

    println!("test virtio blk device success!");
}
