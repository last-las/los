#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod uart_16550;
mod tty;

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

use crate::uart_16550::{REG_THR_OFFSET, REG_RHR_OFFSET, read_reg};
use share::ipc::Msg;
use user_lib::syscall::receive;

pub const INTERRUPT: usize = 1;

#[no_mangle]
fn main() {
    uart_16550::init();

    loop {
        let mut message = Msg::empty();
        receive(-1, &mut message).unwrap();
        match message.mtype {
            INTERRUPT => {
                let chr = read_reg(REG_RHR_OFFSET);
                println!("{}", chr as char);
            },
            _ => {
                continue;
            }
        };
    }
}