#![no_main]
#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

use crate::cpu::{enable_all_cpus, set_hart_id,};
use core::ptr;

#[macro_use]
mod console;
mod sbi;
mod panic;
mod cpu;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main(hart_id: usize, _: usize) -> ! {
    set_hart_id(hart_id);
    if hart_id == 0 {
        clear_bss();
        enable_all_cpus();
    }
    panic!("couldn't reach here in rust_main");
}

fn clear_bss() {
    extern "C" {
        static mut sbss: u8;
        static mut ebss: u8;
    }
    unsafe {
        let count = &ebss as *const u8 as usize - &sbss as *const u8 as usize;
        ptr::write_bytes(&mut sbss as *mut u8, 0, count);
    }
}