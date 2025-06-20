use bootloader::BootInfo;
#[allow(unused)]
use crate::{gdt, println};
use crate::interrupts::{idt, pic};
use crate::interrupts::pic::PICS;
use crate::{debug_call, print};

pub(crate) mod debug;

pub struct Kernel {
    boot_info: &'static BootInfo,
}

impl Kernel {
    pub fn new(boot_info: &'static BootInfo) -> Self {
        Self {
            boot_info,
        }
    }

    pub fn init(self) -> Self {
        println!("[kernel init]...");
        debug_call!(idt::init_idt);
        debug_call!(gdt::init_gdt);
        debug_call!(pic::init_pics);
        println!("[kernel init] done\n");
        self
    }
    
    #[cfg(not(feature = "test"))]
    pub fn run(self) -> ! {

        let ptr = 0xdeadbeaf as *mut u8;
        unsafe { *ptr = 42; }
        
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