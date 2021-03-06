use crate::config::FRAME_SIZE;
use alloc::vec::Vec;
use crate::mm::{FrameTracker, alloc_continuous_frames};
use share::syscall::error::SysError;
use crate::mm::address::PhysicalAddress;

const KERNEL_STACK_SIZE: usize = 0x1000;
const GUARD_PAGE_SIZE: usize = 0x1000;

pub struct KernelStack {
    pub sp: PhysicalAddress,
    _frames: Vec<FrameTracker>
}

impl KernelStack {
    pub fn new() -> Result<Self, SysError> {
        let frame_num = (KERNEL_STACK_SIZE + GUARD_PAGE_SIZE) / FRAME_SIZE;
        let frames = alloc_continuous_frames(frame_num)?;
        let sp: PhysicalAddress = frames[frame_num-1].0.add(1).into();

        Ok(
            Self {
                sp,
                _frames: frames,
            }
        )
    }

    pub fn sp(&self) -> usize {
        self.sp.val()
    }
}