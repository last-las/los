pub fn cvt_c_like_str_ptr_to_rust(str_ptr: usize) -> &'static str {
    let start = str_ptr as *const u8;
    let mut end = start;
    unsafe {
        while end.read_volatile() != '\0' as u8 {
            end = end.add(1);
        }

        let slice = core::slice::from_raw_parts(start, end as usize - start as usize);
        core::str::from_utf8(slice).unwrap()
    }
}