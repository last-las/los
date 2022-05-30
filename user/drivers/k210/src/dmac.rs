#![allow(unused)]
#![allow(non_camel_case_types)]

//! DMAC peripheral
use self::dmac::{
    channel::{
        self,
        cfg::{HS_SEL_SRC_A, TT_FC_A},
        ctl::SMS_A,
    },
    Dmac,
};

use super::sysctl;

mod dmac;
use core::marker::Sized;
/** Extension trait for adding configure() to DMAC peripheral */
pub trait DMACExt: Sized {
    /// Constrains DVP peripheral
    fn configure(self) -> DMAC;
}

impl DMACExt for Dmac {
    fn configure(self) -> DMAC {
        DMAC::new(self)
    }
}

/** DMAC peripheral abstraction */
pub struct DMAC {
    dmac: Dmac,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum src_dst_select {
    SRC = 0x1,
    DST = 0x2,
    SRC_DST = 0x3,
}

pub use super::sysctl::dma_channel;

pub type master_number = channel::ctl::SMS_A;
pub type address_increment = channel::ctl::SINC_A;
pub type burst_length = channel::ctl::SRC_MSIZE_A;
pub type transfer_width = channel::ctl::SRC_TR_WIDTH_A;

/** Return whether a specific address considered considered memory or peripheral */
fn is_memory(address: u64) -> bool {
    let mem_len = 6 * 1024 * 1024;
    let mem_no_cache_len = 8 * 1024 * 1024;
    // Note: This comes from the Kendryte SDK as-is. No, I have no idea why the AES accelerator
    // input address 0x50450040 is considered memory, either.
    ((address >= 0x80000000) && (address < 0x80000000 + mem_len))
        || ((address >= 0x40000000) && (address < 0x40000000 + mem_no_cache_len))
        || (address == 0x50450040)
}

impl DMAC {
    fn new(dmac: Dmac) -> Self {
        let rv = Self { dmac };
        rv.init();
        rv
    }

    /** Get DMAC ID */
    pub fn read_id(&self) -> u64 {
        self.dmac.id.read()
    }

    /** Get DMAC version */
    pub fn read_version(&self) -> u64 {
        self.dmac.compver.read()
    }

    /** Get AXI ID for channel */
    pub fn read_channel_id(&self, channel_num: dma_channel) -> u64 {
        self.dmac.channel[channel_num.idx()].axi_id.read()
    }

    /** Enable DMAC peripheral. */
    fn enable(&self) {
        self.dmac.cfg.write_dmac_en(true).write_int_en(true);
        // self.dmac
        //     .cfg
        //     .modify(|_, w| w.dmac_en().set_bit().int_en().set_bit());
    }

    /** Disable DMAC peripheral. */
    pub fn disable(&self) {
        self.dmac.cfg.write_dmac_en(false).write_int_en(false);
        // self.dmac
        //     .cfg
        //     .modify(|_, w| w.dmac_en().clear_bit().int_en().clear_bit());
    }

    pub fn src_transaction_complete_int_enable(&self, channel_num: dma_channel) {
        self.dmac.channel[channel_num.idx()]
            .intstatus_en
            .write_src_transcomp(true);
        // .intstatus_en
        // .modify(|_, w| w.src_transcomp().set_bit());
    }

    /** Enable a DMA channel. */
    pub fn channel_enable(&self, channel_num: dma_channel) {
        use dma_channel::*;
        // Note: chX bit names start counting from 1, while channels start counting from 0
        match channel_num {
            CHANNEL0 => {
                self.dmac.chen.channel_n_enable(1);
                // self.dmac
                //     .chen
                //     .modify(|_, w| w.ch1_en().set_bit().ch1_en_we().set_bit());
            }
            CHANNEL1 => {
                self.dmac.chen.channel_n_enable(2);

                // self.dmac
                //     .chen
                //     .modify(|_, w| w.ch2_en().set_bit().ch2_en_we().set_bit());
            }
            CHANNEL2 => {
                self.dmac.chen.channel_n_enable(3);
                // self.dmac
                //     .chen
                //     .modify(|_, w| w.ch3_en().set_bit().ch3_en_we().set_bit());
            }
            CHANNEL3 => {
                self.dmac.chen.channel_n_enable(4);
                // self.dmac
                //     .chen
                //     .modify(|_, w| w.ch4_en().set_bit().ch4_en_we().set_bit());
            }
            CHANNEL4 => {
                self.dmac.chen.channel_n_enable(5);
                // self.dmac
                //     .chen
                //     .modify(|_, w| w.ch5_en().set_bit().ch5_en_we().set_bit());
            }
            CHANNEL5 => {
                self.dmac.chen.channel_n_enable(6);
                // self.dmac
                //     .chen
                //     .modify(|_, w| w.ch6_en().set_bit().ch6_en_we().set_bit());
            }
        }
    }

    /** Disable a DMA channel. */
    pub fn channel_disable(&self, channel_num: dma_channel) {
        use dma_channel::*;
        // Note: chX bit names start counting from 1, while channels start counting from 0
        match channel_num {
            CHANNEL0 => {
                self.dmac.chen.channel_n_disable(1);
                // self.dmac
                //     .chen
                //     .modify(|_, w| w.ch1_en().clear_bit().ch1_en_we().set_bit());
            }
            CHANNEL1 => {
                self.dmac.chen.channel_n_disable(2);
                // self.dmac
                //     .chen
                //     .modify(|_, w| w.ch2_en().clear_bit().ch2_en_we().set_bit());
            }
            CHANNEL2 => {
                self.dmac.chen.channel_n_disable(3);
                // self.dmac
                //     .chen
                //     .modify(|_, w| w.ch3_en().clear_bit().ch3_en_we().set_bit());
            }
            CHANNEL3 => {
                self.dmac.chen.channel_n_disable(4);
                // self.dmac
                //     .chen
                //     .modify(|_, w| w.ch4_en().clear_bit().ch4_en_we().set_bit());
            }
            CHANNEL4 => {
                self.dmac.chen.channel_n_disable(5);
                // self.dmac
                //     .chen
                //     .modify(|_, w| w.ch5_en().clear_bit().ch5_en_we().set_bit());
            }
            CHANNEL5 => {
                self.dmac.chen.channel_n_disable(6);
                // self.dmac
                //     .chen
                //     .modify(|_, w| w.ch6_en().clear_bit().ch6_en_we().set_bit());
            }
        }
    }

    /** Check if a DMA channel is busy. */
    pub fn check_channel_busy(&self, channel_num: dma_channel) -> bool {
        use dma_channel::*;
        match channel_num {
            CHANNEL0 => self.dmac.chen.read_ch_n_en(1),
            CHANNEL1 => self.dmac.chen.read_ch_n_en(2),
            CHANNEL2 => self.dmac.chen.read_ch_n_en(3),
            CHANNEL3 => self.dmac.chen.read_ch_n_en(4),
            CHANNEL4 => self.dmac.chen.read_ch_n_en(5),
            CHANNEL5 => self.dmac.chen.read_ch_n_en(6),
            // CHANNEL0 => self.dmac.chen.read().ch1_en().bit(),
            // CHANNEL1 => self.dmac.chen.read().ch2_en().bit(),
            // CHANNEL2 => self.dmac.chen.read().ch3_en().bit(),
            // CHANNEL3 => self.dmac.chen.read().ch4_en().bit(),
            // CHANNEL4 => self.dmac.chen.read().ch5_en().bit(),
            // CHANNEL5 => self.dmac.chen.read().ch6_en().bit(),
        }
        // Note: Kendryte SDK writes back the value after reading it,
        // is this necessary? It seems not.
    }

    pub fn set_list_master_select(
        &self,
        channel_num: dma_channel,
        sd_sel: src_dst_select,
        mst_num: master_number,
    ) -> Result<(), ()> {
        if !self.check_channel_busy(channel_num) {
            use src_dst_select::*;
            if sd_sel == SRC || sd_sel == SRC_DST {
                self.dmac.channel[channel_num.idx()].ctl.write_sms(mst_num);
            }
            if sd_sel == DST || sd_sel == SRC_DST {
                self.dmac.channel[channel_num.idx()].ctl.write_dms(mst_num);
            }
            // self.dmac.channel[channel_num.idx()].ctl.modify(|_, w| {
            //     let w = if sd_sel == SRC || sd_sel == SRC_DST {
            //         w.sms().variant(mst_num)
            //     } else {
            //         w
            //     };
            //     if sd_sel == DST || sd_sel == SRC_DST {
            //         w.dms().variant(mst_num)
            //     } else {
            //         w
            //     }
            // });
            // Note: there's some weird tmp|= line here in the Kendryte SDK
            // with the result going unused. I've decided to leave this out
            // because I assume it's another C UB workaround.
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn enable_common_interrupt_status(&self) {
        self.dmac
            .com_intstatus_en
            .write_slvif_dec_err(true)
            .write_slvif_wr2ro_err(true)
            .write_slvif_rd2wo_err(true)
            .write_slvif_wronhold_err(true)
            .write_slvif_undefinedreg_dec_err(true);
        // self.dmac.com_intstatus_en.modify(|_, w| {
        //     w.slvif_dec_err()
        //         .set_bit()
        //         .slvif_wr2ro_err()
        //         .set_bit()
        //         .slvif_rd2wo_err()
        //         .set_bit()
        //         .slvif_wronhold_err()
        //         .set_bit()
        //         .slvif_undefinedreg_dec_err()
        //         .set_bit()
        // });
    }

    pub fn enable_common_interrupt_signal(&self) {
        self.dmac
            .com_intsignal_en
            .write_slvif_dec_err(true)
            .write_slvif_wr2ro_err(true)
            .write_slvif_rd2wo_err(true)
            .write_slvif_wronhold_err(true)
            .write_slvif_undefinedreg_dec_err(true);
        // self.dmac.com_intsignal_en.modify(|_, w| {
        //     w.slvif_dec_err()
        //         .set_bit()
        //         .slvif_wr2ro_err()
        //         .set_bit()
        //         .slvif_rd2wo_err()
        //         .set_bit()
        //         .slvif_wronhold_err()
        //         .set_bit()
        //         .slvif_undefinedreg_dec_err()
        //         .set_bit()
        // });
    }

    fn enable_channel_interrupt(&self, channel_num: dma_channel) {
        unsafe {
            let ch = &self.dmac.channel[channel_num.idx()];
            ch.intclear.write(0xffffffff);
            ch.intstatus_en.write(0x2);
        }
    }

    pub fn disable_channel_interrupt(&self, channel_num: dma_channel) {
        unsafe {
            self.dmac.channel[channel_num.idx()].intstatus_en.write(0x0);
        }
    }

    fn channel_interrupt_clear(&self, channel_num: dma_channel) {
        unsafe {
            self.dmac.channel[channel_num.idx()]
                .intclear
                .write(0xffffffff);
        }
    }

    /** Set DMA channel parameters. */
    pub fn set_channel_param(
        &self,
        channel_num: dma_channel,
        src: u64,
        dest: u64,
        src_inc: address_increment,
        dest_inc: address_increment,
        burst_size: burst_length,
        trans_width: transfer_width,
        block_size: u32,
    ) {
        unsafe {
            let ch = &self.dmac.channel[channel_num.idx()];
            let src_is_mem = is_memory(src);
            let dest_is_mem = is_memory(dest);
            let flow_control = match (src_is_mem, dest_is_mem) {
                (false, false) => TT_FC_A::PRF2PRF_DMA,
                (true, false) => TT_FC_A::MEM2PRF_DMA,
                (false, true) => TT_FC_A::PRF2MEM_DMA,
                (true, true) => TT_FC_A::MEM2MEM_DMA,
            };

            /*
             * cfg register must configure before ts_block and
             * sar dar register
             */
            ch.cfg
                .write_tt_fc(flow_control)
                .write_hs_sel_src(if src_is_mem {
                    HS_SEL_SRC_A::SOFTWARE
                } else {
                    HS_SEL_SRC_A::HARDWARE
                })
                .write_hs_sel_dst(if dest_is_mem {
                    HS_SEL_SRC_A::SOFTWARE
                } else {
                    HS_SEL_SRC_A::HARDWARE
                })
                .write_src_per(channel_num as u8)
                .write_dst_per(channel_num as u8)
                .write_src_multblk_type(0)
                .write_dst_multblk_type(0);
            // ch.cfg.modify(|_, w| {
            //     w.tt_fc()
            //         .variant(flow_control)
            //         .hs_sel_src()
            //         .variant(if src_is_mem {
            //             HS_SEL_SRC_A::SOFTWARE
            //         } else {
            //             HS_SEL_SRC_A::HARDWARE
            //         })
            //         .hs_sel_dst()
            //         .variant(if dest_is_mem {
            //             HS_SEL_SRC_A::SOFTWARE
            //         } else {
            //             HS_SEL_SRC_A::HARDWARE
            //         })
            //         // Note: from SVD: "Assign a hardware handshaking interface to source of channel",
            //         // these are set using sysctl::dma_select; this configuration seems to indicate
            //         // that in principle, it's possible to use a different source and destination
            //         // handshaking interface for a channel, but that would sacrifice the interface of
            //         // another channel.
            //         .src_per()
            //         .bits(channel_num as u8)
            //         .dst_per()
            //         .bits(channel_num as u8)
            //         .src_multblk_type()
            //         .bits(0)
            //         .dst_multblk_type()
            //         .bits(0)
            // });

            ch.sar.write(src);
            ch.dar.write(dest);

            ch.ctl
                .write_sms(SMS_A::AXI_MASTER_1)
                .write_dms(SMS_A::AXI_MASTER_2)
                .write_sinc(src_inc)
                .write_dinc(dest_inc)
                .write_src_tr_width(trans_width)
                .write_dst_tr_width(trans_width)
                .write_src_msize(burst_size)
                .write_dst_msize(burst_size);
            // ch.ctl.modify(|_, w| {
            //     w.sms()
            //         .variant(SMS_A::AXI_MASTER_1)
            //         .dms()
            //         .variant(SMS_A::AXI_MASTER_2)
            //         /* master select */
            //         .sinc()
            //         .variant(src_inc)
            //         .dinc()
            //         .variant(dest_inc)
            //         /* address incrememt */
            //         .src_tr_width()
            //         .variant(trans_width)
            //         .dst_tr_width()
            //         .variant(trans_width)
            //         /* transfer width */
            //         .src_msize()
            //         .variant(burst_size)
            //         .dst_msize()
            //         .variant(burst_size)
            // });

            // ch.block_ts.write(|w| w.block_ts().bits(block_size - 1));
            ch.block_ts.write_block_ts(block_size - 1);
            /*the number of (blcok_ts +1) data of width SRC_TR_WIDTF to be */
            /* transferred in a dma block transfer */
        }
    }
    /** Initialize DMA controller */
    pub fn init(&self) {
        sysctl::clock_enable(sysctl::clock::DMA);

        /* reset dmac */
        // self.dmac.reset.modify(|_, w| w.rst().set_bit());
        self.dmac.reset.write_rst(true);
        while self.dmac.reset.read_rst() {
            // IDLE
        }

        /* clear common register interrupt */
        self.dmac
            .com_intclear
            .write_slvif_dec_err(true)
            .write_slvif_wr2ro_err(true)
            .write_slvif_rd2wo_err(true)
            .write_slvif_wronhold_err(true)
            .write_slvif_undefinedreg_dec_err(true);
        // self.dmac.com_intclear.modify(|_, w| {
        //     w.slvif_dec_err()
        //         .set_bit()
        //         .slvif_wr2ro_err()
        //         .set_bit()
        //         .slvif_rd2wo_err()
        //         .set_bit()
        //         .slvif_wronhold_err()
        //         .set_bit()
        //         .slvif_undefinedreg_dec_err()
        //         .set_bit()
        // });

        /* disable dmac and disable interrupt */
        self.dmac.cfg.write_dmac_en(false).write_int_en(false);
        // self.dmac
        //     .cfg
        //     .modify(|_, w| w.dmac_en().clear_bit().int_en().clear_bit());

        while self.dmac.cfg.read() != 0 {
            // IDLE
        }
        /* disable all channel before configure */
        /* Note: changed from the SDK code, which doesn't clear channel 4 and 5,
         * and doesn't set associated _we bits */
        self.dmac
            .chen
            .channel_n_disable(1)
            .channel_n_disable(2)
            .channel_n_disable(3)
            .channel_n_disable(4)
            .channel_n_disable(5)
            .channel_n_disable(6);
        // self.dmac.chen.modify(|_, w| {
        //     w.ch1_en()
        //         .clear_bit()
        //         .ch1_en_we()
        //         .set_bit()
        //         .ch2_en()
        //         .clear_bit()
        //         .ch2_en_we()
        //         .set_bit()
        //         .ch3_en()
        //         .clear_bit()
        //         .ch3_en_we()
        //         .set_bit()
        //         .ch4_en()
        //         .clear_bit()
        //         .ch4_en_we()
        //         .set_bit()
        //         .ch5_en()
        //         .clear_bit()
        //         .ch5_en_we()
        //         .set_bit()
        // });

        self.enable();
    }

    /** Start a single DMA transfer. */
    pub fn set_single_mode(
        &self,
        channel_num: dma_channel,
        src: u64,
        dest: u64,
        src_inc: address_increment,
        dest_inc: address_increment,
        burst_size: burst_length,
        trans_width: transfer_width,
        block_size: u32,
    ) {
        self.channel_interrupt_clear(channel_num);
        self.channel_disable(channel_num);
        self.wait_idle(channel_num);
        self.set_channel_param(
            channel_num,
            src,
            dest,
            src_inc,
            dest_inc,
            burst_size,
            trans_width,
            block_size,
        );
        self.enable();
        self.channel_enable(channel_num);
    }

    /** Wait for dmac work done. */
    pub fn wait_done(&self, channel_num: dma_channel) {
        self.wait_idle(channel_num);
    }

    /** Determine if a DMA channel is idle or not. */
    pub fn is_idle(&self, channel_num: dma_channel) -> bool {
        !self.check_channel_busy(channel_num)
    }

    /** Wait for a DMA channel to be idle. */
    pub fn wait_idle(&self, channel_num: dma_channel) {
        while !self.is_idle(channel_num) {}
        self.channel_interrupt_clear(channel_num); /* clear interrupt */
    }
}
