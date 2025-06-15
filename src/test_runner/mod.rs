use qemu_bindings::exit::{exit_qemu, QemuExitCode};
use uart_16550_driver::{serial_print, serial_println};

pub fn runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run()
    }
    exit_qemu(QemuExitCode::Success)
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where T: Fn() {
    fn run(&self) {
        serial_print!("{}... ", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}