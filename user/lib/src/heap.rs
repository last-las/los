use buddy_system_allocator::LockedHeap;
use crate::syscall::brk;

const USER_HEAP_SIZE: usize = 0x10_000;

// TODO-FUTURE: The size of heap should be dynamic rather than fix size `USER_HEAP_ALLOCATOR`
#[global_allocator]
static USER_HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() {
    let cur_pos = brk(None).unwrap();
    brk(Some(cur_pos + USER_HEAP_SIZE)).unwrap();
    unsafe {
        USER_HEAP_ALLOCATOR.lock().init(cur_pos, USER_HEAP_SIZE);
    }
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("User heap allocation error, Layout = {:?}", layout);
}