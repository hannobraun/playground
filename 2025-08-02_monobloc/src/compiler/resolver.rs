use std::collections::BTreeMap;

use crate::compiler::{
    ir::{Binding, Intrinsic, Type},
    nodes::{Node, NodeId, NodeKind},
};

pub struct Resolver {
    bindings: Vec<Binding>,

    binding_calls_by_node: BTreeMap<NodeId, Binding>,
    bindings_definitions_by_node: BTreeMap<NodeId, Vec<Binding>>,
    intrinsics_by_node: BTreeMap<NodeId, Intrinsic>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            bindings: Vec::new(),

            binding_calls_by_node: BTreeMap::new(),
            bindings_definitions_by_node: BTreeMap::new(),
            intrinsics_by_node: BTreeMap::new(),
        }
    }

    pub fn process_node(&mut self, node: &Node) {
        match &node.kind {
            NodeKind::Binding { names } => {
                let mut bindings = Vec::new();

                for name in names.iter().rev() {
                    let index = self.bindings.len().try_into().expect(
                        "More than `u32::MAX` bindings per scope are not \
                        supported.",
                    );
                    let binding = Binding {
                        name: name.clone(),
                        index,
                        ty: Type::I32,
                    };

                    bindings.push(binding.clone());
                    self.bindings.push(binding);
                }

                self.bindings_definitions_by_node.insert(node.id, bindings);
            }
            NodeKind::Identifier { name } => {
                if let Some(intrinsic) = resolve_intrinsic(name) {
                    self.intrinsics_by_node.insert(node.id, intrinsic);
                }

                if let Some(binding) = self
                    .bindings
                    .iter()
                    .rev()
                    .find(|binding| &binding.name == name)
                    .cloned()
                {
                    self.binding_calls_by_node.insert(node.id, binding);
                }
            }

            _ => {
                // Node is not relevant for the resolver.
            }
        }
    }

    pub fn binding_call_at(&self, node: &NodeId) -> Option<&Binding> {
        self.binding_calls_by_node.get(node)
    }

    pub fn binding_definitions_at(&self, node: &NodeId) -> &[Binding] {
        self.bindings_definitions_by_node
            .get(node)
            .map(|bindings| bindings.as_slice())
            .unwrap_or(&[])
    }

    pub fn intrinsic_at(&self, node: &NodeId) -> Option<&Intrinsic> {
        self.intrinsics_by_node.get(node)
    }

    pub fn bindings_for_root(&self) -> &Vec<Binding> {
        &self.bindings
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
