use crate::{
    Effect,
    instructions::{Instruction, Instructions, Labels},
    runtime::{
        Operands,
        call_stack::{CallStack, CallStackUnderflow},
    },
    value::Value,
};

pub fn step(
    instructions: &Instructions,
    labels: &Labels,
    operands: &mut Operands,
    memory: &mut [i32],
    current_instruction: &mut usize,
    call_stack: &mut CallStack,
) -> Result<StepOutcome, Effect> {
    let Some(instruction) = instructions.get(*current_instruction) else {
        return Ok(StepOutcome::Finished);
    };

    match instruction {
        Instruction::Add => {
            let b = operands.pop()?;
            let a = operands.pop()?;

            let Some(value) = i32::checked_add(a.inner, b.inner) else {
                return Err(Effect::IntegerOverflow);
            };

            operands.push(Value { inner: value });
        }
        Instruction::Divide => {
            let b = operands.pop()?;
            let a = operands.pop()?;

            if b.inner == 0 {
                return Err(Effect::DivisionByZero);
            }

            let Some(value) = i32::checked_div(a.inner, b.inner) else {
                return Err(Effect::IntegerOverflow);
            };

            operands.push(Value { inner: value });
        }
        Instruction::Drop { index } => {
            // This implementation is more complicated than what we could do, if
            // we added a `drop` method to `Operands`, based on the capabilities
            // of the underlying `Vec`.
            //
            // However, this new method would also need to handle stack
            // underflow correctly, leading to more complexity and more testing
            // effort, compared to just writing some stupid code here that uses
            // a limited set of primitives.

            let mut side_stack = Vec::new();

            for _ in 0..*index {
                let value = operands.pop()?;
                side_stack.push(value);
            }

            operands.pop()?;

            for value in side_stack.into_iter().rev() {
                operands.push(value);
            }
        }
        Instruction::Equal => {
            let b = operands.pop()?;
            let a = operands.pop()?;

            let value = if a.inner == b.inner { 1 } else { 0 };

            operands.push(Value { inner: value });
        }
        Instruction::Jump => {
            let address = operands.pop()?;

            let address = address.into_address()?;
            *current_instruction = address;

            return Ok(StepOutcome::Ready);
        }
        Instruction::JumpIf => {
            let address = operands.pop()?;
            let condition = operands.pop()?;

            if condition.inner != 0 {
                let address = address.into_address()?;
                *current_instruction = address;

                return Ok(StepOutcome::Ready);
            }
        }
        Instruction::Larger => {
            let b = operands.pop()?;
            let a = operands.pop()?;

            let value = if a.inner > b.inner { 1 } else { 0 };

            operands.push(Value { inner: value });
        }
        Instruction::Multiply => {
            let b = operands.pop()?;
            let a = operands.pop()?;

            let Some(value) = i32::checked_mul(a.inner, b.inner) else {
                return Err(Effect::IntegerOverflow);
            };

            operands.push(Value { inner: value });
        }
        Instruction::Pick { index } => {
            // The comment from the `Drop` implementation applies here too.

            let index = *index;

            let mut side_stack = Vec::new();

            for _ in 0..=index {
                let value = operands.pop()?;
                side_stack.push(value);
            }

            let value = side_stack[index];

            for value in side_stack.into_iter().rev() {
                operands.push(value);
            }

            operands.push(value);
        }
        Instruction::PushReturnAddress => {
            *current_instruction += 1;
            call_stack.push(*current_instruction);
            return Ok(StepOutcome::Ready);
        }
        Instruction::PushValue { value } => {
            operands.push(*value);
        }
        Instruction::Read => {
            let address = operands.pop()?;

            let address = address.into_address()?;
            let Some(value) = memory.get(address).copied() else {
                return Err(Effect::OutOfBoundsAddress);
            };

            operands.push(Value { inner: value });
        }
        Instruction::Reference { name } => {
            if let Some(&address) = labels.get(name) {
                operands.push(address);
            } else {
                return Err(Effect::InvalidReference);
            }
        }
        Instruction::Remainder => {
            let b = operands.pop()?;
            let a = operands.pop()?;

            if b.inner == 0 {
                return Err(Effect::DivisionByZero);
            }

            let Some(value) = i32::checked_rem(a.inner, b.inner) else {
                return Err(Effect::IntegerOverflow);
            };

            operands.push(Value { inner: value });
        }
        Instruction::Return => match call_stack.pop() {
            Ok(address) => {
                *current_instruction = address;
            }
            Err(CallStackUnderflow) => {
                return Ok(StepOutcome::Finished);
            }
        },
        Instruction::Roll { num_operands } => {
            // The comment from the `Drop` implementation applies here too.

            let mut side_stack = Vec::new();

            for _ in 1..*num_operands {
                let value = operands.pop()?;
                side_stack.push(value);
            }

            let value = operands.pop()?;

            for value in side_stack.into_iter().rev() {
                operands.push(value);
            }

            operands.push(value);
        }
        Instruction::Smaller => {
            let b = operands.pop()?;
            let a = operands.pop()?;

            let value = if a.inner < b.inner { 1 } else { 0 };

            operands.push(Value { inner: value });
        }
        Instruction::SmallerOrEqual => {
            let b = operands.pop()?;
            let a = operands.pop()?;

            let value = if a.inner <= b.inner { 1 } else { 0 };

            operands.push(Value { inner: value });
        }
        Instruction::Subtract => {
            let b = operands.pop()?;
            let a = operands.pop()?;

            let Some(value) = i32::checked_sub(a.inner, b.inner) else {
                return Err(Effect::IntegerOverflow);
            };

            operands.push(Value { inner: value });
        }
        Instruction::Trigger { effect } => {
            return Err(*effect);
        }
        Instruction::Write => {
            let address = operands.pop()?;
            let value = operands.pop()?;

            let address = address.into_address()?;
            let Some(slot) = memory.get_mut(address) else {
                return Err(Effect::OutOfBoundsAddress);
            };

            *slot = value.inner;
        }
    }

    *current_instruction += 1;
    Ok(StepOutcome::Ready)
}

pub enum StepOutcome {
    Ready,
    Finished,
}
