#![no_main]
#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(allocator_api)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;
extern crate buddy_system_allocator;
extern crate alloc;
extern crate spin;
extern crate riscv;

use core::ptr;

use processor::enable_other_harts;
use processor::set_hart_id;

use crate::processor::{CPU_NUMS, suspend_current_hart};
use crate::task::increase_alive_hart;
use crate::sbi::{sbi_shutdown, sbi_console_putchar};

#[macro_use]
mod console;
#[macro_use]
mod log;
mod sbi;
mod panic;
mod task;
mod trap;
mod syscall;
mod processor;
mod loader;
mod timer;
mod config;
mod mm;
mod paging;


global_asm!(include_str!("entry.asm"));

#[no_mangle]
#[link_section = ".text.rust_main"]
pub extern "C" fn kmain(hart_id: usize, _: usize) -> ! {
    set_hart_id(hart_id);
    // info!("hello world");
    println!("{}", hart_id);
    // sbi_console_putchar(0x61  as char);
    suspend_current_hart();
    panic!("This is fucking rust_main!");
    if hart_id == 0 {
        environment_check();
        // heap_allocator::init_heap();
        trap::init_stvec();
        timer::enable_time_interrupt();
        increase_alive_hart();

        #[cfg(feature = "test")]
        {
            run_tests();
            panic!("Test completed successfully.");
        }

        #[cfg(not(feature = "test"))]
        {
            task::load_tasks();
            enable_other_harts();
            info!("start running");
            processor::run_on_current_hart();
        }
    } else {
        suspend_current_hart();

        #[cfg(not(feature = "test"))]
        {
            increase_alive_hart();
            other_hart_init_task();
            info!("start running");
            processor::run_on_current_hart();
        }
    }

    unreachable!("couldn't reach here in rust_main");
}

fn environment_check() {
    // Should make the constant CPU_NUMS >= the environment variable when panic.
    assert!(CPU_NUMS >= env!("CPU_NUMS").parse::<usize>().unwrap());
}

fn other_hart_init_task() {
    trap::init_stvec();
}

#[cfg(feature = "test")]
pub fn run_tests() {
    info!("starting running test cases.\n");
    task::test_task_mod();
}