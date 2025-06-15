#![no_std]

use core::default::Default;
use vga_buffer_driver::prelude::Writer;
use core::fmt::Write;

pub struct Kernel {
    writer: Writer,
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}

impl Kernel {
    pub fn new() -> Self {
        Self { writer: Default::default() }
    }
    
    pub fn run(mut self) -> ! {
        write!(self.writer, "Hello, world!");
        
        #[allow(clippy::empty_loop)]
        loop {}
    }
}