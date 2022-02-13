global_asm!(include_str!("trap.asm"));

extern "C" {
    pub fn __enter_user_mode() -> !;
    pub fn __from_user_mode();
}

#[no_mangle]
pub fn before_enter_user_mode() {
    unsafe {
        __enter_user_mode();
    }
}