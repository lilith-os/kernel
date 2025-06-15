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

    pub fn run(self) -> ! {
        println!("Running...");
        #[allow(clippy::empty_loop)]
        loop {}
    }
    
    pub fn run_tests(self, test_main: impl Fn()) -> ! {
        test_main();
        #[allow(clippy::empty_loop)]
        loop {}
    }
}