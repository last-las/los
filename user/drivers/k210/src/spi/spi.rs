use self::{
    baudr::BAUDR, ctrlr0::CTRLR0, ctrlr1::CTRLR1, dmacr::DMACR, dmardlr::DMARDLR, dmatdlr::DMATDLR,
    dr::DR, endian::ENDIAN, imr::IMR, rxflr::RXFLR, ser::SER, spi_ctrlr0::SPI_CTRLR0, sr::SR,
    ssienr::SSIENR, txflr::TXFLR,
};

/// Serial Peripheral Interface 0 (master)
pub struct SPI0 {
    #[doc = "0x00 - Control Register 0"]
    pub ctrlr0: CTRLR0,
    #[doc = "0x04 - Control Register 1"]
    pub ctrlr1: CTRLR1,
    #[doc = "0x08 - Enable Register"]
    pub ssienr: SSIENR,
    #[doc = "0x10 - Slave Enable Register"]
    pub ser: SER,
    #[doc = "0x14 - Baud Rate Select"]
    pub baudr: BAUDR,
    #[doc = "0x20 - Transmit FIFO Level Register"]
    pub txflr: TXFLR,
    #[doc = "0x24 - Receive FIFO Level Register"]
    pub rxflr: RXFLR,
    #[doc = "0x28 - Status Register"]
    pub sr: SR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x4c - DMA Control Register"]
    pub dmacr: DMACR,
    #[doc = "0x50 - DMA Transmit Data Level"]
    pub dmatdlr: DMATDLR,
    #[doc = "0x54 - DMA Receive Data Level"]
    pub dmardlr: DMARDLR,
    #[doc = "0x60 - Data Register"]
    pub dr: DR,
    #[doc = "0xf4 - SPI Control Register"]
    pub spi_ctrlr0: SPI_CTRLR0,
    #[doc = "0x118 - ENDIAN"]
    pub endian: ENDIAN,
}

impl SPI0 {
    pub fn new() -> Self {
        Self {
            ctrlr0: CTRLR0 {},
            ctrlr1: CTRLR1 {},
            ssienr: SSIENR {},
            ser: SER {},
            baudr: BAUDR {},
            txflr: TXFLR {},
            rxflr: RXFLR {},
            sr: SR {},
            imr: IMR {},
            dmacr: DMACR {},
            dmatdlr: DMATDLR {},
            dmardlr: DMARDLR {},
            dr: DR {},
            spi_ctrlr0: SPI_CTRLR0 {},
            endian: ENDIAN {},
        }
    }
}

pub mod ctrlr0 {
    const REG: usize = 0x00;
    const ADDRESS: usize = SPI0_ADDRESS + REG;

    use core::convert::From;

    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SPI0_ADDRESS;

    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(u8)]
    pub enum WORK_MODE_A {
        #[doc = "0: MODE_0"]
        MODE0 = 0,
        #[doc = "1: MODE_1"]
        MODE1 = 1,
        #[doc = "2: MODE_2"]
        MODE2 = 2,
        #[doc = "3: MODE_3"]
        MODE3 = 3,
    }
    impl From<WORK_MODE_A> for u8 {
        #[inline(always)]
        fn from(variant: WORK_MODE_A) -> Self {
            variant as _
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(u8)]
    pub enum TMOD_A {
        #[doc = "0: TRANS_RECV"]
        TRANS_RECV = 0,
        #[doc = "1: TRANS"]
        TRANS = 1,
        #[doc = "2: RECV"]
        RECV = 2,
        #[doc = "3: EEROM"]
        EEROM = 3,
    }
    impl From<TMOD_A> for u8 {
        #[inline(always)]
        fn from(variant: TMOD_A) -> Self {
            variant as _
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(u8)]
    pub enum FRAME_FORMAT_A {
        #[doc = "0: STANDARD"]
        STANDARD = 0,
        #[doc = "1: DUAL"]
        DUAL = 1,
        #[doc = "2: QUAD"]
        QUAD = 2,
        #[doc = "3: OCTAL"]
        OCTAL = 3,
    }
    impl From<FRAME_FORMAT_A> for u8 {
        #[inline(always)]
        fn from(variant: FRAME_FORMAT_A) -> Self {
            variant as _
        }
    }

    pub struct CTRLR0 {}

    impl CTRLR0 {
        fn read(&self) -> u32 {
            dev_read_u32(ADDRESS).unwrap() as u32
        }
        fn write(&self, value: u32) -> &Self {
            dev_write_u32(ADDRESS, value).unwrap();
            &self
        }
        pub fn write_work_mode(&self, variant: WORK_MODE_A) -> &Self {
            let v = self.read();
            let variant: u8 = variant.into();
            let v =(v & !(0x03 << 6)) | (((variant as u32) & 0x03) << 6);
            self.write(v)
        }
        pub fn write_tmod(&self, variant: TMOD_A) -> &Self {
            let v = self.read();
            let variant: u8 = variant.into();
            let v =(v & !(0x03 << 8)) | (((variant as u32) & 0x03) << 8);
            self.write(v)
        }
        pub fn write_frame_format(&self, variant: FRAME_FORMAT_A) -> &Self {
            let v = self.read();
            let variant: u8 = variant.into();
            let v = (v & !(0x03 << 21)) | (((variant as u32) & 0x03) << 21);
            self.write(v)
        }
        pub fn write_data_length(&self, value: u8) -> &Self {
            let v = self.read();
            let v = (v & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
            self.write(v)
        }
    }
}

pub mod ctrlr1 {
    const REG: usize = 0x04;
    const ADDRESS: usize = SPI0_ADDRESS + REG;

    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SPI0_ADDRESS;

    pub struct CTRLR1 {}

    impl CTRLR1 {
        fn read(&self) -> u32 {
            dev_read_u32(ADDRESS).unwrap() as u32
        }
        pub fn write(&self, value: u32) -> &Self {
            dev_write_u32(ADDRESS, value).unwrap();
            &self
        }
    }
}

pub mod ssienr {
    const REG: usize = 0x08;
    const ADDRESS: usize = SPI0_ADDRESS + REG;
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SPI0_ADDRESS;
    pub struct SSIENR {}
    impl SSIENR {
        pub fn read(&self) -> u32 {
            dev_read_u32(ADDRESS).unwrap() as u32
        }
        pub fn write(&self, value: u32) -> &Self {
            dev_write_u32(ADDRESS, value).unwrap();
            &self
        }
    }
}

pub mod ser {
    const REG: usize = 0x10;
    const ADDRESS: usize = SPI0_ADDRESS + REG;
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SPI0_ADDRESS;
    pub struct SER {}
    impl SER {
        pub fn read(&self) -> u32 {
            dev_read_u32(ADDRESS).unwrap() as u32
        }
        pub fn write(&self, value: u32) -> &Self {
            dev_write_u32(ADDRESS, value).unwrap();
            &self
        }
    }
}

pub mod baudr {
    const REG: usize = 0x14;
    const ADDRESS: usize = SPI0_ADDRESS + REG;
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SPI0_ADDRESS;
    pub struct BAUDR {}
    impl BAUDR {
        pub fn read(&self) -> u32 {
            dev_read_u32(ADDRESS).unwrap() as u32
        }
        pub fn write(&self, value: u32) -> &Self {
            dev_write_u32(ADDRESS, value).unwrap();
            &self
        }
    }
}

pub mod txflr {
    const REG: usize = 0x20;
    const ADDRESS: usize = SPI0_ADDRESS + REG;
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SPI0_ADDRESS;
    pub struct TXFLR {}
    impl TXFLR {
        pub fn read(&self) -> u32 {
            dev_read_u32(ADDRESS).unwrap() as u32
        }
        fn write(&self, value: u32) -> &Self {
            dev_write_u32(ADDRESS, value).unwrap();
            &self
        }
    }
}

pub mod rxflr {
    const REG: usize = 0x24;
    const ADDRESS: usize = SPI0_ADDRESS + REG;
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SPI0_ADDRESS;
    pub struct RXFLR {}
    impl RXFLR {
        pub fn read(&self) -> u32 {
            dev_read_u32(ADDRESS).unwrap() as u32
        }
        pub fn write(&self, value: u32) -> &Self {
            dev_write_u32(ADDRESS, value).unwrap();
            &self
        }
    }
}

pub mod sr {
    const REG: usize = 0x28;
    const ADDRESS: usize = SPI0_ADDRESS + REG;
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SPI0_ADDRESS;
    pub struct SR {}
    impl SR {
        pub fn read(&self) -> u32 {
            dev_read_u32(ADDRESS).unwrap() as u32
        }
        pub fn write(&self, value: u32) -> &Self {
            dev_write_u32(ADDRESS, value).unwrap();
            &self
        }
    }
}

pub mod imr {
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SPI0_ADDRESS;

    const REG: usize = 0x2c;
    const ADDRESS: usize = SPI0_ADDRESS + REG;

    pub struct IMR {}

    impl IMR {
        pub fn read(&self) -> u32 {
            dev_read_u32(ADDRESS).unwrap() as u32
        }
        pub fn write(&self, value: u32) -> &Self {
            dev_write_u32(ADDRESS, value).unwrap();
            &self
        }
    }
}

pub mod dmacr {
    use super::SPI0_ADDRESS;
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    const REG: usize = 0x4c;
    const ADDRESS: usize = SPI0_ADDRESS + REG;

    pub struct DMACR {}

    impl DMACR {
        pub fn read(&self) -> u32 {
            dev_read_u32(ADDRESS).unwrap() as u32
        }
        pub fn write(&self, value: u32) -> &Self {
            dev_write_u32(ADDRESS, value).unwrap();
            &self
        }
    }
}

pub mod dmatdlr {
    const REG: usize = 0x50;
    const ADDRESS: usize = SPI0_ADDRESS + REG;
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SPI0_ADDRESS;

    pub struct DMATDLR {}
    impl DMATDLR {
        pub fn read(&self) -> u32 {
            dev_read_u32(ADDRESS).unwrap() as u32
        }
        pub fn write(&self, value: u32) -> &Self {
            dev_write_u32(ADDRESS, value).unwrap();
            &self
        }
    }
}

pub mod dmardlr {
    const REG: usize = 0x54;
    const ADDRESS: usize = SPI0_ADDRESS + REG;
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SPI0_ADDRESS;
    pub struct DMARDLR {}
    impl DMARDLR {
        pub fn read(&self) -> u32 {
            dev_read_u32(ADDRESS).unwrap() as u32
        }
        pub fn write(&self, value: u32) -> &Self {
            dev_write_u32(ADDRESS, value).unwrap();
            &self
        }
    }
}

pub mod dr {
    const REG: usize = 0x60;
    const ADDRESS: usize = SPI0_ADDRESS + REG;
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SPI0_ADDRESS;
    #[derive(Clone, Copy)]
    pub struct DR {}
    impl DR {
        // read dr[n]
        pub fn read(&self) -> u32 {
            dev_read_u32(ADDRESS).unwrap() as u32
        }
        // write dr[n]
        pub fn write(&self, value: u32) -> &Self {
            dev_write_u32(ADDRESS, value).unwrap();
            &self
        }
    }
}

pub mod spi_ctrlr0 {
    const REG: usize = 0xf4;
    const ADDRESS: usize = SPI0_ADDRESS + REG;
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SPI0_ADDRESS;

    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(u8)]
    pub enum AITM_A {
        #[doc = "0: STANDARD"]
        STANDARD = 0,
        #[doc = "1: ADDR_STANDARD"]
        ADDR_STANDARD = 1,
        #[doc = "2: AS_FRAME_FORMAT"]
        AS_FRAME_FORMAT = 2,
    }
    impl From<AITM_A> for u8 {
        #[inline(always)]
        fn from(variant: AITM_A) -> Self {
            variant as _
        }
    }

    pub struct SPI_CTRLR0 {}
    impl SPI_CTRLR0 {
        pub fn read(&self) -> u32 {
            dev_read_u32(ADDRESS).unwrap() as u32
        }
        fn write(&self, value: u32) -> &Self {
            dev_write_u32(ADDRESS, value).unwrap();
            &self
        }

        pub fn write_aitm(&self, variant: AITM_A) -> &Self {
            let v = self.read();
            let variant: u8 = variant.into();
            let v = (v & !0x03) | ((variant as u32) & 0x03);
            self.write(v)
        }

        pub fn write_addr_length(&self, value: u8) -> &Self {
            let v = self.read();
            let v = (v & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
            self.write(v)
        }

        pub fn write_inst_length(&self, value: u8) -> &Self {
            let v = self.read();
            let v = (v & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
            self.write(v)
        }

        pub fn write_wait_cycle(&self, value: u8) -> &Self {
            let v = self.read();
            let v = (v & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
            self.write(v)
        }
    }
}

pub mod endian {
    const REG: usize = 0x118;
    const ADDRESS: usize = SPI0_ADDRESS + REG;
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    pub struct ENDIAN {}
    impl ENDIAN {
        pub fn read(&self) -> u32 {
            dev_read_u32(ADDRESS).unwrap() as u32
        }
        pub fn write(&self, value: u32) -> &Self {
            dev_write_u32(ADDRESS, value).unwrap();
            &self
        }
    }

    use super::SPI0_ADDRESS;
}

const SPI0_ADDRESS: usize = 0x5200_0000;
