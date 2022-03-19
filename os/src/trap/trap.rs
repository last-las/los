use core::arch::global_asm;

global_asm!(include_str!("trap.asm"));

extern "C" {
    pub fn __enter_user_mode() -> !;
    pub fn __from_user_mode();
}