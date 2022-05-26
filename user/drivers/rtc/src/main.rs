#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::panic;

use crate::rtc::Rtc;
pub mod mods;
mod rtc;
#[macro_use]
extern crate user_lib;

use mods::*;

#[no_mangle]
fn main() {
    let mut rtc = Rtc::new();
    rtc.init();

    rtc.set_time(2001, 12, 11, 10, 9, 8);

    let (hour, min, sec) = rtc.read_time();
    let (year, mon, day) = rtc.read_date();

    //? date&time test
    assert!(year == 2001 && mon == 12 && day == 11 && hour == 10 && min == 9 && sec == 8);
    // panic!(
    //     "{}/{}/{} {}:{:02}:{:02} <{}>",
    //     year, mon, day, hour, min, sec, week
    // ); // 2001/12/11 10:9:8

    rtc.set_alarm(2022, 5, 26, 22, 25, 00);
    let (hour, min, sec) = rtc.read_alarm_time();
    let (year, mon, day) = rtc.read_alarm_date();

    //? date&time test
    assert!(year == 2022 && mon == 5 && day == 26 && hour == 22 && min == 25 && sec == 0);
}
