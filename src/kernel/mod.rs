use bootloader::BootInfo;
use x86_64::VirtAddr;
#[allow(unused)]
use crate::{gdt, println};
use crate::interrupts::{idt, pic};
use crate::interrupts::pic::PICS;
use crate::{debug_call, print};
use crate::memory::{active_level_4_table, translate_addr};

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

        let boot_info = self.boot_info;
        
        let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

        let addresses = [
            // the identity-mapped vga buffer page
            0xb8000,
            // some code page
            0x201008,
            // some stack page
            0x0100_0020_1a10,
            // virtual address mapped to physical address 0
            boot_info.physical_memory_offset,
        ];

        for &address in &addresses {
            let virt = VirtAddr::new(address);
            let phys = unsafe { translate_addr(virt, phys_mem_offset) };
            println!("{:?} -> {:?}", virt, phys);
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