use share::syscall::error::SysError;
use crate::mm::address::PhysicalAddress;

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
            _ => {
                panic!("kcall_write_dev: unknown size")
            }
        }
    }

    Ok(0)
}