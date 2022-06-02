#![cfg_attr(feature = "const_fn", feature(const_mut_refs, const_fn_fn_ptr_basics))]
#![no_std]
#![allow(dead_code)]
#![feature(const_mut_refs)]
#[cfg(feature = "use_spin")]
extern crate spin;
extern crate alloc;
use core::alloc::{GlobalAlloc, Layout};
use core::cmp::{max, min};
use core::fmt;
use core::mem::size_of;
#[cfg(feature = "use_spin")]
use core::ops::Deref;
use core::ptr::NonNull;
#[cfg(feature = "use_spin")]
use spin::Mutex;

mod frame;
pub mod linked_list;
mod syscall;
#[macro_use]
mod console;

pub use syscall::*;
pub use frame::*;

/// A heap that uses buddy system
///
/// # Usage
///
/// Create a heap and add a memory region to it:
/// ```
/// # use core::mem::size_of;
/// use buddy_allocator::Heap;
/// let mut heap = Heap::empty();
/// # let space: [usize; 100] = [0; 100];
/// # let begin: usize = space.as_ptr() as usize;
/// # let end: usize = begin + 100 * size_of::<usize>();
/// # let size: usize = 100 * size_of::<usize>();
/// unsafe {
///     heap.init(begin, size);
///     // or
///     heap.add_to_heap(begin, end);
/// }
/// ```
pub struct Heap {
    // buddy system with max order of 32
    free_list: [linked_list::LinkedList; 32],

    // statistics
    // the number of bytes that user requests
    user: usize,
    // the number of bytes that are actually allocated
    // generally, allocated >= user_requests
    allocated: usize,
    // the total number of bytes in the heap
    total: usize,
}

impl Heap {
    /// Create an empty heap
    pub const fn new() -> Self {
        Heap {
            free_list: [linked_list::LinkedList::new(); 32],
            user: 0,
            allocated: 0,
            total: 0,
        }
    }

    /// Create an empty heap
    pub const fn empty() -> Self {
        Self::new()
    }

    /// Add a range of memory [start, end) to the heap
    pub unsafe fn add_to_heap(&mut self, mut start: usize, mut end: usize) {
        // avoid unaligned access on some platforms
        start = (start + size_of::<usize>() - 1) & (!size_of::<usize>() + 1);
        end = end & (!size_of::<usize>() + 1);
        assert!(start <= end);
        let mut total = 0;
        let mut current_start = start;
        while current_start + size_of::<usize>() <= end {
            let lowbit = current_start & (!current_start + 1);
            let size = min(lowbit, prev_power_of_two(end - current_start));
            total += size;

            self.free_list[size.trailing_zeros() as usize].push(current_start as *mut usize);
            current_start += size;
        }
        self.total += total;
    }

    // please rescue my life T^T
    /// Add a range of memory [start, end) to the heap
    pub unsafe fn add_to_heap_rescue(&mut self, mut start: usize, mut end: usize) {
        // avoid unaligned access on some platforms
        start = (start + size_of::<usize>() - 1) & (!size_of::<usize>() + 1);
        end = end & (!size_of::<usize>() + 1);
        assert!(start <= end);
        let mut total = 0;
        let size = end - start;
        total += size;
        self.free_list[size.trailing_zeros() as usize].push(start as *mut usize);
        self.total += total;
    }

    /// Add a range of memory [start, end) to the heap
    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.add_to_heap(start, start + size);
    }

    /// Alloc a range of memory from the heap satifying `layout` requirements
    pub fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        // next_power_of_two() returns the smallest pow(2, m) >= layout.size()
        // let size = 要分配的大小，最小要满足4/8字节、对齐方式
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        );
        // trailing_zeros()返回二进制表示中尾随零的数量
        // let n = Wrapping(0b0101000i64);
        // assert_eq!(n.trailing_zeros(), 3);
        let class = size.trailing_zeros() as usize;
        for i in class..self.free_list.len() {
            // Find the first non-empty size class
            if !self.free_list[i].is_empty() {
                // Split buffers
                for j in (class + 1..i + 1).rev() {
                    if let Some(block) = self.free_list[j].pop() {
                        unsafe {
                            // 把freelist[j]拆成两个freelist[j - 1]，地址靠后的先入栈，前面一个再入栈
                            // j = 15, 1 << (j - 1) = 1 << 14 = pow(2, 14)
                            self.free_list[j - 1]
                                .push((block as usize + (1 << (j - 1))) as *mut usize);
                            self.free_list[j - 1].push(block);
                        }
                    } else {
                        return Err(());
                    }
                }

                let result = NonNull::new(
                    self.free_list[class]
                        .pop()
                        .expect("current block should have free space now")
                        as *mut u8,
                );
                return if let Some(result) = result {
                    self.user += layout.size();
                    self.allocated += size;
                    Ok(result)
                } else {
                    Err(())
                }
            }
        }
        Err(())
    }

    /// Dealloc a range of memory from the heap
    pub fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        // 找到dealloc的大小
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        );
        let class = size.trailing_zeros() as usize;
        unsafe {
            // Put back into free list，把要回收的block放回链表尾
            self.free_list[class].push(ptr.as_ptr() as *mut usize);

            // Merge free buddy lists
            let mut current_ptr = ptr.as_ptr() as usize;
            let mut current_class = class;
            while current_class < self.free_list.len() {
                // 找到伙伴块的地址
                let buddy = current_ptr ^ (1 << current_class);
                let mut flag = false;
                for block in self.free_list[current_class].iter_mut() {
                    if block.value() as usize == buddy {
                        // 拿出伙伴块
                        block.pop();
                        flag = true;
                        break;
                    }
                }

                // Free buddy found
                if flag {
                    self.free_list[current_class].pop();
                    current_ptr = min(current_ptr, buddy);
                    current_class += 1;
                    self.free_list[current_class].push(current_ptr as *mut usize);
                } else {
                    break;
                }
            }
        }

        self.user -= layout.size();
        self.allocated -= size;
    }

    /// Return the number of bytes that user requests
    pub fn stats_alloc_user(&self) -> usize {
        self.user
    }

    /// Return the number of bytes that are actually allocated
    pub fn stats_alloc_actual(&self) -> usize {
        self.allocated
    }

    /// Return the total number of bytes in the heap
    pub fn stats_total_bytes(&self) -> usize {
        self.total
    }
}

impl fmt::Debug for Heap {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Heap")
            .field("user", &self.user)
            .field("allocated", &self.allocated)
            .field("total", &self.total)
            .finish()
    }
}

/// A locked version of `Heap`
///
/// # Usage
///
/// Create a locked heap and add a memory region to it:
/// ```
/// # use core::mem::size_of;
/// use buddy_allocator::LockedHeapOrigin;
/// let mut heap = LockedHeapOrigin::new();
/// # let space: [usize; 100] = [0; 100];
/// # let begin: usize = space.as_ptr() as usize;
/// # let end: usize = begin + 100 * size_of::<usize>();
/// # let size: usize = 100 * size_of::<usize>();
/// unsafe {
///     heap.lock().init(begin, size);
///     // or
///     heap.lock().add_to_heap(begin, end);
/// }
/// ```
#[cfg(feature = "use_spin")]
pub struct LockedHeapOrigin(Mutex<Heap>);

#[cfg(feature = "use_spin")]
impl LockedHeapOrigin {
    /// Creates an empty heap
    pub const fn new() -> LockedHeapOrigin {
        LockedHeapOrigin(Mutex::new(Heap::new()))
    }

    /// Creates an empty heap
    pub const fn empty() -> LockedHeapOrigin {
        LockedHeapOrigin(Mutex::new(Heap::new()))
    }
}

#[cfg(feature = "use_spin")]
impl Deref for LockedHeapOrigin {
    type Target = Mutex<Heap>;

    fn deref(&self) -> &Mutex<Heap> {
        &self.0
    }
}

#[cfg(feature = "use_spin")]
unsafe impl GlobalAlloc for LockedHeapOrigin {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .alloc(layout)
            .ok()
            .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().dealloc(NonNull::new_unchecked(ptr), layout)
    }
}

/// A locked version of `Heap` with rescue before oom
///
/// # Usage
///
/// Create a locked heap:
/// ```
/// use std::alloc::Layout;
/// use buddy_allocator::{Heap, LockedHeap};
/// let heap = LockedHeap::new(|heap: &mut Heap, layout: Layout| {});
/// ```
///
/// Before oom, the allocator will try to call rescue function and try for one more time.
#[cfg(feature = "use_spin")]
pub struct LockedHeap {
    inner: Mutex<Heap>,
    rescue: fn(&mut Heap, Layout),
}

#[cfg(feature = "use_spin")]
impl LockedHeap {
    /// Creates an empty heap with rescue function
    pub const fn new(rescue: fn(&mut Heap, Layout)) -> LockedHeap {
        LockedHeap {
            inner: Mutex::new(Heap::new()),
            rescue,
        }
    }

    pub const fn empty() -> LockedHeap {
        LockedHeap {
            inner: Mutex::new(Heap::new()),
            rescue: |heap: &mut Heap, layout: Layout| unsafe {
                // get current brk
                let cur_brk = brk(None).unwrap();
                // get request
                let request_size = layout.size();
                let allocated_size = max(
                    layout.size().next_power_of_two(),
                    max(layout.align(), size_of::<usize>()),
                );
                let class = allocated_size.trailing_zeros() as usize;
                if cur_brk & !(0xFFFFFFFFFFFFFFFF << class) == 0 {
                    // 对齐
                    let new_brk = brk(Some(cur_brk + allocated_size)).unwrap();
                    heap.add_to_heap_rescue(cur_brk, new_brk);
                } else {
                    // 没对齐
                    let aligned_new_brk = (cur_brk & (0xFFFFFFFFFFFFFFFF << class)) + (1usize << class);
                    let new_brk = brk(Some(aligned_new_brk + allocated_size)).unwrap();
                    heap.add_to_heap(cur_brk, aligned_new_brk);
                    heap.add_to_heap_rescue(aligned_new_brk, new_brk);
                }
            },
        }
    }
}

// LockedHeap = LockedHeapOrigin + Rescue_function
// Rescue function will be called when out of memory^^
#[cfg(feature = "use_spin")]
impl Deref for LockedHeap {
    type Target = Mutex<Heap>;

    fn deref(&self) -> &Mutex<Heap> {
        &self.inner
    }
}

#[cfg(feature = "use_spin")]
unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut inner = self.inner.lock();
        match inner.alloc(layout) {
            Ok(allocation) => {
                allocation.as_ptr()
            },
            Err(_) => {
                (self.rescue)(&mut inner, layout);
                let result = inner
                    .alloc(layout)
                    .ok()
                    .map_or(0 as *mut u8, |allocation| allocation.as_ptr());
                result
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.inner
            .lock()
            .dealloc(NonNull::new_unchecked(ptr), layout)
    }
}

pub(crate) fn prev_power_of_two(num: usize) -> usize {
    // leading_zeros()返回二进制表示中前导零的数量
    // let n = Wrapping(u64::MAX) >> 2;
    // assert_eq!(n.leading_zeros(), 2);
    1 << (8 * (size_of::<usize>()) - num.leading_zeros() as usize - 1)
}
