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
                operator: Operator::Drop0,
            } => {
                operands.pop()?;
            }
            Instruction::Operator {
                operator: Operator::Unknown,
            } => {
                return Err(Effect::UnknownOperator);
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
}

pub enum Operator {
    Integer { value: i32 },
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
