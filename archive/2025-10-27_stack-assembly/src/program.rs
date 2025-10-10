use crate::{
    Effect,
    compiler::compile,
    instructions::{Instructions, Labels},
    runtime::{CallStack, Operands, StepOutcome, step},
};

/// # A StackAssembly program
pub struct Program {
    instructions: Instructions,
    labels: Labels,
    operands: Operands,
    memory: [i32; 1024],
    current_instruction: usize,
    call_stack: CallStack,
    effect: Option<Effect>,
}

impl Program {
    /// # Create a `Program` instance by compiling the provided code
    pub fn compile(input: &str) -> Self {
        let (instructions, labels) = compile(input);

        Self {
            instructions,
            labels,
            operands: Operands::new(),
            memory: [0; 1024],
            current_instruction: 0,
            call_stack: CallStack::new(),
            effect: None,
        }
    }

    /// # Call [`Program::compile`], then [`Program::continue_`]
    pub fn compile_and_run(input: &str) -> Self {
        let mut program = Self::compile(input);
        program.continue_();

        program
    }

    /// # Access the operand stack
    pub fn operands(&mut self) -> &mut Vec<i32> {
        self.operands.inner()
    }

    /// # Access the memory
    pub fn memory(&mut self) -> &mut [i32] {
        &mut self.memory
    }

    /// # Access the call stack
    pub fn call_stack(&self) -> &Vec<usize> {
        self.call_stack.inner()
    }

    /// # Access the currently triggered effect
    pub fn effect(&self) -> Option<&Effect> {
        self.effect.as_ref()
    }

    /// # Continue the program until it finishes or triggers an effect
    pub fn continue_(&mut self) {
        // If an effect had been triggered before, continuing the program clears
        // it.
        if self.effect.take().is_some() {
            // To continue, we need to advance beyond the instruction that
            // triggered the effect.
            self.current_instruction += 1;
        }

        loop {
            match step(
                &self.instructions,
                &self.labels,
                &mut self.operands,
                &mut self.memory,
                &mut self.current_instruction,
                &mut self.call_stack,
            ) {
                Ok(StepOutcome::Ready) => {
                    continue;
                }
                Ok(StepOutcome::Finished) => {
                    break;
                }
                Err(effect) => {
                    self.effect = Some(effect);
                    break;
                }
            }
        }
    }
}
