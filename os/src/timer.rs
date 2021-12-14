use riscv::register::{time, sie, sstatus, stvec, sip};
use crate::sbi::timer::sbi_set_timer;
use crate::sbi::{sbi_legacy_set_timer, sbi_shutdown};

const CLOCK_FREQUENCY: usize = 12500000;
const MSEC_PER_SEC: usize = 1000;

pub fn enable_time_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

pub fn get_time() -> usize {
    time::read()
}

pub fn get_time_ms() -> usize {
    get_time() / (CLOCK_FREQUENCY / MSEC_PER_SEC)
}

// should consider differences between user and kernel tasks.
pub fn set_timer_ms(slice_ms: usize) {
    sbi_legacy_set_timer(get_time() +  CLOCK_FREQUENCY * slice_ms / MSEC_PER_SEC);
}