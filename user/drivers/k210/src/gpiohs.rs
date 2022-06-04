#![allow(unused)]

//! GPIOHS peripheral

use user_lib::syscall::{dev_read_u32, dev_write_u32};

use super::gpio;
use super::utils::{get_bit, set_bit};

const GPIOHS_ADDRESS: usize = 0x3800_1000;

const INPUT_VAL: usize = 0x00;
const INPUT_EN: usize = 0x04;
const OUTPUT_EN: usize = 0x08;
const OUTPUT_VAL: usize = 0x0c;

fn read_output_en() -> u32 {
    dev_read_u32(GPIOHS_ADDRESS + OUTPUT_EN).unwrap() as u32
}

fn read_input_en() -> u32 {
    dev_read_u32(GPIOHS_ADDRESS + INPUT_EN).unwrap() as u32
}

fn write_output_en(value: u32) {
    dev_write_u32(GPIOHS_ADDRESS + OUTPUT_EN, value).unwrap();
}

fn write_input_en(value: u32) {
    dev_write_u32(GPIOHS_ADDRESS + INPUT_EN, value).unwrap();
}

fn read_input_val() -> u32 {
    dev_read_u32(GPIOHS_ADDRESS + INPUT_VAL).unwrap() as u32
}

pub fn read_output_val() -> u32 {
    dev_read_u32(GPIOHS_ADDRESS + OUTPUT_VAL).unwrap() as u32
}

fn write_output_val(value: u32) {
    dev_write_u32(GPIOHS_ADDRESS + OUTPUT_VAL, value).unwrap();
}

/** Set input/output direction for a GPIOHS pin */
pub fn set_direction(pin: u8, direction: gpio::direction) {
    unsafe {
        let v = read_output_en();
        write_output_en(set_bit(v, pin, direction == gpio::direction::OUTPUT));

        assert_eq!(
            read_output_en(),
            set_bit(v, pin, direction == gpio::direction::OUTPUT)
        );

        let v = read_input_en();
        write_input_en(set_bit(v, pin, direction == gpio::direction::INPUT));

        assert_eq!(
            read_input_en(),
            set_bit(v, pin, direction == gpio::direction::INPUT)
        );

        // let ptr = pac::GPIOHS::ptr();
        // (*ptr)
        //     .output_en
        //     .modify(|r, w| w.bits(set_bit(r.bits(), pin, direction == gpio::direction::OUTPUT)));
        // (*ptr)
        //     .input_en
        //     .modify(|r, w| w.bits(set_bit(r.bits(), pin, direction == gpio::direction::INPUT)));
    }
}

/** Set output value for a GPIOHS pin */
pub fn set_pin(pin: u8, value: bool) {
    unsafe {
        // let ptr = pac::GPIOHS::ptr();
        // (*ptr)
        //     .output_val
        //     .modify(|r, w| w.bits(set_bit(r.bits(), pin, value)));
        let v = read_output_val();
        write_output_val(set_bit(v, pin, value));

        assert_eq!(read_output_val(), set_bit(v, pin, value));
    }
}

/** Get input value for a GPIOHS pin */
pub fn get_pin(pin: u8) -> bool {
    unsafe {
        // let ptr = pac::GPIOHS::ptr();
        // get_bit((*ptr).input_val.read().bits(), pin)
        let v = read_input_val();
        get_bit(v, pin)
    }
}
