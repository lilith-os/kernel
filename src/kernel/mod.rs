use crate::interrupts::idt::init_idt;
use crate::println;

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
        init_idt();
        self
    }
    
    #[cfg(not(feature = "test"))]
    pub fn run(self) -> ! {
        println!("Running...");
        x86_64::instructions::interrupts::int3();
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