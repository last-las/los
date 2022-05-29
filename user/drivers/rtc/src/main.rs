#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::panic;

use crate::mods::date_mod::read_date;
use crate::mods::time_mod::read_time;
use crate::rtc::Rtc;
pub mod mods;
mod rtc;
#[macro_use]
extern crate user_lib;

use mods::*;
use share::ipc::Msg;
use share::ipc::*;
use user_lib::syscall::*;

use core::assert;

#[no_mangle]
fn main() {
    let mut rtc = Rtc::new();
    rtc.init();

    println!("rtc init.");

    rtc.irq_register(1);

    // interrupt_ctrl_mod::rtc_alarm_irq_register(0);

    let (y, mn, d, h, m, s) = (2022, 5, 27, 19, 55, 00);
    rtc.timer_set(y, mn, d, h, m, s);

    let (hour, min, sec) = rtc.read_time();
    let (year, mon, day) = rtc.read_date();

    //? date&time test
    assert!(year == y && mon == mn && day == d && hour == h && min == m && sec == s);

    rtc.set_alarm(2022, 5, 27, 19, 56, 00);
    let (hour, min, sec) = rtc.read_alarm_time();
    let (year, mon, day) = rtc.read_alarm_date();

    //? date&time test
    assert!(year == 2022 && mon == 5 && day == 27 && hour == 19 && min == 56 && sec == 0);

    let mut message = Msg::empty();

    loop {
        receive(-1, &mut message).unwrap();

        let nr = message.args[DEVICE];
        assert_eq!(nr, 0);

        match message.mtype {
            INTERRUPT => do_interrupt(&mut rtc),
            OPEN => do_open(&mut rtc, message),
            READ => do_read(&mut rtc, message),
            WRITE => do_write(&mut rtc, message),
            IOCTL => do_ioctl(&mut rtc, message),
            CLOSE => do_close(&mut rtc, message),

            _ => {
                panic!("Unknown message type:{}", message.mtype);
            }
        }
    }
}

pub fn do_interrupt(rtc: &mut Rtc) {
    rtc.tick_interrupt_enable(true);
    println!("interrupt! time: {:?}", rtc.read_time());
}

pub fn do_open(rtc: &mut Rtc, message: Msg) {}

pub fn do_read(rtc: &mut Rtc, message: Msg) {}
pub fn do_write(rtc: &mut Rtc, message: Msg) {}
pub fn do_ioctl(rtc: &mut Rtc, message: Msg) {}
pub fn do_close(rtc: &mut Rtc, message: Msg) {}
