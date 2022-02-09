#![no_main]
#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate lazy_static;
extern crate buddy_system_allocator;
extern crate alloc;
extern crate spin;
extern crate riscv;

use core::ptr;
use processor::set_hart_id;
use processor::enable_other_harts;
use crate::task::increase_alive_hart;
use crate::processor::{CPU_NUMS,suspend_current_hart};

#[macro_use]
mod console;
#[macro_use]
mod log;
mod sbi;
mod panic;
mod task;
mod heap_allocator;
mod trap;
mod syscall;
mod processor;
mod loader;
mod timer;
mod config;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main(hart_id: usize, _: usize) -> ! {
    set_hart_id(hart_id);
    if hart_id == 0 {
        environment_check();
        do_init_jobs();
        increase_alive_hart();

        if is_test_mode() {
            run_tests();
            panic!("running tests successfully.");
        }

        task::load_tasks();
        enable_other_harts();
        task::load_and_run_a_task();
    } else {
        suspend_current_hart();
        increase_alive_hart();
        other_hart_init_task();
        task::load_and_run_a_task();
    }

    unreachable!("couldn't reach here in rust_main");
}

fn environment_check() {
    // Should make the constant CPU_NUMS >= the environment variable when panic.
    assert!(CPU_NUMS >= env!("CPU_NUMS").parse::<usize>().unwrap());
}

fn do_init_jobs() {
    clear_bss();
    heap_allocator::init_heap();
    trap::init_stvec();
    timer::enable_time_interrupt();
}

fn is_test_mode() -> bool {
    env!("TEST_MODE") == "1"
}

fn other_hart_init_task() {
    trap::init_stvec();
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

pub fn run_tests() {
    info!("starting running test cases.\n");
    task::test_task_mod();
}


