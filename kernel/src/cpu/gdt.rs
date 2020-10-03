use crate::paging::*;

pub static mut PTR: DescriptorTablePointer = DescriptorTablePointer { limit: 0, base: 0 };
pub static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

#[derive(Debug)]
#[repr(C, packed)]
pub struct DescriptorTablePointer {
    pub limit: u16,
    pub base:  u64,
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct GlobalDescriptorTable {
    descriptors: [u64; 8],
    size: usize,
}

#[derive(Debug)]
pub struct SegmentSelector(u16);

impl GlobalDescriptorTable {

    //============================================================
    //
    //
    //============================================================
    pub const fn new() -> GlobalDescriptorTable {
        GlobalDescriptorTable {
            descriptors: [0; 8],
            size: 1,
        }
    }

    //============================================================
    //
    //
    //============================================================
    pub fn add_entry(&mut self, descriptor: SegmentDescriptor) -> SegmentSelector {

        let index = self.size;

        match descriptor {
            UserSegment(value) => {
                self.descriptors[self.size] = value;
                self.size += 1;
            }
            SystemSegment(value) => {
                self.descriptors[self.size]   = value as u64;
                self.descriptors[self.size+1] = (value >> 64) as u64;
                self.size += 2;
            }
        }

        let rpl = match descriptor {
            UserSegment(value) => (value & 0x3) >> 45,
            SystemSegment(_)   => 0,
        };

        SegmentSelector::new(index as u16, rpl as u8)
    }
}

impl SegmentSelector {

    //============================================================
    /// Create a new SegmentSelector
    //
    //============================================================
    pub const fn new(index: u16, rpl: u8) -> SegmentSelector {
        SegmentSelector(index << 3 | (rpl as u16))
    }
}

//============================================================
// PER CPU
//
//============================================================
pub fn init() {
    unsafe {
        let kernel_code = GDT.add_entry(UserSegment(0x00209a0000000000));      // Kernel Code
        let kernel_data = GDT.add_entry(UserSegment(0x0000920000000000));      // Kernel Data
        let user_code   = GDT.add_entry(UserSegment(0x0020fa0000000000));      // User Code (Ring-3)
        let user_data   = GDT.add_entry(UserSegment(0x0000f20000000000));      // User Data (Ring-3)
        // GDT.add_entry(SegmentDescriptor(xxxx));                             // TSS (todo)

        PTR.limit = (GDT.size * 8 - 1) as u16;
        PTR.base  = GDT.descriptors.as_ptr() as u64;

        llvm_asm!("lgdt ($0)" :: "r" (&PTR) : "memory");

        crate::println!("kernel-code: {}, kernel-data: {}", kernel_code.0, kernel_data.0);

        load_cs(kernel_code);
        llvm_asm!("movw $0, %ds " :: "r" (kernel_data.0) : "memory");
        llvm_asm!("movw $0, %es " :: "r" (kernel_data.0) : "memory");
    }
}

pub unsafe fn load_cs(sel: SegmentSelector) {
    llvm_asm!("pushq $0; \
          leaq  1f(%rip), %rax; \
          pushq %rax; \
          lretq; \
          1:" :: "ri" (sel.0 as usize) : "rax" "memory");
}

#[derive(Debug)]
pub enum SegmentDescriptor {
    UserSegment(u64),       // Code and data segments
    SystemSegment(u128),    // TSS descriptor
}
use SegmentDescriptor::*;
