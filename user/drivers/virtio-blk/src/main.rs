#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod virtio_driver;

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate bitflags;
extern crate volatile;
extern crate log;

use user_lib::syscall::{continuous_alloc, virt_to_phys};

#[no_mangle]
fn main() {
    let start_va = continuous_alloc(0x4000).unwrap();
    let start_pa = virt_to_phys(start_va).unwrap();
    println!("alloc start_va: {:#x}", start_va);
    println!("alloc start_pa: {:#x}", start_pa);
}
