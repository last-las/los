use core::ptr;
use spin::Mutex;
use crate::mm::address::{PhysicalAddress, PhysicalPageNum};
use crate::config::FRAME_SIZE;

/* TODO-FUTURE: For each frame, implement frame descriptor which indicates whether the frame is
    mapped to a file,or it should be writen back to the file system when deallocating, etc.
    So that we can implement page fault interrupt and mmap.

    struct FrameDescriptor {
        flags: u8,
        fs_handle: [a handle in the file system],
        offset: usize,
        ...
    }
*/

pub fn alloc_frame() -> Option<FrameTracker> {
    FRAME_ALLOCATOR.lock().alloc()
}

pub fn available_frame() -> usize {
    FRAME_ALLOCATOR.lock().available_frame()
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

    pub fn fill_with(&mut self, data: &[u8]) {
        let pa: PhysicalAddress = self.0.into();
        let byte_arr: &mut [u8;FRAME_SIZE] = pa.as_mut();
        let len = data.len();
        let (left, right) = byte_arr.split_at_mut(len);
        left.copy_from_slice(data);
        right.fill(0);
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        FRAME_ALLOCATOR.lock().dealloc(self.0)
    }
}

lazy_static!{
    // update with an inner one.
    pub static ref FRAME_ALLOCATOR: Mutex<BitMapFrameAllocator> = Mutex::new(BitMapFrameAllocator::empty());
}

// TODO-FUTURE: improve performance... bitmap implementation is slow
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
        self.end_ppn = PhysicalPageNum::new(((end.val() + 1) >> 12) - 1);
        self.bitmap_len =  (size_in_bytes - 1)  / FRAME_SIZE / 64 + 1;

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

        assert_eq!((self.bitmap()[map_index] >> bit_index) & 1, 1);
        self.bitmap()[map_index] ^= (1 << bit_index);
    }

    pub fn available_frame(&self) -> usize {
        let bit_map = self.bitmap();
        let mut cnt = 0;
        for i in 0..bit_map.len() {
            cnt += bit_map[i].count_zeros();
        }
        cnt as usize
    }

    pub fn bitmap(&self) -> &mut [u64] {
        unsafe {
            core::slice::from_raw_parts_mut(PhysicalAddress::from(self.start_ppn).as_mut(), self.bitmap_len,)
        }
    }
}
