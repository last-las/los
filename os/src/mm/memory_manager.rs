use crate::mm::page_table::{PageTable, PTEFlags};
use alloc::vec::Vec;
use crate::mm::frame_allocator::FrameTracker;
use crate::mm::address::{VirtualAddress, VirtualPageNum};
use crate::config::{FRAME_SIZE, MAX_USER_ADDRESS};
use alloc::boxed::Box;
use core::fmt::{Debug, Formatter};
use crate::mm::{alloc_frame, address};

#[allow(unused)]
pub struct MemoryManager {
    pub page_table: PageTable,
    region_list: RegionList,
    brk_start: VirtualAddress,
    // the start of programme break. This value should not be changed after initializing
    brk: VirtualAddress,
}

impl MemoryManager {
    pub fn new(data: &[u8]) -> Option<(Self, usize, usize)> {
        let mut page_table = PageTable::new_user_table();
        let mut region_list = RegionList::empty();

        let elf = xmas_elf::ElfFile::new(data).unwrap();
        let elf_header = elf.header;
        if elf_header.pt1.magic != [0x7f, 0x45, 0x4c, 0x46] {
            return None;
        }

        let ph_count = elf_header.pt2.ph_count();
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() != xmas_elf::program::Type::Load {
                continue;
            }

            let start = VirtualAddress::new(ph.virtual_addr() as usize);
            let size = ph.mem_size() as usize;

            let mut flags = RegionFlags::empty();
            let ph_flags = ph.flags();
            if ph_flags.is_read() { flags |= RegionFlags::R; }
            if ph_flags.is_write() { flags |= RegionFlags::W; }
            if ph_flags.is_execute() { flags |= RegionFlags::X; }

            let mut memory_region = MemoryRegion::new(start, size, flags);

            let segment_data =
                &elf.input[ph.offset() as usize..(ph.offset() as usize + ph.file_size() as usize)];
            memory_region.map_and_fill(&mut page_table, segment_data);

            region_list.insert(Box::new(memory_region));
        }

        let stack_top = MAX_USER_ADDRESS;
        let mut stack_region = MemoryRegion::new(
            VirtualAddress::new(stack_top - FRAME_SIZE),
            FRAME_SIZE,
            RegionFlags::R | RegionFlags::W);
        stack_region.map_and_fill(&mut page_table, &[]);
        region_list.insert(Box::new(stack_region));

        let brk = region_list.find_unused_region(0).unwrap();

        let mem_manager = Self { page_table, region_list, brk_start: brk, brk };
        let pc = elf_header.pt2.entry_point() as usize;

        println!("{:#x}", mem_manager.page_table.find_pte(VirtualPageNum::new(0x1)).unwrap().0);

        Some((mem_manager, pc, stack_top))
    }

    #[allow(unused)]
    pub fn brk(&mut self, arg: Option<VirtualAddress>) -> VirtualAddress {
        if arg.is_none() {
            return self.brk;
        }

        let new_brk = arg.unwrap();
        if new_brk > self.brk {}
        unimplemented!();
    }

    pub fn satp(&self) -> usize {
        self.page_table.satp()
    }
}

pub struct RegionList {
    region_head: Option<Box<MemoryRegion>>,
    length: usize,
}

impl Debug for RegionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut head = self.region_head.as_ref();
        while head.is_some() {
            let inner = head.unwrap();
            f.write_fmt(format_args!("{:#?}->", inner))?;
            head = head.unwrap().next.as_ref();
        }
        f.write_fmt(format_args!("None"))?;
        Ok(())
    }
}

impl RegionList {
    pub fn empty() -> Self {
        Self {
            region_head: None,
            length: 0,
        }
    }

    pub fn insert(&mut self, mut region: Box<MemoryRegion>) {
        let front = self.find_front(region.start);
        if front.is_none() {
            region.next = self.region_head.take();
            self.region_head = Some(region);
        } else {
            let mut front = front.unwrap();
            region.next = front.next.take();
            front.next = Some(region);
        }

        self.length += 1;
        self.shrink();
    }

    pub fn find_unused_region(&self, size: usize) -> Option<VirtualAddress> {
        // The address has to be higher than .bss and lower than stack.
        if self.region_head.is_none() { //hasn't initialized yet thus return none
            return None;
        }

        let mut cur = self.region_head.as_ref().unwrap();
        while cur.next.is_some() {
            let next = cur.next.as_ref().unwrap();
            if cur.end().add(size) <= next.start {
                return Some(cur.end());
            }

            cur = next;
        }

        None
    }

    #[allow(unused)]
    pub fn delete(&mut self, va: VirtualAddress, size: usize) -> bool {
        unimplemented!();
    }

    fn find_front(&mut self, va: VirtualAddress) -> Option<&mut Box<MemoryRegion>> {
        if self.region_head.is_none() {
            return None;
        }

        let mut cur = self.region_head.as_mut().unwrap();
        let start = cur.start;
        let end = start.add(cur.region_size);
        if va < start {
            return None;
        }
        assert!(va >= end);

        loop {
            if cur.next.is_none() {
                break;
            }

            let next = cur.next.as_ref().unwrap();
            let start = next.start;
            let end = start.add(next.region_size);
            if va < start {
                break;
            }
            assert!(va >= end);
            cur = cur.next.as_mut().unwrap();
        }

        Some(cur)
    }

    fn shrink(&mut self) {
        if self.region_head.is_none() || self.region_head.as_ref().unwrap().next.is_none() {
            return;
        }
        let mut pre = self.region_head.as_mut().unwrap();
        let mut next = pre.next.as_ref().unwrap();

        loop {
            if pre.flags == next.flags && pre.end() == next.start { // merge
                let mut _next = pre.next.take().unwrap();
                pre.region_size += _next.region_size;
                pre.frames.append(&mut _next.frames);
                pre.next = _next.next.take();

                self.length -= 1;
            } else {
                pre = pre.next.as_mut().unwrap();
            }

            if pre.next.is_none() {
                break;
            }
            next = pre.next.as_ref().unwrap();
        }
    }
}


pub struct MemoryRegion {
    frames: Vec<FrameTracker>,
    start: VirtualAddress,
    region_size: usize,
    flags: RegionFlags,
    next: Option<Box<MemoryRegion>>,
}

impl Debug for MemoryRegion {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("Region(start: {:#?}, end:{:#?})", self.start, self.start.add(self.region_size)))
    }
}

impl MemoryRegion {
    pub fn new(start: VirtualAddress, region_size: usize, flags: RegionFlags) -> Self {
        Self {
            frames: Vec::new(),
            start,
            region_size: address::align(region_size),
            flags,
            next: None,
        }
    }

    pub fn map_and_fill(&mut self, page_table: &mut PageTable, data: &[u8]) {
        let start_vpn: VirtualPageNum = self.start.into();
        let end_vpn: VirtualPageNum = self.end().into();
        let mut flags = PTEFlags::V | PTEFlags::U;
        if self.flags.contains(RegionFlags::R) { flags |= PTEFlags::R };
        if self.flags.contains(RegionFlags::W) { flags |= PTEFlags::W };
        if self.flags.contains(RegionFlags::X) { flags |= PTEFlags::X };

        let mut start = 0;
        let len = data.len();
        for vpn in start_vpn..end_vpn {
            let mut frame = alloc_frame().unwrap();
            page_table.map(frame.0, vpn, flags);
            frame.fill_with(&data[start..len.min(start + FRAME_SIZE)]);
            self.frames.push(frame);

            if start + FRAME_SIZE >= len {
                start = len;
            } else {
                start += FRAME_SIZE;
            }
        }
    }

    pub fn end(&self) -> VirtualAddress {
        self.start.add(self.region_size)
    }
}

bitflags! {
    pub struct RegionFlags: u8 {
        const R = 1 << 0;
        const W = 1 << 1;
        const X = 1 << 2;
    }
}

#[cfg(test)]
mod test {
    use crate::mm::memory_manager::{MemoryRegion, RegionFlags, RegionList};
    use crate::mm::address::VirtualAddress;
    use crate::config::FRAME_SIZE;
    use alloc::boxed::Box;

    #[test]
    pub fn test_insert_and_shrink_on_region_list() {
        let start = VirtualAddress::new(0x80200000);
        let region1 = MemoryRegion::new(
            start.add(FRAME_SIZE * 0), FRAME_SIZE,
            RegionFlags::R
        );
        let region2 = MemoryRegion::new(
            start.add(FRAME_SIZE * 1), FRAME_SIZE,
            RegionFlags::R
        );
        let region3 = MemoryRegion::new(
            start.add(FRAME_SIZE * 2), FRAME_SIZE,
            RegionFlags::R
        );
        let region4 = MemoryRegion::new(
            start.add(FRAME_SIZE * 3), FRAME_SIZE,
            RegionFlags::R
        );
        let region5 = MemoryRegion::new(
            start.add(FRAME_SIZE * 4), FRAME_SIZE,
            RegionFlags::R
        );

        let mut region_list = RegionList::empty();
        region_list.insert(Box::new(region1));
        assert_eq!(region_list.length, 1);
        region_list.insert(Box::new(region3));
        assert_eq!(region_list.length, 2);
        region_list.insert(Box::new(region5));
        assert_eq!(region_list.length, 3);
        region_list.insert(Box::new(region2));
        assert_eq!(region_list.length, 2);
        region_list.insert(Box::new(region4));
        assert_eq!(region_list.length, 1);
    }
}
