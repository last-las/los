mod kernel_stack;
mod task_struct;
mod task_context;
mod task_manager;
mod user_stack_allocator;
mod test;

use task_manager::TASK_MANAGER;
use crate::processor::{get_hart_id, take_task_in_current_hart, set_task_in_current_hart, run_task_on_current_hart, suspend_current_hart};
use crate::loader::insert_apps;
use spin::Mutex;

pub use test::test_task_mod;
pub use task_struct::TaskStruct;
pub use task_manager::fetch_a_task_from_task_manager;
pub use task_context::TaskContext;
use crate::task::task_manager::add_a_task_to_task_manager;
use alloc::sync::Arc;
use crate::sbi::timer::sbi_set_timer;
use crate::timer::set_timer_ms;

pub fn load_tasks() {
    let v;
    unsafe {
        v = insert_apps();
    }
    println!("apps num : {}", v.len());
    let mut i = 0;
    for pc in v {
        let task = Arc::new(TaskStruct::new_user_task(pc, i));
        add_a_task_to_task_manager(task);
        i += 1;
    }
}

pub fn stop_current_and_run_next_task() {
    debug!("hart{} scheduling...", get_hart_id());
    let current_task = take_task_in_current_hart();
    add_a_task_to_task_manager(current_task);
    load_and_run_a_task();
}

pub fn exit_current_and_run_next_task() {
    take_task_in_current_hart();
    load_and_run_a_task();
}

pub fn load_and_run_a_task() {
    if let Some(next_task) = fetch_a_task_from_task_manager() {
        debug!("hart{} runs on task:{}", get_hart_id(), next_task.pid);
        set_task_in_current_hart(next_task);
        set_timer_ms(10);
        run_task_on_current_hart();
    } else {
        debug!("hart {} stopped successfully.", get_hart_id());
        decrease_alive_hart();

        if get_alive_hart_cnt() <= 0 {
            panic!("Every hart has stopped. Shutdown the system.");
        } else {
            suspend_current_hart();
        }
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