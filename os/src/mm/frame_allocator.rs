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

#[allow(unused)]
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

#[cfg(not(test))]
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
            self.bitmap()[map_index] |= 1 << bit_index;
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
            *entry |= 1 << bit_index;
            return Some(FrameTracker::new(ppn));
        }

        None
    }

    pub fn dealloc(&mut self, ppn: PhysicalPageNum) {
        assert!(ppn.0 > self.start_ppn.0 && ppn.0 <= self.end_ppn.0);
        let map_index = (ppn.0 - self.start_ppn.0) / 64;
        let bit_index = (ppn.0 - self.start_ppn.0) % 64;

        assert_eq!((self.bitmap()[map_index] >> bit_index) & 1, 1);
        self.bitmap()[map_index] ^= 1 << bit_index;
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

#[cfg(test)]
mod test {
    use super::*;

    const RAM_SIZE: usize = 0x8000000;
    #[link_section = ".data"]
    static REAL_RAM: [u8; RAM_SIZE + FRAME_SIZE] = [0; RAM_SIZE + FRAME_SIZE];

    fn acquire_aligned_ptr_and_size(arr: &[u8]) -> (usize, usize) {
        let origin_ptr = arr.as_ptr() as usize;
        let aligned_ptr = (origin_ptr + FRAME_SIZE - 1) / FRAME_SIZE * FRAME_SIZE;

        (aligned_ptr, arr.len() - (aligned_ptr - origin_ptr))
    }

    #[test]
    pub fn test_alloc_and_dealloc_on_small_ram() {
        let mut fake_ram = [0; FRAME_SIZE * 10];

        let mut bmf_allocator = BitMapFrameAllocator::empty();
        let (ptr, size) = acquire_aligned_ptr_and_size(&fake_ram);
        // println!("heap start:{:#x}, heap size:{:#x}", ptr, size);
        let start = PhysicalAddress::new(ptr);
        let end = PhysicalAddress::new(ptr + size);
        bmf_allocator.init(start, end);

        // 1. test init.
        assert_eq!(bmf_allocator.bitmap()[0], 1);
        assert_eq!(bmf_allocator.bitmap_len, 1);

        // 2. test alloc.
        let mut frame_trackers: [FrameTracker; 8] =
            [
                FrameTracker::new(PhysicalPageNum::new(0)),
                FrameTracker::new(PhysicalPageNum::new(0)),
                FrameTracker::new(PhysicalPageNum::new(0)),
                FrameTracker::new(PhysicalPageNum::new(0)),
                FrameTracker::new(PhysicalPageNum::new(0)),
                FrameTracker::new(PhysicalPageNum::new(0)),
                FrameTracker::new(PhysicalPageNum::new(0)),
                FrameTracker::new(PhysicalPageNum::new(0)),

            ];
        let mut bitmap_entry0_val = 1;

        for i in 0..8 {
            frame_trackers[i] = bmf_allocator.alloc().unwrap();
            // println!("{:#x}", frame_trackers[i].0.0);
            assert_eq!(frame_trackers[i].0.0, bmf_allocator.start_ppn.0 + 1 + i);
            bitmap_entry0_val |= (1 << (i + 1));
            assert_eq!(bmf_allocator.bitmap()[0], bitmap_entry0_val);
        }

        assert!(bmf_allocator.alloc().is_none());

        // 3. test dealloc
        let index = 5;
        let ppn = frame_trackers[index].0;
        bmf_allocator.dealloc(frame_trackers[index].0);
        assert_eq!(bmf_allocator.bitmap()[0], 0b110111111);

        frame_trackers[index] = bmf_allocator.alloc().unwrap();
        assert_eq!(ppn.0, frame_trackers[index].0.0);
        assert_eq!(bmf_allocator.bitmap()[0], 0b111111111);

        for i in 0..8 {
            bmf_allocator.dealloc(frame_trackers[i].0);
        }
        assert_eq!(bmf_allocator.bitmap()[0], 0b1);
    }

    #[test]
    pub fn test_alloc_and_dealloc_on_real_ram() {
        // 1. test init.
        let mut bmf_allocator = BitMapFrameAllocator::empty();
        let (ptr,_) = unsafe {
            acquire_aligned_ptr_and_size(REAL_RAM.as_slice())
        };
        let size = RAM_SIZE;

        let start = PhysicalAddress::new(ptr);
        let end = PhysicalAddress::new(ptr + size);
        bmf_allocator.init(start, end);

        let bitmap_len = 512;
        assert_eq!(bmf_allocator.bitmap_len, bitmap_len);
        assert_eq!(bmf_allocator.bitmap()[0], 1);

        // 2. test alloc.
        let mut ppn = start.val() >> 12;
        ppn += 1;
        let mut frame_tracker = FrameTracker::new(PhysicalPageNum::new(0));

        for i in 0..63 {
            frame_tracker =  bmf_allocator.alloc().unwrap();
            assert_eq!(frame_tracker.0.0, ppn);
            ppn += 1;
        }
        assert_eq!(bmf_allocator.bitmap()[0], u64::MAX);

        for i in 1..bitmap_len {
            assert_eq!(bmf_allocator.bitmap()[i], 0);

            for j in 0..64 {
                frame_tracker = bmf_allocator.alloc().unwrap();
                assert_eq!(frame_tracker.0.0, ppn);
                ppn += 1;
            }

            assert_eq!(bmf_allocator.bitmap()[i], u64::MAX);
        }

        assert!(bmf_allocator.alloc().is_none());

        let the_bitmap_frame: &mut [u8] = unsafe {
            core::slice::from_raw_parts_mut(start.as_mut(), FRAME_SIZE)
        };
        for i in 0..FRAME_SIZE {
            assert_eq!(the_bitmap_frame[i], u8::MAX);
        }

        // 3. test dealloc
        let offset = 128;
        let dealloc_ppn = PhysicalPageNum::new((start.val() >> 12) + offset);
        bmf_allocator.dealloc(dealloc_ppn);
        frame_tracker = bmf_allocator.alloc().unwrap();
        assert_eq!(frame_tracker.0.0, dealloc_ppn.0);

        let mut start_ppn =  (start.val() >> 12) + 1;
        for i in 0..(64 * bitmap_len - 1) {
            bmf_allocator.dealloc(PhysicalPageNum::new(start_ppn));
            start_ppn += 1;
        }
        assert_eq!(the_bitmap_frame[0], 0b1);
        for i in 1..FRAME_SIZE {
            assert_eq!(the_bitmap_frame[i], 0);
        }

    }
}