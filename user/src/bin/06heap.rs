#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use alloc::vec::Vec;
use alloc::boxed::Box;

const FRAME_SIZE: usize = 0x1000;

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
    println!("allocating success.");
}

fn allocating_large_memory() {
    println!("test allocating large memory");
    let boxes: [Box<[u8; FRAME_SIZE]>; 4] =  [
        Box::new([0; FRAME_SIZE]),
        Box::new([0; FRAME_SIZE]),
        Box::new([0; FRAME_SIZE]),
        Box::new([0; FRAME_SIZE]),
    ];
    for i in 0..boxes.len() {
        let v = boxes[i].as_ptr() as usize;
        println!("Chunk start size:{:#x}", v);
    }

    println!("allocating success.");
}