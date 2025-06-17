use pic8259::ChainedPics;

pub mod hardware_interrupts;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe {
   ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) 
});

pub fn init_pics() {
   unsafe { PICS.lock().initialize() };
   x86_64::instructions::interrupts::enable();
}