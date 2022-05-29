//! (TODO) Direct Memory Access Controller (DMAC)
use crate::sysctl;

use self::{
    cfg::CFG,
    channel::{
        axi_id::AXI_ID, block_ts::BLOCK_TS, ctl::CTL, dar::DAR, intclear::INTCLEAR,
        intstatus_en::INTSTATUS_EN, sar::SAR,
    },
    chen::CHEN,
    com_intclear::COM_INTCLEAR,
    com_intsignal_en::COM_INTSIGNAL_EN,
    com_intstatus_en::COM_INTSTATUS_EN,
    compver::COMPVER,
    id::ID,
    reset::RESET,
};

pub struct Dmac {
    #[doc = "0x00 - ID Register"]
    pub id: ID,
    #[doc = "0x08 - COMPVER Register"]
    pub compver: COMPVER,
    #[doc = "0x10 - Configure Register"]
    pub cfg: CFG,
    #[doc = "0x18 - Channel Enable Register"]
    pub chen: CHEN,
    _reserved4: [u8; 16usize],
    #[doc = "0x30 - Interrupt Status Register"]
    // pub intstatus: INTSTATUS,
    #[doc = "0x38 - Common Interrupt Clear Register"]
    pub com_intclear: COM_INTCLEAR,
    #[doc = "0x40 - Common Interrupt Status Enable Register"]
    pub com_intstatus_en: COM_INTSTATUS_EN,
    #[doc = "0x48 - Common Interrupt Signal Enable Register"]
    pub com_intsignal_en: COM_INTSIGNAL_EN,
    #[doc = "0x50 - Common Interrupt Status"]
    // pub com_intstatus: COM_INTSTATUS,
    #[doc = "0x58 - Reset register"]
    pub reset: RESET,
    _reserved10: [u8; 160usize],
    #[doc = "0x100 - Channel configuration"]
    pub channel: [CHANNEL; 6],
}

pub struct CHANNEL {
    #[doc = "0x00 - SAR Address Register"]
    pub sar: SAR,
    #[doc = "0x08 - DAR Address Register"]
    pub dar: DAR,
    #[doc = "0x10 - Block Transfer Size Register"]
    pub block_ts: BLOCK_TS,
    #[doc = "0x18 - Control Register"]
    pub ctl: CTL,
    #[doc = "0x20 - Configure Register"]
    pub cfg: channel::cfg::CFG,
    #[doc = "0x28 - Linked List Pointer register"]
    // pub llp: self::channel::LLP,
    #[doc = "0x30 - Channel Status Register"]
    // pub status: self::channel::STATUS,
    #[doc = "0x38 - Channel Software handshake Source Register"]
    // pub swhssrc: self::channel::SWHSSRC,
    #[doc = "0x40 - Channel Software handshake Destination Register"]
    // pub swhsdst: self::channel::SWHSDST,
    #[doc = "0x48 - Channel Block Transfer Resume Request Register"]
    // pub blk_tfr: self::channel::BLK_TFR,
    #[doc = "0x50 - Channel AXI ID Register"]
    pub axi_id: AXI_ID,
    #[doc = "0x58 - AXI QOS Register"]
    // pub axi_qos: self::channel::AXI_QOS,
    _reserved12: [u8; 32usize],
    #[doc = "0x80 - Interrupt Status Enable Register"]
    pub intstatus_en: INTSTATUS_EN,
    #[doc = "0x88 - Channel Interrupt Status Register"]
    // pub intstatus: self::channel::INTSTATUS,
    #[doc = "0x90 - Interrupt Signal Enable Register"]
    // pub intsignal_en: self::channel::INTSIGNAL_EN,
    #[doc = "0x98 - Interrupt Clear Register"]
    pub intclear: INTCLEAR,
    _reserved16: [u8; 88usize],
    // #[doc = "0xf8 - Padding to make structure size 256 bytes so that channels\\[\\]is an array"]
    // pub _reserved: self::channel::_RESERVED,
}

pub mod id {
    const REG: usize = 0x00;
    const ADDRESS: usize = DMAC_ADDRESS + REG;

    use user_lib::syscall::{dev_read, dev_write};

    use super::DMAC_ADDRESS;
    pub struct ID {}

    impl ID {
        pub fn read(&self) -> u64 {
            dev_read(ADDRESS, 8).unwrap() as u64 as u64
        }
        fn write(&self, value: u64) {
            dev_write(ADDRESS, value as usize, 8).unwrap();
        }
    }
}

pub mod compver {
    const REG: usize = 0x08;
    const ADDRESS: usize = DMAC_ADDRESS + REG;

    use user_lib::syscall::{dev_read, dev_write};

    use super::DMAC_ADDRESS;
    pub struct COMPVER {}

    impl COMPVER {
        pub fn read(&self) -> u64 {
            dev_read(ADDRESS, 8).unwrap() as u64 as u64
        }
        fn write(&self, value: u64) {
            dev_write(ADDRESS, value as usize, 8).unwrap();
        }
    }
}

pub mod cfg {
    const REG: usize = 0x10;
    const ADDRESS: usize = DMAC_ADDRESS + REG;

    use user_lib::syscall::{dev_read, dev_write};

    use super::DMAC_ADDRESS;
    pub struct CFG {}

    impl CFG {
        pub fn read(&self) -> u64 {
            dev_read(ADDRESS, 8).unwrap() as u64
        }
        fn write(&self, value: u64) -> &Self {
            dev_write(ADDRESS, value as usize, 8).unwrap();
            &self
        }

        pub fn write_dmac_en(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !0x01) | ((value as u64) & 0x01);
            self.write(v)
        }

        pub fn write_int_en(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !(0x01 << 1)) | (((value as u64) & 0x01) << 1);
            self.write(v)
        }
    }
}

pub mod chen {
    const REG: usize = 0x18;
    const ADDRESS: usize = DMAC_ADDRESS + REG;

    use user_lib::syscall::{dev_read, dev_write};

    use super::DMAC_ADDRESS;
    pub struct CHEN {}

    impl CHEN {
        fn read(&self) -> u64 {
            dev_read(ADDRESS, 8).unwrap() as u64
        }
        fn write(&self, value: u64) -> &Self {
            dev_write(ADDRESS, value as usize, 8).unwrap();
            &self
        }
        // n为0-5 表示channel[n]
        fn write_ch_en(&self, value: bool, n: usize) -> &Self {
            let v = self.read();
            let v = (v & !(0x01 << n)) | (((value as u64) & 0x01) << n);
            self.write(v)
        }
        // n: channel[x]的x-1+8
        fn write_ch_en_ew(&self, value: bool, n: usize) -> &Self {
            let v = self.read();
            let v = (v & !(0x01 << n)) | (((value as u64) & 0x01) << n);
            self.write(v)
        }

        pub fn channel_n_enable(&self, n: usize) -> &Self {
            self.write_ch_en(true, n - 1)
                .write_ch_en_ew(true, n - 1 + 8)
        }

        pub fn channel_n_disable(&self, n: usize) -> &Self {
            self.write_ch_en(false, n - 1)
                .write_ch_en_ew(true, n - 1 + 8)
        }

        pub fn read_ch_n_en(&self, n: usize) -> bool {
            let v = self.read();
            ((v >> (n - 1)) & 0x01) != 0
        }
    }
}

pub mod com_intclear {
    const REG: usize = 0x38;
    const ADDRESS: usize = DMAC_ADDRESS + REG;

    use user_lib::syscall::{dev_read, dev_write};

    use super::DMAC_ADDRESS;
    pub struct COM_INTCLEAR {}

    impl COM_INTCLEAR {
        fn read(&self) -> u64 {
            dev_read(ADDRESS, 8).unwrap() as u64
        }
        fn write(&self, value: u64) -> &Self {
            dev_write(ADDRESS, value as usize, 8).unwrap();
            &self
        }

        pub fn write_slvif_dec_err(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !0x01) | ((value as u64) & 0x01);
            self.write(v)
        }

        pub fn write_slvif_wr2ro_err(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !(0x01 << 1)) | (((value as u64) & 0x01) << 1);
            self.write(v)
        }

        pub fn write_slvif_rd2wo_err(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !(0x01 << 2)) | (((value as u64) & 0x01) << 2);
            self.write(v)
        }

        pub fn write_slvif_wronhold_err(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !(0x01 << 3)) | (((value as u64) & 0x01) << 3);
            self.write(v)
        }

        pub fn write_slvif_undefinedreg_dec_err(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !(0x01 << 8)) | (((value as u64) & 0x01) << 8);
            self.write(v)
        }
    }
}

pub mod com_intstatus_en {
    const REG: usize = 0x40;
    const ADDRESS: usize = DMAC_ADDRESS + REG;

    use user_lib::syscall::{dev_read, dev_write};

    use super::DMAC_ADDRESS;
    pub struct COM_INTSTATUS_EN {}

    impl COM_INTSTATUS_EN {
        fn read(&self) -> u64 {
            dev_read(ADDRESS, 8).unwrap() as u64
        }
        fn write(&self, value: u64) -> &Self {
            dev_write(ADDRESS, value as usize, 8).unwrap();
            &self
        }

        pub fn write_slvif_dec_err(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !0x01) | ((value as u64) & 0x01);
            self.write(v)
        }

        pub fn write_slvif_wr2ro_err(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !(0x01 << 1)) | (((value as u64) & 0x01) << 1);
            self.write(v);
            &self
        }

        pub fn write_slvif_rd2wo_err(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !(0x01 << 2)) | (((value as u64) & 0x01) << 2);
            self.write(v)
        }

        pub fn write_slvif_wronhold_err(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !(0x01 << 3)) | (((value as u64) & 0x01) << 3);
            self.write(v)
        }

        pub fn write_slvif_undefinedreg_dec_err(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !(0x01 << 8)) | (((value as u64) & 0x01) << 8);
            self.write(v)
        }
    }
}

pub mod com_intsignal_en {
    const REG: usize = 0x48;
    const ADDRESS: usize = DMAC_ADDRESS + REG;

    use user_lib::syscall::{dev_read, dev_write};

    use super::DMAC_ADDRESS;
    pub struct COM_INTSIGNAL_EN {}

    impl COM_INTSIGNAL_EN {
        fn read(&self) -> u64 {
            dev_read(ADDRESS, 8).unwrap() as u64
        }
        fn write(&self, value: u64) -> &Self {
            dev_write(ADDRESS, value as usize, 8).unwrap();
            &self
        }

        pub fn write_slvif_dec_err(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !0x01) | ((value as u64) & 0x01);
            self.write(v);
            &self
        }

        pub fn write_slvif_wr2ro_err(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !(0x01 << 1)) | (((value as u64) & 0x01) << 1);
            self.write(v)
        }

        pub fn write_slvif_rd2wo_err(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !(0x01 << 2)) | (((value as u64) & 0x01) << 2);
            self.write(v)
        }

        pub fn write_slvif_wronhold_err(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !(0x01 << 3)) | (((value as u64) & 0x01) << 3);
            self.write(v)
        }

        pub fn write_slvif_undefinedreg_dec_err(&self, value: bool) -> &Self {
            let v = self.read();
            let v = (v & !(0x01 << 8)) | (((value as u64) & 0x01) << 8);
            self.write(v)
        }
    }
}

pub mod channel {
    const REG_CHANNEL: usize = 0x100;
    pub mod sar {
        const REG: usize = 0x00;
        const ADDRESS: usize = DMAC_ADDRESS + REG_CHANNEL + REG;

        use user_lib::syscall::{dev_read, dev_write};

        use crate::dmac::dmac::DMAC_ADDRESS;

        use super::REG_CHANNEL;
        pub struct SAR {}

        impl SAR {
            fn read(&self) -> u64 {
                dev_read(ADDRESS, 8).unwrap() as u64
            }
            pub fn write(&self, value: u64) {
                dev_write(ADDRESS, value as usize, 8).unwrap();
            }
        }
    }

    pub mod dar {
        const REG: usize = 0x08;
        const ADDRESS: usize = DMAC_ADDRESS + REG_CHANNEL + REG;

        use user_lib::syscall::{dev_read, dev_write};

        use crate::dmac::dmac::DMAC_ADDRESS;

        use super::REG_CHANNEL;
        pub struct DAR {}

        impl DAR {
            fn read(&self) -> u64 {
                dev_read(ADDRESS, 8).unwrap() as u64
            }
            pub fn write(&self, value: u64) {
                dev_write(ADDRESS, value as usize, 8).unwrap();
            }
        }
    }

    pub mod block_ts {
        const REG: usize = 0x10;
        const ADDRESS: usize = DMAC_ADDRESS + REG_CHANNEL + REG;

        use user_lib::syscall::{dev_read, dev_write};

        use crate::dmac::dmac::DMAC_ADDRESS;

        use super::REG_CHANNEL;
        pub struct BLOCK_TS {}

        impl BLOCK_TS {
            fn read(&self) -> u64 {
                dev_read(ADDRESS, 8).unwrap() as u64
            }
            fn write(&self, value: u64) -> &Self {
                dev_write(ADDRESS, value as usize, 8).unwrap();
                &self
            }

            pub fn write_block_ts(&self, value: u32) -> &Self {
                let v = self.read();
                let v = (v & !0x003f_ffff) | ((value as u64) & 0x003f_ffff);
                self.write(v)
            }
        }
    }

    pub mod ctl {
        use core::convert::From;
        #[doc = "Source master select\n\nValue on reset: 0"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum SMS_A {
            #[doc = "0: AXI master 1"]
            AXI_MASTER_1 = 0,
            #[doc = "1: AXI master 2"]
            AXI_MASTER_2 = 1,
        }
        impl From<SMS_A> for bool {
            #[inline(always)]
            fn from(variant: SMS_A) -> Self {
                variant as u8 != 0
            }
        }

        #[doc = "Source address increment\n\nValue on reset: 0"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum SINC_A {
            #[doc = "0: Increment address"]
            INCREMENT = 0,
            #[doc = "1: Don't increment address"]
            NOCHANGE = 1,
        }
        impl From<SINC_A> for bool {
            #[inline(always)]
            fn from(variant: SINC_A) -> Self {
                variant as u8 != 0
            }
        }

        #[doc = "Destination address increment"]
        pub type DINC_A = SINC_A;

        #[doc = "Source transfer width\n\nValue on reset: 0"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[repr(u8)]
        pub enum SRC_TR_WIDTH_A {
            #[doc = "0: 8 bits"]
            WIDTH_8 = 0,
            #[doc = "1: 16 bits"]
            WIDTH_16 = 1,
            #[doc = "2: 32 bits"]
            WIDTH_32 = 2,
            #[doc = "3: 64 bits"]
            WIDTH_64 = 3,
            #[doc = "4: 128 bits"]
            WIDTH_128 = 4,
            #[doc = "5: 256 bits"]
            WIDTH_256 = 5,
            #[doc = "6: 512 bits"]
            WIDTH_512 = 6,
        }
        impl From<SRC_TR_WIDTH_A> for u8 {
            #[inline(always)]
            fn from(variant: SRC_TR_WIDTH_A) -> Self {
                variant as _
            }
        }

        #[doc = "Destination transfer width"]
        pub type DST_TR_WIDTH_A = SRC_TR_WIDTH_A;

        #[doc = "Source burst transaction length\n\nValue on reset: 0"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[repr(u8)]
        pub enum SRC_MSIZE_A {
            #[doc = "0: 1 data item"]
            LENGTH_1 = 0,
            #[doc = "1: 4 data items"]
            LENGTH_4 = 1,
            #[doc = "2: 8 data items"]
            LENGTH_8 = 2,
            #[doc = "3: 16 data items"]
            LENGTH_16 = 3,
            #[doc = "4: 32 data items"]
            LENGTH_32 = 4,
            #[doc = "5: 64 data items"]
            LENGTH_64 = 5,
            #[doc = "6: 128 data items"]
            LENGTH_128 = 6,
            #[doc = "7: 256 data items"]
            LENGTH_256 = 7,
            #[doc = "8: 512 data items"]
            LENGTH_512 = 8,
            #[doc = "9: 1024 data items"]
            LENGTH_1024 = 9,
        }
        impl From<SRC_MSIZE_A> for u8 {
            #[inline(always)]
            fn from(variant: SRC_MSIZE_A) -> Self {
                variant as _
            }
        }

        #[doc = "Destination burst transaction length"]
        pub type DST_MSIZE_A = SRC_MSIZE_A;

        const REG: usize = 0x50;
        const ADDRESS: usize = DMAC_ADDRESS + REG_CHANNEL + REG;

        use user_lib::syscall::{dev_read, dev_write};

        use crate::dmac::dmac::DMAC_ADDRESS;

        use super::REG_CHANNEL;
        pub type DMS_A = SMS_A;
        pub struct CTL {}

        impl CTL {
            pub fn read(&self) -> u64 {
                dev_read(ADDRESS, 8).unwrap() as u64
            }
            fn write(&self, value: u64) -> &Self {
                dev_write(ADDRESS, value as usize, 8).unwrap();
                &self
            }

            pub fn write_sms(&self, variant: SMS_A) -> &Self {
                let v = self.read();
                let var: bool = variant.into();
                let v = (v & !0x01) | ((var as u64) & 0x01);
                self.write(v)
            }

            pub fn write_dms(&self, variant: DMS_A) -> &Self {
                let v = self.read();
                let var: bool = variant.into();
                let v = (v & !(0x01 << 2)) | (((var as u64) & 0x01) << 2);
                self.write(v)
            }

            pub fn write_sinc(&self, variant: SINC_A) -> &Self {
                let v = self.read();
                let var: bool = variant.into();
                let v = (v & !(0x01 << 4)) | (((var as u64) & 0x01) << 4);
                self.write(v)
            }

            pub fn write_dinc(&self, variant: DINC_A) -> &Self {
                let v = self.read();
                let var: bool = variant.into();
                let v = (v & !(0x01 << 6)) | (((var as u64) & 0x01) << 6);
                self.write(v)
            }

            pub fn write_src_tr_width(&self, variant: SRC_TR_WIDTH_A) -> &Self {
                let v = self.read();
                let var: u8 = variant.into();
                let v = (v & !(0x07 << 8)) | (((var as u64) & 0x07) << 8);
                self.write(v)
            }

            pub fn write_dst_tr_width(&self, variant: SRC_TR_WIDTH_A) -> &Self {
                let v = self.read();
                let var: u8 = variant.into();
                let v = (v & !(0x07 << 11)) | (((var as u64) & 0x07) << 11);
                self.write(v)
            }

            pub fn write_src_msize(&self, variant: SRC_MSIZE_A) -> &Self {
                let v = self.read();
                let var: u8 = variant.into();
                let v = (v & !(0x0f << 14)) | (((var as u64) & 0x0f) << 14);
                self.write(v)
            }

            pub fn write_dst_msize(&self, variant: DST_MSIZE_A) -> &Self {
                let v = self.read();
                let var: u8 = variant.into();
                let v = (v & !(0x0f << 18)) | (((var as u64) & 0x0f) << 18);
                self.write(v)
            }
        }
    }

    pub mod axi_id {
        const REG: usize = 0x50;
        const ADDRESS: usize = DMAC_ADDRESS + REG_CHANNEL + REG;

        use user_lib::syscall::{dev_read, dev_write};

        use crate::dmac::dmac::DMAC_ADDRESS;

        use super::REG_CHANNEL;
        pub struct AXI_ID {}

        impl AXI_ID {
            pub fn read(&self) -> u64 {
                dev_read(ADDRESS, 8).unwrap() as u64
            }
            fn write(&self, value: u64) {
                dev_write(ADDRESS, value as usize, 8).unwrap();
            }
        }
    }

    pub mod intstatus_en {
        const REG: usize = 0x80;
        const ADDRESS: usize = DMAC_ADDRESS + REG_CHANNEL + REG;

        use user_lib::syscall::{dev_read, dev_write};

        use crate::dmac::dmac::DMAC_ADDRESS;

        use super::REG_CHANNEL;
        pub struct INTSTATUS_EN {}

        impl INTSTATUS_EN {
            pub fn read(&self) -> u64 {
                dev_read(ADDRESS, 8).unwrap() as u64
            }
            pub fn write(&self, value: u64) -> &Self {
                dev_write(ADDRESS, value as usize, 8).unwrap();
                &self
            }

            pub fn write_src_transcomp(&self, value: bool) -> &Self {
                let v = self.read();
                let v = (v & !(0x01 << 3)) | (((value as u64) & 0x01) << 3);
                self.write(v)
            }
        }
    }

    pub mod intclear {
        const REG: usize = 0x98;
        const ADDRESS: usize = DMAC_ADDRESS + REG_CHANNEL + REG;

        use user_lib::syscall::{dev_read, dev_write};

        use crate::dmac::dmac::DMAC_ADDRESS;

        use super::REG_CHANNEL;
        pub struct INTCLEAR {}

        impl INTCLEAR {
            pub fn read(&self) -> u64 {
                dev_read(ADDRESS, 8).unwrap() as u64
            }
            pub fn write(&self, value: u64) {
                dev_write(ADDRESS, value as usize, 8).unwrap();
            }
        }
    }

    pub mod cfg {
        const REG: usize = 0x20;
        const ADDRESS: usize = DMAC_ADDRESS + REG_CHANNEL + REG;

        use user_lib::syscall::{dev_read, dev_write};

        use crate::dmac::dmac::DMAC_ADDRESS;

        use super::REG_CHANNEL;
        #[doc = "Transfer type and flow control\n\nValue on reset: 0"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        #[repr(u8)]
        pub enum TT_FC_A {
            #[doc = "0: Transfer memory to memory and flow controller is DMAC"]
            MEM2MEM_DMA = 0,
            #[doc = "1: Transfer memory to peripheral and flow controller is DMAC"]
            MEM2PRF_DMA = 1,
            #[doc = "2: Transfer peripheral to memory and flow controller is DMAC"]
            PRF2MEM_DMA = 2,
            #[doc = "3: Transfer peripheral to peripheral and flow controller is DMAC"]
            PRF2PRF_DMA = 3,
            #[doc = "4: Transfer peripheral to memory and flow controller is source peripheral"]
            PRF2MEM_PRF = 4,
            #[doc = "5: Transfer peripheral to peripheral and flow controller is source peripheral"]
            PRF2PRF_SRCPRF = 5,
            #[doc = "6: Transfer memory to peripheral and flow controller is destination peripheral"]
            MEM2PRF_PRF = 6,
            #[doc = "7: Transfer peripheral to peripheral and flow controller is destination peripheral"]
            PRF2PRF_DSTPRF = 7,
        }
        impl From<TT_FC_A> for u8 {
            #[inline(always)]
            fn from(variant: TT_FC_A) -> Self {
                variant as _
            }
        }

        #[doc = "Source software or hardware handshaking select\n\nValue on reset: 0"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum HS_SEL_SRC_A {
            #[doc = "0: Hardware handshaking is used"]
            HARDWARE = 0,
            #[doc = "1: Software handshaking is used"]
            SOFTWARE = 1,
        }
        impl From<HS_SEL_SRC_A> for bool {
            #[inline(always)]
            fn from(variant: HS_SEL_SRC_A) -> Self {
                variant as u8 != 0
            }
        }

        pub type HS_SEL_DST_A = HS_SEL_SRC_A;
        pub struct CFG {}

        impl CFG {
            fn read(&self) -> u64 {
                dev_read(ADDRESS, 8).unwrap() as u64
            }
            fn write(&self, value: u64) -> &Self {
                dev_write(ADDRESS, value as usize, 8).unwrap();
                &self
            }

            pub fn write_tt_fc(&self, variant: TT_FC_A) -> &Self {
                let v = self.read();
                let var: u8 = variant.into();
                let v = (v & !(0x07 << 32)) | (((var as u64) & 0x07) << 32);
                self.write(v)
            }

            pub fn write_hs_sel_src(&self, variant: HS_SEL_SRC_A) -> &Self {
                let v = self.read();
                let var: bool = variant.into();
                let v = (v & !(0x01 << 35)) | (((var as u64) & 0x01) << 35);
                self.write(v)
            }

            pub fn write_hs_sel_dst(&self, variant: HS_SEL_DST_A) -> &Self {
                let v = self.read();
                let var: bool = variant.into();
                let v = (v & !(0x01 << 36)) | (((var as u64) & 0x01) << 36);
                self.write(v)
            }

            pub fn write_src_per(&self, value: u8) -> &Self {
                let v = self.read();
                let v = (v & !(0x0f << 39)) | (((value as u64) & 0x0f) << 39);
                self.write(v)
            }

            pub fn write_dst_per(&self, value: u8) -> &Self {
                let v = self.read();
                let v = (v & !(0x0f << 44)) | (((value as u64) & 0x0f) << 44);
                self.write(v);
                &self
            }

            pub fn write_src_multblk_type(&self, value: u8) -> &Self {
                let v = self.read();
                let v = (v & !0x03) | ((value as u64) & 0x03);
                self.write(v)
            }

            pub fn write_dst_multblk_type(&self, value: u8) -> &Self {
                let v = self.read();
                let v = (v & !(0x03 << 2)) | (((value as u64) & 0x03) << 2);
                self.write(v)
            }
        }
    }
}

pub mod reset {
    const REG: usize = 0x58;
    const ADDRESS: usize = DMAC_ADDRESS + REG;

    use user_lib::syscall::{dev_read, dev_write};

    use super::DMAC_ADDRESS;
    pub struct RESET {}

    impl RESET {
        fn read(&self) -> u64 {
            dev_read(ADDRESS, 8).unwrap() as u64
        }
        fn write(&self, value: u64) {
            dev_write(ADDRESS, value as usize, 8).unwrap();
        }

        pub fn write_rst(&self, value: bool) {
            let v = self.read();
            let v = (v & !0x01) | ((value as u64) & 0x01);
            self.write(v);
        }

        pub fn read_rst(&self) -> bool {
            let v = self.read();
            (v & 0x01) != 0
        }
    }
}

const DMAC_ADDRESS: usize = 0x5000_0000;
