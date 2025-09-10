use std::collections::BTreeMap;

use crate::compiler::{code::nodes::NodeId, ir::Signature};

pub struct Signatures {
    inner: BTreeMap<NodeId, Signature>,
}

impl Signatures {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, id: NodeId, signature: Signature) {
        self.inner.insert(id, signature);
    }

    pub fn get(&self, id: &NodeId) -> &Signature {
        self.inner.get(id).expect("Block not available")
    }
}
