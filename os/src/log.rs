pub const LOG_STATE: &str = env!("LOG");

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(
                concat!("\x1b[31m","[ERROR-hart{}] ", $fmt, "\n\x1b[0m")
                ,$crate::processor::get_hart_id() $(, $($arg)+)?
            )
        );
    }
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        if $crate::log::LOG_STATE.eq("ERROR") {
            $crate::console::print(format_args!(
                    concat!("\x1b[33m","[WARN-hart{}] ", $fmt, "\n\x1b[0m")
                    ,$crate::processor::get_hart_id() $(, $($arg)+)?
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
                    concat!("\x1b[34m","[INFO-hart{}] ", $fmt, "\n\x1b[0m")
                    ,$crate::processor::get_hart_id() $(, $($arg)+)?
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
                        concat!("\x1b[32m","[DEBUG-hart{}] ", $fmt, "\n\x1b[0m")
                        ,$crate::processor::get_hart_id() $(, $($arg)+)?
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
                        concat!("\x1b[90m","[TRACE-hart{}] ", $fmt, "\n\x1b[0m")
                        ,$crate::processor::get_hart_id() $(, $($arg)+)?
                    )
                );
            },
            _ => {}
        }
    }
}

