use core::ptr;

// | size     | node size          | previous node
// |----------|--------------------|
// | size     | node size          |
// | previous | 0x0000000000000000 | <- in linked list
// | next     | 0x0000000000000000 | <- in linked list
// |          |                    |
// | ...      | ...                |
// |          |                    |
// | size     | node size          |
// |----------|--------------------|
// | size     | node size          | next node

const NODE_MINIMUM_SIZE : u64 = 32;
const MINIMUM_NODE_SIZE : u64 = 32;

pub struct NodeHeader {
    pub size     : u64,                // Size and busy/free flag (bit 0)
    pub previous : *mut NodeHeader,    // Previous node in linked list (null if first)
    pub next     : *mut NodeHeader,    // Next node in linked list (null if last)
    //  ...
    //  ...
    pub size_end : u64,               // Size and busy/free flag (bit 0)
}

pub trait NodeHeaderExt<'a> {
    unsafe fn unbox(self) -> &'a mut NodeHeader;
}

impl<'a> NodeHeaderExt<'a> for *mut NodeHeader {
    #[inline] unsafe fn unbox(self) -> &'a mut NodeHeader { &mut *self }
}

//#[derive(Copy, Clone)]
pub struct Node {
    pub buffer : *mut NodeHeader,
}

impl Node {

    //============================================================
    //
    //
    //============================================================
    pub const fn from(ptr: *mut u8) -> Self {
        Node { buffer : ptr as *mut NodeHeader }
    }

    //============================================================
    //
    //
    //============================================================
    pub unsafe fn new(ptr: *mut u8, size: u64) -> Self {

        //ptr::write_bytes(ptr, 0, size as usize);    // DEBUG - memset(0)

        let node = Node::from(ptr);

        node.buffer.unbox().size     = size;
        node.buffer.unbox().previous = ptr::null_mut();
        node.buffer.unbox().next     = ptr::null_mut();

        let node_end = ptr.add(size as usize - 8);

        ptr::write(node_end as *mut u64, size);     // set node-end size

        node
    }

    //============================================================
    //
    //
    //============================================================
    pub fn size(&self) -> u64 {
        unsafe { self.buffer.unbox().size & (!0x1) }
    }

    //============================================================
    //
    //
    //============================================================
    pub fn is_free(& self) -> bool {
        unsafe { (self.buffer.unbox().size & 0x1) == 0 }
    }

    //============================================================
    //
    //
    //============================================================
    pub fn set_free(&mut self, free : bool) {
        unsafe {
            let node_end = self.buffer.cast::<u8>().add(self.size() as usize - 8);

            let sizex = self.size() | (!free as u64);

            self.buffer.unbox().size = sizex;

            ptr::write(node_end as *mut u64, sizex);
        }
    }

    //============================================================
    //
    //
    //============================================================
    pub fn split(self, size: u64) -> (Node, Node) {

        debug_assert!(self.is_free());
        debug_assert!(self.size() > size);
        debug_assert!(self.size() - size > MINIMUM_NODE_SIZE);

        unsafe {
            let node1 = self.buffer.cast::<u8>();
            let node2 = self.buffer.cast::<u8>().add(size as usize);

            let leftover = self.size() - size;

            (Node::new(node1, size), Node::new(node2, leftover))
        }
    }

    //============================================================
    //
    //
    //============================================================
    pub unsafe fn coalesce(self, next : Node) -> Node {

        debug_assert!(self.is_free());
        debug_assert!(next.is_free());
        debug_assert!(self.buffer.cast::<u8>().add(self.size() as usize)==next.buffer.cast::<u8>());

        let newsize = self.size() + next.size();
        Node::new(self.buffer.cast::<u8>(), newsize)
    }

    //============================================================
    //
    //
    //============================================================
    pub unsafe fn previous(&self) -> Option<Node> {

        let previous_size_ptr = self.buffer.cast::<u64>().sub(1);
        let previous_size     = ptr::read(previous_size_ptr) & (!0x1);

        match previous_size {
            0 => None,
            _ => Some(Node::from(self.buffer.cast::<u8>().sub(previous_size as usize)))
        }
    }

    //============================================================
    //
    //
    //============================================================
    pub unsafe fn next(&self) -> Option<Node> {

        let next_size_ptr = self.buffer.cast::<u8>().add(self.size() as usize);
        let next_size     = ptr::read(next_size_ptr.cast::<u64>()) & (!0x1);

        match next_size {
            0 => None,
            _ => Some(Node::from(next_size_ptr))
        }
    }
}
