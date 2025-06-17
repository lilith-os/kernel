use crate::interrupts::pic::PIC_1_OFFSET;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET, 
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
    
    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}