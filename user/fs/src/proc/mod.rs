use crate::proc::fs_struct::FsStruct;
use alloc::rc::Rc;

pub mod fs_struct;

pub fn get_fs_struct_by_pid(pid: usize) -> Option<Rc<FsStruct>> {
    unimplemented!();
}

pub fn init_proc_fs_struct(pid: usize) {
    unimplemented!();
}

pub fn rm_proc_fs_struct(pid: usize) {
    unimplemented!();
}