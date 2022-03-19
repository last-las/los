use crate::mm::frame_allocator::{FrameTracker, alloc_frame};
use alloc::vec::Vec;
use crate::mm::address::{PhysicalAddress, VirtualAddress, PAGE_SIZE_BITS, PhysicalPageNum, VirtualPageNum};
use crate::config::FRAME_SIZE;
use crate::paging::__kernel_start;
use alloc::alloc::Global;
use core::alloc::Allocator;
use riscv::register::satp;


const PAGE_TABLE_ENTRY_NUM: usize = FRAME_SIZE / 8;

pub struct PageTable<T: Allocator = Global> {
    root_table_frame: FrameTracker,
    sub_table_frames: Vec<FrameTracker, T>,
}

impl<T: Allocator> PageTable<T> {
    pub fn new_kernel_table(allocator: T) -> Self {
        let mut root_table_frame = alloc_frame().unwrap();
        root_table_frame.clear();
        Self {
            root_table_frame,
            sub_table_frames: Vec::<FrameTracker, T>::new_in(allocator),
        }
    }

    pub fn map_with_offset(
        &mut self,
        start: usize, end: usize, offset: usize,
        flags: PTEFlags) {
        for addr in (start..end).step_by(FRAME_SIZE) {
            let ppn = PhysicalAddress::new(addr).into();
            let vpn = VirtualAddress::new(addr + offset).into();
            self.map(
                ppn,
                vpn,
                flags,
            );
            assert_eq!(self.find(vpn).unwrap().0, ppn.0);
        }
    }

    pub fn map(&mut self,
               physical_page_num: PhysicalPageNum, virtual_page_num: VirtualPageNum,
               flags: PTEFlags) {
        let mut table: &mut [PageTableEntry; PAGE_TABLE_ENTRY_NUM] =
            PhysicalAddress::from(self.root_table_frame.0).as_mut();

        let mut vpns = virtual_page_num.vpn();
        vpns.reverse();

        for i in 0..3 {
            let pte = &mut table[vpns[i]];

            if i == 2 {
                assert!(!pte.is_valid());
                *pte = PageTableEntry::new(flags, physical_page_num);
                return;
            }

            if pte.is_valid() {
                table = PhysicalAddress::new(pte.ppn() << PAGE_SIZE_BITS).as_mut();
            } else {
                let mut new_frame = alloc_frame().unwrap();
                new_frame.clear();
                pte.write_ppn(new_frame.0);
                pte.set_valid();
                table = PhysicalAddress::from(new_frame.0).as_mut();
                self.sub_table_frames.push(new_frame);
            }
        }
    }

    pub fn find(&self, virtual_page_num: VirtualPageNum) -> Option<PhysicalPageNum> {
        match self.find_pte(virtual_page_num) {
            Some(pte) => Some(PhysicalPageNum::new(pte.ppn())),
            None => None
        }
    }

    pub fn find_pte(&self, virtual_page_num: VirtualPageNum) -> Option<PageTableEntry> {
        let mut table: &mut [PageTableEntry; PAGE_TABLE_ENTRY_NUM] =
            PhysicalAddress::from(self.root_table_frame.0).as_mut();

        let mut vpns = virtual_page_num.vpn();
        vpns.reverse();

        for i in 0..3 {
            let pte: &mut PageTableEntry = &mut table[vpns[i]];

            if pte.is_valid() {
                if i == 2 {
                    return Some(PageTableEntry::raw(pte.0));
                }

                table = PhysicalAddress::new(pte.ppn() << PAGE_SIZE_BITS).as_mut();
            } else {
                break;
            }
        }

        None
    }
    pub fn satp(&self) -> usize {
        self.root_table_frame.0.0
    }
}

impl PageTable {
    pub fn new_user_table() -> Self {
        let mut user_table = Self::new();
        user_table.copy_kernel_entries();
        user_table
    }

    fn new() -> Self {
        let mut root_table_frame = alloc_frame().unwrap();
        root_table_frame.clear();
        Self {
            root_table_frame,
            sub_table_frames: Vec::new(),
        }
    }

    fn copy_kernel_entries(&mut self) {
        let new_table: &mut [PageTableEntry; PAGE_TABLE_ENTRY_NUM] =
            PhysicalAddress::from(self.root_table_frame.0).as_mut();

        // copy kernel PageTableEntries.
        let current_table = get_current_table();
        let kernel_start_vpn: VirtualPageNum = VirtualAddress::new(__kernel_start as usize).into();
        for i in kernel_start_vpn.vpn()[2]..PAGE_TABLE_ENTRY_NUM {
            new_table[i].0 = current_table[i].0;
        }
    }
}

fn get_current_table() -> &'static [PageTableEntry; PAGE_TABLE_ENTRY_NUM] {
    let ppn = PhysicalPageNum::new(satp::read().ppn());
    let pa: PhysicalAddress = ppn.into();
    pa.as_mut()
}

pub struct PageTableEntry(pub usize);

impl PageTableEntry {
    pub fn raw(v: usize) -> Self {
        Self {
            0: v
        }
    }

    pub fn new(flags: PTEFlags, ppn: PhysicalPageNum) -> Self {
        Self {
            0: ppn.0 << 10 | flags.bits as usize,
        }
    }

    pub fn is_valid(&self) -> bool {
        (self.0 & 1) == 1
    }

    pub fn set_valid(&mut self) {
        self.0 |= 1;
    }

    pub fn ppn(&self) -> usize {
        (self.0 >> 10) & 0xFFFFFFFFFFF
    }

    pub fn write_ppn(&mut self, ppn: PhysicalPageNum) {
        self.0 &= 0b11_1111_1111; // empty possible writen ppn.
        self.0 |= (ppn.0 << 10);
    }
}

bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}