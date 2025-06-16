#[allow(unused)]
use crate::{gdt, println};
use crate::interrupts::idt;

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
        idt::init_idt();
        gdt::init_gdt();
        self
    }
    
    #[cfg(not(feature = "test"))]
    pub fn run(self) -> ! {
        println!("Running...");

        fn stack_overflow() {
            stack_overflow();
        }
        
        stack_overflow();
        
        println!("Done!");
        #[allow(clippy::empty_loop)]
        loop {}
    }
    
    #[cfg(feature = "test")]
    pub fn run_tests(self, test_main: impl Fn()) -> ! {
        test_main();
        #[allow(clippy::empty_loop)]
        loop {}
    }
}