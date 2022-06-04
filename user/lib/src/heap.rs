#[allow(unused_imports)]
use core::alloc::Layout;
use buddy_allocator::{LockedHeap};
use crate::syscall::{brk, getpid};

// 20220527 change from 0x80_000 to 0x2000
const USER_HEAP_SIZE: usize = 0x2000;

// TODO-FUTURE: The size of heap should be dynamic rather than fix size `USER_HEAP_ALLOCATOR`
#[global_allocator]
static USER_HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() {
    // get current brk
    let cur_pos = brk(None).unwrap();
    // adjust to new brk, with allocating PhysPage operation
    brk(Some(cur_pos + USER_HEAP_SIZE)).unwrap();
    unsafe {
        USER_HEAP_ALLOCATOR.lock().init(cur_pos, USER_HEAP_SIZE);
    }
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    let pid = getpid();
    panic!("User process {} heap allocation error, Layout = {:?}", pid, layout);
}