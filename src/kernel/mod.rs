use bootloader::BootInfo;
use x86_64::structures::paging::{OffsetPageTable, Page};
use x86_64::{VirtAddr};
#[allow(unused)]
use crate::{gdt, println};
use crate::interrupts::{idt, pic};
use crate::{debug_call, memory};
use crate::memory::{create_example_mapping, BootInfoFrameAllocator};

pub(crate) mod debug;

pub struct Kernel {
    boot_info: &'static BootInfo,
    mapper: OffsetPageTable<'static>,
    frame_allocator: BootInfoFrameAllocator,
}

impl Kernel {
    pub fn new(boot_info: &'static BootInfo) -> Self {
        Self {
            boot_info,
            mapper: {
                let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
                unsafe { memory::init(phys_mem_offset) }
            },
            frame_allocator: unsafe {
                BootInfoFrameAllocator::new(&boot_info.memory_map)
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
    pub fn run(mut self) -> ! {
        let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
        create_example_mapping(page, &mut self.mapper, &mut self.frame_allocator);

        let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
        unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) }

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