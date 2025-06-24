#![no_std]
#![no_main]

use embedded_graphics::Drawable;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{Point, Primitive, RgbColor, Size, WebColors};
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle, Triangle};
use uart_16550_driver::serial_println;
use crate::frame_buffer::{Display};
use crate::limine_requests::{BASE_REVISION, FRAME_BUFFER_REQUEST};
use crate::terminal::Terminal;
use core::fmt::Write;
use qemu_bindings::exit::{exit_qemu, QemuExitCode};

mod limine_requests;

mod terminal;
mod frame_buffer;

#[unsafe(no_mangle)]
unsafe extern "C" fn _main() -> ! {
    assert!(BASE_REVISION.is_supported());

    let frame_buffer = FRAME_BUFFER_REQUEST.get_response().unwrap();
    if let Some(frame_buffer) = frame_buffer.framebuffers().next() {
        let mut term = Terminal::new(frame_buffer);
        //term.grid();
        write!(term, "Hello World! {} ", 100).unwrap();
        term.write(format_args!("Hello World! {}", 10));
        term.new_line();
        term.write(format_args!("Bye World!"));
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
    serial_println!("{}", _info);
    exit_qemu(QemuExitCode::Failure);
    hlt_loop()
}