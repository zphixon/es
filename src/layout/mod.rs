
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

// use binary_tree_from_somewhere;
// type Layout = BinaryTree<LayoutNode>;

pub enum LayoutNode {
    Empty,
    HorizontalSplit,
    VerticalSplit,
    Buffer(Buffer)
}

//#[derive(Debug)]
//pub enum LayoutNode {
//    Empty,
//    Buf(i64),
//    Split(Box<Split>),
//}
//
//#[derive(Debug)]
//pub enum Split {
//    Vertical(LayoutNode, LayoutNode),
//    Horizontal(LayoutNode, LayoutNode)
//}
//
//impl LayoutNode {
//    pub fn vertical_split(self) -> LayoutNode {
//        let last = self.last_id();
//        LayoutNode::Split(Box::new(Split::Vertical(self, LayoutNode::Buf(last))))
//    }
//
//    pub fn horizontal_split(self) -> LayoutNode {
//        let last = self.last_id();
//        LayoutNode::Split(Box::new(Split::Horizontal(self, LayoutNode::Buf(last))))
//    }
//
//    pub fn last_id(&self) -> i64 {
//        3
//    }
//}

