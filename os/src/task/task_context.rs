use crate::trap:: __enter_user_mode;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskContext {
    pub ra: usize,
    pub sp: usize,
    pub s: [usize; 12],
}

impl TaskContext {
    pub fn empty() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    pub fn new(kernel_sp: usize) -> Self {
        Self {
            ra: __enter_user_mode as usize,
            sp: kernel_sp,
            s: [0; 12],
        }
    }
}