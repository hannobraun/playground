use std::collections::BTreeMap;

use crate::compiler::{code::nodes::NodeId, ir::Signature};

pub struct Signatures {
    pub signatures_by_block: BTreeMap<NodeId, Signature>,
}

impl Signatures {
    pub fn new() -> Self {
        Self {
            signatures_by_block: BTreeMap::new(),
        }
    }
}
