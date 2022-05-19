use crate::vfs::NameIData;
use crate::proc::get_fs_struct_by_pid;

pub static CUR_PID: usize = 0;

pub fn do_getcwd() {
    let fs_struct = get_fs_struct_by_pid(CUR_PID).unwrap();
    let name = fs_struct.pwd.name.as_str();
    unimplemented!()
}

pub fn do_pipe2() {
    unimplemented!();
}

pub fn do_dup() {
    unimplemented!()
}

pub fn do_dup3() {
    unimplemented!()
}

pub fn do_chdir(path: &str) {
    unimplemented!()
}


pub fn do_open(path: &str, flag: usize) {
    let nd = path_lookup(path, flag);
    unimplemented!()
}

pub fn do_close() {
    unimplemented!()
}

pub fn do_read(fs: usize) {
    let fs_struct = get_fs_struct_by_pid(CUR_PID).unwrap();
    let file = fs_struct.fd_table[fs].as_ref().unwrap().clone();
    file.read();
}

pub fn do_write() {
    unimplemented!()
}

fn path_lookup(path: &str, flag: usize) -> NameIData {
    unimplemented!();
}
