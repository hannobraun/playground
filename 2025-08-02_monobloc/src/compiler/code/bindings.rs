use std::collections::BTreeMap;

use crate::compiler::code::{nodes::NodeId, types::Type};

pub struct Bindings {
    pub bindings_in_root: Vec<Binding>,
    pub bindings_by_block: BTreeMap<NodeId, Vec<Binding>>,

    pub binding_definitions_by_node: BTreeMap<NodeId, Vec<Binding>>,
    pub binding_calls_by_node: BTreeMap<NodeId, Binding>,
}

#[derive(Clone, Debug)]
pub struct Binding {
    pub name: String,
    pub index: u32,
    pub ty: Type,
}
