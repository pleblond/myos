mod idt;
use core::mem;
use crate::cpu::gdt::{DescriptorTablePointer};
use idt::{InterruptDescriptorTable, InterruptStackFrame};

pub static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn initialize() {

    unsafe { IDT.breakpoint.set_handler_fn(breakpoint_handler); }

    let ptr = DescriptorTablePointer {
        base: unsafe { (&IDT) as *const _ as u64 },
        limit: (mem::size_of::<InterruptDescriptorTable>() - 1) as u16,
    };

    unsafe { llvm_asm!("lidt ($0)" :: "r" (&ptr) : "memory"); }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    crate::println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
