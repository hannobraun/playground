use crate::compiler::code::tokens::IntegerFormat;

pub struct Nodes {
    root: Vec<Node>,
}

impl Nodes {
    pub fn new() -> Self {
        Self { root: Vec::new() }
    }

    pub fn add_to_root(&mut self, node: Node) {
        self.root.push(node);
    }

    pub fn root(&self) -> &[Node] {
        &self.root
    }

    pub fn into_root(self) -> Vec<Node> {
        self.root
    }
}

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
