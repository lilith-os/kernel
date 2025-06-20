use bootloader::BootInfo;
use x86_64::structures::paging::{OffsetPageTable, Translate};
use x86_64::{VirtAddr};
#[allow(unused)]
use crate::{gdt, println};
use crate::interrupts::{idt, pic};
use crate::{debug_call, memory};

pub(crate) mod debug;

pub struct Kernel {
    boot_info: &'static BootInfo,
    mapper: OffsetPageTable<'static>,
}

impl Kernel {
    pub fn new(boot_info: &'static BootInfo) -> Self {
        Self {
            boot_info,
            mapper: {
                let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
                unsafe { memory::init(phys_mem_offset) }
            }
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
            let phys = self.mapper.translate_addr(virt);
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