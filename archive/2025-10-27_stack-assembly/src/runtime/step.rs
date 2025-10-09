use crate::{
    Effect,
    instructions::{Instruction, Instructions, Labels},
    runtime::{
        Operands,
        call_stack::{CallStack, CallStackUnderflow},
    },
};

pub fn step(
    instructions: &Instructions,
    labels: &Labels,
    operands: &mut Operands,
    current_instruction: &mut usize,
    call_stack: &mut CallStack,
) -> Result<StepOutcome, Effect> {
    let Some(instruction) = instructions.get(*current_instruction) else {
        return Ok(StepOutcome::Finished);
    };

    match instruction {
        Instruction::Call => {
            call_stack.push(*current_instruction);

            let address = operands.pop()?;

            let address = address.into_address()?;
            *current_instruction = address;

            return Ok(StepOutcome::Ready);
        }
        Instruction::CallIf => {
            call_stack.push(*current_instruction);

            let address = operands.pop()?;
            let condition = operands.pop()?;

            if condition.inner != 0 {
                let address = address.into_address()?;
                *current_instruction = address;

                return Ok(StepOutcome::Ready);
            }
        }
        Instruction::Drop0 => {
            operands.pop()?;
        }
        Instruction::PushValue { value } => {
            operands.push(*value);
        }
        Instruction::Reference { name } => {
            if let Some(&address) = labels.get(name) {
                operands.push(address);
            } else {
                return Err(Effect::InvalidReference);
            }
        }
        Instruction::Return => match call_stack.pop() {
            Ok(address) => {
                *current_instruction = address;
            }
            Err(CallStackUnderflow) => {
                return Ok(StepOutcome::Finished);
            }
        },
        Instruction::Trigger { effect } => {
            return Err(*effect);
        }
    }

    *current_instruction += 1;
    Ok(StepOutcome::Ready)
}

pub enum StepOutcome {
    Ready,
    Finished,
}
