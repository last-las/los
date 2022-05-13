use share::syscall::error::{SysError, ENOENT};
use crate::task::{get_task_data_by_name, TrapContext};
use crate::processor::clone_cur_task_in_this_hart;
use crate::mm::memory_manager::MemoryManager;
use core::arch::asm;
use share::util::{cvt_c_like_str_ptr_to_rust, cvt_c_like_str_array_ptr_to_rust};
use alloc::vec::Vec;
use alloc::string::String;
use crate::mm::page_table::PageTable;
use share::ffi::{CString, CStrArray, c_char, CStr};

pub fn do_exec(path_ptr: usize, argv: *const *const c_char, envp: *const *const c_char) -> Result<usize, SysError> {
    let data = get_target_elf_data_by(path_ptr)?;
    let (mem_manager, pc, user_sp) = MemoryManager::new(data)?;
    let (arg_vec, env_vec) = read_arg_and_env_in_current_addr_space(argv, envp);
    switch_to_new_addr_space(&mem_manager.page_table);
    let user_sp = unsafe { push_arg_and_env_onto_stack(arg_vec, env_vec, user_sp) };
    modify_current_task_struct(mem_manager, pc, user_sp);

    Ok(0)
}

fn get_target_elf_data_by(path_ptr: usize) -> Result<&'static [u8], SysError> {
    let path: &str = cvt_c_like_str_ptr_to_rust(path_ptr);
    let result = get_task_data_by_name(path);
    result.ok_or(SysError::new(ENOENT))
}

fn read_arg_and_env_in_current_addr_space(argv_ptr: *const *const c_char, envp_ptr: *const *const c_char)
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

fn modify_current_task_struct(mem_manager: MemoryManager,pc: usize, user_sp: usize) {
    let cur_task = clone_cur_task_in_this_hart();
    let mut inner = cur_task.acquire_inner_lock();
    let trap_context_ref = inner.trap_context_ref();
    *trap_context_ref = TrapContext::new(pc, user_sp);
    inner.mem_manager = mem_manager;
}

fn get_cstring_vec_from_str_array_ptr(str_array_ptr: *const *const c_char) -> Vec<CString> {
    let mut vec = Vec::new();
    for cstr_ptr in CStrArray::copy_from_ptr(str_array_ptr).iter() {
        let cstr = unsafe {
            CStr::from_ptr(cstr_ptr)
        };
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
    *sp -= *sp & 0b111;
    *sp -= core::mem::size_of::<usize>();
    (*sp as *mut usize).write(0);

    for env in vec.iter().rev() {
        *sp -= core::mem::size_of::<usize>();
        (*sp as *mut usize).write(*env);
    }
}
