use buddy_system_allocator::LockedHeap;
use crate::syscall::sys_brk;

const USER_HEAP_SIZE: usize = 0x4000;

// TODO-FUTURE: The size of heap should be dynamic rather than fix size `USER_HEAP_ALLOCATOR`
#[global_allocator]
static USER_HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() {
    let cur_pos = sys_brk(None) as usize;
    sys_brk(Some(cur_pos + USER_HEAP_SIZE));
    unsafe {
        USER_HEAP_ALLOCATOR.lock().init(cur_pos, USER_HEAP_SIZE);
    }
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("User heap allocation error, Layout = {:?}", layout);
}