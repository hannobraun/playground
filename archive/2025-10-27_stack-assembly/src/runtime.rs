use std::collections::BTreeMap;

pub struct Evaluator {
    current_instruction: usize,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            current_instruction: 0,
        }
    }

    pub fn step(
        &mut self,
        instructions: &[Instruction],
        labels: &BTreeMap<String, i32>,
        operands: &mut Operands,
    ) -> Result<StepOutcome, Effect> {
        let Some(instruction) = instructions.get(self.current_instruction)
        else {
            return Ok(StepOutcome::Finished);
        };

        match instruction {
            Instruction::Operator {
                operator: Operator::Integer { value },
            } => {
                operands.push(*value);
            }
            Instruction::Operator {
                operator: Operator::Apply,
            } => {
                let address = operands.pop()?;
                if let Ok(address) = address.try_into() {
                    self.current_instruction = address;
                    return Ok(StepOutcome::Ready);
                } else {
                    return Err(Effect::InvalidInstructionAddress);
                }
            }
            Instruction::Operator {
                operator: Operator::Drop0,
            } => {
                operands.pop()?;
            }
            Instruction::Operator {
                operator: Operator::Unknown,
            } => {
                return Err(Effect::UnknownOperator);
            }
            Instruction::Reference { name } => {
                if let Some(&address) = labels.get(name) {
                    // So far, we don't track the actual addresses of
                    // functions. Let's push a placeholder for now.
                    operands.push(address);
                } else {
                    return Err(Effect::InvalidReference);
                }
            }
            Instruction::Return => {
                // So far, we don't support nested function applications, so any
                // return will just end the program.
                return Ok(StepOutcome::Finished);
            }
        }

        self.current_instruction += 1;
        Ok(StepOutcome::Ready)
    }
}

pub enum StepOutcome {
    Ready,
    Finished,
}

pub enum Instruction {
    Operator { operator: Operator },
    Reference { name: String },
    Return,
}

pub enum Operator {
    Integer { value: i32 },

    Apply,
    Drop0,

    Unknown,
}

pub struct Operands {
    inner: Vec<i32>,
}

impl Operands {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, value: i32) {
        self.inner.push(value);
    }

    pub fn pop(&mut self) -> Result<i32, StackUnderflow> {
        self.inner.pop().ok_or(StackUnderflow)
    }

    pub fn inner(&self) -> &Vec<i32> {
        &self.inner
    }
}

pub struct StackUnderflow;

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
