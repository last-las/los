use core::panic::PanicInfo;
use crate::sbi::sbi_shutdown;

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    if let Some(location) = panic_info.location() {
        println!("Panicked at {}:{} {}", location.file(), location.line(), panic_info.message().unwrap());
    } else {
        println!("Panicked: {}", panic_info.message().unwrap());
    }
    sbi_shutdown();
    loop{}
}