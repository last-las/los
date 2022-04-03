#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::io::read_line;

#[no_mangle]
fn main() {
    loop {
        print!("root@los$ ");
        let line = read_line();
        println!("read from stdin: {}, length: {}", line, line.len());
    }
}
