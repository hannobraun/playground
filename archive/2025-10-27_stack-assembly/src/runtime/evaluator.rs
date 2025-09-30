use std::collections::BTreeMap;

use crate::{
    Effect,
    runtime::{Instruction, Operands, Operator},
};

pub struct Evaluator {
    call_stack: Vec<usize>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            call_stack: vec![0],
        }
    }

    pub fn step(
        &mut self,
        instructions: &[Instruction],
        labels: &BTreeMap<String, i32>,
        operands: &mut Operands,
    ) -> Result<StepOutcome, Effect> {
        let Some(current_instruction) = self.call_stack.last_mut() else {
            return Ok(StepOutcome::Finished);
        };
        let Some(instruction) = instructions.get(*current_instruction) else {
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
                    *current_instruction += 1;
                    self.call_stack.push(address);
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
                self.call_stack.pop();
                return Ok(StepOutcome::Ready);
            }
        }

        *current_instruction += 1;
        Ok(StepOutcome::Ready)
    }
}

pub enum StepOutcome {
    Ready,
    Finished,
}
