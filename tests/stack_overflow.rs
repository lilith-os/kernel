#![no_std]
#![no_main]

use qemu_bindings::exit::{exit_qemu, QemuExitCode};
use volatile::Volatile;
use kernel_lib::{init_test_entry};

init_test_entry!();

fn test_main() {
    use uart_16550_driver::{serial_print, serial_println};
    serial_print!("{}... ", core::any::type_name_of_val(&stack_overflow));
    stack_overflow();
    serial_println!("[failed]");
    exit_qemu(QemuExitCode::Failure)
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    Volatile::new(0).read();
}