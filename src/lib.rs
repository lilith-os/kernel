#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner::runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_writer;
pub mod print;
pub mod kernel;
pub mod test_runner;
pub mod panic_handler;
pub mod error;

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
   kernel::Kernel::new().run_tests(test_main)
}

#[test_case]
fn lib_test() {
    println!("bye world!");
}
#[test_case]
fn lib_test_2() {
    println!("bye world!");
}