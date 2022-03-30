use share::syscall::error::{SysError, ENOENT};
use crate::task::{get_task_data_by_name, TrapContext};
use crate::processor::clone_cur_task_in_this_hart;
use crate::mm::memory_manager::MemoryManager;
use crate::util::cvt_c_like_str_ptr_to_rust;
use core::arch::asm;

#[allow(unused_variables)]
pub fn do_exec(path_ptr: usize, argv_ptr: usize, envp_ptr: usize) -> Result<usize, SysError> {
    println!("do_exec");
    let path: &str = cvt_c_like_str_ptr_to_rust(path_ptr);
    let result = get_task_data_by_name(path);
    if result.is_none() {
        return Err(SysError::new(ENOENT));
    }
    let data = result.unwrap();

    let cur_task = clone_cur_task_in_this_hart();
    let mut inner = cur_task.acquire_inner_lock();
    let (mem_manager, pc, user_sp) = MemoryManager::new(data)?;

    let _: TrapContext = inner.kernel_stack.pop();
    let trapContext = TrapContext::new(pc, user_sp);
    inner.kernel_stack.push(trapContext);

    inner.mem_manager = mem_manager;

    let satp = 8 << 60 | inner.mem_manager.page_table.satp();
    riscv::register::satp::write(satp);
    unsafe {
        asm!{"sfence.vma"}
    }

    Ok(0)
}