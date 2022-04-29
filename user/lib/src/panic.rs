use core::panic::PanicInfo;
use crate::syscall::exit;

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    if let Some(location) = panic_info.location() {
        sbi_println!("Panicked at {}:{} {}", location.file(), location.line(), panic_info.message().unwrap());
    } else {
        sbi_println!("Panicked: {}", panic_info.message().unwrap());
    }
    exit(1);
    // This loop should never be reached.
    loop{}
}