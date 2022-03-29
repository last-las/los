use crate::mm::page_table::{PageTable, PTEFlags};
use alloc::vec::Vec;
use crate::mm::frame_allocator::FrameTracker;
use crate::mm::address::{VirtualAddress, VirtualPageNum, PhysicalAddress};
use crate::config::{FRAME_SIZE, MAX_USER_ADDRESS};
use alloc::boxed::Box;
use core::fmt::{Debug, Formatter};
use crate::mm::{alloc_frame, address};
use core::arch::asm;
use share::syscall::error::{SysError, EACCES};

#[allow(unused)]
pub struct MemoryManager {
    pub page_table: PageTable,
    pub region_list: RegionList,
    pub brk_start: VirtualAddress,
    // the start of programme break. This value should not be changed after initializing
    pub brk: VirtualAddress,
}

impl MemoryManager {
    pub fn new(data: &[u8]) -> Result<(Self, usize, usize), SysError> {
        let page_table = PageTable::new_user_table()?;
        let region_list = RegionList::empty();
        let mut mem_manager = MemoryManager {
            page_table,
            region_list,
            brk_start: VirtualAddress::new(0),
            brk: VirtualAddress::new(0),
        };

        let elf = xmas_elf::ElfFile::new(data).unwrap();
        let elf_header = elf.header;
        if elf_header.pt1.magic != [0x7f, 0x45, 0x4c, 0x46] {
            return Err(SysError::new(EACCES));
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

            let segment_data: &[u8] =
                &elf.input[ph.offset() as usize..(ph.offset() as usize + ph.file_size() as usize)];
            mem_manager.add_area(start, address::ceil(size), flags, Some(segment_data))?;
        }

        let stack_top = MAX_USER_ADDRESS;
        mem_manager.add_area(
            VirtualAddress::new(stack_top - FRAME_SIZE), FRAME_SIZE,
            RegionFlags::R | RegionFlags::W, None,
        )?;

        let brk = mem_manager.region_list.find_unused_region(1).unwrap();
        mem_manager.brk = brk;
        mem_manager.brk_start = brk;

        let pc = elf_header.pt2.entry_point() as usize;
        Ok((mem_manager, pc, stack_top))
    }

    pub fn clone(&self) -> Result<Self, SysError> {
        let mut page_table = PageTable::new_user_table()?;
        let mut region_list = RegionList::empty();
        for mem in self.region_list.iter() {
            let new_region = mem.clone_with_new_frames()?;
            new_region.mapped_by(&mut page_table)?;
            region_list.insert(Box::new(new_region));
        }

        Ok(
            Self {
                page_table,
                region_list,
                brk_start: self.brk_start,
                brk: self.brk,
            }
        )
    }

    pub fn add_area(&mut self, start: VirtualAddress, size: usize, flags: RegionFlags, data: Option<&[u8]>) -> Result<(), SysError> {
        let mut memory_region = MemoryRegion::new(start, size, flags);
        let data = match data {
            Some(data) => data,
            None => &[]
        };
        memory_region.fill(data)?;
        memory_region.mapped_by(&mut self.page_table)?;

        self.region_list.insert(Box::new(memory_region));

        Ok(())
    }

    pub fn delete_area(&mut self, start: VirtualAddress, size: usize) -> bool {
        if self.region_list.is_region_exists(start, size) {
            assert!(self.region_list.delete(start, size));
            self.unmap_area(start, size);
            true
        } else {
            false
        }
    }

    fn unmap_area(&mut self, start: VirtualAddress, size: usize) {
        let start_vpn = start.into();
        let end_vpn = start.add(size).into();
        for vpn in start_vpn..end_vpn {
            self.page_table.unmap(vpn);
            if vpn.0 == 0x5 {
                continue;
            }

            unsafe {
                asm!(
                "sfence.vma {}, x0",
                in(reg) vpn.0 << 12,
                );
            }
        }
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
        let front = self.find_last_region_before(region.start);
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

    pub fn is_region_exists(&self, mut region_start: VirtualAddress, size: usize) -> bool {
        let region_end = region_start.add(size);
        if self.region_head.is_none() {
            return false;
        }
        let mut cur_region = self.region_head.as_ref().unwrap();
        while !cur_region.contain(region_start) {
            if cur_region.next.is_none() {
                return false;
            }

            cur_region = cur_region.next.as_ref().unwrap();
        }

        while cur_region.contain(region_start) {
            if cur_region.contain(region_end.minus(1)) {
                return true;
            }
            if cur_region.next.is_none() {
                break;
            }
            region_start = cur_region.end();
            cur_region = cur_region.next.as_ref().unwrap();
        }
        return false;
    }

    pub fn delete(&mut self, mut cur_del_start: VirtualAddress, mut size: usize) -> bool {
        if self.region_head.is_none() {
            return false;
        }

        let cur_region_wrapped = self.find_first_region_containing(cur_del_start);
        if cur_region_wrapped.is_none() { return false; }

        let mut cur_region = cur_region_wrapped.unwrap();
        while cur_region.contain(cur_del_start) { // delete the region
            let gap = cur_region.end().0 - cur_del_start.0;
            if gap >= size {
                let re = cur_region.delete(cur_del_start, size);
                if re {
                    self.length += 1;
                }
                break;
            }
            cur_region.delete(cur_del_start, gap);
            cur_del_start = cur_del_start.add(gap);
            size -= gap;

            let next_wrapped = cur_region.next.as_mut();
            if next_wrapped.is_none() { // Though size is not zero.. break here anyway
                break;
            }
            cur_region = next_wrapped.unwrap();
        }

        self.remove_empty_region();
        true
    }

    pub fn iter(&self) -> RegionListIter {
        RegionListIter::new(self.region_head.as_ref())
    }

    fn find_first_region_containing(&mut self, va: VirtualAddress) -> Option<&mut Box<MemoryRegion>> {
        if self.region_head.is_none() { return None; }

        let mut start = self.region_head.as_mut().unwrap();
        while !start.contain(va) {
            let next_wrapped = start.next.as_mut();
            if next_wrapped.is_none() {
                return None;
            }
            start = next_wrapped.unwrap();
        }

        Some(start)
    }

    fn remove_empty_region(&mut self) {
        if self.region_head.is_none() { return; }

        let mut start = self.region_head.as_mut().unwrap();
        while start.next.is_some() {
            let ref_next = start.next.as_ref().unwrap();
            if ref_next.region_size == 0 {
                let next = start.next.take().unwrap();
                start.next = next.next;
                self.length -= 1;
                if start.next.is_none() { break; }
            } else {
                start = start.next.as_mut().unwrap();
            }
        }

        let start = self.region_head.as_mut().unwrap();
        if start.region_size == 0 {
            self.region_head = start.next.take();
            self.length -= 1;
        }
    }

    fn find_last_region_before(&mut self, va: VirtualAddress) -> Option<&mut Box<MemoryRegion>> {
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

    #[cfg(test)]
    fn length(&self) -> usize {
        let mut length = 0;
        let mut cur = self.region_head.as_ref();

        while cur.is_some() {
            length += 1;
            cur = cur.unwrap().next.as_ref();
        }

        length
    }
}

pub struct RegionListIter<'a> {
    current_region: Option<&'a Box<MemoryRegion>>,
}

impl<'a> RegionListIter<'a> {
    pub fn new(current_region: Option<&'a Box<MemoryRegion>>) -> Self {
        Self {
            current_region
        }
    }
}

impl<'a> Iterator for RegionListIter<'a> {
    type Item = &'a Box<MemoryRegion>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        if self.current_region.is_some() {
            result = self.current_region.take();
            self.current_region = result.as_ref().unwrap().next.as_ref();
        }
        result
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
        f.write_fmt(format_args!("Region(start:{:#x}, end:{:#x})", self.start.0, self.start.add(self.region_size - 1).0))
    }
}

impl MemoryRegion {
    pub fn new(start: VirtualAddress, region_size: usize, flags: RegionFlags) -> Self {
        Self {
            frames: Vec::new(),
            start,
            region_size,
            flags,
            next: None,
        }
    }

    pub fn clone_with_new_frames(&self) -> Result<Self, SysError> {
        let mut frames = Vec::new();
        for i in 0..self.region_size / FRAME_SIZE {
            let frame = alloc_frame()?;
            let frame_data: &[u8; FRAME_SIZE] = PhysicalAddress::from(self.frames[i].0).as_mut();
            frame.fill_with(frame_data);

            frames.push(frame);
        }

        Ok(
            Self {
                frames,
                start: self.start,
                region_size: self.region_size,
                flags: self.flags,
                next: None,
            }
        )
    }

    pub fn fill(&mut self, data: &[u8]) -> Result<(), SysError> {
        let mut start = 0;
        let len = data.len();
        for _ in (0..self.region_size).step_by(FRAME_SIZE) {
            let mut frame = alloc_frame()?;
            frame.fill_with(&data[start..len.min(start + FRAME_SIZE)]);
            self.frames.push(frame);

            if start + FRAME_SIZE >= len {
                start = len;
            } else {
                start += FRAME_SIZE;
            }
        }

        Ok(())
    }

    pub fn mapped_by(&self, page_table: &mut PageTable) -> Result<(), SysError> {
        let mut flags = PTEFlags::V | PTEFlags::U;
        if self.flags.contains(RegionFlags::R) { flags |= PTEFlags::R };
        if self.flags.contains(RegionFlags::W) { flags |= PTEFlags::W };
        if self.flags.contains(RegionFlags::X) { flags |= PTEFlags::X };

        let start_vpn: VirtualPageNum = self.start.into();
        let end_vpn: VirtualPageNum = self.end().into();
        let mut frame_iter = self.frames.iter();

        for vpn in start_vpn..end_vpn {
            let frame = frame_iter.next().unwrap();
            page_table.map(frame.0, vpn, flags)?;
        }

        Ok(())
    }

    /*pub fn map_and_fill(&mut self, page_table: &mut PageTable, data: &[u8]) {
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
    }*/

    pub fn delete(&mut self, del_region_start: VirtualAddress, size: usize) -> bool {
        let del_region_end = del_region_start.add(size);
        assert!(del_region_start.is_aligned() && del_region_end.is_aligned());
        assert!(del_region_end <= self.end());

        let mut is_new_region = false;
        let start_index = (del_region_start.0 - self.start.0) >> 12;
        let end_index = (del_region_end.0 - self.start.0) >> 12;
        let deleted_frames;

        if del_region_end == self.end() { // delete at the end
            deleted_frames = self.frames.drain(start_index..);

            self.region_size -= size;
        } else if del_region_start > self.start { // delete in the mid
            let new_region_size = self.end().0 - del_region_end.0;
            let remained_frames: Vec<FrameTracker> = self.frames.drain(end_index..).collect();
            deleted_frames = self.frames.drain(start_index..);

            let mut next_region = MemoryRegion::new(del_region_end, new_region_size, self.flags);
            next_region.frames = remained_frames;
            next_region.next = self.next.take();

            self.next = Some(Box::new(next_region));
            self.region_size -= size + new_region_size;
            is_new_region = true;
        } else { // delete from start
            assert_eq!(del_region_start, self.start);
            deleted_frames = self.frames.drain(..end_index);

            self.start = del_region_end;
            self.region_size -= size;
        }
        assert_eq!(deleted_frames.len(), size >> 12);
        is_new_region
    }

    pub fn end(&self) -> VirtualAddress {
        self.start.add(self.region_size)
    }

    pub fn contain(&self, va: VirtualAddress) -> bool {
        va >= self.start && va < self.end()
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
    use crate::mm::address::{VirtualAddress, VirtualPageNum, PhysicalAddress, ceil};
    use crate::mm::frame_allocator::FRAME_ALLOCATOR;
    use crate::config::FRAME_SIZE;
    use alloc::boxed::Box;
    use crate::mm::page_table::PageTable;
    use share::syscall::error::SysError;

    const REGION_SIZE: usize = FRAME_SIZE * 20;

    #[test]
    pub fn test_delete_front_on_memory_region() {
        let _ = init_frame_allocator();

        let mut page_table = PageTable::new().unwrap();
        let start = VirtualAddress::new(0);
        let size = FRAME_SIZE * 5;
        let mut memory_region = MemoryRegion::new(start, size, RegionFlags::R);
        memory_region.fill(&[]).unwrap();
        memory_region.mapped_by(&mut page_table).unwrap();
        memory_region.delete(start, FRAME_SIZE);

        assert_eq!(memory_region.start.0, FRAME_SIZE);
        assert!(memory_region.next.is_none());
        assert_eq!(memory_region.region_size, size - FRAME_SIZE);
        assert_eq!(memory_region.frames.len(), 5 - 1);

        let mut vpn = VirtualPageNum::new(1);
        for frame in memory_region.frames.iter() {
            assert_eq!(frame.0, page_table.translate(vpn).unwrap());
            vpn = vpn.add(1);
        }
    }

    #[test]
    pub fn test_delete_middle_on_memory_region() {
        let _ = init_frame_allocator();

        let mut page_table = PageTable::new().unwrap();
        let start = VirtualAddress::new(0);
        let size = FRAME_SIZE * 5;
        let mut memory_region = MemoryRegion::new(start, size, RegionFlags::R);
        memory_region.fill(&[]).unwrap();
        memory_region.mapped_by(&mut page_table).unwrap();
        memory_region.delete(start.add(FRAME_SIZE * 2), FRAME_SIZE);

        assert_eq!(memory_region.start.0, 0);
        assert!(memory_region.next.is_some());
        assert_eq!(memory_region.region_size, FRAME_SIZE * 2);
        assert_eq!(memory_region.frames.len(), 2);

        let mut vpn = VirtualPageNum::new(0);
        for frame in memory_region.frames.iter() {
            assert_eq!(frame.0, page_table.translate(vpn).unwrap());
            vpn = vpn.add(1);
        }

        let next_region = memory_region.next.unwrap();
        assert_eq!(next_region.start.0, FRAME_SIZE * 3);
        assert!(next_region.next.is_none());
        assert_eq!(next_region.region_size, FRAME_SIZE * 2);
        assert_eq!(next_region.frames.len(), 2);

        let mut vpn = VirtualPageNum::new(3);
        for frame in next_region.frames.iter() {
            assert_eq!(frame.0, page_table.translate(vpn).unwrap());
            vpn = vpn.add(1);
        }
    }

    #[test]
    pub fn test_delete_tail_on_memory_region() {
        let _ = init_frame_allocator();

        let mut page_table = PageTable::new().unwrap();
        let start = VirtualAddress::new(0);
        let size = FRAME_SIZE * 5;
        let mut memory_region = MemoryRegion::new(start, size, RegionFlags::R);
        memory_region.fill(&[]).unwrap();
        memory_region.mapped_by(&mut page_table).unwrap();
        memory_region.delete(start.add(FRAME_SIZE * 4), FRAME_SIZE);

        assert_eq!(memory_region.start.0, 0);
        assert!(memory_region.next.is_none());
        assert_eq!(memory_region.region_size, size - FRAME_SIZE);
        assert_eq!(memory_region.frames.len(), 5 - 1);

        let mut vpn = VirtualPageNum::new(0);
        for frame in memory_region.frames.iter() {
            assert_eq!(frame.0, page_table.translate(vpn).unwrap());
            vpn = vpn.add(1);
        }
    }

    #[test]
    pub fn test_insert_and_shrink_on_region_list() {
        let start = VirtualAddress::new(0x80200000);
        let region1 = MemoryRegion::new(
            start.add(FRAME_SIZE * 0), FRAME_SIZE,
            RegionFlags::R,
        );
        let region2 = MemoryRegion::new(
            start.add(FRAME_SIZE * 1), FRAME_SIZE,
            RegionFlags::R,
        );
        let region3 = MemoryRegion::new(
            start.add(FRAME_SIZE * 2), FRAME_SIZE,
            RegionFlags::R,
        );
        let region4 = MemoryRegion::new(
            start.add(FRAME_SIZE * 3), FRAME_SIZE,
            RegionFlags::R,
        );
        let region5 = MemoryRegion::new(
            start.add(FRAME_SIZE * 4), FRAME_SIZE,
            RegionFlags::R,
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

    #[test]
    pub fn test_delete_single_region_on_region_list() {
        let _ = init_frame_allocator();
        let mut page_table = PageTable::new().unwrap();
        let mut region_list = create_a_testing_region_list(&mut page_table).unwrap();

        // 1. delete second region
        let va = VirtualAddress::new(FRAME_SIZE * 2);
        assert!(region_list.delete(va, FRAME_SIZE * 3));
        assert_eq!(region_list.length, 3);
        assert_eq!(region_list.length(), 3);

        // 2. delete fourth region
        let va = VirtualAddress::new(FRAME_SIZE * 7);
        assert!(region_list.delete(va, FRAME_SIZE * 2));
        assert_eq!(region_list.length, 2);
        assert_eq!(region_list.length(), 2);

        // 3. delete first region
        let va = VirtualAddress::new(0);
        assert!(region_list.delete(va, FRAME_SIZE * 2));
        assert_eq!(region_list.length(), 1);
        assert_eq!(region_list.length, 1);
    }

    #[test]
    pub fn test_delete_part_of_region_on_region_list() {
        let _ = init_frame_allocator();
        let mut page_table = PageTable::new().unwrap();
        let mut region_list = create_a_testing_region_list(&mut page_table).unwrap();

        // 1. delete 3rd frame
        let va = VirtualAddress::new(FRAME_SIZE * 3);
        assert!(region_list.delete(va, FRAME_SIZE));
        assert_eq!(region_list.length, 5);
        assert_eq!(region_list.length(), 5);

        // 2. delete 1st frame
        let va = VirtualAddress::new(0);
        assert!(region_list.delete(va, FRAME_SIZE));
        assert_eq!(region_list.length, 5);
        assert_eq!(region_list.length(), 5);

        // 3. delete 8th frame
        let va = VirtualAddress::new(FRAME_SIZE * 8);
        assert!(region_list.delete(va, FRAME_SIZE));
        assert_eq!(region_list.length, 5);
        assert_eq!(region_list.length(), 5);
    }

    #[test]
    pub fn test_is_region_exist_on_region_list() {
        let _ = init_frame_allocator();
        let mut page_table = PageTable::new().unwrap();
        let mut region_list = create_a_testing_region_list(&mut page_table).unwrap();

        let start = VirtualAddress::new(0);
        let size = 9 * FRAME_SIZE;
        assert!(region_list.is_region_exists(start, size));

        // del 4th frame
        let del_start = VirtualAddress::new(FRAME_SIZE * 3);
        let del_size = FRAME_SIZE;
        assert!(region_list.delete(del_start, del_size));

        // 1st-9th frames should not exist because of missing 4th.
        assert!(!region_list.is_region_exists(start, size));
        // 1st-4th and 4th-9th frames should not exist.
        assert!(!region_list.is_region_exists(VirtualAddress::new(0), FRAME_SIZE * 4));
        assert!(!region_list.is_region_exists(VirtualAddress::new(FRAME_SIZE * 3), FRAME_SIZE * 6));
        // 1st-3rd and 5th-9th frames should exist
        assert!(region_list.is_region_exists(VirtualAddress::new(0), FRAME_SIZE * 3));
        assert!(region_list.is_region_exists(VirtualAddress::new(FRAME_SIZE * 4), FRAME_SIZE * 5));
    }

    #[test]
    pub fn test_delete_continuous_regions_on_region_list() {
        let _ = init_frame_allocator();
        let mut page_table = PageTable::new().unwrap();
        let mut region_list = create_a_testing_region_list(&mut page_table).unwrap();

        // 1. delete 2nd,3rd frames
        let va = VirtualAddress::new(FRAME_SIZE);
        assert!(region_list.delete(va, FRAME_SIZE * 2));
        assert_eq!(region_list.length, 4);
        assert_eq!(region_list.length(), 4);

        // 2. delete 5th-9th frames
        let va = VirtualAddress::new(FRAME_SIZE * 4);
        assert!(region_list.delete(va, FRAME_SIZE * 5));
        assert_eq!(region_list.length, 2);
        assert_eq!(region_list.length(), 2);
    }

    #[test]
    pub fn test_iter_on_region_list() {
        let _ = init_frame_allocator();
        let mut page_table = PageTable::new().unwrap();
        let mut region_list = create_a_testing_region_list(&mut page_table).unwrap();
        let mut region_iter = region_list.iter();
        let region = region_iter.next().unwrap();
        assert_eq!(region.start, VirtualAddress::new(0));
        assert_eq!(region.region_size, 2 * FRAME_SIZE);

        let region = region_iter.next().unwrap();
        assert_eq!(region.start, VirtualAddress::new(2 * FRAME_SIZE));
        assert_eq!(region.region_size, 3 * FRAME_SIZE);

        let region = region_iter.next().unwrap();
        assert_eq!(region.start, VirtualAddress::new(5 * FRAME_SIZE));
        assert_eq!(region.region_size, 2 * FRAME_SIZE);

        let region = region_iter.next().unwrap();
        assert_eq!(region.start, VirtualAddress::new(7 * FRAME_SIZE));
        assert_eq!(region.region_size, 2 * FRAME_SIZE);

        assert!(region_iter.next().is_none());
    }

    // TODO: test region_list's sortable feature
    fn init_frame_allocator() -> Box<[u8; REGION_SIZE]> {
        let frame_region: Box<[u8; REGION_SIZE]> = Box::new([0; REGION_SIZE]);
        let start = ceil(frame_region.as_ptr() as usize);
        let end = start + REGION_SIZE - FRAME_SIZE;
        let mut inner = FRAME_ALLOCATOR.lock();
        inner.init(PhysicalAddress::new(start), PhysicalAddress::new(end));

        frame_region
    }

    fn create_a_testing_region_list(page_table: &mut PageTable) -> Result<RegionList, SysError> {
        /*
            create a region list like this: (a region is described as "[start, end, RegionFlags]")
            [0, 8k, R | W] -> [8k, 20k, W] -> [20k, 28k, X] -> [28k, 36k, R]
        */
        let mut region_list = RegionList::empty();

        let mut va = VirtualAddress::new(0);
        let size = 2 * FRAME_SIZE;
        let mut region = MemoryRegion::new(va, size, RegionFlags::R | RegionFlags::W);
        region.fill(&[])?;
        region.mapped_by(page_table)?;
        region_list.insert(Box::new(region));
        va = va.add(size);

        let size = 3 * FRAME_SIZE;
        let mut region = MemoryRegion::new(va, size, RegionFlags::W);
        region.fill(&[])?;
        region.mapped_by(page_table)?;
        region_list.insert(Box::new(region));
        va = va.add(size);

        let size = 2 * FRAME_SIZE;
        let mut region = MemoryRegion::new(va, size, RegionFlags::X);
        region.fill(&[])?;
        region.mapped_by(page_table)?;
        region_list.insert(Box::new(region));
        va = va.add(size);

        let size = 2 * FRAME_SIZE;
        let mut region = MemoryRegion::new(va, size, RegionFlags::R);
        region.fill(&[])?;
        region.mapped_by(page_table)?;
        region_list.insert(Box::new(region));
        va = va.add(size);

        assert_eq!(region_list.length, 4);

        Ok(region_list)
    }
}
