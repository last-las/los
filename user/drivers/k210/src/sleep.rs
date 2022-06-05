use user_lib::syscall::get_time;

use super::sysctl;

pub fn time_sleep(n: usize) {
    let start = get_time();
    while get_time() < start + n {}
}

pub fn usleep(n: usize) {
    let freq = sysctl::clock_get_freq(sysctl::clock::CPU) as usize / 62;
    time_sleep(freq * n / 1000000);
}
