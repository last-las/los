#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use crate::syscall::exit;
use crate::heap::init_heap;
use core::ptr;

extern crate alloc;

#[macro_use]
pub mod console;
pub mod syscall;
mod panic;
mod heap;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() {
    clear_bss();
    init_heap();
    main();
    exit(0);
    panic!("unreachable in _start.");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        let count = ebss as usize - sbss as usize;
        ptr::write_bytes( sbss as usize as *mut u8, 0, count);
    }
}

#[no_mangle]
#[linkage = "weak"]
fn main() {
    panic!("main function not found.");
}