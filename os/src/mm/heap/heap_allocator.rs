use buddy_system_allocator::LockedHeap;

const KERNEL_HEAP_SIZE: usize = 0x300_0000;

#[cfg_attr(not(test), global_allocator)]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

pub fn print_heap_usage() {
    println!("{:?}", HEAP_ALLOCATOR.lock());
}

#[cfg_attr(not(test), alloc_error_handler)]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, Layout = {:?}", layout);
}