use crate::compiler::code::tokens::IntegerFormat;

pub struct Nodes {
    root: Block,
    next_id: NodeId,
}

impl Nodes {
    pub fn new() -> Self {
        Self {
            root: Block { nodes: Vec::new() },
            next_id: NodeId { inner: 1 }, // ID `0` is reserved for root block
        }
    }

    pub fn make_node(&mut self, kind: NodeKind) -> Node {
        let id = self.next_id;
        self.next_id.inner += 1;

        Node { id, kind }
    }

    pub fn add_to_root(&mut self, node: Node) {
        self.root.nodes.push(node);
    }

    pub fn root(&self) -> &Block {
        &self.root
    }

    pub fn into_root(self) -> Block {
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
    Block { block: Block },
    Comment { text: String },
    Identifier { name: String },
    Integer { value: u32, format: IntegerFormat },
}

#[derive(Debug, Default)]
pub struct Block {
    pub nodes: Vec<Node>,
}
