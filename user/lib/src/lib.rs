#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use crate::heap::init_heap;
use crate::syscall::exit;
use core::arch::global_asm;
use core::ptr;

extern crate alloc;
#[macro_use]
pub mod console;
pub mod env;
mod heap;
pub mod io;
mod panic;
pub mod syscall;
pub mod termios;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_start(argv: *const *const u8, envp: *const *const u8) {
    clear_bss();
    init_heap();
    env::parse_argv(argv);
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
        ptr::write_bytes(sbss as usize as *mut u8, 0, count);
    }
}

#[no_mangle]
#[linkage = "weak"]
fn main() {
    panic!("main function not found.");
}
