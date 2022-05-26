#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::panic;

use crate::rtc::Rtc;
use crate::rtc::{date_mod, time_mod};
mod rtc;
#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() {
    let mut rtc = Rtc::new();
    rtc.init();

    rtc.set_time(2001, 12, 11, 10, 9, 8);

    let (hour, min, sec) = time_mod::read_time();
    let (year, mon, day, week) = date_mod::read_date();

    //? date&time test
    assert!(year == 2001 && mon == 12 && day == 11 && hour == 10 && min == 9 && sec == 8);
    // panic!("{}/{}/{} {}:{:02}:{:02} <{}>", y, m, d, h1, m1, s1, w); // 2001/12/11 10:9:8
}
