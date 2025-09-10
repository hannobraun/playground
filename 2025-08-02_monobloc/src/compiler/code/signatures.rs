use std::collections::BTreeMap;

use crate::compiler::{code::nodes::NodeId, ir::Signature};

pub struct Signatures {
    inner: Vec<Signature>,
    by_block: BTreeMap<NodeId, usize>,
}

impl Signatures {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            by_block: BTreeMap::new(),
        }
    }

    pub fn insert_if_necessary_and_return_index(
        &mut self,
        signature: Signature,
    ) -> usize {
        if let Some((index, _)) = self
            .inner
            .iter_mut()
            .enumerate()
            .find(|(_, s)| **s == signature)
        {
            index
        } else {
            let index = self.inner.len();
            self.inner.push(signature);
            index
        }
    }

    pub fn insert_and_assign_to_block(
        &mut self,
        block: NodeId,
        signature: Signature,
    ) {
        let index = self.insert_if_necessary_and_return_index(signature);
        self.by_block.insert(block, index);
    }

    pub fn get_for_block(&self, id: &NodeId) -> &Signature {
        let index = self
            .by_block
            .get(id)
            .copied()
            .expect("Signature not available");

        &self.inner[index]
    }
}
