#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner::runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_writer;
pub mod print;
pub mod kernel;
pub mod test_runner;
pub mod panic_handler;
pub mod error;
mod interrupts;

#[cfg(all(feature = "test", test))]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
   kernel::Kernel::new()
       .init()
       .run_tests(test_main)
}