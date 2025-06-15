#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel_lib::test_runner::runner)]
#![reexport_test_harness_main = "test_main"]

use kernel_lib::{init_test_entry, println};

init_test_entry!();

#[test_case]
fn test_println() {
    println!("test_println output");
}