use share::syscall::error::{SysError, EINVAL};
use crate::mm::address::{PhysicalAddress, VirtualAddress};
use crate::task::get_task_by_pid;

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
/// Copy a slice from caller task to `dst_proc` task.
pub fn kcall_virt_copy(src_ptr: usize, dst_proc: usize, dst_ptr: usize, length: usize) -> Result<usize, SysError> {
    let dst_task = get_task_by_pid(dst_proc);
    if dst_task.is_none() {
        return Err(SysError::new(EINVAL));
    }
    let dst_task = dst_task.unwrap();
    let dst_task_inner = dst_task.acquire_inner_lock();
    let dst_ptr_pa =
        dst_task_inner.mem_manager.page_table.translate_va(VirtualAddress::new(dst_ptr))
            .ok_or(SysError::new(EINVAL))?;

    let mut dst_data: &mut [u8] = unsafe {
        core::slice::from_raw_parts_mut(dst_ptr_pa.as_raw_mut(), length)
    };
    let src_data: & [u8] = unsafe {
        core::slice::from_raw_parts(src_ptr as *const u8, length)
    };
    dst_data.copy_from_slice(src_data);

    Ok(0)
}