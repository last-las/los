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
pub use task_struct::{TaskStruct, TaskStructInner, RuntimeFlags};
pub use task_manager::{fetch_a_task_from_manager, add_a_task_to_manager, get_task_by_pid};
pub use task_context::TaskContext;
pub use trap_context::TrapContext;
pub use pid::alloc_pid;
use crate::task::task_manager::rm_task_from_manager;
pub use crate::task::task_manager::return_task_to_manager;
use alloc::sync::Arc;
use crate::processor::__switch;
use crate::paging::KERNEL_SATP;
use lazy_static::*;
use alloc::vec::Vec;
use alloc::vec;
use share::ffi::CStr;

lazy_static! {
    pub static ref APP_NAMES: Vec<CStr<'static>> =get_app_names();

    pub static ref APP_DATA: Vec<&'static [u8]> = unsafe {
        get_app_ref_data()
    };
}

pub fn print_app_names() {
    println!("apps num : {}", APP_DATA.len());
    for i in 0..APP_DATA.len() {
        println!("app[{}] name: {:?}", i, APP_NAMES[i]);
    }
}

pub fn load_init_tasks() {
    #[cfg(feature = "board_qemu")]
    let tasks = vec!["init", "terminal", "virtio-blk", "fs"];
    #[cfg(feature = "board_k210")]
        let tasks = vec!["init", "terminal", "sdcard", "fs"];
    for task_name in tasks {
        let data = get_task_data_by_name(task_name).unwrap_or_else(|| {
            panic!("{} doesn't exist!", task_name);
        });
        let task = Arc::new(TaskStruct::new(data).unwrap());

        // set min_priority for these tasks.
        let mut priority = 0;
        if task_name == "init" { // make sure normal user processes' min_priority is higher than device and fs.
            priority = 3;
        } else if task_name == "fs" { // make sure fs' min_priority is higher than device.
            priority = 2;
        }
        let mut inner = task.acquire_inner_lock();
        inner.min_priority = priority;
        inner.priority = priority;
        drop(inner);

        add_a_task_to_manager(task);
    }
}

pub fn get_task_data_by_name(name: &str) -> Option<&'static [u8]> {
    let result = APP_NAMES.iter().enumerate().find(|(_, app_name)| {
        name == app_name.as_str()
    });
    result.map(|(index, _)| {
        APP_DATA[index]
    })
}

pub fn schedule(runtime_flag: RuntimeFlags) {
    debug!("schedule...");
    let current_task = take_task_in_current_hart();
    let mut inner = current_task.acquire_inner_lock();
    inner.flag = runtime_flag;
    let mut current_task_context_ptr= 0;

    match inner.flag {
        RuntimeFlags::RECEIVING(_) | RuntimeFlags::SENDING(_) => {
            current_task_context_ptr = inner.task_context_ptr();
            drop(inner);
            drop(current_task);
        },
        RuntimeFlags::READY => {
            current_task_context_ptr = inner.task_context_ptr();
            drop(inner);
            return_task_to_manager(current_task);
        },
        RuntimeFlags::ZOMBIE(exit_code) => {
            info!("task {} exit with exit_code:{}",current_task.pid(), exit_code);
            move_cur_task_children_to_init(inner.children.clone());
            drop(inner);
            rm_task_from_manager(current_task);

            let kernel_satp = 8 << 60 | unsafe { KERNEL_SATP };
            riscv::register::satp::write(kernel_satp);
        },
        RuntimeFlags::RUNNING => panic!("schedule error!")
    };

    let hart_context_ptr = get_current_hart_context_ptr();
    unsafe {
        __switch(current_task_context_ptr, hart_context_ptr);
    }
}

fn move_cur_task_children_to_init(children: Vec<Arc<TaskStruct>>) {
    let init_task = get_task_by_pid(0).unwrap();
    let mut init_task_inner = init_task.acquire_inner_lock();
    init_task_inner.children.extend(children);
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