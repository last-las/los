use riscv::register::sie;
use crate::mm::address::PhysicalAddress;

const UART_IRQ: u32 = 0x10;

// https://osblog.stephenmarz.com/ch5.html
pub const PLIC_START_ADDRESS: usize = 0xc00_0000;
const PLIC_PRIORITY: usize = 0xc00_0000;
const PLIC_PENDING: usize = 0xc00_1000;
const PLIC_ENABLE: usize = 0xc00_2000;
const PLIC_THRESHOLD: usize  = 0xc20_0000;
const PLIC_CLAIM: usize = 0xc20_0004;
const PLIC_COMPLETE: usize = 0xc20_0004;

pub fn enable_external_interrupt() {
    unsafe {
        sie::set_sext();
    }
}

pub fn init() {
    set_threshold(0);
    enable(UART_IRQ);
    set_priority(UART_IRQ, 1);
}

pub fn enable(id: u32) {
    let enable_reg: *mut u32 = PhysicalAddress::new(PLIC_ENABLE).as_raw_mut();
    let actual_id = 1 << id;
    unsafe {
        enable_reg.write_volatile(enable_reg.read_volatile() | actual_id);
    }
}

pub fn set_priority(id: u32, prio: u8) {
    let prio_reg:*mut u32 = PhysicalAddress::new(PLIC_PRIORITY).as_raw_mut();
    let actual_prio = prio as u32 & 7;
    unsafe {
        prio_reg.add(id as usize).write_volatile(actual_prio);
    }
}

pub fn set_threshold(tsh: u8) {
    let tsh_reg:*mut u32 = PhysicalAddress::new(PLIC_THRESHOLD).as_raw_mut();
    let actual_tsh = tsh & 7;
    unsafe {
        tsh_reg.write_volatile(actual_tsh as u32);
    }
}

pub fn next_interrupt_no() -> Option<u32> {
    let claim_reg: *mut u32 = PhysicalAddress::new(PLIC_CLAIM).as_raw_mut();
    let claim_no;
    unsafe {
        claim_no = claim_reg.read_volatile();
    }
    if claim_no == 0 {
        None
    }
    else {
        Some(claim_no)
    }
}

pub fn complete(id: u32) {
    let complete_reg: *mut u32 = PhysicalAddress::new(PLIC_CLAIM).as_raw_mut();
    unsafe {
        complete_reg.write_volatile(id);
    }
}