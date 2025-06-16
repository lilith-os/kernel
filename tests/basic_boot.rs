#![no_std]
#![no_main]

use qemu_bindings::exit::{exit_qemu, QemuExitCode};
use kernel_lib::{init_test_entry, println, test_main};

init_test_entry!();

test_main!(test_println);

fn test_println() {
    println!("test_println output");
}