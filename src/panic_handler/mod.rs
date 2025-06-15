#[cfg(not(feature = "test"))]
mod base_panic_handler {
    use core::panic::PanicInfo;
    use crate::println;

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        println!("{}", info);
        loop {}
    }
}

#[cfg(feature = "test")]
mod test_panic_handler {
    use core::panic::PanicInfo;
    use qemu_bindings::exit::{exit_qemu, QemuExitCode};
    use uart_16550_driver::serial_println;

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        serial_println!("[failed]\n");
        serial_println!("Error: {}\n", info);
        exit_qemu(QemuExitCode::Failure);
        loop {}
    }
}
