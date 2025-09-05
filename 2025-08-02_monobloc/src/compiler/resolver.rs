use std::collections::BTreeMap;

use crate::compiler::{
    ir::{Binding, Intrinsic, Type},
    nodes::{Node, NodeId, NodeKind},
};

pub struct Resolver {
    bindings: Vec<Binding>,

    binding_definitions_by_node: BTreeMap<NodeId, Vec<Binding>>,
    binding_calls_by_node: BTreeMap<NodeId, Binding>,
    intrinsics_by_node: BTreeMap<NodeId, Intrinsic>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            bindings: Vec::new(),

            binding_definitions_by_node: BTreeMap::new(),
            binding_calls_by_node: BTreeMap::new(),
            intrinsics_by_node: BTreeMap::new(),
        }
    }

    pub fn process_node(&mut self, node: &Node) {
        process_node(
            node,
            &mut self.bindings,
            &mut self.binding_definitions_by_node,
            &mut self.binding_calls_by_node,
            &mut self.intrinsics_by_node,
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
        self.intrinsics_by_node.get(node)
    }

    pub fn bindings_in_root(&self) -> &Vec<Binding> {
        &self.bindings
    }
}

fn process_node(
    node: &Node,
    bindings: &mut Vec<Binding>,
    binding_definitions_by_node: &mut BTreeMap<NodeId, Vec<Binding>>,
    binding_calls_by_node: &mut BTreeMap<NodeId, Binding>,
    intrinsics_by_node: &mut BTreeMap<NodeId, Intrinsic>,
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
                bindings.push(binding);
            }

            binding_definitions_by_node
                .insert(node.id, bindings_from_this_operator);
        }
        NodeKind::Identifier { name } => {
            if let Some(intrinsic) = resolve_intrinsic(name) {
                intrinsics_by_node.insert(node.id, intrinsic);
            }

            if let Some(binding) = bindings
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

fn resolve_intrinsic(name: &str) -> Option<Intrinsic> {
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
