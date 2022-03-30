use alloc::sync::Arc;
use crate::task::{TaskStruct, add_a_task_to_manager, KernelStack, RuntimeFlags};
use crate::processor::clone_cur_task_in_this_hart;
use share::syscall::error::SysError;

#[allow(unused_variables)]
pub fn do_fork(flags: u32, stack: usize, ptid_ptr: usize, tls_ptr: usize, ctid_ptr: usize) -> Result<usize, SysError>{
    let cur_task = clone_cur_task_in_this_hart();

    let child_task = Arc::new(cur_task.copy_process(flags, stack, ptid_ptr, tls_ptr, ctid_ptr)?);
    let mut inner = cur_task.acquire_inner_lock();
    inner.children.push(Arc::clone(&child_task));
    drop(inner);

    let child_pid = child_task.pid();
    add_a_task_to_manager(child_task);
    Ok(child_pid)
}

