use x86_64::structures::idt::InterruptStackFrame;
use crate::{print, println};
use crate::interrupts::pic::hardware_interrupts::InterruptIndex;
use crate::interrupts::pic::PICS;

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame)
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");
    
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}