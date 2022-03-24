mod ipc;

use crate::task::{exit_current_and_run_next_task, stop_current_and_run_next_task};
use crate::processor::{get_hart_id, get_cur_task_in_this_hart};
use core::str::from_utf8;
use crate::timer::get_time_ms;
use crate::syscall::ipc::{sys_send, sys_receive};
use crate::mm::address::{VirtualAddress, PhysicalAddress, ceil};
use crate::mm::memory_manager::RegionFlags;
use crate::config::{MMAP_START_ADDRESS, FRAME_SIZE};

const SYSCALL_SEND: usize = 1;
const SYSCALL_RECEIVE: usize = 2;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_BRK: usize = 214;
const SYSCALL_TEST: usize = 1234;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_SEND => sys_send(args[0], args[1]),
        SYSCALL_RECEIVE => sys_receive(args[0], args[1]),
        SYSCALL_WRITE => sys_write(args[0], VirtualAddress::new(args[1]), args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as isize),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(),
        SYSCALL_BRK => sys_brk(VirtualAddress::new(args[0])),
        SYSCALL_TEST =>  sys_test(),
        _ => {
            panic!("unknown syscall_id {}", syscall_id);
        }
    }
}

pub fn sys_test() -> isize {
    let value = 1234;
    stop_current_and_run_next_task();
    value
}

pub fn sys_write(fd: usize, buf_ptr_va: VirtualAddress, length: usize) -> isize {
    if fd != 1 {
        return -1;
    }
    let buf_ptr_pa: PhysicalAddress = buf_ptr_va.into();
    let buf_ptr: *const u8 = buf_ptr_pa.as_raw();
    let buffer = unsafe {
        core::slice::from_raw_parts(buf_ptr, length)
    };
    print!("{}", from_utf8(buffer).unwrap());
    0
}

pub fn sys_exit(exit_code: isize) -> isize {
    info!("task exit with exit_code:{}", exit_code);
    exit_current_and_run_next_task();
    0
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_brk(mut new_brk: VirtualAddress) -> isize {
    let cur_task = get_cur_task_in_this_hart();
    let mut inner = cur_task.acquire_inner_lock();
    let mut brk = inner.mem_manager.brk;
    let mut size = new_brk.0.abs_diff(brk.0);

    if new_brk.0 == 0 {
        return brk.0 as isize;
    }

    if new_brk >= brk { // alloc
        if new_brk.0 >= MMAP_START_ADDRESS {
            return -1;
        }

        if ! brk.is_aligned() {
            if size <= (FRAME_SIZE - brk.offset()) {
                inner.mem_manager.brk = new_brk;
                return new_brk.0 as isize;
            }
            size -=(FRAME_SIZE - brk.offset());
            brk = brk.ceil().into();
        }
        inner.mem_manager.add_area(brk, ceil(size), RegionFlags::W | RegionFlags::R, None);
    } else { // dealloc
        let brk_start = inner.mem_manager.brk_start;
        if new_brk < brk_start {
            return 0;
        }
        size += new_brk.offset();
        new_brk = new_brk.floor().into();
        inner.mem_manager.delete_area(new_brk, ceil(size));
    }
    inner.mem_manager.brk = new_brk;
    new_brk.0 as isize
}

pub fn sys_yield() -> isize {
    stop_current_and_run_next_task();
    0
}