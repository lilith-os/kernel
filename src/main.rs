#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel_lib::test_runner::runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use kernel_lib::kernel::Kernel;

#[cfg(not(feature = "test"))]
entry_point!(kernel_entry);

#[cfg(not(feature = "test"))]
fn kernel_entry(boot_info: &'static BootInfo) -> ! {
    Kernel::new(boot_info)
        .init()
        .run()
}

#[cfg(all(feature = "test", test))]
entry_point!(test_kernel_entry);
#[cfg(all(feature = "test", test))]
fn test_kernel_entry(boot_info: &'static BootInfo) -> ! {
    Kernel::new(boot_info)
        .init()
        .run_tests(test_main)
}

#[cfg(test)]
mod test {
    use kernel_lib::println;

    #[test_case]
    fn bin_test() {
        println!("hello world!");
    }
}