use crate::config::{
    FRAME_SIZE, KERNEL_MAPPING_OFFSET, RAM_MAPPING_OFFSET, RAM_SIZE, RAM_START_ADDRESS,
    UART_BASE_ADDRESS, VIRTIO0_START_ADDRESS,
};
use crate::kmain;
use crate::mm::address::PhysicalAddress;
use crate::mm::alloc_frame;
use crate::mm::heap::stupid_allocator::StupidAllocator;
use crate::mm::page_table::{PTEFlags, PageTable};
use crate::mm::FRAME_ALLOCATOR;
use crate::plic::PLIC_START_ADDRESS;
use crate::processor::suspend_current_hart;
use crate::sbi::sbi_shutdown;
use core::arch::asm;

extern "C" {
    pub fn __kernel_start();
    pub fn __kernel_end();
    pub fn __text_start();
    pub fn __text_end();
    pub fn __rodata_start();
    pub fn __rodata_end();
    pub fn __data_start();
    pub fn __data_end();
    pub fn __bss_start();
    pub fn __bss_end();
}

pub static mut KERNEL_SATP: usize = 0;

#[no_mangle]
#[allow(unreachable_code)]
#[link_section = ".text.paging"]
pub extern "C" fn enable_paging(hart_id: usize, device_tree: usize) {
    if hart_id != 0 {
        suspend_current_hart();
    } else {
        let start = PhysicalAddress::new(__kernel_end as usize);
        let end = PhysicalAddress::new(RAM_START_ADDRESS + RAM_SIZE);
        let mut frame_allocator = FRAME_ALLOCATOR.lock();
        frame_allocator.init(start, end);
        drop(frame_allocator);

        let tmp_heap_frame = alloc_frame().unwrap();
        tmp_heap_frame.fill_with(&[]);
        let tmp_heap_allocator = StupidAllocator::new(tmp_heap_frame.0 .0 << 12, FRAME_SIZE);
        let mut root_table = PageTable::new_kernel_table(tmp_heap_allocator).unwrap();

        unsafe {
            // higher half kernel
            root_table
                .map_with_offset(
                    __text_start as usize,
                    __text_end as usize,
                    KERNEL_MAPPING_OFFSET,
                    PTEFlags::V | PTEFlags::R | PTEFlags::X,
                )
                .unwrap();
            root_table
                .map_with_offset(
                    __rodata_start as usize,
                    __rodata_end as usize,
                    KERNEL_MAPPING_OFFSET,
                    PTEFlags::V | PTEFlags::R,
                )
                .unwrap();
            root_table
                .map_with_offset(
                    __data_start as usize,
                    __data_end as usize,
                    KERNEL_MAPPING_OFFSET,
                    PTEFlags::V | PTEFlags::R | PTEFlags::W,
                )
                .unwrap();
            root_table
                .map_with_offset(
                    __bss_start as usize,
                    __bss_end as usize,
                    KERNEL_MAPPING_OFFSET,
                    PTEFlags::V | PTEFlags::R | PTEFlags::W,
                )
                .unwrap();

            // ram mapping
            root_table
                .map_with_offset(
                    RAM_START_ADDRESS,
                    RAM_START_ADDRESS + RAM_SIZE,
                    RAM_MAPPING_OFFSET,
                    PTEFlags::V | PTEFlags::R | PTEFlags::W,
                )
                .unwrap();
            // uart mapping
            root_table
                .map_with_offset(
                    UART_BASE_ADDRESS,
                    UART_BASE_ADDRESS + FRAME_SIZE,
                    RAM_MAPPING_OFFSET,
                    PTEFlags::V | PTEFlags::R | PTEFlags::W,
                )
                .unwrap();
            // plic mapping
            root_table
                .map_with_offset(
                    PLIC_START_ADDRESS,
                    PLIC_START_ADDRESS + 0x100_0000,
                    RAM_MAPPING_OFFSET,
                    PTEFlags::V | PTEFlags::R | PTEFlags::W,
                )
                .unwrap();
            root_table
                .map_with_offset(
                    PLIC_START_ADDRESS,
                    PLIC_START_ADDRESS + 0x100_0000,
                    RAM_MAPPING_OFFSET,
                    PTEFlags::V | PTEFlags::R | PTEFlags::W,
                )
                .unwrap();
            // virtio mapping
            root_table
                .map_with_offset(
                    VIRTIO0_START_ADDRESS,
                    VIRTIO0_START_ADDRESS + FRAME_SIZE,
                    RAM_MAPPING_OFFSET,
                    PTEFlags::V | PTEFlags::R | PTEFlags::W,
                )
                .unwrap();

            // set global satp for all harts
            KERNEL_SATP = root_table.satp();
        }

        core::mem::forget(root_table);
        core::mem::drop(tmp_heap_frame); // This frame is no longer useful.
    }

    unsafe {
        asm! {
        "csrw stvec, {stvec}",
        "add sp, sp, {k_offset}",
        "csrw satp, {satp}",
        "sfence.vma",
        "call kmain", // When pc runs here, load fault occurs and stvec will be set to pc,
                      // so this instruction will never be executed.
        stvec = in(reg) kmain as usize + KERNEL_MAPPING_OFFSET,
        k_offset = in(reg) KERNEL_MAPPING_OFFSET,
        satp = in(reg) 8 << 60 | KERNEL_SATP,

        in("a0") hart_id,
        in("a1") device_tree,
        options(noreturn, nostack),
        }
    }

    panic!("never gonna reach here!");
}
