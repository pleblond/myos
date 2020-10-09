#![feature(llvm_asm)]
#![feature(alloc_error_handler)] // at the top of the file
#![feature(const_fn)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};  //rc::Rc, vec,

use core::panic::PanicInfo;
use bootloader::BootInfo;

mod cpu;
mod console;
mod interrupts;
mod ktty;
mod paging;
mod memory;
mod heap;

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {

    println!("\n============== BOOTINFO ================\n");
    println!("DUMP {:#?}", boot_info);
    println!("\n========================================\n");

    println!("Loading IDT...");
    interrupts::initialize();
    println!("Loading GDT (replacing trampoline)...");
    cpu::gdt::init();

    println!("Initializing Frame Allocator...");
    memory::FrameAllocator::init(boot_info);

    println!("Initializing Paging...");
    let mut mapper = paging::Mapper::new();

    println!("Initializing Heap Allocator...");
    heap::HeapAllocator::init();

    println!("\nTesting int3...\n");
    unsafe { llvm_asm!("int3"); }
    println!("Testing int3... SURVIVED!");

    println!("\nBYE...");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n{}", info);
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
