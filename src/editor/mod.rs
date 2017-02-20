
use std::mem;

use buffer::*;
use layout::*;

#[derive(Debug)]
pub struct Editor {
    buffers: LayoutNode,
    current_buffer: u64
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            buffers: LayoutNode::Buf(Buffer::empty()),
            current_buffer: 0
        }
    }

    pub fn vertical_split(&mut self) {
        self.buffers = LayoutNode::vertical_split(mem::replace(&mut self.buffers, LayoutNode::Empty));
    }

    pub fn horizontal_split(&mut self) {
        self.buffers = LayoutNode::horizontal_split(mem::replace(&mut self.buffers, LayoutNode::Empty));
    }
}

