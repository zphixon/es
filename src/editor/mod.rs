
use std::mem;

use buffer::*;
use layout::*;
use EsError;

#[derive(Debug)]
pub struct EsEditor {
    buffers: LayoutNode,
    current_buffer: u64
}

impl EsEditor {
    pub fn new() -> EsEditor {
        EsEditor {
            buffers: LayoutNode::Buf(EsBuffer::empty()),
            current_buffer: 0
        }
    }

    // b
    //
    //   v
    //  / \
    // b   b

    //   h
    //  / \
    // b   b
    //
    //     v
    //    / \
    //   h   b
    //  / \
    // b   b
    //pub fn vertical_split(&mut self) {
    //    self.buffers = LayoutNode::Split(Box::new(Split::Vertical(self.buffers, LayoutNode::Buf(EsBuffer::test()))));
    //}

    pub fn vertical_split(&mut self) {
        self.buffers = self.buffers.vertical_split();
        //self.buffers = LayoutNode::vertical_split(self.buffers);
        //mem::replace(&mut self.buffers, LayoutNode);
    }
}

