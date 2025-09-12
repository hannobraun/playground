use std::collections::BTreeMap;

use crate::compiler::code::{nodes::NodeId, types::Type};

pub struct Bindings {
    pub in_root: Vec<Binding>,
    pub by_block: BTreeMap<NodeId, Vec<Binding>>,

    pub definitions_by_node: BTreeMap<NodeId, Vec<Binding>>,
    pub calls_by_node: BTreeMap<NodeId, Binding>,
}

impl Bindings {
    pub fn new() -> Self {
        Self {
            in_root: Vec::new(),
            by_block: BTreeMap::new(),

            definitions_by_node: BTreeMap::new(),
            calls_by_node: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Binding {
    pub name: String,
    pub index: u32,
    pub ty: Type,
}
