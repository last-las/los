use spin::Mutex;
use crate::config::MAX_TASK_NUMBER;
use alloc::vec::Vec;

const KERNEL_STACK_SIZE: usize = 0x1000;
const GUARD_PAGE_SIZE: usize = 0x1000;
const KERNEL_STACKS_MEMORY_SIZE: usize = (KERNEL_STACK_SIZE + GUARD_PAGE_SIZE) * (MAX_TASK_NUMBER + 1);
// TODO future: kernelStack should also be managed by FRAME_ALLOCATOR rather than an independent struct.
static mut KERNEL_STACK: [u8; KERNEL_STACKS_MEMORY_SIZE] = [0; KERNEL_STACKS_MEMORY_SIZE];

pub struct KernelStack {
    bp: usize,
    pub sp: usize,
}

impl KernelStack {
    pub fn new() -> Self {
        let bp = KSTACK_ALLOCATOR.lock().alloc();
        let sp = bp;
        Self {
            bp: bp as usize,
            sp: sp as usize,
        }
    }

    pub fn push<T>(&mut self, x: T) {
        let space = KERNEL_STACK_SIZE - (self.bp - self.sp);
        let x_size = core::mem::size_of::<T>();
        assert!(x_size <= space);

        self.sp -= x_size;
        unsafe {
            (self.sp as *mut T).write(x);
        }
    }

    pub fn pop<T>(&mut self) -> T {
        let x_size = core::mem::size_of::<T>();
        assert!(self.sp + x_size <= self.bp);

        let x;
        unsafe {
            x = (self.sp as *const T).read();
        }
        self.sp += x_size;
        x
    }

    // WARN: The life time spec here actually violate the rust rule, because the return value points
    // to an address on the heap. However, this function is only used to acquire the task_context now.
    pub fn get_mut<T>(&self) -> &'static mut T {
        let x_size = core::mem::size_of::<T>();
        assert!(self.sp + x_size <= self.bp);
        unsafe {
            (self.sp as *mut T).as_mut().unwrap()
        }
    }
}

#[cfg(not(test))]
impl Drop for KernelStack {
    fn drop(&mut self) {
        KSTACK_ALLOCATOR.lock().free(self.bp);
    }
}

lazy_static! {
    static ref KSTACK_ALLOCATOR: Mutex<KernelStackAllocator> = Mutex::new(
        KernelStackAllocator::new()
    );
}

pub struct KernelStackAllocator {
    current: usize,
    recycled: Vec<usize>,
    base_addr: usize,
}

impl KernelStackAllocator {
    fn new() -> Self {
        unsafe {
            Self {
                current: 0,
                recycled: Vec::new(),
                base_addr: &KERNEL_STACK[0] as *const _ as usize,
            }
        }
    }

    fn alloc(&mut self) -> usize {
        let slot;
        if self.recycled.is_empty() {
            assert!(self.current < MAX_TASK_NUMBER);
            slot = self.current;
            self.current += 1;
        } else {
            slot = self.recycled.pop().unwrap();
        }
        let pos = (slot + 1) * (GUARD_PAGE_SIZE + KERNEL_STACK_SIZE);
        unsafe {
            assert_eq!(&KERNEL_STACK[pos] as *const _ as usize, self.base_addr + pos);
        }
        self.base_addr + pos
    }

    fn free(&mut self, bp: usize) {
        let slot = ((bp - self.base_addr) >> 13) - 1;
        assert!(slot < MAX_TASK_NUMBER);
        self.recycled.push(slot);
    }
}

#[cfg(test)]
mod test {
    use crate::config::MAX_TASK_NUMBER;
    use crate::task::kernel_stack::{KernelStackAllocator, KERNEL_STACK, KERNEL_STACK_SIZE, GUARD_PAGE_SIZE, KernelStack};

    #[test]
    pub fn test_alloc_max_stacks_twice_on_kernel_stack_allocator() {
        let mut bp_arr: [usize; MAX_TASK_NUMBER] = [0; MAX_TASK_NUMBER];
        let mut kstack_allocator = KernelStackAllocator::new();
        // first time
        for i in 0..MAX_TASK_NUMBER {
            bp_arr[i] = kstack_allocator.alloc();
            unsafe {
                assert_eq!(
                    bp_arr[i],
                    &KERNEL_STACK[(i + 1) * (KERNEL_STACK_SIZE + GUARD_PAGE_SIZE)] as *const _ as usize
                );
            }
        }
        for bp in bp_arr.iter() {
            kstack_allocator.free(*bp);
        }

        // second time, the allocated task numbers should be inverted
        for i in 0..MAX_TASK_NUMBER {
            bp_arr[i] = kstack_allocator.alloc();
            unsafe {
                assert_eq!(
                    bp_arr[i],
                    &KERNEL_STACK[(MAX_TASK_NUMBER - i) * (KERNEL_STACK_SIZE + GUARD_PAGE_SIZE)]
                        as *const _ as usize
                );
            }
        }
    }

    #[test]
    pub fn test_push_and_pop_an_usize_variable_on_kernel_stack() {
        let mut stack = KernelStack::new();
        let x: usize = 12345;
        stack.push(x);
        assert_eq!(stack.bp, stack.sp + core::mem::size_of::<usize>());
        assert_eq!(x, stack.pop());
    }

    #[test]
    pub fn test_push_and_pop_max_size_on_kernel_stack() {
        let mut stack = KernelStack::new();
        let old_arrays: [u8; KERNEL_STACK_SIZE] = [61; KERNEL_STACK_SIZE];
        stack.push(old_arrays);
        let new_arrays: [u8; KERNEL_STACK_SIZE] = stack.pop();
        (0..KERNEL_STACK_SIZE).into_iter().for_each(|i| {
            assert_eq!(new_arrays[i], 61);
        });
        assert_eq!(stack.bp, stack.sp);

    }

    #[test]
    #[should_panic]
    pub fn test_overflow_on_kernel_stack() {
        let mut stack = KernelStack::new();
        let old_arrays: [u8; KERNEL_STACK_SIZE + 1] = [0; KERNEL_STACK_SIZE + 1];
        stack.push(old_arrays);
    }
}
