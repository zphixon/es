
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
    Empty,
    Buf(Buffer),
    Split(Box<Split>),
}

#[derive(Debug)]
pub enum Split {
    Vertical(LayoutNode, LayoutNode),
    Horizontal(LayoutNode, LayoutNode)
}

impl LayoutNode {
    pub fn vertical_split(node: LayoutNode) -> LayoutNode {
        LayoutNode::Split(Box::new(Split::Vertical(node, LayoutNode::Buf(Buffer::empty()))))
    }

    pub fn horizontal_split(node: LayoutNode) -> LayoutNode {
        LayoutNode::Split(Box::new(Split::Horizontal(node, LayoutNode::Buf(Buffer::empty()))))
    }
}

