#![allow(unused)]

//! GPIOHS peripheral

use user_lib::syscall::{dev_read_u32, dev_write_u32};

use super::gpio;
use super::utils::{get_bit, set_bit};
use crate::gpiohs::gpiohs::Gpiohs;


pub struct GPIOHS {
    gpiohs: Gpiohs,
}

impl GPIOHS {
    pub fn new() -> Self {
        Self {
            gpiohs: Gpiohs::new()
        }
    }
}


pub mod gpiohs {
    use crate::gpiohs::gpiohs::input_val::INPUT_VAL;
    use crate::gpiohs::gpiohs::input_en::INPUT_EN;
    use crate::gpiohs::gpiohs::output_en::OUTPUT_EN;
    use crate::gpiohs::gpiohs::output_val::OUTPUT_VAL;

    const GPIOHS_ADDRESS: usize = 0x3800_1000;

    pub struct Gpiohs {
        #[doc = "0x00 - Input Value Register"]
        pub input_val: INPUT_VAL,
        #[doc = "0x04 - Pin Input Enable Register"]
        pub input_en: INPUT_EN,
        #[doc = "0x08 - Pin Output Enable Register"]
        pub output_en: OUTPUT_EN,
        #[doc = "0x0c - Output Value Register"]
        pub output_val: OUTPUT_VAL,
    }

    impl Gpiohs {
        pub fn new() -> Self {
            Self {
                input_val: INPUT_VAL {},
                input_en: INPUT_EN {},
                output_en: OUTPUT_EN {},
                output_val: OUTPUT_VAL {},
            }
        }
    }

    pub mod input_val {
        use crate::gpiohs::gpiohs::GPIOHS_ADDRESS;
        use user_lib::syscall::{dev_read_u32, dev_write_u32};

        const REG: usize = 0x00;
        const ADDRESS: usize = GPIOHS_ADDRESS + REG;

        pub struct INPUT_VAL {}

        impl INPUT_VAL {
            pub fn read(&self) -> u32 {
                dev_read_u32(ADDRESS).unwrap() as u32
            }
            pub fn write(&self, value: u32) -> &Self {
                dev_write_u32(ADDRESS, value).unwrap();
                &self
            }
        }
    }

    pub mod input_en {
        use crate::gpiohs::gpiohs::GPIOHS_ADDRESS;
        use user_lib::syscall::{dev_read_u32, dev_write_u32};

        const REG: usize = 0x04;
        const ADDRESS: usize = GPIOHS_ADDRESS + REG;

        pub struct INPUT_EN {}

        impl INPUT_EN {
            pub fn read(&self) -> u32 {
                dev_read_u32(ADDRESS).unwrap() as u32
            }
            pub fn write(&self, value: u32) -> &Self {
                dev_write_u32(ADDRESS, value).unwrap();
                &self
            }
        }
    }

    pub mod output_en {
        use crate::gpiohs::gpiohs::GPIOHS_ADDRESS;
        use user_lib::syscall::{dev_read_u32, dev_write_u32};

        const REG: usize = 0x08;
        const ADDRESS: usize = GPIOHS_ADDRESS + REG;

        pub struct OUTPUT_EN {}

        impl OUTPUT_EN {
            pub fn read(&self) -> u32 {
                dev_read_u32(ADDRESS).unwrap() as u32
            }
            pub fn write(&self, value: u32) -> &Self {
                dev_write_u32(ADDRESS, value).unwrap();
                &self
            }
        }
    }

    pub mod output_val {
        use crate::gpiohs::gpiohs::GPIOHS_ADDRESS;
        use user_lib::syscall::{dev_read_u32, dev_write_u32};

        const REG: usize = 0x0c;
        const ADDRESS: usize = GPIOHS_ADDRESS + REG;

        pub struct OUTPUT_VAL {}

        impl OUTPUT_VAL {
            pub fn read(&self) -> u32 {
                dev_read_u32(ADDRESS).unwrap() as u32
            }
            pub fn write(&self, value: u32) -> &Self {
                dev_write_u32(ADDRESS, value).unwrap();
                &self
            }
        }
    }
}


/** Set input/output direction for a GPIOHS pin */
pub fn set_direction(pin: u8, direction: gpio::direction) {
    unsafe {
        let gpio = GPIOHS::new().gpiohs;

        gpio.output_en.write(set_bit(gpio.output_en.read(),pin,direction == gpio::direction::OUTPUT));
        // let v = read_output_en();
        // write_output_en(set_bit(v, pin, direction == gpio::direction::OUTPUT));

        // assert_eq!(
        //     read_output_en(),
        //     set_bit(v, pin, direction == gpio::direction::OUTPUT)
        // );

        gpio.input_en.write(set_bit(gpio.input_en.read(), pin,direction == gpio::direction::INPUT ));

        // let v = read_input_en();
        // write_input_en(set_bit(v, pin, direction == gpio::direction::INPUT));
        //
        // assert_eq!(
        //     read_input_en(),
        //     set_bit(v, pin, direction == gpio::direction::INPUT)
        // );

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
        let gpio = GPIOHS::new().gpiohs;
        gpio.output_val.write(set_bit(gpio.output_val.read(), pin, value));
        // let v = read_output_val();
        // write_output_val(set_bit(v, pin, value));
        //
        // assert_eq!(read_output_val(), set_bit(v, pin, value));
    }
}

/** Get input value for a GPIOHS pin */
pub fn get_pin(pin: u8) -> bool {
    unsafe {
        let gpio = GPIOHS::new().gpiohs;
        // let ptr = pac::GPIOHS::ptr();
        // get_bit((*ptr).input_val.read().bits(), pin)
        // let v = read_input_val();
        get_bit(gpio.input_val.read(), pin)
    }
}
