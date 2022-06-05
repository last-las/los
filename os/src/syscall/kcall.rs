use share::syscall::error::{SysError, EINVAL, ESRCH, EFAULT, ENAMETOOLONG, EBADF};
use crate::mm::address::{PhysicalAddress, VirtualAddress};
use crate::task::{get_task_by_pid, RuntimeFlags, schedule};
use crate::processor::{get_cur_task_context_in_this_hart, get_cur_task_in_this_hart};
use crate::mm::memory_manager::{RegionFlags, RegionType};
use crate::config::FRAME_SIZE;
use share::ffi::CStr;
use crate::sbi::sbi_console_getchar;
use share::ipc::{Msg, READ, DEVICE, PROC_NR, BUFFER, LENGTH, TERMINAL_PID, REPLY_STATUS, WRITE};
use crate::syscall::ipc::{kcall_send, kcall_receive};
use core::str::from_utf8;
use crate::paging::KERNEL_SATP;
use core::arch::asm;

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
            4 => {
                let word: *const u32 = dev_pa.as_raw();
                word.read_volatile() as usize
            },
            8 => {
                let word: *const u64 = dev_pa.as_raw();
                word.read_volatile() as usize
            },
            _ => {
                panic!("kcall_read_dev: unknown size: {}", byte_size);
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
            8 => {
                let dword: *mut u64 = dev_pa.as_raw_mut();
                dword.write_volatile(val as u64)
            },
            _ => {
                panic!("kcall_write_dev: unknown size: {}", byte_size);
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
    let task = get_cur_task_in_this_hart();
    let mut inner = task.acquire_inner_lock();
    let size = (size + FRAME_SIZE) & !(FRAME_SIZE - 1);
    let start = inner.mem_manager.alloc_area(
        size, RegionFlags::W | RegionFlags::R, RegionType::CONTINUOUS
    )?;

    Ok(start.0)
}

pub fn kcall_virt_to_phys(virt_addr: usize) -> Result<usize, SysError> {
    let task = get_cur_task_in_this_hart();
    let inner = task.acquire_inner_lock();
    let va = VirtualAddress::new(virt_addr);
    let pa =
        inner.mem_manager.page_table.translate_va(va).ok_or(SysError::new(EFAULT))?;

    Ok(pa.0)
}

#[cfg(feature = "board_k210")]
pub fn kcall_sdcard_read(block_id: usize, buf_ptr: usize, length: usize) -> Result<usize, SysError> {
    let task = get_cur_task_in_this_hart();
    let pid = task.pid();
    unsafe {
        switch_to_page_table_by_satp(KERNEL_SATP);
    }

    let buf = get_mut_byte_slice_in_proc(pid, buf_ptr, length)?;
    crate::sdcard::read_block(block_id, buf);
    let cur_task_inner = task.acquire_inner_lock();
    let satp = cur_task_inner.mem_manager.page_table.satp();
    switch_to_page_table_by_satp(satp);
    Ok(0)
}

#[cfg(feature = "board_k210")]
pub fn kcall_sdcard_write(block_id: usize, buf_ptr: usize, length: usize) -> Result<usize, SysError> {
    let task = get_cur_task_in_this_hart();
    let pid = task.pid();
    unsafe {
        switch_to_page_table_by_satp(KERNEL_SATP);
    }

    let buf = get_byte_slice_in_proc(pid, buf_ptr, length)?;
    crate::sdcard::write_block(block_id, buf);
    let cur_task_inner = task.acquire_inner_lock();
    let satp = cur_task_inner.mem_manager.page_table.satp();
    switch_to_page_table_by_satp(satp);
    Ok(0)
}

fn switch_to_page_table_by_satp(satp: usize) {
    let satp = 8 << 60 | satp;
    riscv::register::satp::write(satp);
    unsafe {
        asm! {
        "sfence.vma",
        "fence.i"
        }
    }
}

pub fn kcall_copy_c_path(proc: usize, path_ptr: usize, buf_ptr: usize, size: usize) -> Result<usize, SysError> {
    let path_proc = get_task_by_pid(proc).ok_or(SysError::new(ESRCH))?;
    let path_proc_inner = path_proc.acquire_inner_lock();
    let path_va = VirtualAddress::new(path_ptr);
    let path_pa =
        path_proc_inner.mem_manager.page_table.translate_va(path_va).ok_or(SysError::new(EFAULT))?;

    let c_str = CStr::from_ptr(path_pa.as_raw());
    let path_length = c_str.as_bytes().len();
    if path_length > size {
        return Err(SysError::new(ENAMETOOLONG));
    }

    let dst_slice = unsafe {
        core::slice::from_raw_parts_mut(buf_ptr as *mut u8, path_length)
    };
    dst_slice.copy_from_slice(c_str.as_bytes());

    Ok(c_str.as_bytes().len())
}


pub fn kcall_sbi_read(fd: usize, buf_ptr: *mut u8, length: usize) -> Result<usize, SysError> {
    if fd != 0 {
        return Err(SysError::new(EBADF));
    }

    let buffer = unsafe {
        core::slice::from_raw_parts_mut(buf_ptr, length)
    };
    let mut cnt = 0;
    for i in 0..length {
        let mut result;
        loop {
            result = sbi_console_getchar();
            // info!("result is: {:#x}", result);
            if result == -1 {
                schedule(RuntimeFlags::READY);
                continue;
            }
            break;
        }
        buffer[i] = result as usize as u8;
        cnt += 1;
    }

    Ok(cnt)
}

pub fn kcall_sbi_write(fd: usize, buf_ptr: *const u8, length: usize) -> Result<usize, SysError>{
    if fd != 1 {
        return Err(SysError::new(EBADF));
    }

    let buffer = unsafe {
        core::slice::from_raw_parts(buf_ptr, length)
    };
    print!("{}", from_utf8(buffer).unwrap());
    Ok(0)
}

pub fn kcall_terminal_read(fd: usize, buf_ptr: usize, length: usize) -> Result<usize, SysError> {
    if fd != 0 {
        return Err(SysError::new(EBADF));
    }

    let mut message = Msg::empty();
    let cur_pid = get_cur_task_in_this_hart().pid();
    message.src_pid = cur_pid;
    message.mtype = READ;
    message.args[DEVICE] = 0;
    message.args[PROC_NR] = cur_pid;
    message.args[BUFFER] = buf_ptr;
    message.args[LENGTH] = length;
    kcall_send(TERMINAL_PID, &message as *const _ as usize).unwrap();
    kcall_receive(TERMINAL_PID as isize, &mut message as *mut _ as usize).unwrap();

    Ok(message.args[REPLY_STATUS])
}

pub fn kcall_terminal_write(fd: usize, buf_ptr: usize, length: usize) -> Result<usize, SysError> {
    if fd != 1 {
        return Err(SysError::new(EBADF));
    }

    let mut message = Msg::empty();
    let cur_pid = get_cur_task_in_this_hart().pid();
    message.src_pid = cur_pid;
    message.mtype = WRITE;
    message.args[DEVICE] = 0;
    message.args[PROC_NR] = cur_pid;
    message.args[BUFFER] = buf_ptr;
    message.args[LENGTH] = length;
    kcall_send(TERMINAL_PID, &message as *const _ as usize).unwrap();
    kcall_receive(TERMINAL_PID as isize, &mut message as *mut _ as usize).unwrap();

    Ok(message.args[REPLY_STATUS])
}

fn get_byte_slice_in_proc(pid: usize, ptr: usize, length: usize) -> Result<&'static [u8], SysError> {
    let task = get_task_by_pid(pid).ok_or(SysError::new(EINVAL))?;
    let task_inner = task.acquire_inner_lock();
    let ptr_va = VirtualAddress::new(ptr);
    let ptr_pa = task_inner.mem_manager.page_table.translate_va(ptr_va)
        .ok_or(SysError::new(EFAULT))?;
    unsafe {
        Ok(core::slice::from_raw_parts(ptr_pa.as_raw(), length))
    }
}

fn get_mut_byte_slice_in_proc(pid: usize, ptr: usize, length: usize) -> Result<&'static mut [u8], SysError> {
    let task = get_task_by_pid(pid).ok_or(SysError::new(EINVAL))?;
    let task_inner = task.acquire_inner_lock();
    let ptr_va = VirtualAddress::new(ptr);
    let ptr_pa = task_inner.mem_manager.page_table.translate_va(ptr_va)
        .ok_or(SysError::new(EFAULT))?;
    unsafe {
        Ok(core::slice::from_raw_parts_mut(ptr_pa.as_raw_mut(), length))
    }
}
