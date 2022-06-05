use user_lib::syscall::{dev_read_u8, dev_write_u8};
use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::collections::VecDeque;
use share::terminal::Termios;
use crate::standard::UartStandard;
use crate::standard::ns16550a::Ns16550a;
use crate::standard::uarths::Uarths;

pub struct Uart {
    pub instance: Box<dyn UartStandard>,

    pub read_buffer: VecDeque<u8>,
    pub usr_buffer: Vec<u8>,
    pub in_caller: usize,
    pub in_proc: usize,
    pub in_left: usize,
    pub buf_ptr: usize,
    pub pgrp: Option<usize>,

    pub termios: Termios,
}

impl Uart {
    pub fn new() -> Self {
        #[cfg(feature = "board_qemu")]
            let instance: Box<dyn UartStandard> = Box::new(Ns16550a);
        #[cfg(feature = "board_k210")]
            let instance: Box<dyn UartStandard> = Box::new(Uarths);
        instance.init();
        Self {
            instance,
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
        self.instance.read()
    }

    pub fn dev_write(&self, byte: u8) {
        self.instance.write(byte)
    }

    pub fn enable_recv_intr(&self) {
        self.instance.enable_recv_intr()
    }
}