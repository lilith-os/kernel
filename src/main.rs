#![no_std]
#![no_main]

use core::panic::PanicInfo;
use vga_buffer_driver::prelude::Writer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    
    let mut writer = Writer::default();
    
    writer.write_string("Hello, world!\n");

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}