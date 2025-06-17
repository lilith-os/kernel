#[allow(unused)]
use crate::{gdt, println};
use crate::interrupts::{idt, pic};
use crate::interrupts::pic::PICS;
use crate::{debug_call, print};

mod debug;

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

    pub fn init(self) -> Self {
        println!("[kernel]...");
        debug_call!(idt::init_idt);
        debug_call!(gdt::init_gdt);
        debug_call!(pic::init_pics);
        println!("[kernel] done\n");
        self
    }
    
    #[cfg(not(feature = "test"))]
    pub fn run(self) -> ! {
        println!("Running...");
        
        println!("Done!");
        #[allow(clippy::empty_loop)]
        loop {
            x86_64::instructions::hlt();
        }
    }
    
    #[cfg(feature = "test")]
    pub fn run_tests(self, test_main: impl Fn()) -> ! {
        test_main();
        #[allow(clippy::empty_loop)]
        loop {
            x86_64::instructions::hlt();
        }
    }
}