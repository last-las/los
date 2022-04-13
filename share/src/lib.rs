#![no_std]

pub mod ipc;
pub mod syscall;
pub mod util;
pub mod ffi;
pub mod terminal;

extern crate alloc;
#[macro_use]
extern crate bitflags;