use core::ptr;
use spin::Mutex;
use crate::mm::address::{PhysicalAddress, PhysicalPageNum};
use crate::config::FRAME_SIZE;

pub fn alloc_frame() -> Option<FrameTracker> {
    FRAME_ALLOCATOR.lock().alloc()
}

pub struct FrameTracker(pub PhysicalPageNum);

impl FrameTracker {
    pub fn new(ppn: PhysicalPageNum) -> Self {
        Self {
            0: ppn,
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            core::ptr::write_bytes(PhysicalAddress::from(self.0).as_mut::<u8>(), 0, FRAME_SIZE);
        }
    }
}

lazy_static!{
    // update with an inner one.
    pub static ref FRAME_ALLOCATOR: Mutex<BitMapFrameAllocator> = Mutex::new(BitMapFrameAllocator::empty());
}

pub struct BitMapFrameAllocator {
    start_ppn: PhysicalPageNum,
    end_ppn: PhysicalPageNum,
    bitmap_len: usize,
}

impl BitMapFrameAllocator {
    pub fn empty() -> Self {
        Self {
            start_ppn: PhysicalPageNum::new(0),
            end_ppn: PhysicalPageNum::new(0),
            bitmap_len: 0,
        }
    }

    pub fn init(&mut self, start: PhysicalAddress, end: PhysicalAddress) {
        let size_in_bytes: usize = end.val() - start.val();

        self.start_ppn = start.into();
        self.bitmap_len =  (size_in_bytes - 1)  / FRAME_SIZE / 64 + 1;
        self.end_ppn = PhysicalPageNum::new(((end.val() + 1) >> 12) - 1);

        // println!("start {:#x}", self.start_ppn.0);
        // println!("end {:#x}", self.end_ppn.0);

        self.bitmap().fill_with(|| 0);

        for offset in 0..((size_in_bytes - 1) / FRAME_SIZE / (FRAME_SIZE * 8) + 1) {
            let map_index = offset / 64;
            let bit_index = offset % 64;
            assert_eq!((self.bitmap()[map_index] >> bit_index) & 1, 0);
            self.bitmap()[map_index] |= (1 << bit_index);
        }
    }

    pub fn alloc(&mut self) -> Option<FrameTracker> {
        if let Some((index, entry)) =
        self.bitmap().iter_mut().enumerate().find(|(_, e)| **e != u64::MAX) {
            let bit_index = entry.trailing_ones() as usize;

            let ppn = PhysicalPageNum::new(self.start_ppn.0  + index * 64 + bit_index);
            if ppn > self.end_ppn {
                return None;
            }

            assert_eq!((*entry >> bit_index) & 1, 0);
            *entry |= (1 << bit_index);
            return Some(FrameTracker::new(ppn));
        }

        None
    }

    pub fn dealloc(&mut self, ppn: PhysicalPageNum) {
        assert!(ppn.0 > self.start_ppn.0 && ppn.0 <= self.end_ppn.0);
        let map_index = (ppn.0 - self.start_ppn.0) / 64;
        let bit_index = (ppn.0 - self.start_ppn.0) % 64;

        // println!("map_index:{}, bit_index:{}", map_index, bit_index);
        assert_eq!((self.bitmap()[map_index] >> bit_index) & 1, 1);
        self.bitmap()[map_index] ^= (1 << bit_index);
    }

    pub fn bitmap(&self) -> &mut [u64] {
        unsafe {
            core::slice::from_raw_parts_mut(PhysicalAddress::from(self.start_ppn).as_mut(), self.bitmap_len,)
        }
    }
}
