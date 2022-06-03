#![no_std]

pub mod ipc;
pub mod syscall;
pub mod ffi;
pub mod terminal;
pub mod file;
pub mod device;
pub mod mmap;

extern crate alloc;
#[macro_use]
extern crate bitflags;