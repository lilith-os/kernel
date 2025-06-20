use bootloader::BootInfo;
use x86_64::VirtAddr;
#[allow(unused)]
use crate::{gdt, println};
use crate::interrupts::{idt, pic};
use crate::interrupts::pic::PICS;
use crate::{debug_call, print};
use crate::memory::active_level_4_table;

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

        let l4_table = unsafe { active_level_4_table(VirtAddr::new(self.boot_info.physical_memory_offset)) };

        for (i, entry) in l4_table.iter().enumerate() {
            if !entry.is_unused() {
                println!("L4 {:?}\n", entry);
            }
        }

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