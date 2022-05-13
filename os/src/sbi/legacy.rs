use core::arch::asm;

#[allow(unused)]
const SBI_LEGACY_SET_TIMER: usize = 0x00;
const SBI_LEGACY_CONSOLE_PUTCHAR: usize = 0x01;
#[allow(unused)]
const SBI_LEGACY_CONSOLE_GETCHAR: usize = 0x02;
#[allow(unused)]
const SBI_LEGACY_SEND_IPI: usize = 0x04;
const SBI_LEGACY_SHUTDOWN: usize = 0x08;

#[inline(always)]
fn sbi_legacy_call(eid: usize, args: [usize; 3]) -> usize {
    let retval: usize;

    unsafe {
        asm!(
        "ecall",
        in("a7") eid,
        inout("a0") args[0] => retval,
        in("a1") args[1],
        in("a2") args[2],
        );
    }
    retval
}

#[allow(unused)]
pub fn sbi_legacy_set_timer(stime_value: usize) {
    sbi_legacy_call(SBI_LEGACY_SET_TIMER, [stime_value, 0, 0]);
}

pub fn sbi_console_putchar(ch: char) {
    sbi_legacy_call(SBI_LEGACY_CONSOLE_PUTCHAR, [ch as usize, 0, 0]);
}

pub fn sbi_console_getchar() -> isize {
    sbi_legacy_call(SBI_LEGACY_CONSOLE_GETCHAR, [0, 0, 0]) as isize
}

#[allow(unused)]
pub fn sbi_legacy_send_ipi(hart_mask: usize) {
    sbi_legacy_call(SBI_LEGACY_SEND_IPI, [hart_mask, 0, 0]);
}

pub fn sbi_shutdown() {
    sbi_legacy_call(SBI_LEGACY_SHUTDOWN, [0, 0, 0]);
}