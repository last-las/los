const SBI_CONSOLE_PUTCHAR: usize = 0x01;
#[allow(unused)]
const SBI_CONSOLE_GETCHAR: usize = 0x02;
const SBI_SHUTDOWN: usize = 0x08;

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

pub fn sbi_console_putchar(ch: char) {
    sbi_legacy_call(SBI_CONSOLE_PUTCHAR, [ch as usize, 0, 0]);
}

pub fn sbi_shutdown() {
    sbi_legacy_call(SBI_SHUTDOWN, [0, 0, 0]);
}