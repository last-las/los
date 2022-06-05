use alloc::string::String;
use crate::syscall::read;

const STDIN: usize = 0;
const BS: u8 = 0x08;
const LF: u8 = 0x0a;
const CR: u8 = 0x0d;
const DL: u8 = 0x7f;

pub fn read_line() -> String {
    let mut line = String::new();
    let mut buf = [0];
    let mut cnt = 0;
    loop {
        assert_eq!(read(STDIN, &mut buf).unwrap(), 1);
        match buf[0] {
            LF | CR =>
                break,
            DL | BS => {
                if cnt != 0 {
                    print!("{}", BS as char);
                    print!(" ");
                    print!("{}", BS as char);
                    cnt -= 1;
                }
                line.pop();
            },
            _ => {
                cnt += 1;
                print!("{}",buf[0] as char);
                line.push(buf[0] as char);
            }
        }
    }
    println!("");
    line
}