use core::mem::size_of;

mod blk;
mod header;
mod queue;
mod hal;
mod volatile;

use hal::*;
pub use blk::VirtIOBlk;
pub use header::VirtIOHeader;

const PAGE_SIZE: usize = 0x1000;

/// The type returned by driver methods.
pub type Result<T = ()> = core::result::Result<T, Error>;

// pub struct Error {
//     kind: ErrorKind,
//     reason: &'static str,
// }

/// The error type of VirtIO drivers.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// The buffer is too small.
    BufferTooSmall,
    /// The device is not ready.
    NotReady,
    /// The queue is already in use.
    AlreadyUsed,
    /// Invalid parameter.
    InvalidParam,
    /// Failed to alloc DMA memory.
    DmaError,
    /// I/O Error
    IoError,
}

/// Align `size` up to a page.
fn align_up(size: usize) -> usize {
    (size + PAGE_SIZE) & !(PAGE_SIZE - 1)
}

/// Pages of `size`.
fn pages(size: usize) -> usize {
    (size + PAGE_SIZE - 1) / PAGE_SIZE
}

/// Convert a struct into buffer.
unsafe trait AsBuf: Sized {
    fn as_buf(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self as *const _ as _, size_of::<Self>()) }
    }
    fn as_buf_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut _ as _, size_of::<Self>()) }
    }
}
