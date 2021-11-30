use crate::sbi::sbi_send_ipi;

pub fn set_hart_id(hart_id: usize) {
    unsafe {
        asm!(
        "mv tp, {0}",
        in(reg) hart_id,
        );
    }
}

#[allow(unused)]
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

pub fn enable_all_cpus() {
    let sbi_ret = sbi_send_ipi(usize::MAX, usize::MAX);
    assert_eq!(sbi_ret.error, 0);
}

#[test]
fn cpu() {
    let hart_id = 2;
    set_hart_id(hart_id);
    assert_eq!(hart_id, get_hart_id());
}