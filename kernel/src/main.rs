#![no_std]
#![no_main]

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{DrawTarget, RgbColor};
use crate::frame_buffer::{Display};
use crate::limine_requests::{BASE_REVISION, FRAME_BUFFER_REQUEST};

mod limine_requests;
mod frame_buffer;

#[unsafe(no_mangle)]
unsafe extern "C" fn _main() -> ! {
    assert!(BASE_REVISION.is_supported());
    
    let frame_buffer = FRAME_BUFFER_REQUEST.get_response().unwrap();
    if let Some(frame_buffer) = frame_buffer.framebuffers().next() {
        let mut display = Display::new(frame_buffer);
        display.clear(Rgb888::MAGENTA).unwrap();
    }
    
    hlt_loop()
}

fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    hlt_loop()
}