use crate::mm::address::{VirtualAddress, ceil};
use crate::processor::get_cur_task_in_this_hart;
use crate::config::{MMAP_START_ADDRESS, FRAME_SIZE};
use crate::mm::memory_manager::{RegionFlags, RegionType};
use share::syscall::error::{SysError, ENOMEM};
use share::mmap::{Prot, MMAPFlags};
use crate::syscall::file::{do_lseek, do_read};
use alloc::vec::Vec;
use alloc::vec;

pub fn do_brk(new_brk: usize) -> Result<usize, SysError> {
    let mut new_brk = VirtualAddress::new(new_brk);

    let cur_task = get_cur_task_in_this_hart();
    let mut inner = cur_task.acquire_inner_lock();
    let mut brk = inner.mem_manager.brk;
    let mut size = new_brk.0.abs_diff(brk.0);

    if new_brk.0 == 0 {
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
            RegionFlags::W | RegionFlags::R, RegionType::Default, None
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
    Ok(new_brk.0)
}

pub fn do_mmap(start: usize, len: usize, prot: u32, flags: u32, fd: usize, offset: usize) -> Result<usize, SysError> {
    let prot = Prot::from_bits(prot).unwrap();
    let flags = MMAPFlags::from_bits(flags).unwrap();
    let data: Vec<u8>;

    if flags.contains(MMAPFlags::SHARED) | flags.contains(MMAPFlags::PRIVATE) {
        do_lseek(fd, offset, 0)?;
        data = vec![0; len];
        do_read(fd,data.as_ptr() as usize, len)?;
    } else { // ANONYMOUS
        data = Vec::new();
    }

    let mut region_flags = RegionFlags::empty();
    if prot.contains(Prot::READ) { region_flags |= RegionFlags::R };
    if prot.contains(Prot::WRITE) { region_flags |= RegionFlags::W };
    if prot.contains(Prot::EXEC) { region_flags |= RegionFlags::X };

    let region_type;
    if flags.contains(MMAPFlags::SHARED) {
        region_type = RegionType::Shared(fd, offset, len);
    } else {
        region_type = RegionType::Default;
    }

    let cur_task = get_cur_task_in_this_hart();
    let mut inner = cur_task.acquire_inner_lock();
    let size = ceil(len);
    let mut return_addr = VirtualAddress::new(0);
    if start == 0 {
        return_addr = inner.mem_manager.alloc_area(size,  region_flags, region_type, Some(data.as_slice()))?;
    } else {
        inner.mem_manager.add_area(VirtualAddress::new(start), size, region_flags, region_type, Some(data.as_slice()))?;
    }

    Ok(return_addr.0)
}

pub fn do_munmap(start: usize, len: usize) -> Result<usize, SysError> {
    Ok(0)
}