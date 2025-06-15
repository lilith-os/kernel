#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kenel_lib::test_runner::runner)]
#![reexport_test_harness_main = "test_main"]

use kenel_lib::kernel::Kernel;

#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    Kernel::new().run()
}

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    Kernel::new().run_tests(test_main)
}

#[cfg(test)]
mod test {
    use kenel_lib::println;

    #[test_case]
    fn bin_test() {
        println!("hello world!");
        panic!("Oops!");
    }
}