use crate::compiler::code::tokens::IntegerFormat;

#[derive(Debug)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NodeId {
    pub inner: u64,
}

#[derive(Debug)]
pub enum NodeKind {
    Binding { names: Vec<String> },
    Block { nodes: Vec<Node> },
    Comment { text: String },
    Identifier { name: String },
    Integer { value: u32, format: IntegerFormat },
}
