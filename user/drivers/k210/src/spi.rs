//! (TODO) Serial Peripheral Interface (SPI)
use crate::dmac::{address_increment, burst_length, transfer_width, DMAC};

// use crate::pac::SPI0;
use crate::sysctl::{self, dma_channel};

use self::spi::{ctrlr0, spi_ctrlr0, SPI0};
pub mod spi;

use core::convert::TryInto;

const CLK: sysctl::clock = sysctl::clock::SPI0;
const DIV: sysctl::threshold = sysctl::threshold::SPI0;
const DMA_RX: sysctl::dma_select = sysctl::dma_select::SSI0_RX_REQ;
const DMA_TX: sysctl::dma_select = sysctl::dma_select::SSI0_TX_REQ;

pub struct SPI {
    spi: SPI0,
}

impl SPI {
    pub fn new() -> Self {
        Self { spi: SPI0::new() }
    }
}

use core::convert::Into;
use core::marker::Copy;
use core::{assert, cmp};

pub use ctrlr0::FRAME_FORMAT_A as frame_format;
pub use ctrlr0::TMOD_A as tmod;
pub use ctrlr0::WORK_MODE_A as work_mode;
pub use spi_ctrlr0::AITM_A as aitm;

/** Trait for trunction of a SPI frame from u32 register to other unsigned integer types. */
pub trait TruncU32 {
    fn trunc(val: u32) -> Self;
}
impl TruncU32 for u32 {
    fn trunc(val: u32) -> u32 {
        return val;
    }
}
impl TruncU32 for u16 {
    fn trunc(val: u32) -> u16 {
        return (val & 0xffff) as u16;
    }
}
impl TruncU32 for u8 {
    fn trunc(val: u32) -> u8 {
        return (val & 0xff) as u8;
    }
}

impl SPI {
    /// Configure SPI transaction
    pub fn configure(
        &self,
        work_mode: work_mode,
        frame_format: frame_format,
        data_bit_length: u8,
        endian: u32,
        instruction_length: u8,
        address_length: u8,
        wait_cycles: u8,
        instruction_address_trans_mode: aitm,
        tmod: tmod,
    ) {
        assert!(data_bit_length >= 4 && data_bit_length <= 32);
        assert!(wait_cycles < (1 << 5));
        let inst_l: u8 = match instruction_length {
            0 => 0,
            4 => 1,
            8 => 2,
            16 => 3,
            _ => panic!("unhandled intruction length"),
        };

        assert!(address_length % 4 == 0 && address_length <= 60);
        let addr_l: u8 = address_length / 4;

        self.spi.imr.write(0x00);
        self.spi.dmacr.write(0x00);
        self.spi.dmatdlr.write(0x10);
        self.spi.dmardlr.write(0x00);
        self.spi.ser.write(0x00);
        self.spi.ssienr.write(0x00);

        self.spi
            .ctrlr0
            .write_work_mode(work_mode)
            .write_tmod(tmod)
            .write_frame_format(frame_format)
            .write_data_length(data_bit_length - 1);

        self.spi
            .spi_ctrlr0
            .write_aitm(instruction_address_trans_mode)
            .write_addr_length(addr_l)
            .write_inst_length(inst_l)
            .write_wait_cycle(wait_cycles);

        self.spi.endian.write(endian);
    }

    /// Set SPI clock rate
    pub fn set_clk_rate(&self, spi_clk: u32) -> u32 {
        sysctl::clock_enable(CLK);
        sysctl::clock_set_threshold(DIV, 0);
        let clock_freq: u32 = sysctl::clock_get_freq(sysctl::clock::SPI0);
        let spi_baudr = clock_freq / spi_clk;
        // Clamp baudrate divider to valid range
        //panic!("{} / {} = {}", clock_freq, spi_clk, spi_baudr);
        let spi_baudr = cmp::min(cmp::max(spi_baudr, 2), 65534);

        self.spi.baudr.write(spi_baudr);

        clock_freq / spi_baudr
    }

    /// Receive arbitrary data
    // make sure to set tmod to tmod::RECV
    pub fn recv_data<X: TruncU32>(&self, chip_select: u32, rx: &mut [X]) {
        if rx.len() == 0 {
            return;
        }

        self.spi.ctrlr1.write((rx.len() - 1).try_into().unwrap());
        self.spi.ssienr.write(0x01);
        self.spi.dr[0].write(0xffffffff);
        self.spi.ser.write(1 << chip_select);

        let mut fifo_len = 0;
        for val in rx.iter_mut() {
            while fifo_len == 0 {
                fifo_len = self.spi.rxflr.read();
            }
            *val = X::trunc(self.spi.dr[0].read());
            fifo_len -= 1;
        }

        self.spi.ser.write(0x00);
        self.spi.ssienr.write(0x00);
    }

    /// Receive 32-bit data using DMA.
    // make sure to set tmod to tmod::RECV
    pub fn recv_data_dma(
        &self,
        dmac: &DMAC,
        channel_num: dma_channel,
        chip_select: u32,
        rx: &mut [u32],
    ) {
        if rx.len() == 0 {
            return;
        }
        // self.spi
        //     .ctrlr1
        //     .write(|w| w.bits((rx.len() - 1).try_into().unwrap()));
        self.spi.ctrlr1.write((rx.len() - 1).try_into().unwrap());
        self.spi.ssienr.write(0x01);
        self.spi.dmacr.write(0x3); /*enable dma receive */

        sysctl::dma_select(channel_num, DMA_RX);
        dmac.set_single_mode(
            channel_num,
            self.spi.dr.as_ptr() as u64,
            rx.as_ptr() as u64,
            address_increment::NOCHANGE,
            address_increment::INCREMENT,
            burst_length::LENGTH_1,
            transfer_width::WIDTH_32,
            rx.len() as u32,
        );
        self.spi.dr[0].write(0xffffffff);
        self.spi.ser.write(1 << chip_select);
        dmac.wait_done(channel_num);

        self.spi.ser.write(0x00);
        self.spi.ssienr.write(0x00);
    }

    /// Send arbitrary data
    pub fn send_data<X: Into<u32> + Copy>(&self, chip_select: u32, tx: &[X]) {
        self.spi.ser.write(1 << chip_select);
        self.spi.ssienr.write(0x01);

        let mut fifo_len = 0;
        for &val in tx {
            while fifo_len == 0 {
                // fifo_len = 32 - self.spi.txflr.read().bits();
                fifo_len = 32 - self.spi.txflr.read();
            }
            self.spi.dr[0].write(val.into());
            fifo_len -= 1;
        }

        while (self.spi.sr.read() & 0x05) != 0x04 {
            // IDLE
        }
        self.spi.ser.write(0x00);
        self.spi.ssienr.write(0x00);
    }

    /// Send 32-bit data using DMA.
    /// If you want to use this function to send 8-bit or 16-bit data, you need to wrap each
    /// data unit in a 32-bit word.
    /// This is intentionally left to the caller: the difficulty here is to avoid the need for alloc/freeing()
    /// buffers every time as the SDK does because this is highly undesirable!
    pub fn send_data_dma(
        &self,
        dmac: &DMAC,
        channel_num: dma_channel,
        chip_select: u32,
        tx: &[u32],
    ) {
        self.spi.dmacr.write(0x2); /*enable dma transmit*/
        self.spi.ssienr.write(0x01);

        sysctl::dma_select(channel_num, DMA_TX);
        dmac.set_single_mode(
            channel_num,
            tx.as_ptr() as u64,
            self.spi.dr.as_ptr() as u64,
            address_increment::INCREMENT,
            address_increment::NOCHANGE,
            burst_length::LENGTH_4,
            transfer_width::WIDTH_32,
            tx.len() as u32,
        );
        self.spi.ser.write(1 << chip_select);
        dmac.wait_done(channel_num);

        while (self.spi.sr.read() & 0x05) != 0x04 {
            // IDLE
        }
        self.spi.ser.write(0x00);
        self.spi.ssienr.write(0x00);
    }

    /// Send repeated data
    pub fn fill_data(&self, chip_select: u32, value: u32, mut tx_len: usize) {
        self.spi.ser.write(1 << chip_select);
        self.spi.ssienr.write(0x01);

        while tx_len != 0 {
            let fifo_len = (32 - self.spi.txflr.read()) as usize;
            let fifo_len = cmp::min(fifo_len, tx_len);
            for _ in 0..fifo_len {
                self.spi.dr[0].write(value);
            }
            tx_len -= fifo_len;
        }

        while (self.spi.sr.read() & 0x05) != 0x04 {
            // IDLE
        }
        self.spi.ser.write(0x00);
        self.spi.ssienr.write(0x00);
    }

    /// Send repeated data (using DMA)
    pub fn fill_data_dma(
        &self,
        dmac: &DMAC,
        channel_num: dma_channel,
        chip_select: u32,
        value: u32,
        tx_len: usize,
    ) {
        self.spi.dmacr.write(0x2); /*enable dma transmit*/
        self.spi.ssienr.write(0x01);

        sysctl::dma_select(channel_num, DMA_TX);
        let val = [value];
        // simple trick to repeating a value: don't increment the source address
        dmac.set_single_mode(
            channel_num,
            val.as_ptr() as u64,
            self.spi.dr.as_ptr() as u64,
            address_increment::NOCHANGE,
            address_increment::NOCHANGE,
            burst_length::LENGTH_4,
            transfer_width::WIDTH_32,
            tx_len.try_into().unwrap(),
        );
        self.spi.ser.write(1 << chip_select);
        dmac.wait_done(channel_num);

        while (self.spi.sr.read() & 0x05) != 0x04 {
            // IDLE
        }
        self.spi.ser.write(0x00);
        self.spi.ssienr.write(0x00);
    }
}
