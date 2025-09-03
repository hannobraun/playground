use std::collections::BTreeMap;

use crate::compiler::{
    ir::Intrinsic,
    syntax::{Node, NodeId, NodeKind},
};

pub struct Resolver {
    pub intrinsics: BTreeMap<NodeId, Intrinsic>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            intrinsics: BTreeMap::new(),
        }
    }

    pub fn process_node(&mut self, node: &Node) {
        let NodeKind::Identifier { name } = &node.kind else {
            return;
        };

        use Intrinsic::*;
        let intrinsic = match name.as_str() {
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
                return;
            }
        };

        self.intrinsics.insert(node.id, intrinsic);
    }
}
