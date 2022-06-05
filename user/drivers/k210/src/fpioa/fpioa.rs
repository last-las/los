use self::fpioa::Fpioa;

pub struct FPIOA {
    pub fpioa: Fpioa,
}

impl FPIOA {
    pub fn new() -> Self {
        Self {
            fpioa: Fpioa::new(),
        }
    }
}

pub mod fpioa {
    use self::{io::IO, tie_en::TIE_EN, tie_val::TIE_VAL};

    const FPIOA_ADDRESS: usize = 0x502b_0000;
    pub struct Fpioa {
        #[doc = "0x00 - FPIOA GPIO multiplexer io array IO[48]"]
        pub io: IO,
        #[doc = "0xc0 - FPIOA GPIO multiplexer tie enable array"]
        pub tie_en: [TIE_EN; 8],
        #[doc = "0xe0 - FPIOA GPIO multiplexer tie value array"]
        pub tie_val: [TIE_VAL; 8],
    }

    impl Fpioa {
        pub fn new() -> Self {
            Self {
                io: IO {},
                tie_en: [TIE_EN::new(); 8],
                tie_val: [TIE_VAL::new(); 8],
            }
        }
    }

    pub mod io {
        use user_lib::syscall::{dev_read, dev_read_u32, dev_write_u32};

        use super::FPIOA_ADDRESS;

        const REG: usize = 0x00;
        const ADDRESS: usize = FPIOA_ADDRESS + REG;
        #[derive(Clone, Copy)]
        pub struct IO {}
        impl IO {
            pub fn new() -> Self {
                Self {}
            }
            // read io[n] 0: 0-3 1:4-7 2:8-11
            pub fn read(&self, n: usize) -> u32 {
                dev_read_u32(ADDRESS + n * 4).unwrap() as u32
            }
            // write io[n]
            pub fn write(&self, n: usize, value: u32) -> &Self {
                dev_write_u32(ADDRESS + n * 4, value).unwrap();
                &self
            }
            pub fn ch_sel(&self, n: usize, value: u8) -> &Self {
                let v = self.read(n);
                let v = (v & !0xff) | ((value as u32) & 0xff);
                self.write(n, v)
            }

            pub fn pu(&self, n: usize, value: bool) -> &Self {
                let v = self.read(n);
                let v = (v & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
                self.write(n, v)
            }

            pub fn pd(&self, n: usize, value: bool) -> &Self {
                let v = self.read(n);
                let v = (v & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
                self.write(n, v)
            }
        }
    }

    pub mod tie_en {
        use super::FPIOA_ADDRESS;

        const REG: usize = 0xc0;
        const ADDRESS: usize = FPIOA_ADDRESS + REG;
        #[derive(Clone, Copy)]
        pub struct TIE_EN {}

        impl TIE_EN {
            pub fn new() -> Self {
                Self {}
            }
        }
    }

    pub mod tie_val {
        use super::FPIOA_ADDRESS;

        const REG: usize = 0xe0;
        const ADDRESS: usize = FPIOA_ADDRESS + REG;
        #[derive(Clone, Copy)]
        pub struct TIE_VAL {}

        impl TIE_VAL {
            pub fn new() -> Self {
                Self {}
            }
        }
    }
}
