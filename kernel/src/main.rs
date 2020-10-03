#![feature(llvm_asm)]
#![feature(alloc_error_handler)] // at the top of the file
#![feature(const_fn)]
#![no_std]
#![no_main]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};  //rc::Rc, vec,

use core::panic::PanicInfo;
use bootloader::BootInfo;

mod cpu;
mod console;
mod ktty;
mod paging;
mod heap;

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {

    println!("\n============== BOOTINFO ================\n");
    println!("DUMP {:#?}", boot_info);
    println!("\n========================================\n");

    println!("Initializing Heap Allocator...");
    heap::HeapAllocator::init();

    println!("Loading GDT (replacing trampoline)...");
    cpu::gdt::init();
    println!("Loading GDT... SURVIVED!");

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
