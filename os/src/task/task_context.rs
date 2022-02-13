use crate::trap::before_enter_user_mode;

#[repr(C)]
pub struct TaskContext {
    pub ra: usize,
    pub sp: usize,
    pub s: [usize; 12],
}

impl TaskContext {
    pub fn new(kernel_sp: usize) -> Self {
        Self {
            ra: before_enter_user_mode as usize,
            sp: kernel_sp,
            s: [0; 12],
        }
    }
}

#[no_mangle]
pub fn test_task_context() {
    panic!("everything end here!");
}