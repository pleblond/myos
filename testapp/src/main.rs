#![no_std]
#![no_main]
#![feature(llvm_asm)]

#[no_mangle]
pub extern "C" fn _start() {

    print("Hello World!");

    process_exit(0);
}

pub fn print(string : &str) -> u32 {       // TEST SYSCALL
    let res: u32;
    unsafe {
        llvm_asm!("int 0x80" : "={eax}"(res) : "{eax}"(1u32), "{rsi}"(string.as_ptr() as u64), "{ecx}"(string.len() as u32) : : "intel");
    }
    res
}

pub fn process_exit(code: usize) -> ! {
    unsafe {
        llvm_asm!("int 0x80" :: "{eax}"(5u32), "{ecx}"(code as u32) : : "intel");
    }
    unreachable!();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    print("PANIC");
    process_exit(1);
}
