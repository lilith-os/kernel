use core::fmt;
use x86_64::instructions::interrupts::without_interrupts;
use crate::vga_writer::VGA_WRITER;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    
    without_interrupts(|| {
        VGA_WRITER.lock().write_fmt(args).unwrap();
    });
}