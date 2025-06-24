use core::fmt::{Arguments, Write};
use embedded_graphics::Drawable;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{Point, Primitive, RgbColor};
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use embedded_graphics::text::Text;
use limine::framebuffer::Framebuffer;
use uart_16550_driver::serial_println;
use crate::frame_buffer::Display;
use crate::terminal::style::TerminalStyle;

mod style;

pub struct Terminal<'a> {
    display: Display<'a>,
    style: TerminalStyle<'a>,
    rows: u64,
    cols: u64,
    cursor: (u64, u64),
}

impl<'a> Terminal<'a> {
    pub fn new(framebuffer: Framebuffer<'a>) -> Self {
        let display = Display::new(framebuffer);
        let style = TerminalStyle::default();

        let width = display.width();
        let height = display.height();

        let f_width =  style.font_width();
        let f_height =  style.font_height();

        Self {
            display,
            style,
            rows: height / f_height,
            cols: width / f_width,
            cursor: (0, f_height),
        }
    }

    pub fn grid(&mut self) {
        let &mut Self {
            ref mut display, ref style,
            rows, cols, ..
        } = self;

        let width = display.width();
        let height = display.height();

        let f_width =  style.font_width();
        let f_height =  style.font_height();

        for i in 0..rows {
            Line::new(Point::new(0, (f_height * i) as i32), Point::new(width as i32, (f_height * i) as i32))
                .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1))
                .draw(display).unwrap();
        }

        for i in 0..cols {
            Line::new(Point::new((f_width * i) as i32, 0), Point::new((f_width * i) as i32, height as i32))
                .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1))
                .draw(display).unwrap();
        }
    }

    pub fn write(&mut self, args: Arguments) {
        let string = args.as_str().unwrap();
        Text::with_text_style(
            string,
            Point::new(self.cursor.0 as i32, self.cursor.1 as i32 - 2),
            MonoTextStyle::new(self.style.mono_font(), self.style.fg()),
            self.style.text_style()
        )
            .draw(&mut self.display).unwrap();

        self.cursor.0 += string.len() as u64 * self.style.font_width();
    }

    pub fn new_line(&mut self) {
        self.cursor = (0, (self.cursor.1 + self.style.font_height()));
    }
}

impl Write for Terminal<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        serial_println!("{}", s);
        self.write(format_args!("{s}"));
        Ok(())
    }
}