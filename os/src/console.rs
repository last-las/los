use core::fmt;
use core::fmt::Write;
use crate::sbi::sbi_console_putchar;
use spin::Mutex;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            sbi_console_putchar(c);
        }
        Ok(())
    }
}

lazy_static!{
    static ref STDOUT: Mutex<Stdout> = Mutex::new(Stdout);
}

pub fn print(args: fmt::Arguments) {
     STDOUT.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}