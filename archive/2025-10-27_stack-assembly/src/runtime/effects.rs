use crate::runtime::{
    call_stack::InvalidInstructionAddress, operands::StackUnderflow,
};

/// An effect that may be triggered by a program
#[derive(Debug, Eq, PartialEq)]
pub enum Effect {
    /// # Tried to call a function based on an invalid address
    InvalidInstructionAddress,

    /// # Tried to evaluate an invalid reference
    InvalidReference,

    /// # Tried popping a value from empty operand stack
    StackUnderflow,

    /// # Tried to evaluate an unknown operator
    UnknownOperator,

    /// # The program has yielded control to the host
    Yield,
}

impl From<InvalidInstructionAddress> for Effect {
    fn from(InvalidInstructionAddress: InvalidInstructionAddress) -> Self {
        Self::InvalidInstructionAddress
    }
}

impl From<StackUnderflow> for Effect {
    fn from(StackUnderflow: StackUnderflow) -> Self {
        Self::StackUnderflow
    }
}
