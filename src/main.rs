#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kenel_lib::Kernel;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    Kernel::new().run()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}