#![no_std]

pub mod ipc;
pub mod syscall;
pub mod util;
pub mod ffi;

extern crate alloc;
#[macro_use]
extern crate bitflags;