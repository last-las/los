#![no_std]
#![no_main]

extern crate user_lib;
extern crate alloc;

use alloc::string::String;
use user_lib::println;
use user_lib::syscall::{sys_uname};
use share::system::Utsname;

#[no_mangle]
fn main() {
    let mut pointer = Utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    sys_uname(&mut pointer as *mut Utsname as usize);
    println!("{:?}", pointer);
    println!("sysname: {}", convert_to_string(&pointer.sysname));
    println!("nodename: {}", convert_to_string(&pointer.nodename));
    println!("release: {}", convert_to_string(&pointer.release));
    println!("version: {}", convert_to_string(&pointer.version));
    println!("machine: {}", convert_to_string(&pointer.machine));
    println!("domainname: {}", convert_to_string(&pointer.domainname))
}

pub fn convert_to_string(field: &[u8]) -> String {
    let mut string = String::new();
    for a in field.iter() {
        string.push(*a as char);
    }
    string.clone()
}