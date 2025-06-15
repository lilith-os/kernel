#![no_std]

use core::default::Default;
use vga_buffer_driver::prelude::VGAWriter;
use core::fmt::Write;

pub struct Kernel {
    vga_writer: VGAWriter,
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}

impl Kernel {
    pub fn new() -> Self {
        Self { vga_writer: Default::default() }
    }
    
    pub fn run(mut self) -> ! {
        write!(self.vga_writer, "Hello, world!").unwrap();
        
        #[allow(clippy::empty_loop)]
        loop {}
    }
}