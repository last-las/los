use alloc::vec::Vec;
use crate::config::MAX_TASK_NUMBER;
use core::fmt::{Debug, Formatter};
use spin::Mutex;

pub fn alloc_pid() -> Option<Pid> {
    PID_ALLOCATOR.lock().alloc()
}


pub struct Pid(pub(crate) u8);

impl Drop for Pid {
    fn drop(&mut self) {
        PID_ALLOCATOR.lock().free(self.0);
    }
}

impl Debug for Pid {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("Pid:{}", self.0))
    }
}

lazy_static!{
    pub static ref PID_ALLOCATOR: Mutex<PidAllocator> = Mutex::new(PidAllocator::new());
}

pub struct PidAllocator {
    pub bit_map: u64,
    last_pid: Option<u8>,
}

impl PidAllocator {
    fn new() -> Self {
        Self {
            bit_map: 0,
            last_pid: None,
        }
    }

    fn alloc(&mut self) -> Option<Pid> {
        if self.bit_map == u64::MAX {
            None
        } else if self.last_pid.is_none() {
                self.last_pid = Some(0);
                self.bit_map |= 1 << 0;
                Some(Pid(0))
        } else {
            let mut pos = self.last_pid.unwrap();
            loop {
                pos = (pos + 1) % 64;
                if ((1 << pos) & self.bit_map) == 0 {
                    break;
                }
            }
            self.bit_map |= 1 << pos;
            self.last_pid = Some(pos);
            Some(Pid(pos))
        }
    }

    fn free(&mut self, pid: u8) {
        assert_eq!( (self.bit_map >> pid) & 1, 1);
        self.bit_map ^= 1 << pid;
    }

    fn empty(&mut self) {
        self.bit_map = 0;
        self.last_pid = None;
    }
}

/// test case.
pub fn test_pid_allocation() {
    info!("starting pid.rs test cases");

    // test allocate and free MAX_TASK_NUMBER tasks.
    let mut pid_allocator = PID_ALLOCATOR.lock();
    pid_allocator.empty(); // make sure bit_map equal zero,
                            // because other test cases might influence its value
    let mut pids = Vec::new();
    for i in 0..MAX_TASK_NUMBER {
        pids.push(pid_allocator.alloc().unwrap());
        assert_eq!(pids[i].0 as usize, i);
    }
    assert_eq!(pid_allocator.bit_map, u64::MAX);
    drop(pid_allocator);
    drop(pids);
    let pid_allocator = PID_ALLOCATOR.lock();
    assert_eq!(pid_allocator.bit_map, 0);
    drop(pid_allocator);

    info!("end of pid.rs test\n");
}