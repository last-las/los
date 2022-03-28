use crate::mm::address::{VirtualAddress, ceil};
use crate::processor::clone_cur_task_in_this_hart;
use crate::config::{MMAP_START_ADDRESS, FRAME_SIZE};
use crate::mm::memory_manager::RegionFlags;

pub fn do_brk(mut new_brk: usize) -> isize {
    let mut new_brk = VirtualAddress::new(new_brk);

    let cur_task = clone_cur_task_in_this_hart();
    let mut inner = cur_task.acquire_inner_lock();
    let mut brk = inner.mem_manager.brk;
    let mut size = new_brk.0.abs_diff(brk.0);

    if new_brk.0 == 0 {
        return brk.0 as isize;
    }

    if new_brk >= brk { // alloc
        if new_brk.0 >= MMAP_START_ADDRESS {
            return -1;
        }

        if ! brk.is_aligned() {
            if size <= (FRAME_SIZE - brk.offset()) {
                inner.mem_manager.brk = new_brk;
                return new_brk.0 as isize;
            }
            size -= FRAME_SIZE - brk.offset();
            brk = brk.ceil().into();
        }
        inner.mem_manager.add_area(brk, ceil(size), RegionFlags::W | RegionFlags::R, None);
    } else { // dealloc
        let brk_start = inner.mem_manager.brk_start;
        if new_brk < brk_start {
            return 0;
        }
        size += new_brk.offset();
        new_brk = new_brk.floor().into();
        inner.mem_manager.delete_area(new_brk, ceil(size));
    }
    inner.mem_manager.brk = new_brk;
    new_brk.0 as isize
}
