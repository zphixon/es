
//use std::mem;

use buffer::*;
//use layout::*;

#[derive(Debug)]
pub struct Editor {
    //buffer_layout: LayoutNode,
    buffers: Vec<Buffer>,
    current_buffer: u64,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            //buffer_layout: LayoutNode::Buf(0),
            buffers: vec![Buffer::empty()],
            current_buffer: 0,
        }
    }

    pub fn current_buffer(&self) -> &Buffer {
        &self.buffers[self.current_buffer as usize]
    }

    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current_buffer as usize]
    }

    pub fn position(&self) -> (u64, u64) {
        self.current_buffer().position()
    }

    pub fn move_to(&mut self, x: u64, y: u64) {
        self.current_buffer_mut().move_to(x, y);
    }

    pub fn current_line(&self) -> &str {
       self.current_buffer().current_line()
    }

    //pub fn vertical_split(&mut self) {
    //    self.buffer_layout = mem::replace(&mut self.buffer_layout, LayoutNode::Empty).vertical_split();
    //    self.current_buffer = self.buffer_layout.last_id();
    //}

    //pub fn horizontal_split(&mut self) {
    //    self.buffer_layout = mem::replace(&mut self.buffer_layout, LayoutNode::Empty).horizontal_split()
    //    self.current_buffer = self.buffer_layout.last_id();
    //}
}

