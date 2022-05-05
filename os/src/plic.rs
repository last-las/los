#![allow(unused)]

use riscv::register::sie;
use crate::mm::address::PhysicalAddress;
use crate::syscall::notify;
use share::ipc::TERMINAL_PID;

// TODO-FUTURE: plic implement is only worked at single hart right now!!!

const UART_IRQ: u32 = 0x0a;

// I can't find official document about this part... so take a look at xv6-k210 project's memlayout.h
pub const PLIC_START_ADDRESS: usize = 0xc00_0000;
const PLIC_PRIORITY: usize = PLIC_START_ADDRESS + 0x0;
const PLIC_PENDING: usize = PLIC_START_ADDRESS + 0x1000;
const PLIC_M_ENABLE: usize = PLIC_START_ADDRESS + 0x2000;
const PLIC_S_ENABLE: usize = PLIC_START_ADDRESS + 0x2080;
const PLIC_M_THRESHOLD: usize = PLIC_START_ADDRESS + 0x20_0000;
const PLIC_S_THRESHOLD: usize = PLIC_START_ADDRESS + 0x20_1000;
const PLIC_M_CLAIM: usize = PLIC_START_ADDRESS + 0x20_0004;
const PLIC_S_CLAIM: usize = PLIC_START_ADDRESS + 0x20_1004;
const PLIC_M_COMPLETE: usize = PLIC_M_CLAIM;
const PLIC_S_COMPLETE: usize = PLIC_S_CLAIM;

pub fn enable_external_interrupt() {
    unsafe {
        sie::set_sext();
    }
}

pub fn init() {
    set_priority(UART_IRQ, 7);
    set_threshold(0);
    enable(UART_IRQ);
}

pub fn handle_interrupt() {
    if let Some(interrupt) = next_interrupt_number() {
        match interrupt {
            UART_IRQ => {
                notify(TERMINAL_PID).unwrap();
                disable_uart_interrupt();
            },
            _ => {
                panic!("Unknown external interrupt: {}", interrupt);
            },
        }
        complete(interrupt);
    }
}

pub fn set_priority(id: u32, prio: u8) {
    let prio_reg: *mut u32 = PhysicalAddress::new(PLIC_PRIORITY).as_raw_mut();
    let actual_prio = prio as u32 & 7;
    unsafe {
        prio_reg.add(id as usize).write_volatile(actual_prio);
    }
}

pub fn set_threshold(tsh: u8) {
    let tsh_reg: *mut u32 = PhysicalAddress::new(PLIC_S_THRESHOLD).as_raw_mut();
    let actual_tsh = tsh & 7;
    unsafe {
        tsh_reg.write_volatile(actual_tsh as u32);
    }
}

pub fn enable(id: u32) {
    let enable_reg: *mut u32 = PhysicalAddress::new(PLIC_S_ENABLE).as_raw_mut();
    let actual_id = 1 << id;
    unsafe {
        enable_reg.write_volatile(enable_reg.read_volatile() | actual_id);
    }
}


pub fn next_interrupt_number() -> Option<u32> {
    let claim_reg: *mut u32 = PhysicalAddress::new(PLIC_S_CLAIM).as_raw_mut();
    let claim_number;
    unsafe {
        claim_number = claim_reg.read_volatile();
    }
    if claim_number == 0 {
        None
    } else {
        Some(claim_number)
    }
}

pub fn complete(id: u32) {
    let complete_reg: *mut u32 = PhysicalAddress::new(PLIC_S_COMPLETE).as_raw_mut();
    unsafe {
        complete_reg.write_volatile(id);
    }
}

fn disable_uart_interrupt() {
    const UART_BASE_ADDRESS: usize = 0x1000_0000;
    const REG_IER_OFFSET: usize = 1;
    let pa = PhysicalAddress::new(UART_BASE_ADDRESS + REG_IER_OFFSET);
    let byte: *mut u8 = pa.as_raw_mut();
    unsafe {
        byte.write_volatile(0);
    }
}
