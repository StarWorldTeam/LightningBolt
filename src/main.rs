#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod kernel;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel::system::start();
    loop {}
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    println!("{}", panic_info);
    loop {}
}
