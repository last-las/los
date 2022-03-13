use spin::Mutex;
use crate::config::{KERNEL_OFFSET, MEMORY_MAPPING_OFFSET, FRAME_SIZE};
use core::fmt::{Debug, Formatter};
use riscv::addr::VirtAddr;

const VPN_BITMASK: usize = 0x1ff;
const VPN_LENGTH: usize = 9;
pub const PAGE_SIZE_BITS: usize = 12;

// IS_PAGING will affect the action of PhysicalAddress.as_mut
pub static IS_PAGING: Mutex<Paging> = Mutex::new(Paging{ 0: false });

pub struct Paging(pub bool);


#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysicalPageNum(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtualPageNum(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysicalAddress(usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtualAddress(pub usize);

impl Debug for PhysicalPageNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("PhysicalPageNum:{:#x}", self.0))
    }
}

impl PhysicalPageNum {
    pub fn new(v: usize) -> Self {
        Self {
            0: v,
        }
    }
}

impl From<PhysicalAddress> for PhysicalPageNum {
    fn from(pa: PhysicalAddress) -> Self {
        assert!(pa.is_aligned());
        Self {
            0: pa.0 >> 12
        }
    }
}

impl Debug for VirtualPageNum{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("VirtualPageNum:{:#x}", self.0))
    }
}

impl VirtualPageNum {
    pub fn new(v: usize) -> Self {
        Self {
            0: v,
        }
    }

    pub fn vpn(&self) -> [usize; 3]  {
        let mut vpns = [0; 3];
        let mut v = self.0;
        for i in 0..3 {
            vpns[i] = v & 0x1FF;
            v >>= 9;
        }
        vpns
    }
}

impl From<VirtualAddress> for VirtualPageNum {
    fn from(va: VirtualAddress) -> Self {
        assert!(va.is_aligned());
        Self {
            0: va.0 >> 12
        }
    }
}

impl Debug for PhysicalAddress{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("PhysicalAddress:{:#x}", self.0))
    }
}


impl PhysicalAddress {
    pub fn new(val: usize) -> Self {
        Self {
            0: val,
        }
    }

    pub fn val(&self) -> usize {
        let mut offset = 0;
        if IS_PAGING.lock().0 {
            offset = MEMORY_MAPPING_OFFSET;
        }

        self.0 + offset
    }

    pub fn floor2ppn(&self) -> PhysicalPageNum{
        PhysicalPageNum {
            0: self.0 >> 12
        }
    }

    pub fn is_aligned(&self) -> bool {
        self.0 & (FRAME_SIZE - 1) == 0
    }

    pub fn as_mut<T>(&self) -> &'static mut T {
        let mut offset = 0;
        if IS_PAGING.lock().0 {
            offset = MEMORY_MAPPING_OFFSET;
        }

        unsafe {
            ((self.0 + offset) as *mut T).as_mut().unwrap()
        }
    }
}

impl From<PhysicalPageNum> for PhysicalAddress {
    fn from(ppn: PhysicalPageNum) -> Self {
        Self {
            0: ppn.0 << 12,
        }
    }

}

impl Debug for VirtualAddress{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("VirtualAddress:{:#x}", self.0))
    }
}


impl VirtualAddress {
    pub fn new(val: usize) -> Self {
        Self {
            0: val
        }
    }

    pub fn is_aligned(&self) -> bool {
        self.0 & (FRAME_SIZE - 1) == 0
    }
}

