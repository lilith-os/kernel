#![no_std]

use core::default::Default;
mod vga_writer;
pub mod print;

pub struct Kernel {}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}

impl Kernel {
    pub fn new() -> Self {
        Self { }
    }
    
    pub fn run(self) -> ! {
        println!("Running...");
        panic!("Something went wrong");
        #[allow(clippy::empty_loop)]
        loop {}
    }
}