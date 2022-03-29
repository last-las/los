#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::brk;
use share::syscall::error::SysError;

const FRAME_SIZE: usize = 4096;

#[no_mangle]
fn main() {
    test_small_alloc().unwrap();
    test_frame_alloc().unwrap();
    test_invalid_read();
    panic!("brk test failed!");
}

fn test_small_alloc() -> Result<usize, SysError> {
    println!("test small alloc:");
    let cur_pos = brk(None)?;
    println!("Before alloc, heap pos: {:#x}", cur_pos);

    for offset in (0..2 * FRAME_SIZE).step_by(64) {
        brk(Some(cur_pos + offset))?;
        assert_eq!(cur_pos + offset, brk(None)?);
    }
    let final_pos = brk(Some(cur_pos))?;
    println!("After dealloc, heap pos:{:#x}", final_pos);
    assert_eq!(cur_pos, final_pos);
    Ok(0)
}

fn test_frame_alloc() -> Result<usize, SysError> {
    println!("test frame alloc:");
    let cur_pos = brk(None)?;
    println!("Before alloc, heap pos: {:#x}", cur_pos);

    for offset in 1 * FRAME_SIZE..5 * FRAME_SIZE {
        brk(Some(cur_pos + offset))?;
        assert_eq!(cur_pos + offset, brk(None)?);
        unsafe {
            let ptr = (cur_pos + offset - 1) as *mut u8;
            ptr.write(34);
        }
    }

    let final_pos = brk(Some(cur_pos))?;
    println!("After dealloc, heap pos:{:#x}", final_pos);
    assert_eq!(cur_pos, final_pos);

    Ok(0)
}

fn test_invalid_read() {
    println!("read invalid address:(should trigger LoadPageFault)");
    let cur_pos = brk(None).unwrap();

    unsafe {
        let val = (cur_pos as *const u8).read(); // LoadPageFault
        println!("{}", val);
    }

}