/* TODO:
    2. multi cores
    3. support multithreading
    4. support cpu infos
*/

pub const LOG_STATE: &str = env!("LOG");

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(
                concat!("\x1b[31m","[ERROR] ", $fmt, "\n\x1b[0m")
                $(, $($arg)+)?
            )
        );
    }
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        if $crate::log::LOG_STATE.eq("ERROR") {
            $crate::console::print(format_args!(
                    concat!("\x1b[33m","[WARN] ", $fmt, "\n\x1b[0m")
                    $(, $($arg)+)?
                )
            );
        }
    }
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        match $crate::log::LOG_STATE {
            "INFO" | "DEBUG" | "TRACE" => {
                $crate::console::print(format_args!(
                        concat!("\x1b[34m","[INFO] ", $fmt, "\n\x1b[0m")
                        $(, $($arg)+)?
                    )
                );
            },
            _ => {}
        }
    }
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        match $crate::log::LOG_STATE {
            "DEBUG" | "TRACE" => {
                $crate::console::print(format_args!(
                        concat!("\x1b[32m","[DEBUG] ", $fmt, "\n\x1b[0m")
                        $(, $($arg)+)?
                    )
                );
            },
            _ => {}
        }
    }
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        match $crate::log::LOG_STATE {
             "TRACE" => {
                $crate::console::print(format_args!(
                        concat!("\x1b[90m","[TRACE] ", $fmt, "\n\x1b[0m")
                        $(, $($arg)+)?
                    )
                );
            },
            _ => {}
        }
    }
}

