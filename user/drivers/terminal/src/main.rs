#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod uart_16550;

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate lazy_static;

use crate::uart_16550::{REG_THR_OFFSET, REG_RHR_OFFSET, read_reg, Uart, write_reg, REG_IER_OFFSET};
use share::ipc::Msg;
use user_lib::syscall::{receive, dev_write_u8, virt_copy, send, getpid};
use share::ipc::*;
use share::terminal::{Clflag, TC_GET_ATTR, TC_SET_ATTR, TC_GET_PGRP, TC_SET_PGRP, Termios, Ciflag};
use core::mem::size_of;

const BS: u8 = 0x08;
const LF: u8 = 0x0a;
const CR: u8 = 0x0d;
const DL: u8 = 0x7f;
const CTRL_C: u8 = 0x3;

#[no_mangle]
fn main() {
    uart_16550::init();
    let mut uart = Uart::new();
    let mut message = Msg::empty();

    loop {
        receive(-1, &mut message).unwrap();

        let nr = message.args[DEVICE];
        assert_eq!(nr, 0);

        match message.mtype {
            INTERRUPT => do_interrupt(&mut uart),
            OPEN => do_open(&mut uart, message),
            READ => do_read(&mut uart, message),
            WRITE => do_write(&mut uart, message),
            IOCTL => do_ioctl(&mut uart, message),
            CLOSE => do_close(&mut uart, message),

            _ => {
                panic!("Unknown message type:{}", message.mtype);
            }
        }
    }
}

pub fn do_interrupt(uart: &mut Uart) {
    let mut byte = uart.dev_read();
    write_reg(REG_IER_OFFSET, 0x01);

    /* Map CR to LF, ignore CR, or map LF to CR. */
    if byte == CR {
        if uart.termios.c_iflag.contains(Ciflag::IGNCR) {
            return;
        }
        if uart.termios.c_iflag.contains(Ciflag::ICRNL) {
            byte = LF;
        }
    } else if byte == LF {
        if uart.termios.c_iflag.contains(Ciflag::INLCR) {
            byte = CR;
        }
    }

    if uart.termios.c_lflag.contains(Clflag::ECHO) {
        echo(uart, byte);
    }
    uart.read_buffer.push_back(byte);
    transfer_to_usr(uart);
}

pub fn do_open(uart: &mut Uart, message: Msg) {
}

pub fn do_read(uart: &mut Uart, message: Msg) {
    if uart.in_left > 0 {
        return;
    }
    uart.in_caller = message.src_pid;
    uart.in_proc = message.args[PROC_NR];
    uart.buf_ptr = message.args[BUFFER];
    uart.in_left = message.args[LENGTH];

    transfer_to_usr(uart);
}

pub fn do_write(uart: &mut Uart, message: Msg) {
    const BUFFER_SIZE: usize = 512;

    let proc_nr = message.args[PROC_NR];
    let mut buf_ptr = message.args[BUFFER];
    let mut buf_len = message.args[LENGTH];
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut cnt = 0;
    while buf_len != 0 {
        let length = BUFFER_SIZE.min(buf_len);
        virt_copy(proc_nr, buf_ptr, getpid(), buffer.as_ptr() as usize, length).unwrap();
        buf_ptr += length;
        buf_len -= length;
        cnt += length;

        for i in 0..length {
            uart.dev_write(buffer[i]);
        }
    }

    reply(message.src_pid, REPLY, proc_nr, cnt as isize);
}

pub fn do_ioctl(uart: &mut Uart, message: Msg) {
    let proc_nr = message.args[PROC_NR];
    let mut ret = STATUS_OK;

    match message.args[IOCTL_TYPE] {
        TC_GET_ATTR => {
            let src_ptr = &uart.termios as *const _ as usize;
            let size = size_of::<Termios>();
            let dst_ptr = message.args[ADDRESS];
            virt_copy(getpid(), src_ptr, proc_nr, dst_ptr, size).unwrap();
        },
        TC_SET_ATTR => {
            let termios_ptr = message.args[ADDRESS];
            let size = size_of::<Termios>();
            let dst_ptr = &mut uart.termios as *mut _ as usize;
            virt_copy(proc_nr, termios_ptr, getpid(), dst_ptr, size).unwrap();
        },
        TC_GET_PGRP => {

        },
        TC_SET_PGRP => {

        },
        _ => {

        }
    }

    reply(message.src_pid, REPLY, proc_nr, ret as isize);
}

pub fn do_close(uart: &mut Uart, message: Msg) {
}

fn echo(uart: &mut Uart, byte: u8) {
    match byte {
        DL => {
            uart.dev_write(BS);
            uart.dev_write(' ' as u8);
            uart.dev_write(BS);
        },
        _ => {
            uart.dev_write(byte);
        }
    }
}

fn transfer_to_usr(uart: &mut Uart) {
    if uart.in_left == 0 {
        return;
    }

    while !uart.read_buffer.is_empty() && uart.in_left != 0 {
        let byte = uart.read_buffer.pop_front().unwrap();
        match byte {
            LF => {
                if uart.termios.c_lflag.contains(Clflag::ICANON) {
                    uart.in_left = 0;
                } else {
                    uart.in_left -= 1;
                }
            }
            _ => {
                uart.in_left -= 1;
            }
        }
        uart.usr_buffer.push(byte);
    }

    if uart.in_left == 0 {
        let buffer = uart.usr_buffer.as_slice();
        let buffer_ptr = buffer.as_ptr() as usize;
        let length = buffer.len();
        virt_copy(getpid(), buffer_ptr, uart.in_proc, uart.buf_ptr, length);
        uart.usr_buffer.clear();
        reply(uart.in_caller, REPLY, uart.in_proc, length as isize);
    }
}

fn reply(caller: usize, mtype: usize, proc_nr: usize, status: isize) {
    let mut message = Msg::empty();
    message.mtype = mtype;
    message.args[REPLY_PROC_NR] = proc_nr;
    message.args[REPLY_STATUS] = status as usize;

    send(caller, &message).unwrap();
}