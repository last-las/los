use alloc::boxed::Box;
use core::marker::PhantomData;

const KERNEL_STACK_SIZE: usize = 0x1000;

pub struct KernelStack {
    bp: usize,
    pub sp: usize,
    memory: Box<[u8; KERNEL_STACK_SIZE]>
}

impl KernelStack {
    pub fn new() -> Self {
        let memory = Box::new([0; KERNEL_STACK_SIZE]);
        let bp = (memory.as_ptr() as usize) + KERNEL_STACK_SIZE;
        let sp = bp;
        Self {
            bp: bp as usize,
            sp: sp as usize,
            memory,
        }
    }

    pub fn push<T>(&mut self, x:T) {
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
    pub fn get_mut<T>(&self) -> &'static mut T{
        let x_size = core::mem::size_of::<T>();
        assert!(self.sp + x_size <= self.bp);
        unsafe {
            (self.sp as *mut T).as_mut().unwrap()
        }
    }
}

/// test case for kernel stack.
pub fn test_kernel_stack() {
    /// 1. test return value.
    let mut stack = KernelStack::new();
    let x: usize = 12345;
    stack.push(x);
    assert_eq!(stack.bp, stack.sp + core::mem::size_of::<usize>());
    assert_eq!(x, stack.pop());

    /// 2. test push size.
    let old_arrays: [u8; KERNEL_STACK_SIZE] = [61; KERNEL_STACK_SIZE];
    stack.push(old_arrays);
    let new_arrays: [u8; KERNEL_STACK_SIZE] = stack.pop();
    (0..KERNEL_STACK_SIZE).into_iter().for_each(|i| {
        assert_eq!(new_arrays[i], 61);
    });
    assert_eq!(stack.bp, stack.sp);

    /*
        The below test should be panic:
        /// 3. test max size
        let old_arrays: [u8; KERNEL_STACK_SIZE + 1] = [0; KERNEL_STACK_SIZE + 1];
        stack.push(old_arrays);
    */
}