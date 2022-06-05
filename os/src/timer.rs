use crate::sbi::sbi_legacy_set_timer;
use riscv::register::{sie, time};

#[cfg(feature = "board_qemu")]
const CLOCK_FREQUENCY: usize = 12500000;
#[cfg(feature = "board_k210")]
const CLOCK_FREQUENCY: usize = 403000000 / 62;

const MSEC_PER_SEC: usize = 1000;

const TICKS_PER_SEC: usize = 100;
#[allow(unused)]
const USEC_PER_SEC: usize = 1000000;

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

pub fn get_time_s() -> usize {
    time::read() / CLOCK_FREQUENCY
}

pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQUENCY / USEC_PER_SEC)
}

#[allow(unused)]
// should consider differences between user and kernel tasks.
pub fn set_timer_ms(slice_ms: usize) {
    sbi_legacy_set_timer(get_time() + CLOCK_FREQUENCY * slice_ms / MSEC_PER_SEC);
}
