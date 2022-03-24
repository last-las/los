use crate::config::{RAM_MAPPING_OFFSET, FRAME_SIZE};
use core::fmt::{Debug, Formatter};
use core::iter::Step;
use crate::processor::get_cur_task_in_this_hart;

pub const PAGE_SIZE_BITS: usize = 12;

// IS_PAGING determines the action of PhysicalAddress.as_mut()
pub static mut IS_PAGING: bool = false;

pub fn mark_as_paging() {
    unsafe {
        IS_PAGING = true;
    }
}


#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysicalPageNum(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtualPageNum(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysicalAddress(usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtualAddress(pub usize);

/************************************** PhysicalPageNum *******************************************/
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

/************************************** VirtualPageNum *******************************************/
impl Debug for VirtualPageNum {
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

    pub fn vpn(&self) -> [usize; 3] {
        let mut vpns = [0; 3];
        let mut v = self.0;
        for i in 0..3 {
            vpns[i] = v & 0x1FF;
            v >>= 9;
        }
        vpns
    }

    pub fn add(&self, v: usize) -> Self {
        Self {
            0: self.0 + v
        }
    }

    pub fn minus(&self, v: usize) -> Self {
        Self {
            0: self.0 - v
        }
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

impl Step for VirtualPageNum {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if start > end {
            None
        } else {
            Some(end.0 - start.0)
        }
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        if start.0 + count < start.0 {
            None
        } else {
            Some(start.add(count))
        }
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        if start.0 - count > start.0 {
            None
        } else {
            Some(start.minus(count))
        }
    }
}

/************************************** PhysicalAddress *******************************************/
impl Debug for PhysicalAddress {
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
        unsafe {
            if IS_PAGING {
                offset = RAM_MAPPING_OFFSET;
            }
        }

        self.0 + offset
    }

    #[allow(unused)]
    pub fn floor2ppn(&self) -> PhysicalPageNum {
        PhysicalPageNum {
            0: self.0 >> 12
        }
    }

    pub fn is_aligned(&self) -> bool {
        self.0 & (FRAME_SIZE - 1) == 0
    }

    pub fn as_mut<T>(&self) -> &'static mut T {
        unsafe {
            let t: *mut T = self.as_raw_mut();
            t.as_mut().unwrap()
        }
    }

    #[allow(unused)]
    pub fn as_ref<T>(&self) -> &'static T {
        unsafe {
            let t: *const T = self.as_raw();
            t.as_ref().unwrap()
        }
    }

    pub fn as_raw_mut<T>(&self) -> *mut T {
        let mut offset = 0;
        unsafe {
            if IS_PAGING {
                offset = RAM_MAPPING_OFFSET;
            }

            (self.0 + offset) as *mut T
        }
    }

    pub fn as_raw<T>(&self) -> *const T {
        let mut offset = 0;
        unsafe {
            if IS_PAGING {
                offset = RAM_MAPPING_OFFSET;
            }

            (self.0 + offset) as *const T
        }
    }

    pub fn add(&self, v: usize) -> Self {
        Self {
            0: self.0 + v
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

impl From<VirtualAddress> for PhysicalAddress {
    fn from(va: VirtualAddress) -> Self {
        let vpn = va.floor();
        let cur_task = get_cur_task_in_this_hart();
        let cur_task_inner = cur_task.acquire_inner_lock();
        let ppn = cur_task_inner.mem_manager.page_table.translate(vpn).unwrap();

        PhysicalAddress::from(ppn).add(va.offset())
    }
}

/************************************** VirtualAddress *******************************************/
impl Debug for VirtualAddress {
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

    pub fn floor(&self) -> VirtualPageNum {
        VirtualPageNum::new(self.0 / FRAME_SIZE)
    }

    pub fn ceil(&self) -> VirtualPageNum {
        VirtualPageNum::new((self.0 - 1) / FRAME_SIZE + 1)
    }

    pub fn is_aligned(&self) -> bool {
        self.0 & (FRAME_SIZE - 1) == 0
    }

    pub fn add(&self, v: usize) -> Self {
        Self {
            0: self.0 + v
        }
    }

    pub fn minus(&self, v: usize) -> Self {
        Self {
            0: self.0 - v
        }
    }

    pub fn offset(&self) -> usize {
        self.0 & (FRAME_SIZE - 1)
    }
}

impl From<VirtualPageNum> for VirtualAddress {
    fn from(vpn: VirtualPageNum) -> Self {
        Self {
            0: vpn.0 << 12
        }
    }
}
/************************************** other functions *******************************************/
pub fn ceil(v: usize) -> usize {
    ((v - 1) / FRAME_SIZE + 1) * FRAME_SIZE
}

#[cfg(test)]
mod test {
    use crate::config::{KERNEL_MAPPING_OFFSET, RAM_START_ADDRESS};
    use crate::mm::address::VirtualPageNum;

    #[test]
    pub fn test_vpn_on_virtual_page_num() {
        let mut address = KERNEL_MAPPING_OFFSET + RAM_START_ADDRESS;
        address >>= 12;
        let vpn = VirtualPageNum::new(address);
        let mut vpns = vpn.vpn();
        assert_eq!(address & 0x1FF, vpns[0]);
        address >>= 9;
        assert_eq!(address & 0x1FF, vpns[1]);
        address >>= 9;
        assert_eq!(address & 0x1FF, vpns[2]);
    }

    #[test]
    pub fn test_step_trait_on_virtual_page_num() {
        let start = VirtualPageNum::new(0);
        let end = VirtualPageNum::new(50);
        let mut inc: usize = 0;
        for i in start..end {
            assert_eq!(i.0, inc);
            inc += 1;
        }
    }
}