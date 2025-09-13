use std::collections::BTreeMap;

use crate::compiler::{
    code::{
        bindings::{Bindings, LocalBindings},
        intrinsics::Intrinsics,
        nodes::{Node, NodeId, NodeKind},
        signatures::Signatures,
        stack::Stack,
    },
    ir::{Binding, Intrinsic, Type},
};

pub struct Resolver {
    bindings: Bindings,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            bindings: Bindings::new(),
        }
    }

    pub fn process_node(
        &mut self,
        node: &Node,
        stack: &Stack,
        intrinsics: &mut Intrinsics,
        _: &mut Signatures,
    ) {
        process_node(
            node,
            stack,
            &mut self.bindings.in_root,
            &mut self.bindings.by_block,
            &mut self.bindings.definitions_by_node,
            &mut self.bindings.calls_by_node,
            intrinsics,
        );
    }

    pub fn binding_call_at(&self, node: &NodeId) -> Option<&Binding> {
        self.bindings.calls_by_node.get(node)
    }

    pub fn binding_definitions_at(&self, node: &NodeId) -> &[Binding] {
        self.bindings
            .definitions_by_node
            .get(node)
            .map(|bindings| bindings.as_slice())
            .unwrap_or(&[])
    }

    pub fn bindings_in(&self, id: &NodeId) -> &Vec<Binding> {
        if *id == NodeId::root() {
            self.bindings.in_root.inner()
        } else {
            self.bindings
                .by_block
                .get(id)
                .expect("Bindings not available")
        }
    }
}

fn process_node(
    node: &Node,
    stack: &Stack,
    bindings_in_current_block: &mut LocalBindings,
    bindings_by_block: &mut BTreeMap<NodeId, Vec<Binding>>,
    binding_definitions_by_node: &mut BTreeMap<NodeId, Vec<Binding>>,
    binding_calls_by_node: &mut BTreeMap<NodeId, Binding>,
    intrinsics: &mut Intrinsics,
) {
    match &node.kind {
        NodeKind::Binding { names } => {
            let mut bindings_from_this_operator = Vec::new();

            for name in names.iter().rev() {
                let index =
                    bindings_from_this_operator.len().try_into().expect(
                        "More than `u32::MAX` bindings per scope are not \
                        supported.",
                    );
                let binding = Binding {
                    name: name.clone(),
                    index,
                    ty: Type::I32,
                };

                bindings_from_this_operator.push(binding.clone());
                bindings_in_current_block.push(binding);
            }

            binding_definitions_by_node
                .insert(node.id, bindings_from_this_operator);
        }
        NodeKind::Block { block } => {
            let mut bindings_in_this_block = LocalBindings::new();

            for node in &block.nodes {
                process_node(
                    node,
                    stack,
                    &mut bindings_in_this_block,
                    bindings_by_block,
                    binding_definitions_by_node,
                    binding_calls_by_node,
                    intrinsics,
                );
            }

            bindings_by_block.insert(node.id, bindings_in_this_block.inner);
        }
        NodeKind::Identifier { name } => {
            if let Some(intrinsic) = Intrinsic::resolve(name, stack) {
                intrinsics.insert(node.id, intrinsic);
            }

            if let Some(binding) = bindings_in_current_block
                .inner()
                .iter()
                .rev()
                .find(|binding| &binding.name == name)
                .cloned()
            {
                binding_calls_by_node.insert(node.id, binding);
            }
        }

        _ => {
            // Node is not relevant for the resolver.
        }
    }
}
