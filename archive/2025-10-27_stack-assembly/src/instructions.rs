use std::collections::BTreeMap;

use crate::value::Value;

pub type Instructions = Vec<Instruction>;

#[derive(Debug)]
pub enum Instruction {
    Add,
    Drop { index: usize },
    Jump,
    JumpIf,
    Pick { index: usize },
    PushReturnAddress,
    PushValue { value: Value },
    Read,
    Reference { name: String },
    Return,
    Roll { num_operands: usize },
    Trigger { effect: Effect },
    Write,
}

/// An effect that may be triggered by a program
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Effect {
    /// # The program has aborted its run
    Abort,

    /// # Tried to use a negative value as a code or memory address
    InvalidAddress,

    /// # Tried to evaluate an invalid reference
    InvalidReference,

    /// # Tried to use address that is out of bounds
    OutOfBoundsAddress,

    /// # Tried popping a value from empty operand stack
    StackUnderflow,

    /// # Tried to evaluate an unknown operator
    UnknownOperator,

    /// # The program has yielded control to the host
    Yield,
}

pub type Labels = BTreeMap<String, Value>;
