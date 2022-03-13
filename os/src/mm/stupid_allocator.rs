use core::alloc::{Allocator, Layout, AllocError};
use core::ptr::NonNull;
use spin::Mutex;
use crate::mm::address::PhysicalAddress;

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

            if self.is_addr_available(possible_addr, layout.size()) {
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

    fn is_addr_available(&self, addr: usize, size: usize) -> bool {
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