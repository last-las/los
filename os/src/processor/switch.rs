global_asm!(include_str!("switch.asm"));

extern "C" {
    pub fn __switch(target_task_context_ptr: usize);
    pub fn __record_sp(processor_task_context_ptr2: usize);
}