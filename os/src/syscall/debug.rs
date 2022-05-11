use share::syscall::error::SysError;
use alloc::vec::Vec;
use spin::Mutex;
use alloc::string::String;

pub static mut SCHEDULE_RECORD_ENABLE: usize = 0;

pub fn is_schedule_record_enable() -> bool {
    unsafe {
        SCHEDULE_RECORD_ENABLE != 0
    }
}

pub fn append_schedule_record(pid: usize, name: String) {
    SCHEDULE_RECORDS.lock().push((pid, name));
}

lazy_static!{
    pub static ref SCHEDULE_RECORDS: Mutex<Vec<(usize, String)>> = Mutex::new(Vec::new());
}

pub fn debug_schedule_record_enable(val: usize) -> Result<usize, SysError> {
    unsafe {
        SCHEDULE_RECORD_ENABLE = val;
    }
    if val == 0 {
        SCHEDULE_RECORDS.lock().clear();
    }

    Ok(0)
}

pub fn debug_schedule_record_print() -> Result<usize, SysError> {
    let inner = SCHEDULE_RECORDS.lock();
    for (pid, name) in inner.iter() {
        println!("pid:{}, pathname:{}", pid, name);
    }
    Ok(0)
}