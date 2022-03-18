use alloc::vec::Vec;

global_asm!(include_str!("link_app.asm"));

const BASE_ADDRESS: usize = 0x8350_0000;
const APP_SIZE: usize = 0x20000;

extern "C" {
    fn _num_app();
}


pub unsafe fn get_apps_ref_data() -> Vec<&'static [u8]> {
    let nums = (_num_app as *const usize).read();

    let mut start_address_ptr = (_num_app as *mut usize).add(1);
    let mut end_address_ptr = start_address_ptr.add(1);

    let mut apps_ref_data = Vec::new();

    for _ in 0..nums {
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