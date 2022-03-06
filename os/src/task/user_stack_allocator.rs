// TODO: This file should be delete when VA is enabled.

use spin::Mutex;

pub const USER_STACK_SIZE: usize = 0x1000;
pub const MAX_USER_APPS: usize = 20;

// TODO: determine whether align to 4k.
#[link_section = ".rodata.user_stack"]
static  USER_STACK: [u8; USER_STACK_SIZE * MAX_USER_APPS + 1] = [0; USER_STACK_SIZE * MAX_USER_APPS + 1];

pub fn alloc_a_user_stack() -> usize {
    USER_STACK_ALLOCATOR.alloc()
}

lazy_static!{
    static ref USER_STACK_ALLOCATOR: UserStackAllocator = UserStackAllocator::new();
}

struct UserStackAllocator {
    inner: Mutex<UserStackAllocatorInner>,
}

struct  UserStackAllocatorInner {
    stack_num: usize,
}

impl UserStackAllocator {
    fn new() -> UserStackAllocator {
        UserStackAllocator {
            inner: Mutex::new(UserStackAllocatorInner { stack_num: 0})
        }
    }

    fn alloc(&self) -> usize {
        let mut inner = self.inner.lock();
        assert!(inner.stack_num < MAX_USER_APPS);
        inner.stack_num += 1;
        &USER_STACK[inner.stack_num * USER_STACK_SIZE] as *const _ as usize
    }
}

#[cfg(feature = "test")]
pub fn test_user_stack_allocator() {
    let allocator = UserStackAllocator::new();
    let mut sp = &USER_STACK as *const _ as usize;
    // alignment with 4k
    assert_eq!(sp % 0x1000, 0);

    (0..20).into_iter().for_each(|_| {
        sp += USER_STACK_SIZE;
        assert_eq!(sp, allocator.alloc());
    });
}