mod kernel_stack;
mod task_struct;
mod trap_context;
mod task_manager;
mod pid;
mod task_context;

use crate::processor::{get_hart_id, take_task_in_current_hart, set_task_in_current_hart, suspend_current_hart, get_current_hart_context_ptr};
use crate::loader::get_apps_ref_data;
use spin::Mutex;

pub use task_struct::{TaskStruct, RuntimeFlags, ReceiveProc};
pub use task_manager::{fetch_a_task_from_manager, get_task_by_pid};
pub use task_context::TaskContext;
pub use trap_context::TrapContext;
use crate::task::task_manager::{add_a_task_to_manager, rm_task_from_manager};
pub use crate::task::task_manager::return_task_to_manager;
use alloc::sync::Arc;
use crate::timer::set_timer_ms;
use crate::processor::__switch;
use crate::paging::KERNEL_SATP;

pub fn load_tasks() {
    let v;
    unsafe {
        v = get_apps_ref_data();
    }
    println!("apps num : {}", v.len());
    for i in 0..v.len() {
        let data = v[i];
        let wrapped_task = TaskStruct::new(data);
        if wrapped_task.is_none() {
            info!("task{} creating error", i);
            continue;
        }
        let task = Arc::new(wrapped_task.unwrap());
        add_a_task_to_manager(task);
    }
}

pub fn block_current_and_run_next_task() {
    debug!("block and schedule..");
    let current_task = take_task_in_current_hart();
    let current_task_context_ptr = current_task.acquire_inner_lock().task_context_ptr();
    let hart_context_ptr = get_current_hart_context_ptr();
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

pub fn exit_current_and_run_next_task() {
    let current_task = take_task_in_current_hart();
    rm_task_from_manager(current_task);
    let hart_context_ptr = get_current_hart_context_ptr();

    let kernel_satp = 8 << 60 | unsafe { KERNEL_SATP };
    riscv::register::satp::write(kernel_satp);

    unsafe {
        ///seems like there is no need to do sfence.vma ???
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