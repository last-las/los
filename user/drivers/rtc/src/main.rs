#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use crate::rtc::{rtc::RtcTime, Rtc};

mod rtc;
#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() {
    let mut rtc = Rtc::new();
    rtc.init();
    // rtc.set_time(2001, 12, 11, 10, 9, 8);

    // let (y, m, d) = rtc.read_time();
    // panic!("{}-{}-{}", y, m, d);
}
