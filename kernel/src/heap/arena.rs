use core::ptr;
use super::node::{ Node, NodeHeader, NodeHeaderExt };

pub struct Arena {
    pub orders : [*mut NodeHeader; 64],
}

impl Arena {

    //============================================================
    //
    //
    //============================================================
    pub const fn new() -> Self {
        Arena { orders: [ptr::null_mut(); 64] }
    }

    //============================================================
    // simple classification based on size order
    // can be optimized
    //============================================================
    pub fn calculate_class(size : u64) -> u32 {
        63 - (size - 16).leading_zeros()
    }

    //============================================================
    //
    //
    //============================================================
    pub fn push_node(&mut self, node: &Node) {
        unsafe
        {
            let class = Self::calculate_class(node.size());

            let head = self.orders[class as usize];     // *mut NodeHeader
            let node = node.buffer;                     // *mut NodeHeader

            if !head.is_null() {
                node.unbox().next     = head;
                head.unbox().previous = node;
            }

            self.orders[class as usize] = node;
        }
    }

    //============================================================
    //
    //
    //============================================================
    pub fn find_node(&mut self, size: u64) -> Option<Node> {
        unsafe {
            let class_min = Self::calculate_class(size) as usize;
            let class_add = self.orders.iter().skip(class_min).position(|p| !p.is_null());

            class_add.map(|class_add| self.pop_node_internal(class_min + class_add))
        }
    }

    //============================================================
    //
    //
    //============================================================
    unsafe fn pop_node_internal(&mut self, class: usize) -> Node {

        let head = self.orders[class as usize];
        let node = Node::from(head.cast::<u8>());

        if !head.unbox().next.is_null() {
            head.unbox().next.unbox().previous = ptr::null_mut();
        }

        self.orders[class as usize] = head.unbox().next;

        node.buffer.unbox().next = ptr::null_mut();
        node
    }

    //============================================================
    //
    //
    //============================================================
    pub unsafe fn remove_node(&mut self, node: &mut Node) {

        let class = Self::calculate_class(node.size());

        if node.buffer.unbox().previous.is_null() {
            self.pop_node_internal(class as usize);
            return;
        }

        if !node.buffer.unbox().next.is_null() {
            node.buffer.unbox().next.unbox().previous = node.buffer.unbox().previous;
        }

        node.buffer.unbox().previous.unbox().next = node.buffer.unbox().next;

        node.buffer.unbox().previous = ptr::null_mut();
        node.buffer.unbox().next = ptr::null_mut();
    }
}
