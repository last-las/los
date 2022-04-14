use share::syscall::error::{SysError, EINVAL};
use crate::mm::address::{PhysicalAddress, VirtualAddress};
use crate::task::get_task_by_pid;
use crate::processor::clone_cur_task_in_this_hart;
use crate::mm::memory_manager::{RegionFlags, RegionType};
use crate::config::FRAME_SIZE;

pub fn kcall_read_dev(dev_phys_addr: usize, byte_size: usize) -> Result<usize, SysError> {
    let dev_pa = PhysicalAddress::new(dev_phys_addr);
    let ret = unsafe {
        match byte_size {
            1 => {
                let byte: *const u8 = dev_pa.as_raw();
                byte.read_volatile() as usize
            }
            2 => {
                let word: *const u16 = dev_pa.as_raw();
                word.read_volatile() as usize
            }
            _ => {
                panic!("kcall_read_dev: unknown size")
            }
        }
    };
    Ok(ret)
}

pub fn kcall_write_dev(dev_phys_addr: usize, val: usize, byte_size: usize) -> Result<usize, SysError> {
    let dev_pa = PhysicalAddress::new(dev_phys_addr);
    unsafe {
        match byte_size {
            1 => {
                let byte: *mut u8 = dev_pa.as_raw_mut();
                byte.write_volatile(val as u8)
            },
            2 => {
                let word: *mut u16 = dev_pa.as_raw_mut();
                word.write_volatile(val as u16)
            },
            4 => {
                let dword: *mut u32 = dev_pa.as_raw_mut();
                dword.write_volatile(val as u32)
            },
            _ => {
                panic!("kcall_write_dev: unknown size")
            }
        }
    }

    Ok(0)
}
/// Copy a slice from `src_proc` task to `dst_proc` task.
pub fn kcall_virt_copy(src_proc: usize, src_ptr: usize, dst_proc: usize, dst_ptr: usize, length: usize) -> Result<usize, SysError> {
    let src_data = get_byte_slice_in_proc(src_proc, src_ptr, length)?;
    let dst_data = get_mut_byte_slice_in_proc(dst_proc, dst_ptr, length)?;
    dst_data.copy_from_slice(src_data);

    Ok(0)
}

pub fn kcall_continuous_alloc(size: usize) -> Result<usize, SysError> {
    let task = clone_cur_task_in_this_hart();
    let mut inner = task.acquire_inner_lock();
    let size = (size + FRAME_SIZE) & !(FRAME_SIZE - 1);
    let start = inner.mem_manager.alloc_area(
        size, RegionFlags::W | RegionFlags::R, RegionType::CONTINUOUS
    )?;

    Ok(start.0)
}

pub fn kcall_virt_to_phys(virt_addr: usize) -> Result<usize, SysError> {
    let task = clone_cur_task_in_this_hart();
    let inner = task.acquire_inner_lock();
    let va = VirtualAddress::new(virt_addr);
    let pa =
        inner.mem_manager.page_table.translate_va(va).ok_or(SysError::new(EINVAL))?;

    Ok(pa.0)
}


fn get_byte_slice_in_proc(pid: usize, ptr: usize, length: usize) -> Result<&'static [u8], SysError> {
    let task = get_task_by_pid(pid).ok_or(SysError::new(EINVAL))?;
    let task_inner = task.acquire_inner_lock();
    let ptr_va = VirtualAddress::new(ptr);
    let ptr_pa = task_inner.mem_manager.page_table.translate_va(ptr_va)
        .ok_or(SysError::new(EINVAL))?;
    unsafe {
        Ok(core::slice::from_raw_parts(ptr_pa.as_raw(), length))
    }
}

fn get_mut_byte_slice_in_proc(pid: usize, ptr: usize, length: usize) -> Result<&'static mut [u8], SysError> {
    let task = get_task_by_pid(pid).ok_or(SysError::new(EINVAL))?;
    let task_inner = task.acquire_inner_lock();
    let ptr_va = VirtualAddress::new(ptr);
    let ptr_pa = task_inner.mem_manager.page_table.translate_va(ptr_va)
        .ok_or(SysError::new(EINVAL))?;
    unsafe {
        Ok(core::slice::from_raw_parts_mut(ptr_pa.as_raw_mut(), length))
    }
}
