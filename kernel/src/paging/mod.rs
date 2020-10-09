use core::fmt;
pub mod mapper;
pub use mapper::Mapper;

#[derive(Clone, Copy)]
pub struct PhysicalAddress(pub u64);

#[derive(Clone, Copy)]
pub struct VirtualAddress(pub u64);

#[repr(align(4096))]
#[repr(C)]
pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}

#[repr(transparent)]
pub struct PageTableEntry {
    pub entry: u64,
}

impl PageTableEntry {

    pub const fn address(&self) -> PhysicalAddress {
        PhysicalAddress(self.entry & 0x000fffff_fffff000)
    }

    pub const fn is_huge(&self) -> bool {
        (self.entry & 0x80) != 0
    }
}

//============================================================
//
//
//============================================================
pub fn translate_addr(address: VirtualAddress) -> Option<PhysicalAddress> {

    let index4 = (address.0 & 0x0000FF8000000000) >> 39;
    let index3 = (address.0 & 0x0000007FC0000000) >> 30;
    let index2 = (address.0 & 0x000000003FE00000) >> 21;
    let index1 = (address.0 & 0x00000000001FF000) >> 12;
    let index0 = (address.0 & 0x0000000000000FFF) >> 0;

    let mut cr3 : u64;
    unsafe { llvm_asm!("movq %cr3, %rax" : "={eax}"(cr3) ::: "volatile"); }

    let table = unsafe { &(*((0x18000000000 + cr3) as *const PageTable)) };
    let entry = &table.entries[index4 as usize];
    if entry.entry==0 { return None; }

    let table = unsafe { &(*((0x18000000000 + entry.address().0) as *const PageTable)) };
    let entry = &table.entries[index3 as usize];
    if entry.entry==0 { return None; }

    let table = unsafe { &(*((0x18000000000 + entry.address().0) as *const PageTable)) };
    let entry = &table.entries[index2 as usize];
    if entry.entry==0 { return None; }

    if entry.is_huge() {
        return Some(PhysicalAddress(entry.address().0 + (index1<<12) + index0))
    }

    let table = unsafe { &(*((0x18000000000 + entry.address().0) as *const PageTable)) };
    let entry = &table.entries[index1 as usize];
    if entry.entry==0 { return None; }

    Some(PhysicalAddress(entry.address().0 + index0))
}

impl fmt::Debug for VirtualAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VirtualAddress({:#x})", self.0)
    }
}

impl fmt::Debug for PhysicalAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PhysicalAddress({:#x})", self.0)
    }
}
