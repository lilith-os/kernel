use lazy_static::lazy_static;
use spin::mutex::Mutex;
use vga_buffer_driver::prelude::VGAWriter;

lazy_static!(
    pub(crate) static ref VGA_WRITER: Mutex<VGAWriter> = Mutex::new(VGAWriter::default());
);