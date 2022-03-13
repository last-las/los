use crate::mm::page_table::{PageTable, PTEFlags};
use alloc::vec::Vec;
use crate::mm::frame_allocator::FrameTracker;
use crate::mm::address::VirtualAddress;

pub struct MemoryManager {
    page_table: PageTable,
    memory_regions: Vec<MemoryRegion>,
}

impl MemoryManager {
    pub fn new(elf_data: &[u8]) -> Self {
        // 1. 解析elf_data
        // 2. 对于每一个Segment的数据：
            // 获取对应大小的frame， 将数据写入frame，并基于page_table将相应的地址进行映射
        unimplemented!();
    }

    pub fn satp(&self) -> usize {
        unimplemented!();
    }
}

pub struct MemoryRegion {
    frames: Vec<FrameTracker>,
    region_type: RegionType,
    start: VirtualAddress,
    region_size: usize,
}

impl MemoryRegion {
    pub fn new(start: VirtualAddress, region_size: usize, region_type: RegionType) -> Self {
        Self {
            frames: Vec::new(),
            region_type,
            start,
            region_size,
        }
    }

    pub fn write_and_map_data(&self, data: &[u8], page_table: &mut PageTable) {
        let flags = self.region_type.permission();
        unimplemented!();
    }
}

pub enum RegionType {}

impl RegionType {
    pub fn permission(&self) -> PTEFlags {
        unimplemented!();
    }
}
