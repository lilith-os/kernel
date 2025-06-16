#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel_lib::test_runner::runner)]
#![reexport_test_harness_main = "test_main"]

use kernel_lib::kernel::Kernel;

#[cfg(not(feature = "test"))]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    Kernel::new().run()
}

#[cfg(all(feature = "test", test))]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    Kernel::new().run_tests(test_main)
}

#[cfg(test)]
mod test {
    use kernel_lib::println;

    #[test_case]
    fn bin_test() {
        println!("hello world!");
    }
}