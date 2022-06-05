use crate::syscall::{brk, getpid};
use core::alloc::{GlobalAlloc, Layout};
use buddy_system_allocator::LockedHeap;
use core::cmp::max;
use core::mem::size_of;

const USER_HEAP_SIZE: usize = 0x1000;

#[global_allocator]
static USER_HEAP_ALLOCATOR: LockedHeapWrapper = LockedHeapWrapper::empty();

pub fn init_heap() {
    // get current brk
    let cur_pos = brk(None).unwrap();
    // adjust to new brk, with allocating PhysPage operation
    brk(Some(cur_pos + USER_HEAP_SIZE)).unwrap();

    USER_HEAP_ALLOCATOR.init(cur_pos, USER_HEAP_SIZE);
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    let pid = getpid();
    panic!("User process {} heap allocation error, Layout = {:?}", pid, layout);
}

pub struct LockedHeapWrapper{
    inner: LockedHeap
}

impl LockedHeapWrapper {
    pub const fn empty() -> Self {
        Self {
            inner: LockedHeap::empty(),
        }
    }

    pub fn init(&self, start: usize, size: usize) {
        unsafe {
            self.inner.lock().init(start, size);
        }
    }

    pub unsafe fn rescue(&self, layout: Layout) {
        // get current brk
        let cur_brk = brk(None).unwrap();
        // get request
        let allocated_size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        );
        let class = allocated_size.trailing_zeros() as usize;
        if cur_brk & !(0xFFFFFFFFFFFFFFFF << class) == 0 {
            // 对齐
            let new_brk = brk(Some(cur_brk + allocated_size)).unwrap();
            self.inner.lock().add_to_heap(cur_brk, new_brk);
        } else {
            // 没对齐
            let aligned_new_brk = (cur_brk & (0xFFFFFFFFFFFFFFFF << class)) + (1usize << class);
            let new_brk = brk(Some(aligned_new_brk + allocated_size)).unwrap();
            self.inner.lock().add_to_heap(cur_brk, aligned_new_brk);
            self.inner.lock().add_to_heap(aligned_new_brk, new_brk);
        }
    }
}

unsafe impl GlobalAlloc for LockedHeapWrapper {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = self.inner.alloc(layout);
        if ptr as usize == 0 {
            self.rescue(layout);
            self.inner.alloc(layout)
        } else {
            ptr
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.inner.dealloc(ptr, layout);
    }
}