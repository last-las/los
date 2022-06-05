use core::fmt;
use core::fmt::Write;
use crate::syscall::{write, sbi_write};

const STDOUT: usize = 1;

struct Stdout;
struct SbiStdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes()).unwrap();
        Ok(())
    }
}

impl Write for SbiStdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sbi_write(STDOUT, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}
pub fn sbi_print(args: fmt::Arguments) {
    SbiStdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! sbi_println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::sbi_print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}