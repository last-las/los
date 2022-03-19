mod hart;
mod switch;

pub use hart::{
    get_hart_id,
    set_hart_id,
    enable_other_harts,
    suspend_current_hart
};
pub use switch::__switch;
use alloc::sync::Arc;
use crate::task::{TaskStruct, TrapContext, fetch_a_task_from_manager, decrease_alive_hart, get_alive_hart_cnt, RuntimeFlags, TaskContext};
use crate::trap::__enter_user_mode;
use spin::Mutex;
use crate::mm::available_frame;
use core::arch::asm;

pub const CPU_NUMS: usize = 4;

pub fn get_cur_task_context_in_this_hart() -> &'static mut TrapContext {
    PROCESSORS[get_hart_id()].get_current_task().unwrap()
        .acquire_inner_lock()
        .kernel_stack.get_mut()
}

#[allow(unused)]
pub fn get_cur_task_in_this_hart() -> Arc<TaskStruct> {
    PROCESSORS[get_hart_id()].get_current_task().unwrap()
}

pub fn set_task_in_current_hart(new_task: Arc<TaskStruct>) {
    PROCESSORS[get_hart_id()].set_current_task(new_task);
}

pub fn take_task_in_current_hart() -> Arc<TaskStruct> {
    PROCESSORS[get_hart_id()].take_current_task().unwrap()
}

pub fn run_on_current_hart() {
    PROCESSORS[get_hart_id()].run();
}

pub fn get_current_hart_context_ptr() -> usize {
    unsafe {
        &PROCESSORS[get_hart_id()].inner.lock().switcher_context as *const _ as usize
    }
}

lazy_static! {
    static ref PROCESSORS: [Processor; CPU_NUMS] = [
        Processor::new(),Processor::new(),
        Processor::new(),Processor::new(),
    ];
}


pub struct Processor {
    // The reason why the "inner" here is necessary
    inner: Mutex<ProcessorInner>
}

struct ProcessorInner {
    current_task: Option<Arc<TaskStruct>>,
    switcher_context: TaskContext
}

impl Processor{
    fn new() -> Self {
        Self {
            inner: Mutex::new(ProcessorInner {
                current_task: None,
                switcher_context: TaskContext::empty(),
            })
        }
    }

    fn run(&self) {
        let processor_inner = self.inner.lock();
        let hart_context_ptr = unsafe {
            &processor_inner.switcher_context as *const _ as usize
        };
        drop(processor_inner);

        loop {
            if let Some(next_task) = fetch_a_task_from_manager() {
                let mut next_task_inner = next_task.acquire_inner_lock();
                next_task_inner.flag = RuntimeFlags::RUNNING;
                let next_task_context_ptr = next_task_inner.task_context_ptr();
                let satp = 8 << 60 | next_task_inner.mem_manager.satp();
                drop(next_task_inner);
                self.set_current_task(next_task);

                riscv::register::satp::write(satp);
                unsafe {
                    asm!{"sfence.vma"}
                    __switch(hart_context_ptr,
                             next_task_context_ptr);
                }

            } else {
                decrease_alive_hart();

                if get_alive_hart_cnt() <= 0 {
                    panic!("Every hart has stopped. Shutdown the system.");
                } else {
                    info!("stopped");
                    suspend_current_hart();
                }
            }
        }
    }

    fn get_current_task(&self) -> Option<Arc<TaskStruct>> {
        let inner = self.inner.lock();
        if inner.current_task.is_none() {
            None
        } else {
            Some(Arc::clone(inner.current_task.as_ref().unwrap()))
        }
    }

    fn set_current_task(&self, current_task: Arc<TaskStruct>)  {
        let mut processor_inner = self.inner.lock();
        assert!(processor_inner.current_task.is_none());
        processor_inner.current_task = Some(current_task);
    }

    fn take_current_task(&self) -> Option<Arc<TaskStruct>> {
        self.inner.lock().current_task.take()
    }

    #[allow(unused)]
    fn is_empty(&self) -> bool {
        self.inner.lock().current_task.is_none()
    }
}
