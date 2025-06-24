#![no_std]
#![no_main]

use embedded_graphics::Drawable;
use embedded_graphics::mono_font::ascii::FONT_7X13_ITALIC;
use embedded_graphics::mono_font::iso_8859_16::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{Point, Primitive, RgbColor, Size, WebColors};
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle, Triangle};
use embedded_graphics::text::{Alignment, Text};
use uart_16550_driver::serial_println;
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
        Circle::new(Point::new(-100, -100), 500)
            .into_styled(PrimitiveStyle::with_stroke(Rgb888::WHITE, 5))
            .draw(&mut display).unwrap();
        Rectangle::new(Point::new(100, 400), Size::new(300,100))
            .into_styled(PrimitiveStyle::with_fill(Rgb888::new(84, 16, 188)))
            .draw(&mut display).unwrap();
        Triangle::new(Point::new(100, 100), Point::new(200, 400), Point::new(300, 600))
            .into_styled(PrimitiveStyle::with_fill(Rgb888::CSS_AQUA))
            .draw(&mut display).unwrap();
        let style = MonoTextStyle::new(&FONT_10X20, Rgb888::WHITE);

        Text::with_alignment(
            "ZDAROVA SANEK)))",
            Point::new((display.width()/2) as i32 - 10, (display.height()/2) as i32 - 20),
            style,
            Alignment::Center,
        )
            .draw(&mut display).unwrap();
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
    hlt_loop()
}