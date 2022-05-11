use share::syscall::error::SysError;
use crate::task::TrapContext;
use crate::processor::get_cur_task_in_this_hart;
use crate::mm::memory_manager::MemoryManager;
use core::arch::asm;
use alloc::vec::Vec;
use crate::mm::page_table::PageTable;
use share::ffi::{CString, CStrArray, CStr};
use crate::syscall::file::{do_open, do_read, do_fstat, do_close};
use share::file::{OpenFlag, Stat, AT_FD_CWD};
use alloc::vec;
use alloc::string::String;

pub fn do_exec(path_ptr: usize, argv: *const *const u8, envp: *const *const u8) -> Result<usize, SysError> {
    // read file data from fs server.
    let path_cstr = CStr::from_ptr(path_ptr as *const _);
    let path_cstring = CString::from(path_cstr);
    let open_flag = OpenFlag::RDONLY;
    let fd = do_open( AT_FD_CWD as usize,path_cstring.as_ptr() as usize, open_flag.bits() as usize, 0)?;
    let stat = Stat::empty();
    do_fstat(fd, &stat as *const _ as usize)?;
    let data_buffer = vec![0; stat.size];
    do_read(fd, data_buffer.as_ptr() as usize, stat.size)?;
    do_close(fd)?;
    let data = data_buffer.as_slice();

    // create new address space.
    let (mem_manager, pc, user_sp) = MemoryManager::new(data)?;
    let (arg_vec, env_vec) = read_arg_and_env_in_current_addr_space(argv, envp);
    switch_to_new_addr_space(&mem_manager.page_table);
    let user_sp = push_arg_and_env_onto_stack(arg_vec, env_vec, user_sp);

    let mut name = String::from(path_cstring.inner);
    name.pop();
    modify_current_task_struct(mem_manager, pc, user_sp, name);

    Ok(0)
}

fn read_arg_and_env_in_current_addr_space(argv_ptr: *const *const u8, envp_ptr: *const *const u8)
                                          -> (Vec<CString>, Vec<CString>) {
    let arg_cstring_vec = get_cstring_vec_from_str_array_ptr(argv_ptr);
    let env_cstring_vec = get_cstring_vec_from_str_array_ptr(envp_ptr);

    (arg_cstring_vec, env_cstring_vec)
}

fn switch_to_new_addr_space(page_table: &PageTable) {
    let satp = 8 << 60 | page_table.satp();
    riscv::register::satp::write(satp);
    unsafe {
        asm! {"sfence.vma"}
    }
}

fn push_arg_and_env_onto_stack(arg_vec: Vec<CString>, env_vec: Vec<CString>, mut user_sp: usize) -> usize {
    let argc = arg_vec.len();

    unsafe {
        let env_ptr_vec = push_str_vector_onto_stack_in_c_style(env_vec, &mut user_sp);
        let arg_ptr_vec = push_str_vector_onto_stack_in_c_style(arg_vec, &mut user_sp);

        push_usize_vector_onto_stack_in_c_style(env_ptr_vec, &mut user_sp);
        push_usize_vector_onto_stack_in_c_style(arg_ptr_vec, &mut user_sp);

        user_sp -= core::mem::size_of::<usize>();
        (user_sp as *mut usize).write(argc);
    }

    user_sp
}

fn modify_current_task_struct(mem_manager: MemoryManager,pc: usize, user_sp: usize, name: String) {
    let cur_task = get_cur_task_in_this_hart();
    let mut inner = cur_task.acquire_inner_lock();
    let trap_context_ref = inner.trap_context_ref();
    *trap_context_ref = TrapContext::new(pc, user_sp);
    inner.mem_manager = mem_manager;

    inner.name = name;
}

fn get_cstring_vec_from_str_array_ptr(str_array_ptr: *const *const u8) -> Vec<CString> {
    let mut vec = Vec::new();
    for cstr_ptr in CStrArray::copy_from_ptr(str_array_ptr).iter() {
        let cstr = CStr::from_ptr(cstr_ptr);
        vec.push(CString::from(cstr));
    }

    vec
}

/// return a vector containing pointers, each of them points to the first byte of a str.
unsafe fn push_str_vector_onto_stack_in_c_style(vec: Vec<CString>, sp: &mut usize) -> Vec<usize> {
    let mut ptr_vec = Vec::new();

    for cstring in vec.iter().rev() {
        *sp -= cstring.as_bytes_with_nul().len();
        ptr_vec.insert(0, *sp);
        let mut pointer = *sp as *mut u8;
        for c in cstring.as_bytes_with_nul() {
            pointer.write(*c);
            pointer = pointer.add(1);
        }
    }

    ptr_vec
}

unsafe fn push_usize_vector_onto_stack_in_c_style(vec: Vec<usize>, sp: &mut usize) {
    *sp -= core::mem::size_of::<usize>();
    (*sp as *mut usize).write(0);

    for env in vec.iter().rev() {
        *sp -= core::mem::size_of::<usize>();
        (*sp as *mut usize).write(*env);
    }
}
