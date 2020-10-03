// KERNEL-MODE SERIAL DRIVER

pub struct Device {
    pub iobase : u16,
}

impl Device {

    //============================================================
    // ctor
    //
    //============================================================
    pub fn new(iobase: u16) -> Device {
        Device { iobase: iobase }
    }

    //============================================================
    //
    //
    //============================================================
    pub fn send(& self, data: u8) {
        unsafe { llvm_asm!("outb $1, $0" :: "N{dx}"(self.iobase), "{al}"(data) :: "volatile"); }
    }
}
