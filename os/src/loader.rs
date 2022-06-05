use alloc::vec::Vec;
use core::arch::global_asm;
use share::ffi::CStr;

#[cfg(not(test))]
global_asm!(include_str!("link_app.asm"));

extern "C" {
    fn _num_app();
    fn _app_names();
}

pub fn get_app_names() -> Vec<CStr<'static>> {
    let num = get_app_num();
    let mut start = _app_names as usize;
    let mut app_names = Vec::new();
    for _ in 0..num {
        let app_name = CStr::from_ptr(start as *const u8);
        start += app_name.as_bytes_with_nul().len();
        app_names.push(app_name);
    }

    app_names
}

pub unsafe fn get_app_ref_data() -> Vec<&'static [u8]> {
    let num = get_app_num();

    let mut start_address_ptr = (_num_app as *mut usize).add(1);
    let mut end_address_ptr = start_address_ptr.add(1);

    let mut apps_ref_data = Vec::new();

    for _ in 0..num {
        let start_address = start_address_ptr.read();
        let end_address = end_address_ptr.read();

        let app_size = end_address - start_address;
        let src_data = core::slice::from_raw_parts(start_address as *const u8, app_size);

        apps_ref_data.push(src_data);
        start_address_ptr = end_address_ptr;
        end_address_ptr = end_address_ptr.add(1);
    }

    apps_ref_data
}

pub fn get_app_num() -> usize {
    unsafe {
        (_num_app as *const usize).read()
    }
}

