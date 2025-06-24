use core::convert::Infallible;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{Dimensions, DrawTarget, Point, RgbColor, Size};
use embedded_graphics::primitives::Rectangle;
use limine::framebuffer::Framebuffer;

pub struct Display<'a> {
    frame_buffer: Framebuffer<'a>,
}

impl<'a> Display<'a> {
    pub fn new(frame_buffer: Framebuffer<'a>) -> Self {
        let bits_per_pixel = frame_buffer.bpp();
        if bits_per_pixel == 8 * 4 {
            Self { frame_buffer }
        } else {
            panic!("DrawTarget implemented for RGB888, but bpp doesn't match RGB888");
        }
    }

    fn get_pixel(&self, color: Rgb888) -> [u8; 4] {
        let mut n = 0;
        n |= ((color.r() as u32) & ((1 << self.frame_buffer.red_mask_size()) - 1))
            << self.frame_buffer.red_mask_shift();
        n |= ((color.g() as u32) & ((1 << self.frame_buffer.green_mask_size()) - 1))
            << self.frame_buffer.green_mask_shift();
        n |= ((color.b() as u32) & ((1 << self.frame_buffer.blue_mask_size()) - 1))
            << self.frame_buffer.blue_mask_shift();
        n.to_ne_bytes()
    }

    fn frame_buffer_mut(&mut self) -> &mut [u8] {
        // Safety: The memory is mapped to the frame buffer and nothing else is referencing it
        unsafe {
            core::slice::from_raw_parts_mut(
                self.frame_buffer.addr(),
                (self.frame_buffer.pitch() * self.frame_buffer.height()) as usize,
            )
        }
    }

    pub fn width(&self) -> u64 {
        self.frame_buffer.width()
    }

    pub fn height(&self) -> u64 {
        self.frame_buffer.height()
    }
}

impl Dimensions for Display<'_> {
    fn bounding_box(&self) -> Rectangle {
        Rectangle {
            top_left: Point { x: 0, y: 0 },
            size: Size {
                width: self.frame_buffer.width().try_into().unwrap(),
                height: self.frame_buffer.height().try_into().unwrap(),
            },
        }
    }
}

impl DrawTarget for Display<'_> {
    type Color = Rgb888;

    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        let bytes_per_pixel = (self.frame_buffer.bpp() / 8) as usize;
        pixels.into_iter().for_each(|pixel| {
            let point = pixel.0;
            if (0..self.frame_buffer.width()).contains(&(point.x as u64))
                && (0..self.frame_buffer.height()).contains(&(point.y as u64))
            {
                let color = pixel.1;
                let buffer_position = point.y as usize * self.frame_buffer.pitch() as usize
                    + point.x as usize * bytes_per_pixel;
                let pixel = self.get_pixel(color);
                let buffer = self.frame_buffer_mut();
                buffer[buffer_position..buffer_position + bytes_per_pixel].copy_from_slice(&pixel);
            }
        });
        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let pixel = self.get_pixel(color);
        let bytes_per_pixel = (self.frame_buffer.bpp() / 8) as usize;
        let pitch = self.frame_buffer.pitch() as usize;
        let buffer = self.frame_buffer_mut();
        // Draw to the top row
        for x in area.top_left.x..area.top_left.x + area.size.width as i32 {
            if let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(area.top_left.y)) {
                let buffer_position = y * pitch + x * bytes_per_pixel;
                buffer[buffer_position..buffer_position + bytes_per_pixel].copy_from_slice(&pixel);
            }
        }
        // Copy the top row to all other rows
        let top_row_start =
            usize::try_from(area.top_left.y).unwrap_or(0) * pitch + usize::try_from(area.top_left.x).unwrap_or(0) * bytes_per_pixel;
        let top_row = top_row_start..top_row_start + area.size.width as usize * bytes_per_pixel;
        for y in area.top_left.y + 1..area.top_left.y + area.size.height as i32 {
            let row_start = usize::try_from(y).unwrap_or(0) * pitch + usize::try_from(area.top_left.x).unwrap_or(0) * bytes_per_pixel;
            buffer.copy_within(top_row.clone(), row_start);
        }
        Ok(())
    }
}