#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod uart_16550;
mod tty;

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate alloc;

use crate::uart_16550::{REG_THR_OFFSET, REG_RHR_OFFSET};

#[no_mangle]
fn main() {
    uart_16550::init();
    loop {
        // let chr = uart_16550::read_reg(REG_THR_OFFSET);
        // uart_16550::write_reg(REG_RHR_OFFSET, chr);
    }
}