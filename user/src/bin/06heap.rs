#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use alloc::vec::Vec;
use alloc::boxed::Box;

const SIZE: usize = 0x1000 * 4;

#[no_mangle]
fn main() {
    allocating_vector();
    allocating_large_memory();
}

fn allocating_vector() {
    println!("test allocating vector");
    let mut v = Vec::new();
    for i in 0..100 {
        v.push(i);
    }

    for i in (0..100).rev() {
        assert_eq!(i, v.pop().unwrap());
    }
}

fn allocating_large_memory() {
    let b: Box<[u8; SIZE]> = Box::new([0; SIZE]);
    let v = b.as_ptr() as usize;
    println!("Allocating success. Heap start size:{:#x}", v);
}