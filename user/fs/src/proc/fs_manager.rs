use alloc::rc::Rc;
use core::cell::RefCell;
use crate::proc::fs_struct::FsStruct;
use alloc::vec::Vec;

pub fn init_fs_struct_of_proc(fs_struct: Rc<RefCell<FsStruct>>, pid: usize) {
    unsafe {
        FS_STRUCT_MANAGER.init_fs_struct(fs_struct, pid);
    }
}

pub fn get_fs_struct_by_pid(pid: usize) -> Rc<RefCell<FsStruct>> {
    unsafe {
        FS_STRUCT_MANAGER.get_fs_struct(pid).unwrap()
    }
}

pub fn rm_fs_struct_by_pid(pid: usize) {
    unsafe {
        FS_STRUCT_MANAGER.rm_fs_struct(pid);
    }
}

static mut FS_STRUCT_MANAGER: FStructManager = FStructManager::new();

pub struct FStructManager {
    pid2task: Vec<Option<Rc<RefCell<FsStruct>>>>,
}

impl FStructManager {
    pub const fn new() -> Self {
        Self {
            pid2task: Vec::new(),
        }
    }

    pub fn init_fs_struct(&mut self, fs_struct: Rc<RefCell<FsStruct>>, pid: usize) {
        for _ in self.pid2task.len()..pid + 1 {
            self.pid2task.push(None);
        }
        assert!(self.pid2task[pid].is_none());
        self.pid2task[pid] = Some(fs_struct);
    }

    pub fn get_fs_struct(&self, pid: usize) -> Option<Rc<RefCell<FsStruct>>> {
        self.pid2task[pid].clone()
    }

    pub fn rm_fs_struct(&mut self, pid: usize) {
        assert!(self.pid2task[pid].is_some());
        self.pid2task[pid].take();
    }
}