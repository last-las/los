#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use crate::syscall::exit;
use crate::heap::init_heap;
use core::arch::global_asm;
use core::ptr;
use core::ffi::c_void;
use share::ffi::c_char;

extern crate alloc;

#[macro_use]
pub mod console;
pub mod syscall;
pub mod env;
mod panic;
mod heap;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_start(argv: *const *const c_char, envp: *const *const c_char) {
    clear_bss();
    init_heap();
    env::parse_envp(envp);

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