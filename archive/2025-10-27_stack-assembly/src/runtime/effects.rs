use crate::runtime::operands::StackUnderflow;

/// An effect that may be triggered by a program
#[derive(Debug, Eq, PartialEq)]
pub enum Effect {
    /// # Tried to apply a function based on an invalid address
    InvalidInstructionAddress,

    /// # Tried to evaluate an invalid reference
    InvalidReference,

    /// # Tried popping a value from empty operand stack
    StackUnderflow,

    /// # Tried to evaluate an unknown operator
    UnknownOperator,
}

impl From<StackUnderflow> for Effect {
    fn from(StackUnderflow: StackUnderflow) -> Self {
        Self::StackUnderflow
    }
}
