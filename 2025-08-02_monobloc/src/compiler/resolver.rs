use std::collections::BTreeMap;

use crate::compiler::{
    ir::Intrinsic,
    syntax::{Node, NodeId, NodeKind},
};

pub struct Resolver {
    intrinsics_by_node: BTreeMap<NodeId, Intrinsic>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            intrinsics_by_node: BTreeMap::new(),
        }
    }

    pub fn process_node(&mut self, node: &Node) {
        match &node.kind {
            NodeKind::Identifier { name } => {
                if let Some(intrinsic) = resolve_intrinsic(name) {
                    self.intrinsics_by_node.insert(node.id, intrinsic);
                }
            }
            _ => {
                // Node is not relevant for the resolver.
            }
        }
    }

    pub fn intrinsic_at(&self, node: &NodeId) -> Option<&Intrinsic> {
        self.intrinsics_by_node.get(node)
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
