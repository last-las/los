use core::alloc::{Allocator, Layout, AllocError};
use core::ptr::NonNull;
use spin::Mutex;
use crate::mm::address::PhysicalAddress;

// StupidAllocator is only used for kernel page table in enable_paging
pub struct StupidAllocator {
    inner: Mutex<StupidAllocatorInner>,
}

impl StupidAllocator {
    pub fn new(start: usize, size: usize) -> Self {
        Self {
            inner: Mutex::new(StupidAllocatorInner::new(start, size))
        }
    }
}

unsafe impl Allocator for StupidAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.inner.lock().allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.inner.lock().deallocate(ptr, layout)
    }
}

pub struct StupidAllocatorInner {
    start_addr: usize,
    end_addr: usize,
    bitmap_len: usize,
}

impl StupidAllocatorInner {
    pub fn new(start: usize, size: usize) -> Self {
        let bitmap_len = size / 8;
        let mut inner = StupidAllocatorInner {
            start_addr: start,
            end_addr: start + size,
            bitmap_len,
        };
        inner.set_used(start, bitmap_len);

        inner
    }

    fn allocate(&mut self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let mut possible_addr = do_align(self.start_addr, layout.align());
        loop {
            if possible_addr + layout.size() > self.end_addr {
                return Err(AllocError);
            }

            if self.is_region_available(possible_addr, layout.size()) {
                break;
            }

            possible_addr += layout.align();
        }

        self.set_used(possible_addr, layout.size());

        Ok(unsafe {
            NonNull::new_unchecked(
                core::ptr::slice_from_raw_parts_mut(possible_addr as *mut _, layout.size())
            )
        })
    }

    fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout) {
        let addr = ptr.as_ptr() as usize;
        self.set_unused(addr, layout.size());
    }

    fn is_region_available(&self, addr: usize, size: usize) -> bool {
        let bitmap = self.bitmap();
        let relative_addr = addr - self.start_addr;

        for i in 0..size {
            let mut index = (relative_addr + i) / 8;
            let mut bit_offset = (relative_addr + i) % 8;
            if (bitmap[index] >> bit_offset) & 1 == 1 {
                return false;
            }
        }

        return true;
    }

    fn set_used(&mut self, addr: usize, size: usize) {
        let bitmap = self.bitmap();
        let relative_addr = addr - self.start_addr;

        for i in 0..size {
            let mut index = (relative_addr + i) / 8;
            let mut bit_offset = (relative_addr + i) % 8;

            bitmap[index] |= 1 << bit_offset;
        }
    }

    fn set_unused(&mut self, addr: usize, size: usize) {
        let bitmap = self.bitmap();
        let relative_addr = addr - self.start_addr;

        for i in 0..size {
            let mut index = (relative_addr + i) / 8;
            let mut bit_offset = (relative_addr + i) % 8;

            assert_eq!(bitmap[index] >> bit_offset & 1, 1);
            bitmap[index] ^= 1 << bit_offset;
        }
    }

    fn bitmap(&self) -> &mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(
                PhysicalAddress::new(self.start_addr).as_mut(), self.bitmap_len,
            )
        }
    }
}


fn do_align(addr: usize, align: usize) -> usize {
    (addr + align - 1) / align * align
}

#[cfg(test)]
mod test {
    use crate::config::FRAME_SIZE;
    use std::alloc::{Layout, Allocator};
    use std::ptr::NonNull;
    use crate::mm::heap::stupid_allocator::StupidAllocator;
    use alloc::vec::Vec;
    const FREE_MEMORY: usize = FRAME_SIZE - FRAME_SIZE / 8;

    #[test]
    pub fn test_new_and_alloc_a_vector() {
        let mut frame: [u8; FRAME_SIZE] = [0; FRAME_SIZE];
        let allocator = StupidAllocator::new(frame.as_ptr() as usize, FRAME_SIZE);
        let inner = allocator.inner.lock();
        assert_eq!(inner.bitmap_len, FRAME_SIZE / 8);
        drop(inner);

        let free_memory = FRAME_SIZE - FRAME_SIZE / 8;
        let mut v = Vec::new_in(allocator);
        for i in 0..100 {
            v.push(i);
        }
        for i in (0..100).into_iter().rev() {
            assert_eq!(v.pop().unwrap(), i);
        }
        drop(v);
    }

    #[test]
    pub fn test_alloc_largest_memory_within_a_frame() {
        let mut frame: [u8; FRAME_SIZE] = [0; FRAME_SIZE];
        let allocator = StupidAllocator::new(frame.as_ptr() as usize, FRAME_SIZE);
        let free_space = 3584; // 3584 bytes is the largest space stupid_allocator could alloc

        unsafe {
            // allocate `free_space` size
            let big_layout = Layout::from_size_align_unchecked(free_space, 2);
            let mut ptr = allocator.allocate(big_layout).unwrap();
            let inner = allocator.inner.lock();
            assert_eq!(inner.start_addr + FRAME_SIZE / 8, ptr.as_mut_ptr() as usize);
            drop(inner);

            let small_layout = Layout::from_size_align_unchecked(1, 2);
            assert!(allocator.allocate(small_layout).is_err());

            // deallocate
            allocator.deallocate(NonNull::new(ptr.as_mut_ptr()).unwrap(), big_layout);

            // allocate again
            assert!(allocator.allocate(big_layout).is_ok());
        }
    }
}