mod kernel_stack;
mod task_struct;
mod trap_context;
mod task_manager;
mod user_stack_allocator;
mod test;
mod pid;
mod task_context;

use crate::processor::{get_hart_id, take_task_in_current_hart, set_task_in_current_hart, suspend_current_hart, get_current_hart_context_ptr};
use crate::loader::copy_apps_to_base_address;
use spin::Mutex;

#[cfg(feature = "test")]
pub use test::test_task_mod;
pub use task_struct::{TaskStruct, RuntimeFlags, ReceiveProc};
pub use task_manager::{fetch_a_task_from_manager, get_task_by_pid};
pub use task_context::TaskContext;
pub use trap_context::TrapContext;
use crate::task::task_manager::{add_a_task_to_manager, rm_task_from_manager};
pub use crate::task::task_manager::return_task_to_manager;
use alloc::sync::Arc;
use crate::timer::set_timer_ms;
use crate::processor::__switch;

pub fn load_tasks() {
    let v;
    unsafe {
        v = copy_apps_to_base_address();
    }
    println!("apps num : {}", v.len());
    for pc in v {
        info!("the pc is: {:#x}", pc);
        let task = Arc::new(TaskStruct::new(pc));
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
    unsafe {
        __switch(0, hart_context_ptr);
    }
}

/*pub fn load_and_run_a_task() {
    if let Some(next_task) = fetch_a_task_from_manager() {
        debug!("runs on {:?}", next_task.pid_handle);
        set_task_in_current_hart(next_task);
        set_timer_ms(10);
        run_task_on_current_hart();
    } else {
        debug!("stopped successfully.");
        decrease_alive_hart();

        if get_alive_hart_cnt() <= 0 {
            panic!("Every hart has stopped. Shutdown the system.");
        } else {
            suspend_current_hart();
        }
    }
}*/

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