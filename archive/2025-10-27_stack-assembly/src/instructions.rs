use std::collections::BTreeMap;

use crate::value::Value;

pub type Instructions = Vec<Instruction>;

#[derive(Debug)]
pub enum Instruction {
    Drop0,
    Jump,
    JumpIf,
    PushReturnAddress,
    PushValue { value: Value },
    Reference { name: String },
    Return,
    Trigger { effect: Effect },
}

/// An effect that may be triggered by a program
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

pub type Labels = BTreeMap<String, Value>;
