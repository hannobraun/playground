use std::collections::BTreeMap;

use crate::compiler::{code::nodes::NodeId, ir::Signature};

pub struct Signatures {
    by_block: BTreeMap<NodeId, Signature>,
}

impl Signatures {
    pub fn new() -> Self {
        Self {
            by_block: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, id: NodeId, signature: Signature) {
        self.by_block.insert(id, signature);
    }

    pub fn get(&self, id: &NodeId) -> &Signature {
        self.by_block.get(id).expect("Block not available")
    }
}
