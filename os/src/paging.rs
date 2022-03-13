use crate::sbi::{sbi_console_putchar, sbi_shutdown};
use crate::config::{FRAME_SIZE, RAM_SIZE, RAM_START_ADDRESS, KERNEL_OFFSET};
use crate::mm::{stupid_allocator, BitMapFrameAllocator, alloc_frame};
use crate::mm::stupid_allocator::StupidAllocator;
use crate::mm::FRAME_ALLOCATOR;
use crate::mm::page_table::{PageTable, PTEFlags};
use crate::mm::address::{PhysicalAddress, PhysicalPageNum, VirtualAddress, VirtualPageNum};
use spin::Mutex;
use riscv::register::{satp, stvec, sstatus, sie, sip};
use crate::kmain;
use crate::processor::suspend_current_hart;

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

#[no_mangle]
#[link_section = ".text.paging"]
 pub extern "C" fn enable_paging(hart_id: usize, device_tree: usize) {
    let mut frame_allocator = FRAME_ALLOCATOR.lock();
    let start = unsafe { PhysicalAddress::new(__kernel_end as usize) };
    let end = PhysicalAddress::new(RAM_START_ADDRESS + RAM_SIZE);
    // println!("{:#?} {:#?}", start, end);
    frame_allocator.init(start, end);
    drop(frame_allocator);

    let tmp_heap_frame = alloc_frame().unwrap();
    let tmp_heap_allocator = StupidAllocator::new(tmp_heap_frame.0.0 << 12, FRAME_SIZE);
    let mut root_table = PageTable::new_kernel_table(tmp_heap_allocator);

    // higher half kernel
    unsafe {
        root_table.map_a_segment(__text_start as usize, __text_end as usize, KERNEL_OFFSET,
                                 PTEFlags::V | PTEFlags::R | PTEFlags::X);
        root_table.map_a_segment(__rodata_start as usize, __rodata_end as usize, KERNEL_OFFSET,
                                 PTEFlags::V | PTEFlags::R);
        root_table.map_a_segment(__data_start as usize, __data_end as usize, KERNEL_OFFSET,
                                 PTEFlags::V | PTEFlags::R | PTEFlags::W);
        root_table.map_a_segment(__bss_start as usize, __bss_end as usize, KERNEL_OFFSET,
                                 PTEFlags::V | PTEFlags::R | PTEFlags::W);
    }

/*    unsafe {
        root_table.map_a_segment(__text_start as usize, __text_end as usize, 0,
                                 PTEFlags::V | PTEFlags::R | PTEFlags::X);
        root_table.map_a_segment(__rodata_start as usize, __rodata_end as usize, 0,
                                 PTEFlags::V | PTEFlags::R);
        root_table.map_a_segment(__data_start as usize, __data_end as usize, 0,
                                 PTEFlags::V | PTEFlags::R | PTEFlags::W);
        root_table.map_a_segment(__bss_start as usize, __bss_end as usize, 0,
                                 PTEFlags::V | PTEFlags::R | PTEFlags::W);
    }*/

    assert_eq!(
        root_table.find(VirtualPageNum::new((kmain as usize + KERNEL_OFFSET) >> 12)).unwrap().0,
        kmain as usize >> 12
    );

    unsafe {
        stvec::write(kmain as usize + KERNEL_OFFSET, stvec::TrapMode::Direct);
    }
    let satp = root_table.satp();
    core::mem::forget(root_table);

    unsafe {
        asm! {
        "add sp, sp, {sp_bias}",
        // "sfence.vma",
        // "csrw satp, {satp}",
        "call kmain",
        sp_bias = in(reg) KERNEL_OFFSET,
        // satp = in(reg) 8 << 60 | satp,

        in("a0") hart_id,
        in("a1") device_tree,
        options(noreturn, nostack),
        }
    }

    sbi_shutdown();
}


pub fn println(info: &str) {
    for chr in info.chars() {
        sbi_console_putchar(chr);
    }
    sbi_console_putchar(0xa as char);
}