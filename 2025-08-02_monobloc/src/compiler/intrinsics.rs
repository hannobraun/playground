use std::collections::BTreeMap;

use crate::compiler::types::Type;

pub struct Resolver {
    pub intrinsics: BTreeMap<&'static str, (Intrinsic, [&'static [Type]; 2])>,
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
