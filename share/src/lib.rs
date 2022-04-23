#![no_std]

pub mod ipc;
pub mod syscall;
pub mod util;
pub mod ffi;
pub mod terminal;
pub mod file;

extern crate alloc;
#[macro_use]
extern crate bitflags;