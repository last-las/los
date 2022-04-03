mod kernel_stack;
mod task_struct;
mod trap_context;
mod task_manager;
mod pid;
mod task_context;

use crate::processor::{take_task_in_current_hart,get_current_hart_context_ptr};
use crate::loader::{get_app_ref_data, get_app_names};
use spin::Mutex;

pub use kernel_stack::KernelStack;
pub use task_struct::{TaskStruct, RuntimeFlags, ReceiveProc};
pub use task_manager::{fetch_a_task_from_manager, add_a_task_to_manager, get_task_by_pid};
pub use task_context::TaskContext;
pub use trap_context::TrapContext;
use crate::task::task_manager::rm_task_from_manager;
pub use crate::task::task_manager::return_task_to_manager;
use alloc::sync::Arc;
use crate::processor::__switch;
use crate::paging::KERNEL_SATP;
use lazy_static::*;
use alloc::vec::Vec;

lazy_static! {
    pub static ref APP_NAMES: Vec<&'static str> =get_app_names();

    pub static ref APP_DATA: Vec<&'static [u8]> = unsafe {
        get_app_ref_data()
    };
}

pub fn print_app_names() {
    println!("apps num : {}", APP_DATA.len());
    for i in 0..APP_DATA.len() {
        println!("app[{}] name: {}", i, APP_NAMES[i]);
    }
}

pub fn load_init_task() {
    let data = get_task_data_by_name("init").unwrap_or_else(|| {
        panic!("'init' doesn't exist!");
    });
    let task = Arc::new(TaskStruct::new(data).unwrap());
    add_a_task_to_manager(task);
}

pub fn get_task_data_by_name(name: &str) -> Option<&'static [u8]> {
    let result = APP_NAMES.iter().enumerate().find(|(_, &app_name)| {
        name == app_name
    });
    result.map(|(index, _)| {
        APP_DATA[index]
    })
}

pub fn block_current_and_run_next_task() {
    debug!("block and schedule..");
    let current_task = take_task_in_current_hart();
    let current_task_context_ptr = current_task.acquire_inner_lock().task_context_ptr();
    let hart_context_ptr = get_current_hart_context_ptr();
    drop(current_task);
    unsafe {
        __switch(current_task_context_ptr, hart_context_ptr);
    }
}

pub fn stop_current_and_run_next_task() {
    debug!("stop and schedule..");
    let current_task = take_task_in_current_hart();
    let current_task_context_ptr = current_task.acquire_inner_lock().task_context_ptr();
    return_task_to_manager(current_task);
    let hart_context_ptr = get_current_hart_context_ptr();
    unsafe {
        __switch(current_task_context_ptr, hart_context_ptr);
    }
}

pub fn exit_current_and_run_next_task(exit_code: usize) {
    let current_task = take_task_in_current_hart();
    current_task.acquire_inner_lock().flag = RuntimeFlags::ZOMBIE(exit_code);
    rm_task_from_manager(current_task);
    let hart_context_ptr = get_current_hart_context_ptr();

    let kernel_satp = 8 << 60 | unsafe { KERNEL_SATP };
    riscv::register::satp::write(kernel_satp);

    unsafe {
        //seems like there is no need to do sfence.vma ???
        __switch(0, hart_context_ptr);
    }
}

pub fn get_alive_hart_cnt() -> usize {
    ALIVE_HARTS.lock().get_num()
}

pub fn increase_alive_hart() {
    ALIVE_HARTS.lock().increase();
}

pub fn decrease_alive_hart() {
    ALIVE_HARTS.lock().decrease();
}

lazy_static! {
    pub static ref ALIVE_HARTS: Mutex<Counter> = Mutex::new(Counter::new());
}

pub struct Counter(usize);

impl Counter {
    pub fn new() -> Self {
        Counter{
            0: 0
        }
    }

    pub fn get_num(&self) -> usize {
        self.0
    }

    pub fn decrease(&mut self) {
        self.0 -= 1;
    }

    pub fn increase(&mut self) {
        self.0 += 1;
    }
}