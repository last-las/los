use crate::mm::address::{VirtualAddress, ceil};
use crate::processor::get_cur_task_in_this_hart;
use crate::config::{MMAP_START_ADDRESS, FRAME_SIZE};
use crate::mm::memory_manager::{RegionFlags, RegionType};
use share::syscall::error::{SysError, ENOMEM};

pub fn do_brk(new_brk: usize) -> Result<usize, SysError> {
    let mut new_brk = VirtualAddress::new(new_brk);
    static mut cnt: usize = 0;

    let cur_task = get_cur_task_in_this_hart();
    if cur_task.pid() == 6 {
        unsafe {
            cnt += 1;
        }
    }
    let mut inner = cur_task.acquire_inner_lock();
    let mut brk = inner.mem_manager.brk;
    let mut size = new_brk.0.abs_diff(brk.0);

    if new_brk.0 == 0 {
        unsafe {
            if cnt == 3 {
                info!("{:?}\n", inner.mem_manager.region_list);
            }
        }
        return Ok(brk.0);
    }

    if new_brk >= brk { // alloc
        if new_brk.0 >= MMAP_START_ADDRESS {
            return Err(SysError::new(ENOMEM));
        }

        if ! brk.is_aligned() {
            if size <= (FRAME_SIZE - brk.offset()) {
                inner.mem_manager.brk = new_brk;
                return Ok(new_brk.0);
            }
            size -= FRAME_SIZE - brk.offset();
            brk = brk.ceil().into();
        }
        inner.mem_manager.add_area(
            brk, ceil(size),
            RegionFlags::W | RegionFlags::R, RegionType::DEFAULT, None
        )?;
    } else { // dealloc
        let brk_start = inner.mem_manager.brk_start;
        if new_brk < brk_start {
            return Ok(0);
        }
        size += new_brk.offset();
        new_brk = new_brk.floor().into();
        inner.mem_manager.delete_area(new_brk, ceil(size));
    }
    inner.mem_manager.brk = new_brk;

    unsafe {
        if cnt == 4 {
            info!("{:?}\n", inner.mem_manager.region_list);
        }
    }
    Ok(new_brk.0)
}
