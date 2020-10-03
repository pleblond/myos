use alloc::alloc::{ GlobalAlloc, Layout };
use core::ptr;
use core::cmp;
use super::node::{ Node, NodeHeader, NodeHeaderExt };
use super::arena::{ Arena };

#[global_allocator]
pub static mut ALLOCATOR: HeapAllocator = HeapAllocator::new();

#[repr(align(16))]
struct TestSpace([u8; 0x100000]);

static mut TESTSPACE : TestSpace = TestSpace([0; 0x100000]);  // 1MB

pub struct HeapAllocator {
    arena : Arena,
}

impl HeapAllocator {

    //============================================================
    //
    //
    //============================================================
    pub const fn new() -> Self {
        HeapAllocator { arena: Arena::new() }
    }

    //============================================================
    //
    //
    //============================================================
    pub fn init() {
        unsafe {
            let ptr = TESTSPACE.0.as_mut_ptr();

            let len = TESTSPACE.0.len() as u64;

            let SPACE = Node::new(ptr.add(0x8), len-16);

            ALLOCATOR.arena.push_node(&SPACE);
        }
    }

    //============================================================
    //
    //
    //============================================================
    fn allocate(&mut self, layout: Layout) -> *mut u8 {

        let size = cmp::max(layout.size() + 16, 32) as u64;
        let node = self.arena.find_node(size);

        match node {
            None => {
                return ptr::null_mut()  // NO SPACE LEFT
            },
            Some(mut node) => {

                if node.size() - size > 32 {
                    let (slice, leftover) = node.split(size);
                    self.arena.push_node(&leftover);
                    node = slice;
                }

                node.set_free(false);
                node.buffer.cast::<u8>()
            }
        }
    }

    //============================================================
    //
    //
    //============================================================
    unsafe fn deallocate(&mut self, allocation_ptr: *mut u8) {

        let mut allocation = Node::from(allocation_ptr);
        allocation.set_free(true);

        if let Some(mut previous) = allocation.previous() {
            if previous.is_free() {
                self.arena.remove_node(&mut previous);
                allocation = previous.coalesce(allocation);
            }
        }

        if let Some(mut next) = allocation.next() {
            if next.is_free() {
                self.arena.remove_node(&mut next);
                allocation = allocation.coalesce(next);
            }
        }

        // DEBUG - MEMSET 0
        allocation = Node::new(allocation.buffer.cast::<u8>(), allocation.size());

        self.arena.push_node(&allocation);
    }
}

unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        crate::println!("[alloc()] size: 0x{:x}", layout.size());
        ALLOCATOR.allocate(layout).add(8)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        crate::println!("[dealloc()] size: 0x{:x}", layout.size());
        ALLOCATOR.deallocate(ptr.sub(8));
    }
}
