use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use crate::gdt::DOUBLE_FAULT_IST_INDEX;
use crate::interrupts::handlers::{breakpoint_handler, double_fault, keyboard_interrupt_handler, timer_interrupt_handler};
use crate::interrupts::pic::hardware_interrupts::InterruptIndex;

lazy_static!{
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }

        idt[InterruptIndex::Timer.as_u8()]
            .set_handler_fn(timer_interrupt_handler);
        
        idt[InterruptIndex::Keyboard.as_u8()]
            .set_handler_fn(keyboard_interrupt_handler);
        
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
