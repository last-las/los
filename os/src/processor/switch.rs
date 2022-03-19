use core::arch::global_asm;

global_asm!(include_str!("switch.asm"));

extern "C" {
    pub fn __switch(current_task_context_ptr: usize, target_task_context_ptr: usize);
}

