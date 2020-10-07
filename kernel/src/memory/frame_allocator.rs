use bootloader::BootInfo;
use bootloader::bootinfo::MemoryRegionType::Usable;

pub static mut gFRAME_ALLOCATOR: FrameAllocator = FrameAllocator::new();

#[derive(Debug, Clone, Copy)]
struct FrameRange(u64, u64);

#[derive(Debug)]
pub struct FrameAllocator {
    ranges : [FrameRange; 16]
}

impl FrameAllocator {

    //============================================================
    //
    //
    //============================================================
    const fn new() -> FrameAllocator {
        FrameAllocator {
            ranges: [FrameRange(0, 0); 16]
        }
    }

    //============================================================
    //
    //
    //============================================================
    pub fn init(info: &BootInfo) {

        let regions = info.memory_map.iter().filter(|o| o.region_type==Usable);
        let mut ranges = unsafe { gFRAME_ALLOCATOR.ranges.iter_mut() };

        for (region, mut range) in regions.zip(ranges) {
            range.0 = region.range.start_addr() >> 12;
            range.1 = region.range.end_addr() >> 12;
        }

        unsafe { crate::println!("{:#x?}", gFRAME_ALLOCATOR); }
    }

    //============================================================
    //
    //
    //============================================================
    pub fn allocate_frame() -> Option<u64> {

        for range in unsafe { gFRAME_ALLOCATOR.ranges.iter_mut() } {
            if range.0 < range.1 {
                let address = range.0 << 12;
                range.0 += 1;
                return Some(address);
            }
        }
        None
    }
}
