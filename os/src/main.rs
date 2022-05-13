#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(allocator_api)]
#![feature(step_trait)]
#![feature(slice_ptr_get)]
#![feature(result_option_inspect)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;
extern crate buddy_system_allocator;
extern crate alloc;
extern crate spin;
extern crate riscv;

use core::arch::asm;
use core::arch::global_asm;

use processor::enable_other_harts;
use processor::set_hart_id;

use crate::processor::CPU_NUMS;
use crate::task::increase_alive_hart;
use crate::mm::heap::heap_allocator;

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


#[cfg(not(test))]
global_asm!(include_str!("entry.asm"));

#[no_mangle]
#[link_section = ".text.rust_main"]
pub extern "C" fn kmain(hart_id: usize, _: usize) -> ! {
    unsafe {
        asm! { "sfence.vma"} // must do this again.
    }
    set_hart_id(hart_id);

    if hart_id == 0 {
        environment_check();
        mm::address::mark_as_paging();
        heap_allocator::init_heap();
        trap::init_stvec();
        timer::enable_time_interrupt();
        increase_alive_hart();
        task::print_app_names();
        task::load_init_task();
        // enable_other_harts();
        info!("start running");
        processor::run_on_current_hart();
    } else {
        increase_alive_hart();
        other_hart_init_task();
        info!("start running");
        processor::run_on_current_hart();
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

