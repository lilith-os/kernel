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

#[macro_export]
macro_rules! init_test_entry {
    () => {
        #[cfg(all(feature = "test", test))]
        #[unsafe(no_mangle)]
        pub extern "C" fn _start() -> ! {
            kernel_lib::kernel::Kernel::new()
                .init()
                .run_tests(test_main)
        }
    };
}

#[macro_export]
macro_rules! test_main {
    ($($ident:ident),*) => {
        fn test_main() {
            use uart_16550_driver::{serial_print, serial_println};
            $(
                serial_print!("{}... ", core::any::type_name_of_val(&$ident));
                $ident();
                serial_println!("[ok]");
            )*
            exit_qemu(QemuExitCode::Success)
        }
    };
}