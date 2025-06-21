#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner::runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

mod vga_writer;
pub mod print;
pub mod kernel;
pub mod test_runner;
pub mod panic_handler;
pub mod error;
mod interrupts;
mod gdt;
pub mod memory;
pub mod allocator;
pub mod task;

#[cfg(all(feature = "test", test))]
crate::test_runner::entry_point!(kernel_test_main);

#[cfg(all(feature = "test", test))]
fn kernel_test_main(boot_info: &'static crate::test_runner::BootInfo) -> ! {
    kernel::Kernel::new(boot_info)
        .init()
        .run_tests(test_main)
}