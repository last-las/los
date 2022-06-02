use crate::{clock::Clocks, time::Bps};

use self::uarths::UARTHS;

pub mod uarths {
    use self::{div::DIV, rxctrl::RXCTRL, txctrl::TXCTRL};

    const UARTHS_ADDRESS: usize = 0x3800_0000;
    pub struct UARTHS {
        #[doc = "0x00 - Transmit Data Register"]
        // pub txdata: TXDATA,
        #[doc = "0x04 - Receive Data Register"]
        // pub rxdata: RXDATA,
        #[doc = "0x08 - Transmit Control Register"]
        pub txctrl: TXCTRL,
        #[doc = "0x0c - Receive Control Register"]
        pub rxctrl: RXCTRL,
        #[doc = "0x10 - Interrupt Enable Register"]
        // pub ie: IE,
        #[doc = "0x14 - Interrupt Pending Register"]
        // pub ip: IP,
        #[doc = "0x18 - Baud Rate Divisor Register"]
        pub div: DIV,
    }

    impl UARTHS {
        pub fn new() -> Self {
            Self {
                txctrl: TXCTRL {},
                rxctrl: RXCTRL {},
                div: DIV {},
            }
        }
    }

    pub mod div {
        use super::UARTHS_ADDRESS;
        use user_lib::syscall::{dev_read_u32, dev_write_u32};

        const REG: usize = 0x08;
        const ADDRESS: usize = UARTHS_ADDRESS + REG;
        pub struct DIV {}
        impl DIV {
            fn read(&self) -> u32 {
                dev_read_u32(ADDRESS).unwrap() as u32
            }
            pub fn write(&self, value: u32) -> &Self {
                dev_write_u32(ADDRESS, value).unwrap();
                &self
            }
        }
    }

    pub mod txctrl {
        use super::UARTHS_ADDRESS;
        use user_lib::syscall::{dev_read_u32, dev_write_u32};

        const REG: usize = 0x0c;
        const ADDRESS: usize = UARTHS_ADDRESS + REG;
        pub struct TXCTRL {}
        impl TXCTRL {
            fn read(&self) -> u32 {
                dev_read_u32(ADDRESS).unwrap() as u32
            }
            fn write(&self, value: u32) -> &Self {
                dev_write_u32(ADDRESS, value).unwrap();
                &self
            }
            pub fn txen(&self, value: bool) -> &Self {
                let v = self.read();
                let v = (v & !0x01) | ((value as u32) & 0x01);
                self.write(v)
            }
        }
    }

    pub mod rxctrl {
        use user_lib::syscall::{dev_read_u32, dev_write_u32};

        use super::UARTHS_ADDRESS;

        const REG: usize = 0x18;
        const ADDRESS: usize = UARTHS_ADDRESS + REG;
        pub struct RXCTRL {}
        impl RXCTRL {
            fn read(&self) -> u32 {
                dev_read_u32(ADDRESS).unwrap() as u32
            }
            fn write(&self, value: u32) -> &Self {
                dev_write_u32(ADDRESS, value).unwrap();
                &self
            }
            pub fn rxen(&self, value: bool) -> &Self {
                let v = self.read();
                let v = (v & !0x01) | ((value as u32) & 0x01);
                self.write(v)
            }
        }
    }
}

pub fn configure(baud_rate: Bps, clocks: &Clocks) {
    let uart = UARTHS::new();

    let div = clocks.cpu().0 / baud_rate.0 - 1;
    // unsafe {
    //     uart.div.write(|w| w.bits(div));
    // }
    uart.div.write(div);

    // uart.txctrl.write(|w| w.txen().bit(true));
    uart.txctrl.txen(true);
    // uart.rxctrl.write(|w| w.rxen().bit(true));
    uart.rxctrl.rxen(true);
}
