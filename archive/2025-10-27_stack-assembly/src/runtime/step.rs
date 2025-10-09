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
    call_stack: &mut CallStack,
) -> Result<StepOutcome, Effect> {
    let Some(instruction) = call_stack
        .current_instruction()
        .and_then(|address| instructions.get(address))
    else {
        return Ok(StepOutcome::Finished);
    };

    match instruction {
        Instruction::Call => {
            let address = operands.pop()?;

            let address = address.into_address()?;
            call_stack.push(address);

            return Ok(StepOutcome::Ready);
        }
        Instruction::CallIf => {
            let address = operands.pop()?;
            let condition = operands.pop()?;

            if condition.inner != 0 {
                let address = address.into_address()?;
                call_stack.push(address);

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
            Ok(_) => {
                return Ok(StepOutcome::Ready);
            }
            Err(CallStackUnderflow) => {
                return Ok(StepOutcome::Ready);
            }
        },
        Instruction::Trigger { effect } => {
            return Err(*effect);
        }
    }

    call_stack.advance();
    Ok(StepOutcome::Ready)
}

pub enum StepOutcome {
    Ready,
    Finished,
}
