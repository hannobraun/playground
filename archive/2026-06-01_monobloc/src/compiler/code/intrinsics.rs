use std::collections::BTreeMap;

use crate::compiler::code::{nodes::NodeId, stack::Stack, types::Type};

pub struct Intrinsics {
    by_node: BTreeMap<NodeId, Intrinsic>,
}

impl Intrinsics {
    pub fn new() -> Self {
        Self {
            by_node: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, id: NodeId, intrinsic: Intrinsic) {
        self.by_node.insert(id, intrinsic);
    }

    pub fn get(&self, id: &NodeId) -> Option<&Intrinsic> {
        self.by_node.get(id)
    }
}

#[derive(Clone, Copy)]
pub enum Intrinsic {
    // Apply
    Apply,

    // Panics
    Assert,
    Panic,

    // Literals
    Integer { value: u32 },

    // Comparisons
    Equals,
    GreaterThan,
    GreaterThanOrEquals,
    LessThan,
    LessThanOrEquals,
    Not,

    // Arithmetic
    Add,
    Divide,
    Multiply,
    Remainder,
    Subtract,

    // Bitwise operations
    And,
    CountOnes,
    LeadingZeros,
    Or,
    RotateLeft,
    RotateRight,
    ShiftLeft,
    ShiftRight,
    TrailingZeros,
    Xor,
}

impl Intrinsic {
    pub fn resolve(name: &str, _: &Stack) -> Option<Intrinsic> {
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

    pub fn signature(&self) -> Option<[&[Type]; 2]> {
        use Type::*;

        let signature: [&[Type]; 2] = match self {
            Self::Apply => {
                return None;
            }

            Self::Assert => [&[I32], &[]],
            Self::Panic => [&[], &[]],

            Self::Integer { .. } => [&[], &[I32]],

            Self::Equals => [&[I32, I32], &[I32]],
            Self::GreaterThan => [&[I32, I32], &[I32]],
            Self::GreaterThanOrEquals => [&[I32, I32], &[I32]],
            Self::LessThan => [&[I32, I32], &[I32]],
            Self::LessThanOrEquals => [&[I32, I32], &[I32]],
            Self::Not => [&[I32], &[I32]],

            Self::Add => [&[I32, I32], &[I32]],
            Self::Divide => [&[I32, I32], &[I32]],
            Self::Multiply => [&[I32, I32], &[I32]],
            Self::Remainder => [&[I32, I32] as &[_], &[I32]],
            Self::Subtract => [&[I32, I32], &[I32]],

            Self::And => [&[I32, I32], &[I32]],
            Self::CountOnes => [&[I32], &[I32]],
            Self::LeadingZeros => [&[I32], &[I32]],
            Self::Or => [&[I32, I32], &[I32]],
            Self::RotateLeft => [&[I32, I32], &[I32]],
            Self::RotateRight => [&[I32, I32], &[I32]],
            Self::ShiftLeft => [&[I32, I32], &[I32]],
            Self::ShiftRight => [&[I32, I32], &[I32]],
            Self::TrailingZeros => [&[I32], &[I32]],
            Self::Xor => [&[I32, I32], &[I32]],
        };

        Some(signature)
    }
}
