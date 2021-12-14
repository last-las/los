global_asm!(include_str!("trap.asm"));

extern "C" {
    pub fn __enter_user_mode(kernel_stack_sp_ptr: usize) -> !;
    pub fn __from_user_mode();
}