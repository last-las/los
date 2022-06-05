pub trait UartStandard {
    fn init(&self);
    fn read(&self) -> u8;
    fn write(&self, byte: u8);
    fn enable_recv_intr(&self);
}

pub mod ns16550a {
    use crate::standard::UartStandard;
    use user_lib::syscall::{dev_write_u8, dev_read_u8};

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

    pub struct Ns16550a;
    impl Ns16550a {
        fn write_reg(&self, reg: usize, byte: u8) {
            dev_write_u8(UART_BASE_ADDRESS + reg, byte).unwrap();
        }

        fn read_reg(&self, reg: usize) -> u8 {
            dev_read_u8(UART_BASE_ADDRESS + reg).unwrap() as u8
        }
    }


    impl UartStandard for Ns16550a {
        fn init(&self) {
            self.write_reg(REG_IER_OFFSET, 0x00); // disable interrupt
            // don't need to set rate thanks to bootloader
            self.write_reg(REG_LCR_OFFSET, 0x03); // 8 bits
            self.write_reg(REG_FCR_OFFSET, 0x07); // enable FIFO
            self.enable_recv_intr(); // enable receiver interrupt
        }

        fn read(&self) -> u8 {
            self.read_reg(REG_RHR_OFFSET)
        }

        fn write(&self, byte: u8) {
            self.write_reg(REG_THR_OFFSET, byte);
        }

        fn enable_recv_intr(&self) {
            self.write_reg(REG_IER_OFFSET, 0x01);
        }
    }
}

pub mod uarths {
    use crate::standard::UartStandard;
    use user_lib::syscall::{dev_write_u32, dev_read_u32};

    pub const UART_BASE_ADDRESS: usize = 0x3800_0000;
    pub const REG_TXDATA_OFFSET: usize = 0x00;
    pub const REG_RXDATA_OFFSET: usize = 0x04;
    pub const REG_TXCTRL_OFFSET: usize = 0x08;
    pub const REG_RXCTRL_OFFSET: usize = 0x0c;
    pub const REG_IE_OFFSET: usize = 0x10;
    pub const REG_IP_OFFSET: usize = 0x14;
    pub const REG_DIV_OFFSET: usize = 0x18;

    pub struct Uarths;
    impl Uarths {
        fn write_reg(&self, reg: usize, dword: u32) {
            dev_write_u32(UART_BASE_ADDRESS + reg, dword).unwrap();
        }

        fn read_reg(&self, reg: usize) -> u32 {
            dev_read_u32(UART_BASE_ADDRESS + reg).unwrap() as u32
        }
    }

    impl UartStandard for Uarths {
        fn init(&self) {
            // do nothing
        }

        fn read(&self) -> u8 {
            self.read_reg(REG_RXDATA_OFFSET) as u8
        }

        fn write(&self, byte: u8) { // this is stupid, use interrupt to handle this!
            loop {
                let txdata = self.read_reg(REG_TXDATA_OFFSET);
                if (txdata >> 31) & 0x01 == 1 { // fifo is full
                    continue;
                }
                self.write_reg(REG_TXDATA_OFFSET, byte as u32);
                break;
            }
        }

        fn enable_recv_intr(&self) {
            self.write_reg(REG_IE_OFFSET, 0x02);
        }
    }
}