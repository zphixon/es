
use buffer::*;

//      buffer
// -----------------
// buffer |
// -------|  buffer
// buffer |
//
// is something like this
//
//      h
//     / \
//    v   b
//   / \
//  b   h
//     / \
//    b   b

#[derive(Debug)]
pub enum LayoutNode {
    Buf(EsBuffer),
    Split(Box<Split>),
}

#[derive(Debug)]
pub enum Split {
    Vertical(LayoutNode, LayoutNode),
    Horizontal(LayoutNode, LayoutNode)
}

impl LayoutNode {
    //pub fn vertical_split(layout: LayoutNode) -> LayoutNode {
    //    LayoutNode::Split(Box::new(Split::Vertical(layout, LayoutNode::Buf(EsBuffer::empty()))))
    //}
    pub fn vertical_split(self) -> LayoutNode {
        LayoutNode::Split(Box::new(Split::Vertical(self, LayoutNode::Buf(EsBuffer::empty()))))
    }
}

