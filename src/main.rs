#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in b"Hello, World!".iter().enumerate() {
        unsafe {
            vga_buffer.offset(i as isize * 2).write(byte);
            vga_buffer.offset(i as isize * 2 + 1).write(0xff);
        }
    }

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}