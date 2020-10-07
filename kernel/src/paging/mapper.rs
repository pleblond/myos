use core::ptr::NonNull;
use crate::memory::FrameAllocator;
use super::{PageTable, PageTableEntry};

//
pub struct Mapper {
    p4: NonNull<PageTable>,
}

impl Mapper {

    pub fn new() -> Mapper {
        Mapper {
            p4: unsafe { NonNull::new_unchecked(0x18000001000 as *mut PageTable) },
        }
    }

    //============================================================
    /// Map a page to a frame
    //
    //============================================================
    pub fn map_to(&mut self, page: u64, frame: u64, flags: u64) {

        let index4 = ((page & 0x0000FF8000000000) >> 39) as usize;
        let index3 = ((page & 0x0000007FC0000000) >> 30) as usize;
        let index2 = ((page & 0x000000003FE00000) >> 21) as usize;
        let index1 = ((page & 0x00000000001FF000) >> 12) as usize;

        let p3 = Self::get_or_create(unsafe{self.p4.as_mut()}, index4);
        let p2 = Self::get_or_create(p3, index3);
        let p1 = Self::get_or_create(p2, index2);

        // assert!(is_unused);

        p1.entries[index1].entry = frame | flags;
    }

    //============================================================
    //
    //
    //============================================================
    fn get_or_create(page: &mut PageTable, index: usize) -> &mut PageTable {

        if page.entries[index].entry == 0 {

            let frame = FrameAllocator::allocate_frame().unwrap();  // todo: handle no frame available

            page.entries[index].entry = frame | 1 | 2 | 4;  // PRESENT | WRITABLE | USER_ACCESSIBLE;
        }

        let frame = page.entries[index].entry & 0x000fffff_fffff000;

        unsafe { &mut *((0x18000000000 + frame) as *mut PageTable) }
    }
}
