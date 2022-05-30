use core::marker::PhantomData;

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
        #[doc = "0x00 - FPIOA GPIO multiplexer io array"]
        pub io: [IO; 48],
        #[doc = "0xc0 - FPIOA GPIO multiplexer tie enable array"]
        pub tie_en: [TIE_EN; 8],
        #[doc = "0xe0 - FPIOA GPIO multiplexer tie value array"]
        pub tie_val: [TIE_VAL; 8],
    }

    impl Fpioa {
        pub fn new() -> Self {
            Self {
                io: [IO::new(); 48],
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
        pub struct IO {
            ch_sel: u8,
            ds: u8,
            oe_en: bool,
            oe_inv: bool,
            do_sel: bool,
            do_inv: bool,
            pu: bool,
            pd: bool,
            sl: bool,
            ie_en: bool,
            ie_inv: bool,
            di_inv: bool,
            st: bool,
            pad_di: bool,
        }
        impl IO {
            pub fn new() -> Self {
                Self {
                    ch_sel: 0,
                    ds: 0,
                    oe_en: false,
                    oe_inv: false,
                    do_sel: false,
                    do_inv: false,
                    pu: false,
                    pd: false,
                    sl: false,
                    ie_en: false,
                    ie_inv: false,
                    di_inv: false,
                    st: false,
                    pad_di: false,
                }
            }
            fn read(&self) -> u32 {
                dev_read_u32(ADDRESS).unwrap() as u32
            }
            pub fn write(&self, value: u32) -> &Self {
                dev_write_u32(ADDRESS, value).unwrap();
                &self
            }
            pub fn ch_sel(&self, value: u8) -> &Self {
                let v = self.read();
                let v = (v & !0xff) | ((value as u32) & 0xff);
                self.write(v)
            }

            pub fn pu(&self, value: bool) -> &Self {
                let v = self.read();
                let v = (v & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
                self.write(v)
            }

            pub fn pd(&self, value: bool) -> &Self {
                let v = self.read();
                let v = (v & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
                self.write(v)
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
