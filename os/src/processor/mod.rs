use crate::sbi::sbi_send_ipi;

mod hart;

pub use hart::{
    get_hart_id,
    set_hart_id,
    enable_other_harts,
    suspend_current_hart
};
use alloc::sync::Arc;
use crate::task::{TaskStruct, TaskContext};
use crate::trap::__enter_user_mode;
use alloc::string::String;
use alloc::vec;
use core::cell::RefCell;
use spin::Mutex;

pub const CPU_NUMS: usize = 4;

pub fn get_cur_task_context_in_this_hart() -> &'static mut TaskContext {
    PROCESSORS[get_hart_id()].get_current_task().unwrap().kernel_stack.get_mut()
}

pub fn get_cur_task_in_this_hart() -> Arc<TaskStruct> {
    PROCESSORS[get_hart_id()].get_current_task().unwrap()
}

pub fn set_task_in_current_hart(new_task: Arc<TaskStruct>) {
    PROCESSORS[get_hart_id()].set_current_task(new_task);
}

pub fn take_task_in_current_hart() -> Arc<TaskStruct> {
    PROCESSORS[get_hart_id()].take_current_task().unwrap()
}

pub fn run_task_on_current_hart() {
    PROCESSORS[get_hart_id()].run();
}

lazy_static! {
    static ref cpu_nums: usize = env!("CPU_NUMS").parse::<usize>().unwrap();
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
    current_task: Option<Arc<TaskStruct>>
}

impl Processor{
    fn new() -> Self {
        Self {
            inner: Mutex::new(ProcessorInner {
                current_task: None
            })
        }
    }

    // WARN: this implementation might have some problems,
    // otherwise it doesn't make sense that rCore saves two kinds of contents on stack.
    fn run(&self) {
        let inner = self.inner.lock();
        let kernel_stack_sp =
            inner.current_task.as_ref().unwrap().kernel_stack.sp;
        drop(inner); // to avoid dead lock because this function won't reach the end.
        unsafe {
            __enter_user_mode(kernel_stack_sp);
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
        self.inner.lock().current_task = Some(current_task);
    }

    fn take_current_task(&self) -> Option<Arc<TaskStruct>> {
        self.inner.lock().current_task.take()
    }

    fn is_empty(&self) -> bool {
        self.inner.lock().current_task.is_none()
    }
}
