use std::collections::BTreeMap;

use crate::compiler::code::{nodes::NodeId, types::Type};

pub struct Bindings {
    pub in_root: LocalBindings,
    pub by_block: BTreeMap<NodeId, Vec<Binding>>,

    pub definitions_by_node: BTreeMap<NodeId, Vec<Binding>>,
    pub calls_by_node: BTreeMap<NodeId, Binding>,
}

impl Bindings {
    pub fn new() -> Self {
        Self {
            in_root: LocalBindings::new(),
            by_block: BTreeMap::new(),

            definitions_by_node: BTreeMap::new(),
            calls_by_node: BTreeMap::new(),
        }
    }
}

pub struct LocalBindings {
    pub inner: Vec<Binding>,
}

impl LocalBindings {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, binding: Binding) {
        self.inner.push(binding);
    }

    pub fn inner(&self) -> &Vec<Binding> {
        &self.inner
    }
}

#[derive(Clone, Debug)]
pub struct Binding {
    pub name: String,
    pub index: u32,
    pub ty: Type,
}
