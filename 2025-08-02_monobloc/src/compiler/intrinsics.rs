use std::collections::BTreeMap;

use crate::compiler::{
    syntax::{SyntaxElement, SyntaxElementId, SyntaxElementKind},
    tokens::Token,
    types::Type,
};

pub struct Resolver {
    pub intrinsics: BTreeMap<SyntaxElementId, Intrinsic>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            intrinsics: BTreeMap::new(),
        }
    }

    pub fn process_syntax_element(&mut self, syntax_element: &SyntaxElement) {
        let SyntaxElementKind::UnprocessedToken {
            token: Token::Identifier { name },
        } = &syntax_element.kind
        else {
            return;
        };

        use self::Intrinsic::*;
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

        self.intrinsics.insert(syntax_element.id, intrinsic);
    }
}

#[derive(Clone, Copy)]
pub enum Intrinsic {
    // Panics
    Assert,
    Panic,

    // Literals
    Integer { value: i32 },

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
    pub fn signature(&self) -> [&[Type]; 2] {
        use Type::*;

        match self {
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
        }
    }
}
