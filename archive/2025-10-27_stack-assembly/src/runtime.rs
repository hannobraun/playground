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
        operands: &mut Vec<i32>,
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
                operands.pop();
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

/// An effect that may be triggered by a program
#[derive(Debug, Eq, PartialEq)]
pub enum Effect {
    /// # Tried to evaluate an unknown operator
    UnknownOperator,
}
