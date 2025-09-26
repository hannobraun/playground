pub enum Instruction {
    Operator { operator: Operator },
}

pub enum Operator {
    Integer { value: i32 },
    Unknown,
}

/// An effect that may be triggered by a program
#[derive(Debug, Eq, PartialEq)]
pub enum Effect {
    /// # Tried to evaluate an unknown operator
    UnknownOperator,
}
