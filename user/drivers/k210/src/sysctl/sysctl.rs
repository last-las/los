use self::{
    clk_en_cent::CLK_EN_CENT, clk_en_peri::CLK_EN_PERI, clk_sel0::CLK_SEL0, clk_sel1::CLK_SEL1,
    clk_th0::CLK_TH0, clk_th1::CLK_TH1, clk_th2::CLK_TH2, clk_th3::CLK_TH3, clk_th4::CLK_TH4,
    clk_th5::CLK_TH5, clk_th6::CLK_TH6, dma_sel0::DMA_SEL0, dma_sel1::DMA_SEL1, misc::MISC,
    peri_reset::PERI_RESET, pll0::PLL0, pll1::PLL1, pll2::PLL2, pll_lock::PLL_LOCK,
    power_sel::POWER_SEL, soft_reset::SOFT_RESET,
};

pub struct sysctl {
    #[doc = "0x00 - Git short commit id"]
    // pub git_id: GIT_ID,
    #[doc = "0x04 - System clock base frequency"]
    // pub clk_freq: CLK_FREQ,
    #[doc = "0x08 - PLL0 controller"]
    pub pll0: PLL0,
    #[doc = "0x0c - PLL1 controller"]
    pub pll1: PLL1,
    #[doc = "0x10 - PLL2 controller"]
    pub pll2: PLL2,

    #[doc = "0x18 - PLL lock tester"]
    pub pll_lock: PLL_LOCK,
    #[doc = "0x1c - AXI ROM detector"]
    // pub rom_error: ROM_ERROR,
    #[doc = "0x20 - Clock select controller 0"]
    pub clk_sel0: CLK_SEL0,
    #[doc = "0x24 - Clock select controller 1"]
    pub clk_sel1: CLK_SEL1,
    #[doc = "0x28 - Central clock enable"]
    pub clk_en_cent: CLK_EN_CENT,
    #[doc = "0x2c - Peripheral clock enable"]
    pub clk_en_peri: CLK_EN_PERI,
    #[doc = "0x30 - Soft reset ctrl"]
    pub soft_reset: SOFT_RESET,
    #[doc = "0x34 - Peripheral reset controller"]
    pub peri_reset: PERI_RESET,
    #[doc = "0x38 - Clock threshold controller 0"]
    pub clk_th0: CLK_TH0,
    #[doc = "0x3c - Clock threshold controller 1"]
    pub clk_th1: CLK_TH1,
    #[doc = "0x40 - Clock threshold controller 2"]
    pub clk_th2: CLK_TH2,
    #[doc = "0x44 - Clock threshold controller 3"]
    pub clk_th3: CLK_TH3,
    #[doc = "0x48 - Clock threshold controller 4"]
    pub clk_th4: CLK_TH4,
    #[doc = "0x4c - Clock threshold controller 5"]
    pub clk_th5: CLK_TH5,
    #[doc = "0x50 - Clock threshold controller 6"]
    pub clk_th6: CLK_TH6,
    #[doc = "0x54 - Miscellaneous controller"]
    pub misc: MISC,
    #[doc = "0x58 - Peripheral controller"]
    // pub peri: PERI,
    #[doc = "0x5c - SPI sleep controller"]
    // pub spi_sleep: SPI_SLEEP,
    #[doc = "0x60 - Reset source status"]
    // pub reset_status: RESET_STATUS,
    #[doc = "0x64 - DMA handshake selector"]
    pub dma_sel0: DMA_SEL0,
    #[doc = "0x68 - DMA handshake selector"]
    pub dma_sel1: DMA_SEL1,
    #[doc = "0x6c - IO Power Mode Select controller"]
    pub power_sel: POWER_SEL,
}
const SYSCTL_ADDRESS: usize = 0x5044_0000;

pub mod dma_sel0 {
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SYSCTL_ADDRESS;

    const REG: usize = 0x64;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    #[doc = "\n\nValue on reset: 0"]
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(u8)]
    pub enum DMA_SEL0_A {
        #[doc = "0: `0`"]
        SSI0_RX_REQ = 0,
        #[doc = "1: `1`"]
        SSI0_TX_REQ = 1,
        #[doc = "2: `10`"]
        SSI1_RX_REQ = 2,
        #[doc = "3: `11`"]
        SSI1_TX_REQ = 3,
        #[doc = "4: `100`"]
        SSI2_RX_REQ = 4,
        #[doc = "5: `101`"]
        SSI2_TX_REQ = 5,
        #[doc = "6: `110`"]
        SSI3_RX_REQ = 6,
        #[doc = "7: `111`"]
        SSI3_TX_REQ = 7,
        #[doc = "8: `1000`"]
        I2C0_RX_REQ = 8,
        #[doc = "9: `1001`"]
        I2C0_TX_REQ = 9,
        #[doc = "10: `1010`"]
        I2C1_RX_REQ = 10,
        #[doc = "11: `1011`"]
        I2C1_TX_REQ = 11,
        #[doc = "12: `1100`"]
        I2C2_RX_REQ = 12,
        #[doc = "13: `1101`"]
        I2C2_TX_REQ = 13,
        #[doc = "14: `1110`"]
        UART1_RX_REQ = 14,
        #[doc = "15: `1111`"]
        UART1_TX_REQ = 15,
        #[doc = "16: `10000`"]
        UART2_RX_REQ = 16,
        #[doc = "17: `10001`"]
        UART2_TX_REQ = 17,
        #[doc = "18: `10010`"]
        UART3_RX_REQ = 18,
        #[doc = "19: `10011`"]
        UART3_TX_REQ = 19,
        #[doc = "20: `10100`"]
        AES_REQ = 20,
        #[doc = "21: `10101`"]
        SHA_RX_REQ = 21,
        #[doc = "22: `10110`"]
        AI_RX_REQ = 22,
        #[doc = "23: `10111`"]
        FFT_RX_REQ = 23,
        #[doc = "24: `11000`"]
        FFT_TX_REQ = 24,
        #[doc = "25: `11001`"]
        I2S0_TX_REQ = 25,
        #[doc = "26: `11010`"]
        I2S0_RX_REQ = 26,
        #[doc = "27: `11011`"]
        I2S1_TX_REQ = 27,
        #[doc = "28: `11100`"]
        I2S1_RX_REQ = 28,
        #[doc = "29: `11101`"]
        I2S2_TX_REQ = 29,
        #[doc = "30: `11110`"]
        I2S2_RX_REQ = 30,
        #[doc = "31: `11111`"]
        I2S0_BF_DIR_REQ = 31,
        #[doc = "32: `100000`"]
        I2S0_BF_VOICE_REQ = 32,
    }
    impl From<DMA_SEL0_A> for u8 {
        #[inline(always)]
        fn from(variant: DMA_SEL0_A) -> Self {
            variant as _
        }
    }

    pub type DMA_SEL1_A = DMA_SEL0_A;
    pub type DMA_SEL2_A = DMA_SEL0_A;
    pub type DMA_SEL3_A = DMA_SEL0_A;
    pub type DMA_SEL4_A = DMA_SEL0_A;
    pub type DMA_SEL5_A = DMA_SEL0_A;

    pub struct DMA_SEL0 {}

    fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }
    fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_dma_sel0(variant: DMA_SEL0_A) {
        let v = read();
        let value: u8 = variant.into();
        let v = (v & !0x3f) | ((value as u32) & 0x3f);
        write(v);
    }
    pub fn write_dma_sel1(variant: DMA_SEL1_A) {
        let v = read();
        let value: u8 = variant.into();
        let v = (v & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        write(v);
    }
    pub fn write_dma_sel2(variant: DMA_SEL2_A) {
        let v = read();
        let value: u8 = variant.into();
        let v = (v & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        write(v);
    }
    pub fn write_dma_sel3(variant: DMA_SEL3_A) {
        let v = read();
        let value: u8 = variant.into();
        let v = (v & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        write(v);
    }
    pub fn write_dma_sel4(variant: DMA_SEL4_A) {
        let v = read();
        let value: u8 = variant.into();
        let v = (v & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        write(v);
    }
}

pub mod dma_sel1 {
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SYSCTL_ADDRESS;

    const REG: usize = 0x6c;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub type DMA_SEL5_A = super::dma_sel0::DMA_SEL0_A;

    pub struct DMA_SEL1 {}

    fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }
    fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_dma_sel5(variant: DMA_SEL5_A) {
        let v = read();
        let value: u8 = variant.into();
        let v = (v & !0x3f) | ((value as u32) & 0x3f);
        write(v);
    }
}

pub mod clk_en_peri {
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SYSCTL_ADDRESS;

    const REG: usize = 0x2c;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct CLK_EN_PERI {}

    fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }
    fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_rom_clk_en(value: bool) {
        let v = read();
        let v = (v & !0x01) | ((value as u32) & 0x01);
        write(v);
    }
    pub fn write_dma_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        write(v);
    }
    pub fn write_ai_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        write(v);
    }
    pub fn write_dvp_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        write(v);
    }
    pub fn write_fft_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        write(v);
    }
    pub fn write_gpio_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        write(v);
    }

    pub fn write_spi0_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        write(v);
    }

    pub fn write_spi1_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        write(v);
    }

    pub fn write_spi2_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        write(v);
    }

    pub fn write_spi3_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        write(v);
    }

    pub fn write_i2s0_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        write(v);
    }

    pub fn write_i2s1_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        write(v);
    }

    pub fn write_i2s2_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        write(v);
    }
    pub fn write_i2c0_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        write(v);
    }
    pub fn write_i2c1_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        write(v);
    }
    pub fn write_i2c2_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        write(v);
    }
    pub fn write_uart1_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        write(v);
    }
    pub fn write_uart2_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        write(v);
    }
    pub fn write_uart3_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        write(v);
    }
    pub fn write_aes_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        write(v);
    }
    pub fn write_fpioa_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        write(v);
    }
    pub fn write_timer0_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        write(v);
    }
    pub fn write_timer1_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        write(v);
    }
    pub fn write_timer2_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        write(v);
    }
    pub fn write_wdt0_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        write(v);
    }
    pub fn write_wdt1_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        write(v);
    }
    pub fn write_sha_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        write(v);
    }
    pub fn write_otp_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        write(v);
    }
    pub fn write_rtc_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        write(v);
    }
}

pub mod clk_en_cent {
    use user_lib::syscall::{dev_read_u32, dev_write_u32};

    use super::SYSCTL_ADDRESS;

    const REG: usize = 0x28;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct CLK_EN_CENT {}

    fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }
    fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_apb0_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        write(v);
    }

    pub fn write_apb1_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        write(v);
    }

    pub fn write_apb2_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        write(v);
    }

    pub fn write_cpu_clk_en(value: bool) {
        let v = read();
        let v = (v & !0x01) | ((value as u32) & 0x01);
        write(v);
    }

    pub fn write_sram0_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        write(v);
    }

    pub fn write_sram1_clk_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        write(v);
    }
}

pub mod soft_reset {
    use user_lib::syscall::*;

    use super::SYSCTL_ADDRESS;
    const REG: usize = 0x30;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct SOFT_RESET {}

    fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }
    fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_soft_rest(value: bool) {
        let v = read();
        let v = (v & !0x01) | ((value as u32) & 0x01);
        write(v);
    }
}

pub mod peri_reset {
    use user_lib::syscall::*;

    use super::SYSCTL_ADDRESS;
    const REG: usize = 0x30;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct PERI_RESET {}

    fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }
    fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_rom_rest(value: bool) {
        let v = read();
        let v = (v & !0x01) | ((value as u32) & 0x01);
        write(v);
    }

    pub fn write_dma_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        write(v);
    }

    pub fn write_ai_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        write(v);
    }
    pub fn write_dvp_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        write(v);
    }
    pub fn write_fft_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        write(v);
    }
    pub fn write_gpio_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        write(v);
    }
    pub fn write_spi0_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        write(v);
    }
    pub fn write_spi1_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        write(v);
    }
    pub fn write_spi2_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        write(v);
    }
    pub fn write_spi3_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        write(v);
    }
    pub fn write_i2s0_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        write(v);
    }
    pub fn write_i2s1_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        write(v);
    }
    pub fn write_i2s2_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        write(v);
    }
    pub fn write_i2c0_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        write(v);
    }
    pub fn write_i2c1_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        write(v);
    }
    pub fn write_i2c2_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        write(v);
    }
    pub fn write_uart1_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        write(v);
    }
    pub fn write_uart2_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        write(v);
    }
    pub fn write_uart3_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        write(v);
    }
    pub fn write_aes_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        write(v);
    }
    pub fn write_fpio_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        write(v);
    }
    pub fn write_timer0_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        write(v);
    }
    pub fn write_timer1_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        write(v);
    }
    pub fn write_timer2_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        write(v);
    }
    pub fn write_wdt0_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        write(v);
    }
    pub fn write_wdt1_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        write(v);
    }
    pub fn write_sha_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        write(v);
    }
    pub fn write_rtc_rest(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        write(v);
    }
}

pub mod pll0 {
    use super::SYSCTL_ADDRESS;
    use user_lib::syscall::*;

    const REG: usize = 0x08;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct PLL0 {}

    pub fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }

    pub fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_out_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        write(v);
    }

    pub fn write_pwrd(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        write(v);
    }

    pub fn write_reset(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        write(v);
    }
    pub fn write_clkr(value: u8) {
        let v = read();
        let v = (v & !0x0f) | ((value as u32) & 0x0f);
        write(v);
    }
    pub fn read_clkr() -> u8 {
        let v = read();
        (v & 0x0f) as u8
    }

    pub fn write_clkf(value: u8) {
        let v = read();
        let v = (v & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        write(v);
    }
    pub fn read_clkf() -> u8 {
        let v = read();
        ((v >> 4) & 0x3f) as u8
    }

    pub fn write_clkod(value: u8) {
        let v = read();
        let v = (v & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        write(v);
    }

    pub fn read_clkod() -> u8 {
        let v = read();
        ((v >> 10) & 0x0f) as u8
    }

    pub fn write_bwadj(value: u8) {
        let v = read();
        let v = (v & !(0x3f << 14)) | (((value as u32) & 0x3f) << 14);
        write(v);
    }

    pub fn write_bypass(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        write(v);
    }

    pub fn read_bypass() -> bool {
        let v = read();
        ((v >> 23) & 0x01) != 0
    }
}

pub mod pll1 {
    use super::SYSCTL_ADDRESS;
    use user_lib::syscall::*;

    const REG: usize = 0x0c;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct PLL1 {}

    pub fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }

    pub fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_out_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        write(v);
    }

    pub fn write_pwrd(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        write(v);
    }

    pub fn write_reset(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        write(v);
    }
    pub fn write_clkr(value: u8) {
        let v = read();
        let v = (v & !0x0f) | ((value as u32) & 0x0f);
        write(v);
    }
    pub fn read_clkr() -> u8 {
        let v = read();
        (v & 0x0f) as u8
    }

    pub fn write_clkf(value: u8) {
        let v = read();
        let v = (v & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        write(v);
    }
    pub fn read_clkf() -> u8 {
        let v = read();
        ((v >> 4) & 0x3f) as u8
    }

    pub fn write_clkod(value: u8) {
        let v = read();
        let v = (v & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        write(v);
    }

    pub fn read_clkod() -> u8 {
        let v = read();
        ((v >> 10) & 0x0f) as u8
    }

    pub fn write_bwadj(value: u8) {
        let v = read();
        let v = (v & !(0x3f << 14)) | (((value as u32) & 0x3f) << 14);
        write(v);
    }

    pub fn write_bypass(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        write(v);
    }
    pub fn read_bypass() -> bool {
        let v = read();
        ((v >> 23) & 0x01) != 0
    }
}

pub mod pll2 {
    use super::SYSCTL_ADDRESS;
    use user_lib::syscall::*;

    const REG: usize = 0x10;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct PLL2 {}
    pub fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }

    pub fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_out_en(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        write(v);
    }

    pub fn write_pwrd(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        write(v);
    }

    pub fn write_reset(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        write(v);
    }
    pub fn write_clkr(value: u8) {
        let v = read();
        let v = (v & !0x0f) | ((value as u32) & 0x0f);
        write(v);
    }
    pub fn read_clkr() -> u8 {
        let v = read();
        (v & 0x0f) as u8
    }

    pub fn write_clkf(value: u8) {
        let v = read();
        let v = (v & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        write(v);
    }
    pub fn read_clkf() -> u8 {
        let v = read();
        ((v >> 4) & 0x3f) as u8
    }

    pub fn write_clkod(value: u8) {
        let v = read();
        let v = (v & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        write(v);
    }

    pub fn read_clkod() -> u8 {
        let v = read();
        ((v >> 10) & 0x0f) as u8
    }

    pub fn write_bwadj(value: u8) {
        let v = read();
        let v = (v & !(0x3f << 14)) | (((value as u32) & 0x3f) << 14);
        write(v);
    }

    pub fn write_bypass(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        write(v);
    }
    pub fn read_bypass() -> bool {
        let v = read();
        ((v >> 23) & 0x01) != 0
    }

    pub fn write_ckin_sel(value: u8) {
        let v = read();
        let v = (v & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        write(v);
    }

    pub fn read_ckin_sel() -> u8 {
        let v = read();
        ((v >> 26) & 0x03) as u8
    }
}

pub mod pll_lock {
    use user_lib::syscall::*;

    use super::SYSCTL_ADDRESS;
    const REG: usize = 0x18;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct PLL_LOCK {}

    fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }
    fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_pll_lock0(value: u8) {
        let v = read();
        let v = (v & !0x03) | ((value as u32) & 0x03);
        write(v);
    }

    pub fn read_pll_lock0() -> u8 {
        let v = read();
        (v & 0x03) as u8
    }
    pub fn read_pll_lock1() -> u8 {
        let v = read();
        ((v >> 8) & 0x03) as u8
    }
    pub fn read_pll_lock2() -> u8 {
        let v = read();
        ((v >> 16) & 0x03) as u8
    }

    pub fn write_pll_slip_clear0(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        write(v);
    }
    pub fn write_pll_slip_clear1(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        write(v);
    }
    pub fn write_pll_slip_clear2(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        write(v);
    }
}

pub mod clk_sel0 {
    use user_lib::syscall::*;

    use super::SYSCTL_ADDRESS;
    const CLK_SEL0: usize = 0x20;

    pub struct CLK_SEL0 {}

    fn read() -> u32 {
        dev_read_u32(SYSCTL_ADDRESS + CLK_SEL0).unwrap() as u32
    }
    fn write(value: u32) {
        dev_write_u32(SYSCTL_ADDRESS + CLK_SEL0, value).unwrap();
    }
    pub fn write_aclk_sel(value: bool) {
        let v = read();
        let v = (v & !0x01) | ((value as u32) & 0x01);
        write(v);
    }
    pub fn read_aclk_sel() -> bool {
        let v = read();
        (v & 0x01) != 0
    }
    pub fn write_aclk_divider_sel(value: u8) {
        let v = read();
        let v = (v & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        write(v);
    }
    pub fn read_aclk_divider_sel() -> u8 {
        let v = read();
        ((v >> 1) & 0x03) as u8
    }
    pub fn write_apb0_clk_sel(value: u8) {
        let v = read();
        let v = (v & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        write(v);
    }
    pub fn read_apb0_clk_sel() -> u8 {
        let v = read();
        ((v >> 3) & 0x07) as u8
    }
    pub fn read_apb1_clk_sel() -> u8 {
        let v = read();
        ((v >> 6) & 0x07) as u8
    }
    pub fn read_apb2_clk_sel() -> u8 {
        let v = read();
        ((v >> 9) & 0x07) as u8
    }
    pub fn write_apb1_clk_sel(value: u8) {
        let v = read();
        let v = (v & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        write(v);
    }
    pub fn write_apb2_clk_sel(value: u8) {
        let v = read();
        let v = (v & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        write(v);
    }
    pub fn write_spi3_clk_sel(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        write(v);
    }
    pub fn read_spi3_clk_sel() -> bool {
        let v = read();
        ((v >> 12) & 0x01) != 0
    }
    pub fn write_timer0_clk_sel(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        write(v);
    }
    pub fn write_timer1_clk_sel(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        write(v);
    }
    pub fn write_timer2_clk_sel(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        write(v);
    }
    pub fn read_timer0_clk_sel() -> bool {
        let v = read();
        ((v >> 13) & 0x01) != 0
    }
    pub fn read_timer1_clk_sel() -> bool {
        let v = read();
        ((v >> 14) & 0x01) != 0
    }
    pub fn read_timer2_clk_sel() -> bool {
        let v = read();
        ((v >> 15) & 0x01) != 0
    }
}

pub mod clk_sel1 {
    use user_lib::syscall::*;

    use super::SYSCTL_ADDRESS;
    const CLK_SEL0: usize = 0x24;

    pub struct CLK_SEL1 {}

    fn read() -> u32 {
        dev_read_u32(SYSCTL_ADDRESS + CLK_SEL0).unwrap() as u32
    }
    fn write(value: u32) {
        dev_write_u32(SYSCTL_ADDRESS + CLK_SEL0, value).unwrap();
    }

    pub fn write_spi3_sample_clk_sel(value: bool) {
        let v = read();
        let v = (v & !0x01) | ((value as u32) & 0x01);
        write(v);
    }

    pub fn read_spi3_sample_clk_sel() -> bool {
        let v = read();
        (v & 0x01) != 0
    }
}

pub mod misc {
    use super::SYSCTL_ADDRESS;
    use user_lib::syscall::*;

    const REG: usize = 0x54;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct MISC {}

    pub fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }

    pub fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_spi_dvp_data_enbale(value: bool) {
        let v = read();
        let v = (v & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        write(v);
    }
}

pub mod power_sel {
    use super::SYSCTL_ADDRESS;
    use user_lib::syscall::*;

    const REG: usize = 0x6c;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct POWER_SEL {}

    pub fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }

    pub fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }
}

pub mod clk_th0 {
    use super::SYSCTL_ADDRESS;
    use user_lib::syscall::*;

    const REG: usize = 0x38;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct CLK_TH0 {}

    fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }

    fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_sram0_gclk(value: u8) {
        let v = read();
        let v = (v & !0x0f) | ((value as u32) & 0x0f);
        write(v);
    }
    pub fn read_sram0_gclk() -> u8 {
        let v = read();
        (v & 0x0f) as u8
    }

    pub fn write_sram1_gclk(value: u8) {
        let v = read();
        let v = (v & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        write(v);
    }
    pub fn read_sram1_gclk() -> u8 {
        let v = read();
        ((v >> 4) & 0x0f) as u8
    }

    pub fn write_ai_gclk(value: u8) {
        let v = read();
        let v = (v & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        write(v);
    }
    pub fn read_ai_gclk() -> u8 {
        let v = read();
        ((v >> 8) & 0x0f) as u8
    }

    pub fn write_dvp_gclk(value: u8) {
        let v = read();
        let v = (v & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        write(v);
    }
    pub fn read_dvp_gclk() -> u8 {
        let v = read();
        ((v >> 12) & 0x0f) as u8
    }

    pub fn write_rom_gclk(value: u8) {
        let v = read();
        let v = (v & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        write(v);
    }
    pub fn read_rom_gclk() -> u8 {
        let v = read();
        ((v >> 16) & 0x0f) as u8
    }
}

pub mod clk_th1 {
    use super::SYSCTL_ADDRESS;
    use user_lib::syscall::*;

    const REG: usize = 0x3c;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct CLK_TH1 {}

    fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }

    fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_spi0_clk(value: u8) {
        let v = read();
        let v = (v & !0xff) | ((value as u32) & 0xff);
        write(v);
    }
    pub fn read_spi0_clk() -> u8 {
        let v = read();
        (v & 0xff) as u8
    }

    pub fn write_spi1_clk(value: u8) {
        let v = read();
        let v = (v & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        write(v);
    }
    pub fn read_spi1_clk() -> u8 {
        let v = read();
        ((v >> 8) & 0xff) as u8
    }

    pub fn write_spi2_clk(value: u8) {
        let v = read();
        let v = (v & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        write(v);
    }
    pub fn read_spi2_clk() -> u8 {
        let v = read();
        ((v >> 16) & 0xff) as u8
    }

    pub fn write_spi3_clk(value: u8) {
        let v = read();
        let v = (v & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        write(v);
    }
    pub fn read_spi3_clk() -> u8 {
        let v = read();
        ((v >> 24) & 0xff) as u8
    }
}

pub mod clk_th2 {
    use super::SYSCTL_ADDRESS;
    use user_lib::syscall::*;

    const REG: usize = 0x40;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct CLK_TH2 {}

    fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }

    fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_timer0_clk(value: u8) {
        let v = read();
        let v = (v & !0xff) | ((value as u32) & 0xff);
        write(v);
    }
    pub fn read_timer0_clk() -> u8 {
        let v = read();
        (v & 0xff) as u8
    }

    pub fn write_timer1_clk(value: u8) {
        let v = read();
        let v = (v & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        write(v);
    }
    pub fn read_timer1_clk() -> u8 {
        let v = read();
        ((v >> 8) & 0xff) as u8
    }

    pub fn write_timer2_clk(value: u8) {
        let v = read();
        let v = (v & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        write(v);
    }
    pub fn read_timer2_clk() -> u8 {
        let v = read();
        ((v >> 16) & 0xff) as u8
    }
}

pub mod clk_th3 {
    use super::SYSCTL_ADDRESS;
    use user_lib::syscall::*;

    const REG: usize = 0x44;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct CLK_TH3 {}

    fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }

    fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_i2s0_clk(value: u16) {
        let v = read();
        let v = (v & !0xffff) | ((value as u32) & 0xffff);
        write(v);
    }
    pub fn read_i2s0_clk() -> u16 {
        let v = read();
        (v & 0xffff) as u16
    }

    pub fn write_i2s1_clk(value: u16) {
        let v = read();
        let v = (v & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        write(v);
    }
    pub fn read_i2s1_clk() -> u16 {
        let v = read();
        ((v >> 16) & 0xffff) as u16
    }
}

pub mod clk_th4 {
    use super::SYSCTL_ADDRESS;
    use user_lib::syscall::*;

    const REG: usize = 0x48;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct CLK_TH4 {}

    fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }

    fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_i2s2_clk(value: u16) {
        let v = read();
        let v = (v & !0xffff) | ((value as u32) & 0xffff);
        write(v);
    }
    pub fn read_i2s2_clk() -> u16 {
        let v = read();
        (v & 0xffff) as u16
    }

    pub fn write_i2s0_mclk(value: u8) {
        let v = read();
        let v = (v & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        write(v);
    }
    pub fn read_i2s0_mclk() -> u8 {
        let v = read();
        ((v >> 16) & 0xff) as u8
    }

    pub fn write_i2s1_mclk(value: u8) {
        let v = read();
        let v = (v & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        write(v);
    }
    pub fn read_i2s1_mclk() -> u8 {
        let v = read();
        ((v >> 24) & 0xff) as u8
    }
}

pub mod clk_th5 {
    use super::SYSCTL_ADDRESS;
    use user_lib::syscall::*;

    const REG: usize = 0x4c;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct CLK_TH5 {}

    fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }

    fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_i2s2_mclk(value: u8) {
        let v = read();
        let v = (v & !0xff) | ((value as u32) & 0xff);
        write(v);
    }
    pub fn read_i2s2_mclk() -> u8 {
        let v = read();
        (v & 0xff) as u8
    }

    pub fn write_i2c0_clk(value: u8) {
        let v = read();
        let v = (v & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        write(v);
    }
    pub fn read_i2c0_clk() -> u8 {
        let v = read();
        ((v >> 8) & 0xff) as u8
    }

    pub fn write_i2c1_clk(value: u8) {
        let v = read();
        let v = (v & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        write(v);
    }
    pub fn read_i2c1_clk() -> u8 {
        let v = read();
        ((v >> 16) & 0xff) as u8
    }

    pub fn write_i2c2_clk(value: u8) {
        let v = read();
        let v = (v & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        write(v);
    }
    pub fn read_i2c2_clk() -> u8 {
        let v = read();
        ((v >> 24) & 0xff) as u8
    }
}

pub mod clk_th6 {
    use super::SYSCTL_ADDRESS;
    use user_lib::syscall::*;

    const REG: usize = 0x50;
    const ADDRESS: usize = SYSCTL_ADDRESS + REG;

    pub struct CLK_TH6 {}

    pub fn read() -> u32 {
        dev_read_u32(ADDRESS).unwrap() as u32
    }

    pub fn write(value: u32) {
        dev_write_u32(ADDRESS, value).unwrap();
    }

    pub fn write_wdt0_clk(value: u8) {
        let v = read();
        let v = (v & !0xff) | ((value as u32) & 0xff);
        write(v);
    }
    pub fn read_wdt0_clk() -> u8 {
        let v = read();
        (v & 0xff) as u8
    }

    pub fn write_wdt1_clk(value: u8) {
        let v = read();
        let v = (v & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        write(v);
    }
    pub fn read_wdt1_clk() -> u8 {
        let v = read();
        ((v >> 8) & 0xff) as u8
    }
}
