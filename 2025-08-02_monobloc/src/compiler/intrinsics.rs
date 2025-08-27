use std::collections::BTreeMap;

use crate::compiler::{syntax::SyntaxElement, types::Type};

pub struct Resolver {
    pub intrinsics: BTreeMap<&'static str, (Intrinsic, [&'static [Type]; 2])>,
}

impl Resolver {
    pub fn new() -> Self {
        use self::{Intrinsic::*, Type::*};

        let mut intrinsics = BTreeMap::new();
        intrinsics.extend([
            ("%", (Remainder, [&[I32, I32] as &[_], &[I32]])),
            ("*", (Multiply, [&[I32, I32], &[I32]])),
            ("+", (Add, [&[I32, I32], &[I32]])),
            ("-", (Subtract, [&[I32, I32], &[I32]])),
            ("/", (Divide, [&[I32, I32], &[I32]])),
            ("<", (LessThan, [&[I32, I32], &[I32]])),
            ("<=", (LessThanOrEquals, [&[I32, I32], &[I32]])),
            ("=", (Equals, [&[I32, I32], &[I32]])),
            (">", (GreaterThan, [&[I32, I32], &[I32]])),
            (">=", (GreaterThanOrEquals, [&[I32, I32], &[I32]])),
            ("and", (And, [&[I32, I32], &[I32]])),
            ("assert", (Assert, [&[I32], &[]])),
            ("count_ones", (CountOnes, [&[I32], &[I32]])),
            ("leading_zeros", (LeadingZeros, [&[I32], &[I32]])),
            ("not", (Not, [&[I32], &[I32]])),
            ("or", (Or, [&[I32, I32], &[I32]])),
            ("rotate_left", (RotateLeft, [&[I32, I32], &[I32]])),
            ("rotate_right", (RotateRight, [&[I32, I32], &[I32]])),
            ("shift_left", (ShiftLeft, [&[I32, I32], &[I32]])),
            ("shift_right", (ShiftRight, [&[I32, I32], &[I32]])),
            ("trailing_zeros", (TrailingZeros, [&[I32], &[I32]])),
            ("xor", (Xor, [&[I32, I32], &[I32]])),
        ]);

        Self { intrinsics }
    }

    pub fn process_syntax_element(&mut self, syntax_element: &SyntaxElement) {
        let _ = syntax_element;
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
