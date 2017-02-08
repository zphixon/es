
use xi_rope::Rope;
use binary_tree::count::CountTree;

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
pub struct EsBufferTree {
    layout: CountTree<LayoutNode>
}

#[derive(Debug)]
pub enum LayoutNode {
    Buf(EsBuffer),
    Split(Box<Split>),
}

#[derive(Debug)]
pub enum Split {
    Vertical(LayoutNode),
    Horizontal(LayoutNode)
}

#[derive(Debug)]
pub struct EsBuffer {
    content: String,
    filename: String
    // more state stuff later
}

impl EsBuffer {
    pub fn empty() -> EsBuffer {
        EsBuffer {
            //content: Rope::from(""),
            content: "".into(),
            filename: "".into()
        }
    }
}

impl EsBufferTree {
    pub fn new() -> EsBufferTree {
        //let mut b: BTreeMap<u64, LayoutNode> = BTreeMap::new();
        //b.insert(0, LayoutNode::Buf(EsBuffer::empty()));
        let mut b: CountTree<LayoutNode> = CountTree::new();
        b.push_front(LayoutNode::Buf(EsBuffer::empty()));
        EsBufferTree {
            layout: b
        }
    }
}

