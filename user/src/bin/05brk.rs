#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::sys_brk;

const FRAME_SIZE: usize = 4096;

#[no_mangle]
fn main() {
    test_small_alloc();
    test_frame_alloc();
    test_invalid_read();
    panic!("brk test failed!");
}

fn test_small_alloc() {
    println!("test small alloc:");
    let cur_pos = sys_brk(None) as usize;
    println!("Before alloc, heap pos: {:#x}", cur_pos);

    for offset in (0..2 * FRAME_SIZE).step_by(64) {
        sys_brk(Some(cur_pos + offset));
        assert_eq!(cur_pos + offset, sys_brk(None) as usize);
    }
    let final_pos = sys_brk(Some(cur_pos)) as usize;
    println!("After dealloc, heap pos:{:#x}", final_pos);
    assert_eq!(cur_pos, final_pos);
}

fn test_frame_alloc() {
    println!("test frame alloc:");
    let cur_pos = sys_brk(None) as usize;
    println!("Before alloc, heap pos: {:#x}", cur_pos);

    for offset in 1 * FRAME_SIZE..5 * FRAME_SIZE {
        sys_brk(Some(cur_pos + offset));
        assert_eq!(cur_pos + offset, sys_brk(None) as usize);
        unsafe {
            let ptr = (cur_pos + offset - 1) as *mut u8;
            ptr.write(34);
        }
    }

    let final_pos = sys_brk(Some(cur_pos)) as usize;
    println!("After dealloc, heap pos:{:#x}", final_pos);
    assert_eq!(cur_pos, final_pos);
}

fn test_invalid_read() {
    println!("read invalid address:(LoadPageFault)");
    let cur_pos = sys_brk(None) as usize;

    unsafe {
        let val = (cur_pos as *const u8).read(); // LoadPageFault
        println!("{}", val);
    }
}