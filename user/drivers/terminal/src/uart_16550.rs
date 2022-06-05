#![allow(unused)]

use user_lib::syscall::{dev_read_u8, dev_write_u8};
use alloc::vec::Vec;
use alloc::collections::VecDeque;
use share::terminal::Termios;

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


pub struct Uart {
    /// When an interrupt occurs, input byte will be pushed into `read_buffer`.
    pub read_buffer: VecDeque<u8>,
    /// Bytes in `usr_buffer` will be copied to `in_proc` process.
    pub usr_buffer: Vec<u8>,
    /// Pid of the reading message sender.
    ///
    /// When `in_left` reduce to zero, driver sends a reply message to `in_caller` process.
    pub in_caller: usize,
    /// Pid of the process that owns the `buf_ptr`.
    ///
    /// When `in_left` reduce to zero, driver copies bytes from `usr_buffer` to `in_proc` process.
    pub in_proc: usize,
    /// Indicates how many bytes should be read before driver replies to `in_caller` process.
    pub in_left: usize,
    /// The buffer pointer in `in_proc` process space.
    pub buf_ptr: usize,
    pub pgrp: Option<usize>,

    pub termios: Termios,
}

impl Uart {
    pub fn new() -> Self {
        Self {
            read_buffer: VecDeque::new(),
            usr_buffer: Vec::new(),
            in_caller: 0,
            in_proc: 0,
            in_left: 0,
            buf_ptr: 0,
            pgrp: None,
            termios: Termios::default(),
        }
    }

    pub fn dev_read(&self) -> u8 {
        read_reg(REG_RHR_OFFSET)
    }

    pub fn dev_write(&self, byte: u8) {
        write_reg(REG_THR_OFFSET, byte);
    }
}

pub fn init() {
    write_reg(REG_IER_OFFSET, 0x00); // disable interrupt
    // don't need to set rate thanks to bootloader
    write_reg(REG_LCR_OFFSET, 0x03); // 8 bits
    write_reg(REG_FCR_OFFSET, 0x07); // enable FIFO
    write_reg(REG_IER_OFFSET, 0x01); // enable receiver interrupt
}

pub fn write_reg(reg: usize, byte: u8) {
    dev_write_u8(UART_BASE_ADDRESS + reg, byte).unwrap();
}

pub fn read_reg(reg: usize) -> u8 {
    dev_read_u8(UART_BASE_ADDRESS + reg).unwrap() as u8
}
