use core::str::from_utf8;

pub fn do_write(fd: usize, buf_ptr: usize, length: usize) -> isize {
    if fd != 1 {
        return -1;
    }
    let buf_ptr = buf_ptr as *const u8;
    let buffer = unsafe {
        core::slice::from_raw_parts(buf_ptr, length)
    };
    print!("{}", from_utf8(buffer).unwrap());
    0
}
