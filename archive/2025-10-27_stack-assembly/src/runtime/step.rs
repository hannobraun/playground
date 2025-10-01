use std::collections::BTreeMap;

use crate::{
    Effect,
    runtime::{Instruction, Operands, Operator, call_stack::CallStack},
};

pub fn step(
    instructions: &[Instruction],
    labels: &BTreeMap<String, i32>,
    operands: &mut Operands,
    call_stack: &mut CallStack,
) -> Result<StepOutcome, Effect> {
    let Some(instruction) = call_stack
        .current_instruction()
        .and_then(|address| instructions.get(address))
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
            operator: Operator::Call,
        } => {
            let address = operands.pop()?;
            call_stack.push(address)?;
            return Ok(StepOutcome::Ready);
        }
        Instruction::Operator {
            operator: Operator::CallIf,
        } => {
            let address = operands.pop()?;
            let condition = operands.pop()?;

            if condition != 0 {
                call_stack.push(address)?;
                return Ok(StepOutcome::Ready);
            }
        }
        Instruction::Operator {
            operator: Operator::Drop0,
        } => {
            operands.pop()?;
        }
        Instruction::Operator {
            operator: Operator::Yield,
        } => {
            return Err(Effect::Yield);
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
            call_stack.pop();
            return Ok(StepOutcome::Ready);
        }
    }

    call_stack.advance();
    Ok(StepOutcome::Ready)
}

pub enum StepOutcome {
    Ready,
    Finished,
}
