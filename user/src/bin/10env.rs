#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::env::{setenv, getenv, cvt_c_like, EnvironVariable};

#[no_mangle]
fn main() {
    // setenv("PATH", "less", false);
    setenv("TOOL", "good", false);
    setenv("SOOL", "great", false);
    let v = cvt_c_like();

    let mut new_env = EnvironVariable::new();
    new_env.from_c_like(v.as_slice());

    // assert_eq!(getenv("PATH").unwrap(), new_env.get("PATH").unwrap());
    assert_eq!(getenv("TOOL").unwrap(), new_env.get("TOOL").unwrap());
    assert_eq!(getenv("SOOL").unwrap(), new_env.get("SOOL").unwrap());
}