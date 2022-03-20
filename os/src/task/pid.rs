use crate::config::MAX_TASK_NUMBER;
use core::fmt::{Debug, Formatter};
use spin::Mutex;

pub fn alloc_pid() -> Option<PidHandle> {
    PID_ALLOCATOR.lock().alloc()
}


pub struct PidHandle(pub(crate) usize);

impl Drop for PidHandle {
    fn drop(&mut self) {
        PID_ALLOCATOR.lock().free(self.0);
    }
}

impl Debug for PidHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("pid:{}", self.0))
    }
}

lazy_static!{
    pub static ref PID_ALLOCATOR: Mutex<PidAllocator> = Mutex::new(PidAllocator::new());
}

pub struct PidAllocator {
    pub bit_map: u64,
    last_pid: usize,
}

impl PidAllocator {
    fn new() -> Self {
        Self {
            bit_map: 0,
            last_pid: MAX_TASK_NUMBER - 1,
        }
    }

    fn alloc(&mut self) -> Option<PidHandle> {
        if self.bit_map == u64::MAX {
            None
        } else {
            let mut pos = self.last_pid;
            loop {
                pos = (pos + 1) % MAX_TASK_NUMBER;
                if ((1 << pos) & self.bit_map) == 0 {
                    break;
                }
            }
            self.bit_map |= 1 << pos;
            self.last_pid = pos;
            Some(PidHandle(self.last_pid))
        }
    }

    fn free(&mut self, pid: usize) {
        assert!(pid < MAX_TASK_NUMBER);
        assert_eq!( (self.bit_map >> pid) & 1, 1);
        self.bit_map ^= 1 << pid;
    }

    #[allow(unused)]
    fn empty(&mut self) {
        self.bit_map = 0;
        self.last_pid = MAX_TASK_NUMBER - 1;
    }
}

#[cfg(test)]
mod test {
    use alloc::vec::Vec;
    use crate::config::MAX_TASK_NUMBER;
    use super::PID_ALLOCATOR;

    #[test]
    pub fn test_pid_allocation() {
        info!("starting pid.rs test cases");

        // test allocate() and free() MAX_TASK_NUMBER tasks.
        let mut pid_allocator = PID_ALLOCATOR.lock();
        pid_allocator.empty(); // make sure bit_map equal zero,
        // because other test cases might influence its value
        let mut pids = Vec::new();
        for i in 0..MAX_TASK_NUMBER {
            pids.push(pid_allocator.alloc().unwrap());
            assert_eq!(pids[i].0, i);
        }
        assert_eq!(pid_allocator.bit_map, u64::MAX);
        drop(pid_allocator);
        drop(pids);
        let pid_allocator = PID_ALLOCATOR.lock();
        assert_eq!(pid_allocator.bit_map, 0);
        drop(pid_allocator);

        info!("end of pid.rs test\n");
    }
}