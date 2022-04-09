use alloc::sync::Arc;
use crate::task::{TaskStruct, add_a_task_to_manager, KernelStack, RuntimeFlags, TrapContext, TaskContext, alloc_pid, TaskStructInner};
use crate::processor::clone_cur_task_in_this_hart;
use share::syscall::error::{SysError, EAGAIN};
use alloc::vec::Vec;
use spin::Mutex;
// clone

#[allow(unused_variables)]
pub fn do_fork(flags: u32, stack: usize, ptid_ptr: usize, tls_ptr: usize, ctid_ptr: usize) -> Result<usize, SysError>{
    let cur_task = clone_cur_task_in_this_hart();

    let child_task = Arc::new(copy_process(flags, stack, ptid_ptr, tls_ptr, ctid_ptr, &cur_task)?);
    let mut inner = cur_task.acquire_inner_lock();
    inner.children.push(Arc::clone(&child_task));
    drop(inner);

    let child_pid = child_task.pid();
    add_a_task_to_manager(child_task);
    Ok(child_pid)
}

#[allow(unused_variables)]
fn copy_process(flags: u32, stack: usize, ptid_ptr: usize, tls_ptr: usize, ctid_ptr: usize, parent: &Arc<TaskStruct>) -> Result<TaskStruct, SysError>{
    let mut parent_inner = parent.acquire_inner_lock();

    let mem_manager = parent_inner.mem_manager.clone()?;

    let pid_handle = alloc_pid();
    if pid_handle.is_none() {
        return Err(SysError::new(EAGAIN));
    }
    let pid_handle = pid_handle.unwrap();

    let mut kernel_stack = KernelStack::new()?;

    let task_context = TaskContext::new(kernel_stack.sp() - core::mem::size_of::<TrapContext>());

    let mut child_inner = TaskStructInner {
        kernel_stack,
        wait_queue: Vec::new(),
        flag: RuntimeFlags::READY,
        task_context,
        message_holder: None,
        interrupt_flag: false,
        mem_manager,
        priority: parent_inner.priority,
        children: Vec::new(),
        parent: Some(Arc::downgrade(parent))
    };

    // push `trap_context` onto the `kernel_stack`
    let parent_trap_context_ref: &mut TrapContext = parent_inner.trap_context_ref();
    let mut child_trap_context = parent_trap_context_ref.clone();
    child_trap_context.x[10] = 0;
    let child_trap_context_ref = child_inner.trap_context_ref();
    *child_trap_context_ref = child_trap_context;

    Ok(TaskStruct {pid_handle, inner: Mutex::new(child_inner)})
}