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

#[no_mangle]
fn main() {
    println!("Hello, world!");
}
