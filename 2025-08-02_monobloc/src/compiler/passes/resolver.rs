use std::collections::BTreeMap;

use crate::compiler::{
    code::{
        intrinsics::Intrinsics,
        nodes::{Node, NodeId, NodeKind},
        signatures::Signatures,
        stack::Stack,
    },
    ir::{Binding, Intrinsic, Type},
};

pub struct Resolver {
    bindings_in_root: Vec<Binding>,
    bindings_by_block: BTreeMap<NodeId, Vec<Binding>>,

    binding_definitions_by_node: BTreeMap<NodeId, Vec<Binding>>,
    binding_calls_by_node: BTreeMap<NodeId, Binding>,
    intrinsics: Intrinsics,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            bindings_in_root: Vec::new(),
            bindings_by_block: BTreeMap::new(),

            binding_definitions_by_node: BTreeMap::new(),
            binding_calls_by_node: BTreeMap::new(),
            intrinsics: Intrinsics::new(),
        }
    }

    pub fn process_node(
        &mut self,
        node: &Node,
        stack: &Stack,
        _: &mut Signatures,
    ) {
        process_node(
            node,
            stack,
            &mut self.bindings_in_root,
            &mut self.bindings_by_block,
            &mut self.binding_definitions_by_node,
            &mut self.binding_calls_by_node,
            &mut self.intrinsics,
        );
    }

    pub fn binding_call_at(&self, node: &NodeId) -> Option<&Binding> {
        self.binding_calls_by_node.get(node)
    }

    pub fn binding_definitions_at(&self, node: &NodeId) -> &[Binding] {
        self.binding_definitions_by_node
            .get(node)
            .map(|bindings| bindings.as_slice())
            .unwrap_or(&[])
    }

    pub fn intrinsic_at(&self, node: &NodeId) -> Option<&Intrinsic> {
        self.intrinsics.get(node)
    }

    pub fn bindings_in(&self, node: &NodeId) -> &Vec<Binding> {
        self.bindings_by_block
            .get(node)
            .expect("Bindings not available")
    }

    pub fn bindings_in_root(&self) -> &Vec<Binding> {
        &self.bindings_in_root
    }
}

fn process_node(
    node: &Node,
    stack: &Stack,
    bindings_in_current_block: &mut Vec<Binding>,
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
            let mut bindings_in_this_block = Vec::new();

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

            bindings_by_block.insert(node.id, bindings_in_this_block);
        }
        NodeKind::Identifier { name } => {
            if let Some(intrinsic) = resolve_intrinsic(name, stack) {
                intrinsics.insert(node.id, intrinsic);
            }

            if let Some(binding) = bindings_in_current_block
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

fn resolve_intrinsic(name: &str, _: &Stack) -> Option<Intrinsic> {
    use Intrinsic::*;

    let intrinsic = match name {
        "%" => Remainder,
        "*" => Multiply,
        "+" => Add,
        "-" => Subtract,
        "/" => Divide,
        "<" => LessThan,
        "<=" => LessThanOrEquals,
        "=" => Equals,
        ">" => GreaterThan,
        ">=" => GreaterThanOrEquals,
        "and" => And,
        "apply" => Apply,
        "assert" => Assert,
        "count_ones" => CountOnes,
        "leading_zeros" => LeadingZeros,
        "not" => Not,
        "or" => Or,
        "rotate_left" => RotateLeft,
        "rotate_right" => RotateRight,
        "shift_left" => ShiftLeft,
        "shift_right" => ShiftRight,
        "trailing_zeros" => TrailingZeros,
        "xor" => Xor,

        _ => {
            return None;
        }
    };

    Some(intrinsic)
}
