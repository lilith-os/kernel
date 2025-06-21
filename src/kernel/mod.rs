use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use bootloader::BootInfo;
use x86_64::structures::paging::{OffsetPageTable};
use x86_64::{VirtAddr};
#[allow(unused)]
use crate::{gdt, println};
use crate::interrupts::{idt, pic};
use crate::{allocator, debug_call, memory};
use crate::memory::{BootInfoFrameAllocator};
use crate::task::executor::Executor;
use crate::task::keyboard::print_keypresses;
use crate::task::simple_executor::SimpleExecutor;
use crate::task::Task;

pub(crate) mod debug;

pub struct Kernel {
    #[allow(unused)]
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

    pub fn init(mut self) -> Self {
        println!("[kernel init]...");
        debug_call!(idt::init_idt);
        debug_call!(gdt::init_gdt);
        debug_call!(pic::init_pics);
        debug_call!(|| { allocator::heap::heap_init(&mut self.mapper, &mut self.frame_allocator).unwrap(); });
        println!("[kernel init] done\n");
        self
    }
    
    #[cfg(not(feature = "test"))]
    pub fn run(self) -> ! {

        async fn async_number() -> u32 {
            42
        }

        async fn example_task() {
            let number = async_number().await;
            println!("async number: {}", number);
        }

        let mut executor = Executor::new();
        executor.spawn(Task::new(example_task()));
        executor.spawn(Task::new(print_keypresses()));
        executor.run()
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