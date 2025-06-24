use embedded_graphics::mono_font::{MonoFont};
use embedded_graphics::mono_font::ascii::{FONT_6X13, FONT_9X15};
use embedded_graphics::mono_font::iso_8859_16::FONT_7X14;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::RgbColor;
use embedded_graphics::text::{TextStyle, TextStyleBuilder};

pub struct TerminalStyle<'a> {
    foreground_color: Rgb888,
    background_color: Rgb888,
    text_style: TextStyle,
    mono_font: MonoFont<'a>
}

impl Default for TerminalStyle<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalStyle<'_> {
    pub fn new() -> Self {
        Self {
            foreground_color: Rgb888::WHITE,
            background_color: Rgb888::BLACK,
            text_style: TextStyle::default(),
            mono_font: FONT_9X15
        }
    }
    
    pub const fn font_height(&self) -> u64 {
        self.mono_font.character_size.height as u64
    }
    
    pub const fn font_width(&self) -> u64 {
        self.mono_font.character_size.width as u64
    }
    
    pub fn mono_font(&self) -> &MonoFont {
        &self.mono_font
    }
    
    pub fn fg(&self) -> Rgb888 {
        self.foreground_color
    }
    
    pub fn bg(&self) -> Rgb888 {
        self.background_color
    }
    
    pub fn text_style(&self) -> TextStyle {
        self.text_style
    }
}