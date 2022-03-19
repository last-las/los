use core::arch::asm;
use crate::sbi::sbi_send_ipi;
use crate::sbi::hart::sbi_hart_suspend;

pub fn set_hart_id(hart_id: usize) {
    unsafe {
        asm!(
        "mv tp, {0}",
        in(reg) hart_id,
        );
    }
}

pub fn get_hart_id() -> usize {
    let hart_id;

    unsafe {
        asm!(
        "mv {0}, tp",
        out(reg) hart_id,
        )
    }
    hart_id
}

pub fn enable_other_harts() {
    assert_eq!(get_hart_id(), 0);
    let hart_mask = usize::MAX ^ 0b1;
    let sbi_ret = sbi_send_ipi(&hart_mask as *const _ as usize,0);
    assert_eq!(sbi_ret.error, 0);
}

pub fn suspend_current_hart() {
    let sbi_ret =  sbi_hart_suspend(0x00000000, 0,0);
    assert_eq!(sbi_ret.error, 0);
}