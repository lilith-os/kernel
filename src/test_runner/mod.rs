use qemu_bindings::exit::{exit_qemu, QemuExitCode};
use uart_16550_driver::{serial_println};

pub fn runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test()
    }
    exit_qemu(QemuExitCode::Success)
}