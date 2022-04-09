use user_lib::syscall::{dev_read_u8, dev_write_u8};

pub const UART_BASE_ADDRESS: usize = 0x1000_0000;
pub const REG_RHR_OFFSET: usize = 0;
pub const REG_THR_OFFSET: usize = 0;
pub const REG_IER_OFFSET: usize = 1;
pub const REG_IIR_OFFSET: usize = 2;
pub const REG_FCR_OFFSET: usize = 2;
pub const REG_LCR_OFFSET: usize = 3;
pub const REG_MCR_OFFSET: usize = 4;
pub const REG_LSR_OFFSET: usize = 5;
pub const REG_MSR_OFFSET: usize = 6;
pub const REG_SCR_OFFSET: usize = 7;

pub fn init() {
    write_reg(REG_IER_OFFSET, 0x00); // disable interrupt
    // don't need to set rate thanks to bootloader
    write_reg(REG_LCR_OFFSET, 0x03); // 8 bits
    write_reg(REG_FCR_OFFSET, 0x07); // enable FIFO
    write_reg(REG_IER_OFFSET, 0x01) // enable receiver interrupt
}

fn write_reg(reg: usize, byte: u8) {
    dev_write_u8(UART_BASE_ADDRESS + reg, byte).unwrap();
}

pub fn read_reg(reg: usize) -> u8 {
    dev_read_u8(UART_BASE_ADDRESS + reg).unwrap() as u8
}
