#[repr(C)]
pub struct Timespec {
    pub tv_sec: u64,
    pub tv_usec: u64,
}

impl Timespec {
    pub fn empty() -> Self {
        Self {
            tv_sec: 0,
            tv_usec: 0,
        }
    }
}
